use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};
use tauri::{AppHandle, Manager};

const INSTALLED_ENGINE: &str = ".codex/codex-dream-skin-studio";
const STATE_THEME: &str = "Library/Application Support/CodexDreamSkinStudio/theme/theme.json";
const THEME_GALLERY_URL: &str = "https://codexthemes.app/?utm_source=codex_themes_desktop&utm_medium=desktop_app&utm_campaign=theme_gallery";
const MAX_THEME_FILE_SIZE: u64 = 16 * 1024 * 1024;
const MAX_THEME_TOTAL_SIZE: u64 = 32 * 1024 * 1024;
const THEME_CREATION_GUIDE: &str =
    include_str!("../../resources/guides/codex-theme-creation-guide.md");

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedTheme {
    schema_version: u64,
    id: String,
    name: String,
    author: String,
    version: String,
    description: String,
    appearance: String,
    art: Value,
    origin: String,
    category: String,
    preview_path: String,
    installed: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeLibraryResult {
    themes: Vec<ManagedTheme>,
    message: String,
    imported_theme_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeSnapshot {
    status: String,
    active_theme_id: Option<String>,
    message: String,
    is_native_host: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationResult {
    ok: bool,
    verified: bool,
    status: String,
    message: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ScriptStatus {
    session: String,
    #[serde(default)]
    operation: String,
    #[serde(default)]
    operation_message: String,
    #[serde(default)]
    injector_alive: bool,
    #[serde(default)]
    cdp_ok: bool,
    #[serde(default)]
    codex_running: bool,
    #[serde(default)]
    theme_name: String,
    #[serde(default)]
    applied_theme_name: String,
}

fn home_dir() -> Result<PathBuf, String> {
    env::var_os("HOME")
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .ok_or_else(|| "Could not resolve the current user home directory.".to_string())
}

fn installed_engine() -> Result<PathBuf, String> {
    Ok(home_dir()?.join(INSTALLED_ENGINE))
}

fn bundled_engine(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .resource_dir()
        .map(|path| path.join("macos-engine"))
        .map_err(|error| format!("Could not resolve bundled runtime resources: {error}"))
}

fn development_engine() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../macos")
}

fn engine_with_script(app: &AppHandle, script: &str) -> Result<PathBuf, String> {
    let installed = installed_engine()?;
    let bundled = bundled_engine(app)?;
    let development = development_engine();
    // Prefer the application-bundled runtime so an older user-installed engine
    // cannot shadow security or compatibility fixes shipped with the GUI.
    [bundled, development, installed]
        .into_iter()
        .find(|root| root.join("scripts").join(script).is_file())
        .ok_or_else(|| {
            format!("The managed macOS CDP runtime is missing required script: {script}")
        })
}

fn themes_root() -> Result<PathBuf, String> {
    Ok(home_dir()?.join("Library/Application Support/CodexDreamSkinStudio/themes"))
}

fn state_root() -> Result<PathBuf, String> {
    Ok(home_dir()?.join("Library/Application Support/CodexDreamSkinStudio"))
}

fn initialize_directories() -> Result<(), String> {
    let root = state_root()?;
    for child in ["themes", "theme", "cache", "logs", "state", "backups"] {
        let path = root.join(child);
        fs::create_dir_all(&path)
            .map_err(|error| format!("Could not create {}: {error}", path.display()))?;
        #[cfg(unix)]
        fs::set_permissions(&path, fs::Permissions::from_mode(0o700))
            .map_err(|error| format!("Could not protect {}: {error}", path.display()))?;
    }
    Ok(())
}

fn bundled_preset_ids(root: &Path) -> Result<Vec<String>, String> {
    let presets = root.join("presets");
    let mut ids = Vec::new();
    for entry in fs::read_dir(&presets)
        .map_err(|error| format!("Could not inspect bundled presets: {error}"))?
    {
        let entry = entry.map_err(|error| format!("Could not inspect bundled preset: {error}"))?;
        let id = entry.file_name().to_string_lossy().to_string();
        if entry.path().is_dir()
            && id.starts_with("preset-")
            && entry.path().join("theme.json").is_file()
        {
            ids.push(id);
        }
    }
    ids.sort();
    Ok(ids)
}

fn manifest_string(value: &Value, key: &str, fallback: &str) -> String {
    value
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or(fallback)
        .to_string()
}

fn read_managed_theme(path: &Path, built_in_ids: &[String]) -> Result<ManagedTheme, String> {
    let manifest_path = path.join("theme.json");
    let bytes = fs::read(&manifest_path)
        .map_err(|error| format!("Could not read {}: {error}", manifest_path.display()))?;
    if bytes.len() as u64 > 256 * 1024 {
        return Err("theme.json is too large.".to_string());
    }
    let manifest: Value = serde_json::from_slice(&bytes)
        .map_err(|error| format!("Invalid theme.json in {}: {error}", path.display()))?;
    if manifest.get("schemaVersion").and_then(Value::as_u64) != Some(1) {
        return Err("Only theme schemaVersion 1 is supported.".to_string());
    }
    let id = manifest_string(&manifest, "id", "");
    if !valid_theme_id(&id) || path.file_name().and_then(|name| name.to_str()) != Some(&id) {
        return Err("Theme id must match its managed directory name.".to_string());
    }
    let image = manifest_string(&manifest, "image", "");
    if image.is_empty()
        || Path::new(&image).file_name().and_then(|name| name.to_str()) != Some(&image)
    {
        return Err("Theme image must be a file inside the theme directory.".to_string());
    }
    let image_path = path.join(&image);
    if !image_path.is_file() {
        return Err(format!("Theme image is missing: {image}"));
    }
    let preview = manifest
        .get("preview")
        .and_then(Value::as_str)
        .unwrap_or("preview.jpg");
    let preview_path = if Path::new(preview)
        .file_name()
        .and_then(|name| name.to_str())
        == Some(preview)
        && path.join(preview).is_file()
    {
        path.join(preview)
    } else {
        image_path
    };
    let art = manifest.get("art").cloned().unwrap_or_else(|| {
        serde_json::json!({
            "focusX": 0.5, "focusY": 0.5, "safeArea": "auto", "taskMode": "auto"
        })
    });
    Ok(ManagedTheme {
        schema_version: 1,
        id: id.clone(),
        name: manifest_string(&manifest, "name", &id),
        author: manifest_string(&manifest, "author", "Codex Themes"),
        version: manifest_string(&manifest, "version", "1.0.0"),
        description: manifest
            .get("description")
            .and_then(Value::as_str)
            .or_else(|| manifest.get("tagline").and_then(Value::as_str))
            .unwrap_or("A locally managed Codex theme.")
            .to_string(),
        appearance: manifest_string(&manifest, "appearance", "auto"),
        art,
        origin: if built_in_ids.contains(&id) {
            "built-in"
        } else {
            "imported"
        }
        .to_string(),
        category: "Featured".to_string(),
        preview_path: preview_path.to_string_lossy().to_string(),
        installed: true,
    })
}

fn scan_theme_library(root: &Path) -> Result<Vec<ManagedTheme>, String> {
    let built_in_ids = bundled_preset_ids(root)?;
    let library = themes_root()?;
    let mut themes = Vec::new();
    for entry in fs::read_dir(&library)
        .map_err(|error| format!("Could not scan the managed theme library: {error}"))?
    {
        let entry = entry.map_err(|error| format!("Could not inspect a managed theme: {error}"))?;
        let name = entry.file_name().to_string_lossy().to_string();
        if !entry.path().is_dir() || name.starts_with('.') {
            continue;
        }
        if let Ok(theme) = read_managed_theme(&entry.path(), &built_in_ids) {
            themes.push(theme);
        }
    }
    themes.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(themes)
}

fn initialize_library(app: &AppHandle) -> Result<ThemeLibraryResult, String> {
    initialize_directories()?;
    let root = engine_with_script(app, "switch-theme-macos.sh")?;
    for id in bundled_preset_ids(&root)? {
        seed_bundled_preset_to(&root, &themes_root()?, &id)?;
    }
    let themes = scan_theme_library(&root)?;
    Ok(ThemeLibraryResult {
        message: format!("{} themes are ready.", themes.len()),
        themes,
        imported_theme_id: None,
    })
}

fn import_theme_from(app: &AppHandle, source: &Path) -> Result<ThemeLibraryResult, String> {
    initialize_directories()?;
    let source = source
        .canonicalize()
        .map_err(|error| format!("Could not open the selected theme folder: {error}"))?;
    if !source.is_dir() {
        return Err("Select an extracted theme folder, not an individual file.".to_string());
    }
    let manifest_path = source.join("theme.json");
    let manifest_metadata = fs::symlink_metadata(&manifest_path)
        .map_err(|_| "The selected folder does not contain theme.json.".to_string())?;
    if !manifest_metadata.file_type().is_file() || manifest_metadata.len() > 256 * 1024 {
        return Err("theme.json must be a regular file no larger than 256 KB.".to_string());
    }
    let manifest: Value = serde_json::from_slice(
        &fs::read(&manifest_path).map_err(|error| format!("Could not read theme.json: {error}"))?,
    )
    .map_err(|error| format!("theme.json is not valid JSON: {error}"))?;
    if manifest.get("schemaVersion").and_then(Value::as_u64) != Some(1) {
        return Err("Only theme schemaVersion 1 is supported.".to_string());
    }
    let id = manifest_string(&manifest, "id", "");
    if !valid_theme_id(&id) {
        return Err(
            "Theme id may contain letters, numbers, hyphens, and underscores only.".to_string(),
        );
    }
    let image = manifest_string(&manifest, "image", "");
    let image_extension = Path::new(&image)
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if Path::new(&image).file_name().and_then(|name| name.to_str()) != Some(&image)
        || !matches!(image_extension.as_str(), "jpg" | "jpeg" | "png" | "webp")
    {
        return Err("Theme image must be a local JPG, PNG, or WebP filename.".to_string());
    }
    let allowed_preview = manifest
        .get("preview")
        .and_then(Value::as_str)
        .unwrap_or("preview.jpg");
    let mut total_size = 0u64;
    let mut files = Vec::new();
    for entry in fs::read_dir(&source)
        .map_err(|error| format!("Could not inspect the selected folder: {error}"))?
    {
        let entry = entry.map_err(|error| format!("Could not inspect a theme file: {error}"))?;
        let file_type = entry
            .file_type()
            .map_err(|error| format!("Could not inspect a theme file: {error}"))?;
        if !file_type.is_file() {
            return Err("Theme folders may contain regular files only; folders and symbolic links are not allowed.".to_string());
        }
        let name = entry.file_name().to_string_lossy().to_string();
        let allowed =
            name == "theme.json" || name == image || name == allowed_preview || name == "README.md";
        if !allowed {
            return Err(format!("Unsupported file in theme folder: {name}"));
        }
        let size = entry
            .metadata()
            .map_err(|error| format!("Could not inspect {name}: {error}"))?
            .len();
        if size == 0 || size > MAX_THEME_FILE_SIZE {
            return Err(format!("{name} is empty or exceeds the 16 MB file limit."));
        }
        total_size = total_size.saturating_add(size);
        files.push((entry.path(), name));
    }
    if total_size > MAX_THEME_TOTAL_SIZE {
        return Err("The extracted theme exceeds the 32 MB package limit.".to_string());
    }
    if !source.join(&image).is_file() {
        return Err(format!("The referenced theme image is missing: {image}"));
    }
    let destination_root = themes_root()?;
    let destination = destination_root.join(&id);
    if destination.exists() {
        return Err(format!("A theme with id “{id}” is already installed."));
    }
    let temporary = destination_root.join(format!(".{id}.installing-{}", std::process::id()));
    if temporary.exists() {
        fs::remove_dir_all(&temporary)
            .map_err(|error| format!("Could not clear a stale import: {error}"))?;
    }
    fs::create_dir(&temporary).map_err(|error| format!("Could not stage the theme: {error}"))?;
    #[cfg(unix)]
    fs::set_permissions(&temporary, fs::Permissions::from_mode(0o700))
        .map_err(|error| format!("Could not protect the staged theme: {error}"))?;
    let result = (|| {
        for (path, name) in files {
            let target = temporary.join(name);
            fs::copy(path, &target)
                .map_err(|error| format!("Could not copy a theme file: {error}"))?;
            #[cfg(unix)]
            fs::set_permissions(&target, fs::Permissions::from_mode(0o600))
                .map_err(|error| format!("Could not protect a theme file: {error}"))?;
        }
        fs::rename(&temporary, &destination)
            .map_err(|error| format!("Could not publish the imported theme: {error}"))?;
        Ok::<(), String>(())
    })();
    if result.is_err() {
        let _ = fs::remove_dir_all(&temporary);
    }
    result?;
    let root = engine_with_script(app, "switch-theme-macos.sh")?;
    let themes = scan_theme_library(&root)?;
    Ok(ThemeLibraryResult {
        themes,
        message: format!("Imported {}.", manifest_string(&manifest, "name", &id)),
        imported_theme_id: Some(id),
    })
}

fn seed_bundled_preset(root: &Path, theme_id: &str) -> Result<(), String> {
    seed_bundled_preset_to(root, &themes_root()?, theme_id)
}

fn seed_bundled_preset_to(
    root: &Path,
    destination_root: &Path,
    theme_id: &str,
) -> Result<(), String> {
    if !theme_id.starts_with("preset-") {
        return Ok(());
    }
    let destination = destination_root.join(theme_id);
    if destination.join("theme.json").is_file() {
        return Ok(());
    }
    let source = root.join("presets").join(theme_id);
    if !source.join("theme.json").is_file() {
        return Err(format!("Bundled preset is missing: {theme_id}"));
    }
    fs::create_dir_all(destination_root)
        .map_err(|error| format!("Could not create the managed theme library: {error}"))?;
    let temporary = destination_root.join(format!(".{theme_id}.installing-{}", std::process::id()));
    if temporary.exists() {
        fs::remove_dir_all(&temporary).map_err(|error| {
            format!("Could not clear a stale preset staging directory: {error}")
        })?;
    }
    fs::create_dir(&temporary)
        .map_err(|error| format!("Could not stage bundled preset: {error}"))?;
    #[cfg(unix)]
    fs::set_permissions(&temporary, fs::Permissions::from_mode(0o700))
        .map_err(|error| format!("Could not protect preset staging directory: {error}"))?;

    let result = (|| {
        let mut copied = 0usize;
        for entry in fs::read_dir(&source)
            .map_err(|error| format!("Could not read bundled preset: {error}"))?
        {
            let entry =
                entry.map_err(|error| format!("Could not inspect bundled preset: {error}"))?;
            let file_type = entry
                .file_type()
                .map_err(|error| format!("Could not inspect bundled preset file type: {error}"))?;
            if !file_type.is_file() {
                return Err("Bundled presets may contain regular files only.".to_string());
            }
            let metadata = entry
                .metadata()
                .map_err(|error| format!("Could not inspect bundled preset file: {error}"))?;
            if metadata.len() == 0 || metadata.len() > 16 * 1024 * 1024 {
                return Err("Bundled preset contains an empty or oversized file.".to_string());
            }
            let target = temporary.join(entry.file_name());
            fs::copy(entry.path(), &target)
                .map_err(|error| format!("Could not copy bundled preset file: {error}"))?;
            #[cfg(unix)]
            fs::set_permissions(&target, fs::Permissions::from_mode(0o600))
                .map_err(|error| format!("Could not protect bundled preset file: {error}"))?;
            copied += 1;
        }
        if copied < 2 || !temporary.join("theme.json").is_file() {
            return Err("Bundled preset is incomplete.".to_string());
        }
        fs::rename(&temporary, &destination)
            .map_err(|error| format!("Could not publish bundled preset: {error}"))?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_dir_all(&temporary);
    }
    result
}

fn run_script(root: &Path, script: &str, args: &[&str]) -> Result<Output, String> {
    let script_path = root.join("scripts").join(script);
    if !script_path.is_file() {
        return Err(format!(
            "Required runtime script is missing: {}",
            script_path.display()
        ));
    }
    Command::new("/bin/bash")
        .arg(&script_path)
        .args(args)
        .env(
            "PATH",
            "/usr/bin:/bin:/usr/sbin:/sbin:/usr/local/bin:/opt/homebrew/bin",
        )
        .output()
        .map_err(|error| format!("Could not run {script}: {error}"))
}

fn output_error(script: &str, output: &Output) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let detail = if stderr.is_empty() { stdout } else { stderr };
    if detail.is_empty() {
        format!("{script} exited with status {}", output.status)
    } else {
        detail.chars().take(1200).collect()
    }
}

fn active_theme_id() -> Option<String> {
    let path = home_dir().ok()?.join(STATE_THEME);
    let value: Value = serde_json::from_slice(&std::fs::read(path).ok()?).ok()?;
    value.get("id")?.as_str().map(str::to_string)
}

fn status_from_script(app: &AppHandle, deep: bool) -> Result<RuntimeSnapshot, String> {
    let root = engine_with_script(app, "status-dream-skin-macos.sh")?;
    let args = if deep {
        vec!["--json", "--deep"]
    } else {
        vec!["--json"]
    };
    let output = run_script(&root, "status-dream-skin-macos.sh", &args)?;
    if !output.status.success() {
        return Err(output_error("status-dream-skin-macos.sh", &output));
    }
    let raw = String::from_utf8(output.stdout)
        .map_err(|_| "The runtime status response was not valid UTF-8.".to_string())?;
    let script: ScriptStatus = serde_json::from_str(raw.trim())
        .map_err(|error| format!("The runtime returned invalid status JSON: {error}"))?;

    let (status, message) = match script.operation.as_str() {
        "applying" => (
            "applying",
            if script.operation_message.is_empty() {
                "Applying theme"
            } else {
                &script.operation_message
            },
        ),
        "pausing" => (
            "restoring",
            if script.operation_message.is_empty() {
                "Restoring original appearance"
            } else {
                &script.operation_message
            },
        ),
        "failed" => (
            "error",
            if script.operation_message.is_empty() {
                "The last theme operation failed"
            } else {
                &script.operation_message
            },
        ),
        _ if script.session == "active" && script.injector_alive => {
            let verified = if deep { script.cdp_ok } else { true };
            if verified {
                ("active", "Theme active")
            } else {
                ("error", "The saved CDP endpoint could not be verified")
            }
        }
        _ if script.session == "applying" => ("applying", "Applying theme"),
        _ if script.session == "stale" || script.session == "unknown" => {
            ("error", "Theme runtime requires attention")
        }
        _ if script.codex_running => (
            "restart-required",
            "Codex is running without an active theme session",
        ),
        _ => ("connected", "Codex theme runtime is ready"),
    };

    let display_name = if script.applied_theme_name.is_empty() {
        &script.theme_name
    } else {
        &script.applied_theme_name
    };
    let message = if status == "active" && !display_name.is_empty() {
        format!("Active: {display_name}")
    } else {
        message.to_string()
    };

    Ok(RuntimeSnapshot {
        status: status.to_string(),
        active_theme_id: active_theme_id(),
        message,
        is_native_host: true,
    })
}

fn valid_theme_id(theme_id: &str) -> bool {
    theme_id.len() >= 2
        && theme_id.len() <= 64
        && theme_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
}

fn ensure_installed_runtime(app: &AppHandle) -> Result<PathBuf, String> {
    let installed = installed_engine()?;
    if installed.join("scripts/switch-theme-macos.sh").is_file() {
        return engine_with_script(app, "switch-theme-macos.sh");
    }
    let source = engine_with_script(app, "install-dream-skin-macos.sh")?;
    let output = run_script(
        &source,
        "install-dream-skin-macos.sh",
        &["--no-launch", "--no-launchers"],
    )?;
    if !output.status.success() {
        return Err(output_error("install-dream-skin-macos.sh", &output));
    }
    if installed.join("scripts/switch-theme-macos.sh").is_file() {
        engine_with_script(app, "switch-theme-macos.sh")
    } else {
        Err(
            "The macOS theme runtime installer completed without creating the managed engine."
                .to_string(),
        )
    }
}

#[tauri::command]
pub async fn initialize_theme_library(app: AppHandle) -> Result<ThemeLibraryResult, String> {
    tauri::async_runtime::spawn_blocking(move || initialize_library(&app))
        .await
        .map_err(|error| format!("Theme library initialization failed: {error}"))?
}

#[tauri::command]
pub async fn import_theme_folder(app: AppHandle) -> Result<ThemeLibraryResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let selected = rfd::FileDialog::new()
            .set_title("Choose an extracted Codex theme folder")
            .pick_folder();
        match selected {
            Some(path) => import_theme_from(&app, &path),
            None => {
                let mut result = initialize_library(&app)?;
                result.message = "Import cancelled.".to_string();
                Ok(result)
            }
        }
    })
    .await
    .map_err(|error| format!("Theme import task failed: {error}"))?
}

#[tauri::command]
pub async fn open_themes_folder() -> Result<OperationResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        initialize_directories()?;
        let folder = themes_root()?;
        let output = Command::new("/usr/bin/open")
            .arg(&folder)
            .output()
            .map_err(|error| format!("Could not open the theme folder: {error}"))?;
        let ok = output.status.success();
        Ok(OperationResult {
            ok,
            verified: ok,
            status: if ok { "connected" } else { "error" }.to_string(),
            message: if ok {
                "Theme folder opened in Finder.".to_string()
            } else {
                output_error("open", &output)
            },
        })
    })
    .await
    .map_err(|error| format!("Open theme folder task failed: {error}"))?
}

#[tauri::command]
pub async fn delete_theme(app: AppHandle, theme_id: String) -> Result<ThemeLibraryResult, String> {
    if !valid_theme_id(&theme_id) {
        return Err("Theme id contains unsupported characters.".to_string());
    }
    tauri::async_runtime::spawn_blocking(move || {
        initialize_directories()?;
        let root = engine_with_script(&app, "switch-theme-macos.sh")?;
        let built_in_ids = bundled_preset_ids(&root)?;
        if built_in_ids.contains(&theme_id) {
            return Err("Built-in themes cannot be deleted.".to_string());
        }
        if active_theme_id().as_deref() == Some(&theme_id) {
            return Err("Switch to another theme or restore the original appearance before deleting this theme.".to_string());
        }
        let library = themes_root()?.canonicalize()
            .map_err(|error| format!("Could not resolve the managed theme library: {error}"))?;
        let target = library.join(&theme_id);
        let canonical_target = target.canonicalize()
            .map_err(|_| format!("Theme is not installed: {theme_id}"))?;
        if canonical_target.parent() != Some(library.as_path()) || canonical_target == library {
            return Err("The selected theme is outside the managed theme library.".to_string());
        }
        let theme = read_managed_theme(&canonical_target, &built_in_ids)?;
        if theme.origin != "imported" {
            return Err("Only imported themes can be deleted.".to_string());
        }
        fs::remove_dir_all(&canonical_target)
            .map_err(|error| format!("Could not delete the imported theme: {error}"))?;
        let themes = scan_theme_library(&root)?;
        Ok(ThemeLibraryResult {
            themes,
            message: format!("Deleted {}.", theme.name),
            imported_theme_id: None,
        })
    })
    .await
    .map_err(|error| format!("Delete theme task failed: {error}"))?
}

#[tauri::command]
pub async fn export_theme_creation_guide() -> Result<OperationResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let selected = rfd::FileDialog::new()
            .set_title("Save the Codex theme creation guide")
            .set_file_name("codex-theme-creation-guide.md")
            .add_filter("Markdown", &["md"])
            .save_file();
        let Some(path) = selected else {
            return Ok(OperationResult {
                ok: false,
                verified: false,
                status: "connected".to_string(),
                message: "Save cancelled.".to_string(),
            });
        };
        fs::write(&path, THEME_CREATION_GUIDE)
            .map_err(|error| format!("Could not save the creation guide: {error}"))?;
        Ok(OperationResult {
            ok: true,
            verified: true,
            status: "connected".to_string(),
            message: format!(
                "Saved {}.",
                path.file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("the creation guide")
            ),
        })
    })
    .await
    .map_err(|error| format!("Creation guide export task failed: {error}"))?
}

#[tauri::command]
pub async fn open_theme_gallery() -> Result<OperationResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let output = Command::new("/usr/bin/open")
            .arg(THEME_GALLERY_URL)
            .output()
            .map_err(|error| format!("Could not open the theme gallery: {error}"))?;
        let ok = output.status.success();
        Ok(OperationResult {
            ok,
            verified: ok,
            status: if ok { "connected" } else { "error" }.to_string(),
            message: if ok {
                "Theme gallery opened in your browser.".to_string()
            } else {
                output_error("open", &output)
            },
        })
    })
    .await
    .map_err(|error| format!("Open theme gallery task failed: {error}"))?
}

#[tauri::command]
pub async fn get_runtime_status(app: AppHandle) -> Result<RuntimeSnapshot, String> {
    tauri::async_runtime::spawn_blocking(move || status_from_script(&app, false))
        .await
        .map_err(|error| format!("Runtime status task failed: {error}"))?
}

#[tauri::command]
pub async fn apply_theme(app: AppHandle, theme_id: String) -> Result<OperationResult, String> {
    if !valid_theme_id(&theme_id) {
        return Err("Theme id contains unsupported characters.".to_string());
    }
    tauri::async_runtime::spawn_blocking(move || {
        let root = ensure_installed_runtime(&app)?;
        seed_bundled_preset(&root, &theme_id)?;
        let output = run_script(&root, "switch-theme-macos.sh", &["--id", &theme_id])?;
        if !output.status.success() {
            return Ok(OperationResult {
                ok: false,
                verified: false,
                status: "error".to_string(),
                message: output_error("switch-theme-macos.sh", &output),
            });
        }
        let snapshot = status_from_script(&app, true)?;
        let verified =
            snapshot.status == "active" && snapshot.active_theme_id.as_deref() == Some(&theme_id);
        Ok(OperationResult {
            ok: verified,
            verified,
            status: if verified { "active" } else { "error" }.to_string(),
            message: if verified {
                "Your theme is ready in Codex.".to_string()
            } else {
                "Theme switch completed but the exact active revision was not verified.".to_string()
            },
        })
    })
    .await
    .map_err(|error| format!("Apply theme task failed: {error}"))?
}

#[tauri::command]
pub async fn restore_original(app: AppHandle) -> Result<OperationResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let root = engine_with_script(&app, "restore-dream-skin-macos.sh")?;
        let output = run_script(
            &root,
            "restore-dream-skin-macos.sh",
            &["--restore-base-theme", "--restart-codex"],
        )?;
        if !output.status.success() {
            return Ok(OperationResult {
                ok: false,
                verified: false,
                status: "error".to_string(),
                message: output_error("restore-dream-skin-macos.sh", &output),
            });
        }
        let snapshot = status_from_script(&app, false)?;
        let verified = snapshot.active_theme_id.is_none() && snapshot.status != "active";
        Ok(OperationResult {
            ok: verified,
            verified,
            status: if verified { "connected" } else { "error" }.to_string(),
            message: if verified {
                "Codex original appearance was restored.".to_string()
            } else {
                "Restore completed but the inactive state could not be verified.".to_string()
            },
        })
    })
    .await
    .map_err(|error| format!("Restore task failed: {error}"))?
}

#[tauri::command]
pub async fn open_codex() -> Result<OperationResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let output = Command::new("/usr/bin/open")
            .args(["-b", "com.openai.codex"])
            .output()
            .map_err(|error| format!("Could not open Codex: {error}"))?;
        let ok = output.status.success();
        Ok(OperationResult {
            ok,
            verified: ok,
            status: if ok { "connected" } else { "error" }.to_string(),
            message: if ok {
                "Codex was opened.".to_string()
            } else {
                output_error("open", &output)
            },
        })
    })
    .await
    .map_err(|error| format!("Open Codex task failed: {error}"))?
}

#[cfg(test)]
mod tests {
    use super::{development_engine, seed_bundled_preset_to, valid_theme_id};
    use std::fs;

    #[test]
    fn accepts_managed_theme_identifiers() {
        assert!(valid_theme_id("preset-gothic-void-crusade"));
        assert!(valid_theme_id("custom-theme-01"));
        assert!(valid_theme_id("My_Theme-01"));
    }

    #[test]
    fn rejects_shell_and_path_syntax() {
        assert!(!valid_theme_id("../theme"));
        assert!(!valid_theme_id("theme;open"));
        assert!(!valid_theme_id("theme name"));
    }

    #[test]
    fn seeds_a_missing_bundled_preset_atomically() {
        let temporary =
            std::env::temp_dir().join(format!("codex-themes-preset-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&temporary);
        seed_bundled_preset_to(
            &development_engine(),
            &temporary,
            "preset-gothic-void-crusade",
        )
        .expect("preset should seed");
        let seeded = temporary.join("preset-gothic-void-crusade");
        assert!(seeded.join("theme.json").is_file());
        assert!(seeded.join("background.jpg").is_file());
        fs::remove_dir_all(&temporary).expect("temporary preset test directory should be removed");
    }
}
