use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::{
    collections::BTreeSet,
    env, fs,
    io::Read,
    path::{Path, PathBuf},
    process::{Command, Output},
};
use tauri::{AppHandle, Manager};
use zip::ZipArchive;

const INSTALLED_ENGINE: &str = ".codex/codex-dream-skin-studio";
const STATE_THEME: &str = "Library/Application Support/CodexDreamSkinStudio/theme/theme.json";
const THEME_GALLERY_URL: &str = "https://codexthemes.app/?utm_source=codex_themes_desktop&utm_medium=desktop_app&utm_campaign=theme_gallery";
const PROJECT_URL: &str = "https://github.com/NBchitu/CodexThemes-App";
const MAX_THEME_FILE_SIZE: u64 = 16 * 1024 * 1024;
const MAX_THEME_TOTAL_SIZE: u64 = 32 * 1024 * 1024;
const MAX_CODEXTHEME_ARCHIVE_SIZE: u64 = 32 * 1024 * 1024;
const MAX_CODEXTHEME_MANIFEST_SIZE: u64 = 256 * 1024;
const MAX_CODEXTHEME_IMAGE_SIDE: u32 = 16_384;
const MAX_CODEXTHEME_IMAGE_PIXELS: u64 = 50_000_000;
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
pub struct CodexThemePackageSummary {
    path: String,
    id: String,
    name: String,
    author: String,
    version: String,
    description: String,
    already_installed: bool,
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
pub struct ThemeSettings {
    appearance: String,
    task_mode: String,
    safe_area: String,
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

fn validate_codextheme_entry(
    name: &str,
    is_directory: bool,
    unix_mode: Option<u32>,
) -> Result<(), String> {
    if is_directory
        || name.contains('\\')
        || name.starts_with('/')
        || name.split('/').any(|part| part == "..")
    {
        return Err(format!("Unsafe path in codextheme package: {name}"));
    }
    if Path::new(name).file_name().and_then(|value| value.to_str()) != Some(name) {
        return Err(format!(
            "codextheme files must be at the archive root: {name}"
        ));
    }
    if unix_mode.is_some_and(|mode| mode & 0o170000 == 0o120000) {
        return Err(format!(
            "Symbolic links are not allowed in codextheme packages: {name}"
        ));
    }
    if !matches!(name, "theme.json" | "background.jpg") {
        return Err(format!("Unsupported file in codextheme package: {name}"));
    }
    Ok(())
}

fn validate_single_line_text(
    manifest: &Value,
    field: &str,
    maximum: usize,
    required: bool,
) -> Result<(), String> {
    let Some(value) = manifest.get(field) else {
        return if required {
            Err(format!("codextheme-v1 {field} is missing."))
        } else {
            Ok(())
        };
    };
    let value = value
        .as_str()
        .ok_or_else(|| format!("codextheme-v1 {field} must be a string."))?;
    if (required && value.trim().is_empty())
        || value.chars().count() > maximum
        || value.chars().any(char::is_control)
        || value.contains(['\n', '\r'])
    {
        return Err(format!("codextheme-v1 {field} is invalid."));
    }
    Ok(())
}

fn valid_css_number(value: &str, minimum: f64, maximum: f64) -> bool {
    !value.is_empty()
        && value
            .parse::<f64>()
            .is_ok_and(|number| number.is_finite() && (minimum..=maximum).contains(&number))
}

fn valid_css_color(value: &str) -> bool {
    if value.len() >= 4 && value.starts_with('#') {
        return matches!(value.len(), 4 | 5 | 7 | 9)
            && value[1..].bytes().all(|byte| byte.is_ascii_hexdigit());
    }
    let (function, contents) = if let Some(contents) = value
        .strip_prefix("rgb(")
        .and_then(|value| value.strip_suffix(')'))
    {
        ("rgb", contents)
    } else if let Some(contents) = value
        .strip_prefix("rgba(")
        .and_then(|value| value.strip_suffix(')'))
    {
        ("rgba", contents)
    } else {
        return false;
    };
    let parts = contents.split(',').map(str::trim).collect::<Vec<_>>();
    let expected = if function == "rgb" { 3 } else { 4 };
    if parts.len() != expected {
        return false;
    }
    if !parts[..3].iter().all(|component| {
        component.strip_suffix('%').map_or_else(
            || valid_css_number(component, 0.0, 255.0),
            |number| valid_css_number(number, 0.0, 100.0),
        )
    }) {
        return false;
    }
    function == "rgb"
        || parts[3].strip_suffix('%').map_or_else(
            || valid_css_number(parts[3], 0.0, 1.0),
            |number| valid_css_number(number, 0.0, 100.0),
        )
}

fn valid_https_url(value: &str) -> bool {
    if value.len() > 512
        || value.chars().any(char::is_control)
        || value.chars().any(char::is_whitespace)
        || value.contains('\\')
    {
        return false;
    }
    let Some(remainder) = value.strip_prefix("https://") else {
        return false;
    };
    let authority = remainder
        .split_once(['/', '?', '#'])
        .map_or(remainder, |(authority, _)| authority);
    if authority.is_empty() || authority.contains('@') {
        return false;
    }
    let (host, port) = if authority.starts_with('[') {
        let Some(closing) = authority.find(']') else {
            return false;
        };
        let host = &authority[1..closing];
        let suffix = &authority[closing + 1..];
        if suffix.is_empty() {
            (host, None)
        } else if let Some(port) = suffix.strip_prefix(':') {
            (host, Some(port))
        } else {
            return false;
        }
    } else if let Some((host, port)) = authority.rsplit_once(':') {
        if host.contains(':') {
            return false;
        }
        (host, Some(port))
    } else {
        (authority, None)
    };
    if host.is_empty()
        || host.starts_with('.')
        || host.ends_with('.')
        || host.split('.').any(|label| {
            label.is_empty()
                || label.starts_with('-')
                || label.ends_with('-')
                || !label
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        })
    {
        return false;
    }
    match port {
        None => true,
        Some(port) => !port.is_empty() && port.parse::<u16>().is_ok_and(|number| number != 0),
    }
}

fn validate_codextheme_jpeg(bytes: &[u8]) -> Result<(u32, u32), String> {
    if !bytes.starts_with(&[0xff, 0xd8]) {
        return Err("background.jpg is not a JPEG image.".to_string());
    }
    let mut offset = 2usize;
    while offset < bytes.len() {
        while offset < bytes.len() && bytes[offset] == 0xff {
            offset += 1;
        }
        if offset >= bytes.len() {
            break;
        }
        let marker = bytes[offset];
        offset += 1;
        if marker == 0xd9 || marker == 0xda {
            break;
        }
        if marker == 0x00 || marker == 0x01 || (0xd0..=0xd7).contains(&marker) {
            continue;
        }
        let Some(length_bytes) = bytes.get(offset..offset.saturating_add(2)) else {
            return Err("background.jpg contains a truncated JPEG segment.".to_string());
        };
        let segment_length = u16::from_be_bytes([length_bytes[0], length_bytes[1]]) as usize;
        if segment_length < 2 {
            return Err("background.jpg contains an invalid JPEG segment.".to_string());
        }
        let segment_end = offset
            .checked_add(segment_length)
            .filter(|end| *end <= bytes.len())
            .ok_or_else(|| "background.jpg contains a truncated JPEG segment.".to_string())?;
        if matches!(
            marker,
            0xc0 | 0xc1
                | 0xc2
                | 0xc3
                | 0xc5
                | 0xc6
                | 0xc7
                | 0xc9
                | 0xca
                | 0xcb
                | 0xcd
                | 0xce
                | 0xcf
        ) {
            if segment_length < 8 {
                return Err("background.jpg contains an invalid JPEG frame.".to_string());
            }
            let height = u16::from_be_bytes([bytes[offset + 3], bytes[offset + 4]]) as u32;
            let width = u16::from_be_bytes([bytes[offset + 5], bytes[offset + 6]]) as u32;
            let pixels = u64::from(width).saturating_mul(u64::from(height));
            if width == 0
                || height == 0
                || width > MAX_CODEXTHEME_IMAGE_SIDE
                || height > MAX_CODEXTHEME_IMAGE_SIDE
                || pixels > MAX_CODEXTHEME_IMAGE_PIXELS
            {
                return Err(format!(
                    "background.jpg dimensions {width}×{height} exceed the supported image limits."
                ));
            }
            return Ok((width, height));
        }
        offset = segment_end;
    }
    Err("background.jpg does not contain readable JPEG dimensions.".to_string())
}

fn validate_codextheme_manifest(manifest: &Value) -> Result<(), String> {
    const FIELDS: [&str; 19] = [
        "appearance",
        "art",
        "author",
        "brandSubtitle",
        "colors",
        "description",
        "id",
        "image",
        "name",
        "projectLabel",
        "projectPrefix",
        "promoSub",
        "promoTitle",
        "promoUrl",
        "quote",
        "schemaVersion",
        "statusText",
        "tagline",
        "version",
    ];
    const COLOR_FIELDS: [&str; 10] = [
        "accent",
        "accentAlt",
        "background",
        "highlight",
        "line",
        "muted",
        "panel",
        "panelAlt",
        "secondary",
        "text",
    ];
    let object = manifest
        .as_object()
        .ok_or_else(|| "codextheme-v1 theme.json must be an object.".to_string())?;
    let actual = object.keys().map(String::as_str).collect::<BTreeSet<_>>();
    let expected = FIELDS.into_iter().collect::<BTreeSet<_>>();
    if actual != expected {
        return Err("codextheme-v1 theme.json contains missing or unexpected fields.".to_string());
    }
    if manifest.get("schemaVersion").and_then(Value::as_u64) != Some(1) {
        return Err("Only codextheme schemaVersion 1 is supported.".to_string());
    }
    let id = manifest_string(manifest, "id", "");
    if !id.starts_with("preset-") || !valid_theme_id(&id) {
        return Err("codextheme theme id must use the preset- slug format.".to_string());
    }
    for (field, maximum) in [("name", 80), ("author", 80), ("description", 320)] {
        validate_single_line_text(manifest, field, maximum, true)?;
    }
    for (field, maximum) in [
        ("brandSubtitle", 80),
        ("tagline", 160),
        ("projectPrefix", 80),
        ("projectLabel", 120),
        ("statusText", 80),
        ("quote", 200),
        ("promoTitle", 120),
        ("promoSub", 160),
    ] {
        validate_single_line_text(manifest, field, maximum, false)?;
    }
    let version = manifest
        .get("version")
        .and_then(Value::as_str)
        .unwrap_or("");
    let version_parts = version.split('.').collect::<Vec<_>>();
    if version_parts.len() != 3
        || version_parts
            .iter()
            .any(|part| part.parse::<u64>().is_err() || (part.len() > 1 && part.starts_with('0')))
    {
        return Err("codextheme-v1 version must use semantic x.y.z format.".to_string());
    }
    if !matches!(
        manifest.get("appearance").and_then(Value::as_str),
        Some("auto" | "light" | "dark")
    ) {
        return Err("codextheme-v1 appearance is invalid.".to_string());
    }
    if manifest_string(manifest, "image", "") != "background.jpg" {
        return Err("codextheme-v1 theme.json must reference background.jpg.".to_string());
    }
    let art = manifest
        .get("art")
        .and_then(Value::as_object)
        .ok_or_else(|| "codextheme-v1 art is invalid.".to_string())?;
    let art_fields = art.keys().map(String::as_str).collect::<BTreeSet<_>>();
    if art_fields
        != ["focusX", "focusY", "safeArea", "taskMode"]
            .into_iter()
            .collect()
    {
        return Err("codextheme-v1 art contains missing or unexpected fields.".to_string());
    }
    for field in ["focusX", "focusY"] {
        let value = art.get(field).and_then(Value::as_f64).unwrap_or(-1.0);
        if !(0.0..=1.0).contains(&value) {
            return Err(format!("codextheme-v1 art.{field} is invalid."));
        }
    }
    if !matches!(
        art.get("safeArea").and_then(Value::as_str),
        Some("auto" | "left" | "right" | "center" | "none")
    ) {
        return Err("codextheme-v1 art.safeArea is invalid.".to_string());
    }
    if !matches!(
        art.get("taskMode").and_then(Value::as_str),
        Some("auto" | "ambient" | "banner" | "off")
    ) {
        return Err("codextheme-v1 art.taskMode is invalid.".to_string());
    }
    let colors = manifest
        .get("colors")
        .and_then(Value::as_object)
        .ok_or_else(|| "codextheme-v1 colors are invalid.".to_string())?;
    if colors.keys().map(String::as_str).collect::<BTreeSet<_>>()
        != COLOR_FIELDS.into_iter().collect()
    {
        return Err("codextheme-v1 colors must contain exactly 10 supported fields.".to_string());
    }
    if colors
        .values()
        .any(|value| !value.as_str().is_some_and(valid_css_color))
    {
        return Err(
            "codextheme-v1 colors must use supported hex, rgb(), or rgba() syntax.".to_string(),
        );
    }
    if !manifest
        .get("promoUrl")
        .and_then(Value::as_str)
        .is_some_and(valid_https_url)
    {
        return Err("codextheme-v1 promoUrl must be a valid HTTPS URL.".to_string());
    }
    Ok(())
}

fn read_codextheme_archive(source: &Path) -> Result<(Value, Vec<(String, Vec<u8>)>), String> {
    let metadata = fs::symlink_metadata(source)
        .map_err(|error| format!("Could not inspect the selected codextheme package: {error}"))?;
    if !metadata.is_file() || metadata.len() == 0 || metadata.len() > MAX_CODEXTHEME_ARCHIVE_SIZE {
        return Err("The codextheme package is empty or exceeds the 32 MB limit.".to_string());
    }
    let file = fs::File::open(source)
        .map_err(|error| format!("Could not open the selected codextheme package: {error}"))?;
    let mut archive = ZipArchive::new(file).map_err(|error| {
        format!("The selected file is not a valid codextheme ZIP archive: {error}")
    })?;
    if archive.len() != 2 {
        return Err(
            "codextheme-v1 must contain exactly theme.json and background.jpg.".to_string(),
        );
    }
    let mut total_size = 0u64;
    let mut files = Vec::with_capacity(2);
    for index in 0..archive.len() {
        let mut entry = archive
            .by_index(index)
            .map_err(|error| format!("Could not inspect codextheme entry: {error}"))?;
        let name = entry.name().to_string();
        validate_codextheme_entry(&name, entry.is_dir(), entry.unix_mode())?;
        if files.iter().any(|(existing, _)| existing == &name) {
            return Err(format!("Duplicate file in codextheme package: {name}"));
        }
        let file_limit = if name == "theme.json" {
            MAX_CODEXTHEME_MANIFEST_SIZE
        } else {
            MAX_THEME_FILE_SIZE
        };
        if entry.size() == 0 || entry.size() > file_limit {
            return Err(if name == "theme.json" {
                "theme.json is empty or exceeds the 256 KB file limit.".to_string()
            } else {
                format!("{name} is empty or exceeds the 16 MB file limit.")
            });
        }
        total_size = total_size.saturating_add(entry.size());
        if total_size > MAX_THEME_TOTAL_SIZE {
            return Err("The codextheme package exceeds the 32 MB extracted limit.".to_string());
        }
        let declared_size = entry.size();
        let mut bytes = Vec::with_capacity(declared_size.min(file_limit) as usize);
        entry
            .by_ref()
            .take(file_limit + 1)
            .read_to_end(&mut bytes)
            .map_err(|error| {
                format!("Could not read {name} from the codextheme package: {error}")
            })?;
        if bytes.len() as u64 > file_limit {
            return Err(if name == "theme.json" {
                "theme.json exceeds the 256 KB file limit.".to_string()
            } else {
                format!("{name} exceeds the 16 MB file limit.")
            });
        }
        if bytes.len() as u64 != declared_size {
            return Err(format!(
                "The extracted size of {name} did not match its archive metadata."
            ));
        }
        files.push((name, bytes));
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));
    if files
        .iter()
        .map(|(name, _)| name.as_str())
        .collect::<Vec<_>>()
        != ["background.jpg", "theme.json"]
    {
        return Err(
            "codextheme-v1 must contain exactly theme.json and background.jpg.".to_string(),
        );
    }
    let manifest_bytes = files
        .iter()
        .find(|(name, _)| name == "theme.json")
        .map(|(_, bytes)| bytes)
        .ok_or_else(|| "codextheme package is missing theme.json.".to_string())?;
    if manifest_bytes.len() as u64 > MAX_CODEXTHEME_MANIFEST_SIZE {
        return Err("theme.json must not exceed 256 KB.".to_string());
    }
    let manifest: Value = serde_json::from_slice(manifest_bytes)
        .map_err(|error| format!("theme.json is not valid JSON: {error}"))?;
    validate_codextheme_manifest(&manifest)?;
    let background = files
        .iter()
        .find(|(name, _)| name == "background.jpg")
        .map(|(_, bytes)| bytes.as_slice())
        .ok_or_else(|| "codextheme package is missing background.jpg.".to_string())?;
    validate_codextheme_jpeg(background)?;
    Ok((manifest, files))
}

fn install_codextheme_from(
    app: &AppHandle,
    source: &Path,
    overwrite: bool,
) -> Result<ThemeLibraryResult, String> {
    initialize_directories()?;
    let (manifest, files) = read_codextheme_archive(source)?;
    let id = manifest_string(&manifest, "id", "");
    let destination_root = themes_root()?;
    let destination = destination_root.join(&id);
    if destination.exists() && !overwrite {
        return Err(format!("A theme with id “{id}” is already installed."));
    }
    let temporary = destination_root.join(format!(".{id}.installing-{}", std::process::id()));
    if temporary.exists() {
        fs::remove_dir_all(&temporary)
            .map_err(|error| format!("Could not clear a stale codextheme import: {error}"))?;
    }
    fs::create_dir(&temporary)
        .map_err(|error| format!("Could not stage the codextheme package: {error}"))?;
    #[cfg(unix)]
    fs::set_permissions(&temporary, fs::Permissions::from_mode(0o700))
        .map_err(|error| format!("Could not protect the staged codextheme package: {error}"))?;
    let result = (|| {
        for (name, bytes) in files {
            let target = temporary.join(name);
            fs::write(&target, bytes)
                .map_err(|error| format!("Could not write a staged theme file: {error}"))?;
            #[cfg(unix)]
            fs::set_permissions(&target, fs::Permissions::from_mode(0o600))
                .map_err(|error| format!("Could not protect a staged theme file: {error}"))?;
        }
        if destination.exists() {
            let backup = destination_root.join(format!(".{id}.replacing-{}", std::process::id()));
            if backup.exists() {
                fs::remove_dir_all(&backup)
                    .map_err(|error| format!("Could not clear a stale theme backup: {error}"))?;
            }
            fs::rename(&destination, &backup).map_err(|error| {
                format!("Could not prepare the installed theme for replacement: {error}")
            })?;
            if let Err(error) = fs::rename(&temporary, &destination) {
                let _ = fs::rename(&backup, &destination);
                return Err(format!(
                    "Could not replace the installed codextheme package: {error}"
                ));
            }
            let _ = fs::remove_dir_all(&backup);
        } else {
            fs::rename(&temporary, &destination).map_err(|error| {
                format!("Could not publish the imported codextheme package: {error}")
            })?;
        }
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
        message: format!(
            "{} {}.",
            if overwrite { "Replaced" } else { "Imported" },
            manifest_string(&manifest, "name", &id)
        ),
        imported_theme_id: Some(id),
    })
}

#[tauri::command]
pub async fn inspect_codextheme_package(path: String) -> Result<CodexThemePackageSummary, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let source = PathBuf::from(&path);
        let (manifest, _) = read_codextheme_archive(&source)?;
        let id = manifest_string(&manifest, "id", "");
        let already_installed = themes_root()?.join(&id).exists();
        Ok(CodexThemePackageSummary {
            path,
            id,
            name: manifest_string(&manifest, "name", ""),
            author: manifest_string(&manifest, "author", "admin"),
            version: manifest_string(&manifest, "version", ""),
            description: manifest_string(&manifest, "description", ""),
            already_installed,
        })
    })
    .await
    .map_err(|error| format!("codextheme inspection task failed: {error}"))?
}

#[tauri::command]
pub async fn import_codextheme_path(
    app: AppHandle,
    path: String,
    overwrite: bool,
) -> Result<ThemeLibraryResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        install_codextheme_from(&app, Path::new(&path), overwrite)
    })
    .await
    .map_err(|error| format!("codextheme import task failed: {error}"))?
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
    if detail.contains("signature is not valid") || detail.contains("code-signature validation") {
        return "Codex could not be verified. Reinstall the official Codex app, then try again."
            .to_string();
    }
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
        _ if script.codex_running => (
            "restart-required",
            "Codex is running without an active theme session",
        ),
        _ if script.session == "stale" || script.session == "unknown" => {
            ("error", "Theme runtime requires attention")
        }
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
pub async fn import_codextheme_package(app: AppHandle) -> Result<ThemeLibraryResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let selected = rfd::FileDialog::new()
            .set_title("Choose a Codex Themes package")
            .add_filter("Codex Theme", &["codextheme"])
            .pick_file();
        match selected {
            Some(path) => install_codextheme_from(&app, &path, false),
            None => {
                let mut result = initialize_library(&app)?;
                result.message = "Import cancelled.".to_string();
                Ok(result)
            }
        }
    })
    .await
    .map_err(|error| format!("codextheme import task failed: {error}"))?
}

#[cfg(test)]
mod codextheme_tests {
    use super::{
        valid_css_color, valid_https_url, validate_codextheme_entry, validate_codextheme_jpeg,
        validate_codextheme_manifest,
    };
    use serde_json::json;

    fn valid_manifest() -> serde_json::Value {
        json!({
            "schemaVersion": 1, "id": "preset-quiet-studio", "name": "Quiet Studio",
            "author": "admin", "version": "1.0.0", "description": "A quiet theme.",
            "appearance": "dark", "image": "background.jpg",
            "art": {"focusX": 0.7, "focusY": 0.5, "safeArea": "left", "taskMode": "ambient"},
            "brandSubtitle": "CODEX THEMES", "tagline": "Quiet", "projectPrefix": "Project · ",
            "projectLabel": "Choose project", "statusText": "READY", "quote": "FOCUS",
            "colors": {"background": "#111", "panel": "#181818", "panelAlt": "#202020", "accent": "#789", "accentAlt": "#89a", "secondary": "#765", "highlight": "#def", "text": "#fff", "muted": "#aaa", "line": "rgba(1,2,3,.2)"},
            "promoTitle": "Quiet Studio", "promoSub": "CodexThemes.app",
            "promoUrl": "https://codexthemes.app/themes/quiet-studio"
        })
    }

    #[test]
    fn accepts_only_the_two_regular_root_files() {
        assert!(validate_codextheme_entry("theme.json", false, Some(0o100600)).is_ok());
        assert!(validate_codextheme_entry("background.jpg", false, Some(0o100600)).is_ok());
        assert!(validate_codextheme_entry("install.sh", false, Some(0o100700)).is_err());
        assert!(validate_codextheme_entry("assets/background.jpg", false, Some(0o100600)).is_err());
    }

    #[test]
    fn rejects_traversal_directories_and_links() {
        assert!(validate_codextheme_entry("../theme.json", false, Some(0o100600)).is_err());
        assert!(validate_codextheme_entry("theme.json", true, Some(0o040700)).is_err());
        assert!(validate_codextheme_entry("theme.json", false, Some(0o120777)).is_err());
    }

    #[test]
    fn validates_the_complete_native_manifest_boundary() {
        let manifest = valid_manifest();
        assert!(validate_codextheme_manifest(&manifest).is_ok());
        let mut executable = manifest.clone();
        executable
            .as_object_mut()
            .unwrap()
            .insert("script".to_string(), json!("install.sh"));
        assert!(validate_codextheme_manifest(&executable).is_err());
        let mut invalid_art = manifest;
        invalid_art["art"]["focusX"] = json!(1.5);
        assert!(validate_codextheme_manifest(&invalid_art).is_err());
    }

    #[test]
    fn rejects_control_characters_and_multiline_copy() {
        for (field, value) in [
            ("name", "Quiet\nStudio"),
            ("description", "Quiet\u{0000}Studio"),
            ("tagline", "Quiet\rStudio"),
            ("promoTitle", "Quiet\u{0085}Studio"),
        ] {
            let mut manifest = valid_manifest();
            manifest[field] = json!(value);
            assert!(
                validate_codextheme_manifest(&manifest).is_err(),
                "{field} should reject control characters"
            );
        }
    }

    #[test]
    fn accepts_only_bounded_css_color_syntax() {
        for color in [
            "#fff",
            "#ffff",
            "#112233",
            "#11223344",
            "rgb(0, 127, 255)",
            "rgb(0%, 50%, 100%)",
            "rgba(196, 120, 128, .22)",
        ] {
            assert!(valid_css_color(color), "{color} should be accepted");
        }
        for color in [
            "red",
            "var(--secret)",
            "url(https://example.com/a)",
            "#12",
            "#ggg",
            "rgb(256, 0, 0)",
            "rgba(0, 0, 0, 2)",
        ] {
            assert!(!valid_css_color(color), "{color} should be rejected");
        }
    }

    #[test]
    fn validates_https_urls_without_credentials_or_ambiguous_hosts() {
        assert!(valid_https_url(
            "https://codexthemes.app/themes/quiet-studio?source=app#details"
        ));
        assert!(valid_https_url("https://example.com:8443/theme"));
        for url in [
            "http://codexthemes.app/theme",
            "https://user@example.com/theme",
            "https://example.com\\@attacker.invalid/",
            "https://-example.com/theme",
            "https://example.com:0/theme",
            "https://example.com/\nnext",
        ] {
            assert!(!valid_https_url(url), "{url:?} should be rejected");
        }
    }

    fn jpeg_with_dimensions(width: u16, height: u16) -> Vec<u8> {
        vec![
            0xff,
            0xd8,
            0xff,
            0xe0,
            0x00,
            0x02,
            0xff,
            0xc0,
            0x00,
            0x08,
            0x08,
            (height >> 8) as u8,
            height as u8,
            (width >> 8) as u8,
            width as u8,
            0x01,
            0xff,
            0xd9,
        ]
    }

    #[test]
    fn validates_jpeg_magic_dimensions_and_pixel_limits() {
        assert_eq!(
            validate_codextheme_jpeg(&jpeg_with_dimensions(2560, 1440)).unwrap(),
            (2560, 1440)
        );
        assert!(validate_codextheme_jpeg(b"not a jpeg").is_err());
        assert!(validate_codextheme_jpeg(&[0xff, 0xd8, 0xff, 0xc0, 0x00]).is_err());
        assert!(validate_codextheme_jpeg(&jpeg_with_dimensions(0, 1440)).is_err());
        assert!(validate_codextheme_jpeg(&jpeg_with_dimensions(16_385, 100)).is_err());
        assert!(validate_codextheme_jpeg(&jpeg_with_dimensions(10_000, 6_000)).is_err());
    }
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
pub async fn open_project_home() -> Result<OperationResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let output = Command::new("/usr/bin/open")
            .arg(PROJECT_URL)
            .output()
            .map_err(|error| format!("Could not open the GitHub project: {error}"))?;
        let ok = output.status.success();
        Ok(OperationResult {
            ok,
            verified: ok,
            status: if ok { "connected" } else { "error" }.to_string(),
            message: if ok {
                "GitHub project opened in your browser.".to_string()
            } else {
                output_error("open", &output)
            },
        })
    })
    .await
    .map_err(|error| format!("Open GitHub project task failed: {error}"))?
}

#[tauri::command]
pub async fn get_runtime_status(app: AppHandle) -> Result<RuntimeSnapshot, String> {
    tauri::async_runtime::spawn_blocking(move || status_from_script(&app, false))
        .await
        .map_err(|error| format!("Runtime status task failed: {error}"))?
}

#[tauri::command]
pub async fn update_theme_settings(
    app: AppHandle,
    theme_id: String,
    settings: ThemeSettings,
) -> Result<ThemeLibraryResult, String> {
    if !valid_theme_id(&theme_id) {
        return Err("Theme id contains unsupported characters.".to_string());
    }
    if !matches!(settings.appearance.as_str(), "auto" | "light" | "dark")
        || !matches!(
            settings.task_mode.as_str(),
            "auto" | "ambient" | "banner" | "off"
        )
        || !matches!(
            settings.safe_area.as_str(),
            "auto" | "left" | "right" | "center" | "none"
        )
    {
        return Err("Theme settings contain an unsupported value.".to_string());
    }

    tauri::async_runtime::spawn_blocking(move || {
        initialize_directories()?;
        let root = ensure_installed_runtime(&app)?;
        seed_bundled_preset(&root, &theme_id)?;

        let library = themes_root()?
            .canonicalize()
            .map_err(|error| format!("Could not resolve the managed theme library: {error}"))?;
        let theme_directory = library
            .join(&theme_id)
            .canonicalize()
            .map_err(|_| format!("Theme is not installed: {theme_id}"))?;
        if theme_directory.parent() != Some(library.as_path()) {
            return Err("The selected theme is outside the managed theme library.".to_string());
        }

        let manifest_path = theme_directory.join("theme.json");
        let metadata = fs::symlink_metadata(&manifest_path)
            .map_err(|error| format!("Could not inspect theme.json: {error}"))?;
        if !metadata.file_type().is_file() || metadata.len() > 256 * 1024 {
            return Err("theme.json must be a regular file no larger than 256 KB.".to_string());
        }
        let mut manifest: Value = serde_json::from_slice(
            &fs::read(&manifest_path)
                .map_err(|error| format!("Could not read theme.json: {error}"))?,
        )
        .map_err(|error| format!("theme.json is not valid JSON: {error}"))?;
        let object = manifest
            .as_object_mut()
            .ok_or_else(|| "theme.json must contain a JSON object.".to_string())?;
        object.insert("appearance".to_string(), Value::String(settings.appearance));
        let art = object
            .entry("art")
            .or_insert_with(|| Value::Object(Default::default()))
            .as_object_mut()
            .ok_or_else(|| "theme.json art must be an object.".to_string())?;
        art.insert("taskMode".to_string(), Value::String(settings.task_mode));
        art.insert("safeArea".to_string(), Value::String(settings.safe_area));

        let temporary_path =
            theme_directory.join(format!(".theme.json.settings-{}", std::process::id()));
        if temporary_path.exists() {
            fs::remove_file(&temporary_path)
                .map_err(|error| format!("Could not clear stale theme settings: {error}"))?;
        }
        let contents = serde_json::to_vec_pretty(&manifest)
            .map_err(|error| format!("Could not encode theme settings: {error}"))?;
        fs::write(&temporary_path, contents)
            .map_err(|error| format!("Could not stage theme settings: {error}"))?;
        #[cfg(unix)]
        fs::set_permissions(&temporary_path, fs::Permissions::from_mode(0o600))
            .map_err(|error| format!("Could not protect theme settings: {error}"))?;
        fs::rename(&temporary_path, &manifest_path)
            .map_err(|error| format!("Could not save theme settings: {error}"))?;

        Ok(ThemeLibraryResult {
            themes: scan_theme_library(&root)?,
            message: "Theme settings saved.".to_string(),
            imported_theme_id: None,
        })
    })
    .await
    .map_err(|error| format!("Update theme settings task failed: {error}"))?
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
                "Theme applied.".to_string()
            } else {
                "Theme switch completed but the exact active revision was not verified.".to_string()
            },
        })
    })
    .await
    .map_err(|error| format!("Apply theme task failed: {error}"))?
}

#[tauri::command]
pub async fn set_window_border_enabled(
    app: AppHandle,
    enabled: bool,
    style: Option<String>,
) -> Result<OperationResult, String> {
    let style = style.unwrap_or_else(|| "classic-rainbow".to_string());
    if !matches!(
        style.as_str(),
        "classic-rainbow" | "candy-stripe" | "ocean" | "monochrome"
    ) {
        return Err("Unsupported animated window border style.".to_string());
    }
    tauri::async_runtime::spawn_blocking(move || {
        let root = ensure_installed_runtime(&app)?;
        let enabled_arg = if enabled { "true" } else { "false" };
        let output = run_script(
            &root,
            "set-window-border-macos.sh",
            &["--enabled", enabled_arg, "--style", &style],
        )?;
        if !output.status.success() {
            return Ok(OperationResult {
                ok: false,
                verified: false,
                status: "error".to_string(),
                message: output_error("set-window-border-macos.sh", &output),
            });
        }
        let snapshot = status_from_script(&app, false)?;
        let applied_now = snapshot.status == "active";
        Ok(OperationResult {
            ok: true,
            verified: true,
            status: snapshot.status,
            message: match (enabled, applied_now) {
                (true, true) => format!("Animated window border enabled ({style})."),
                (false, true) => "Animated window border disabled.".to_string(),
                (true, false) => {
                    format!(
                        "Animated window border ({style}) will appear the next time a theme runs."
                    )
                }
                (false, false) => "Animated window border disabled.".to_string(),
            },
        })
    })
    .await
    .map_err(|error| format!("Window border task failed: {error}"))?
}

#[tauri::command]
pub async fn set_window_effects_enabled(
    app: AppHandle,
    enabled: bool,
) -> Result<OperationResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let root = ensure_installed_runtime(&app)?;
        let enabled_arg = if enabled { "true" } else { "false" };
        let output = run_script(
            &root,
            "set-window-effects-macos.sh",
            &["--enabled", enabled_arg],
        )?;
        if !output.status.success() {
            return Ok(OperationResult {
                ok: false,
                verified: false,
                status: "error".to_string(),
                message: output_error("set-window-effects-macos.sh", &output),
            });
        }
        let snapshot = status_from_script(&app, false)?;
        let applied_now = snapshot.status == "active";
        Ok(OperationResult {
            ok: true,
            verified: true,
            status: snapshot.status,
            message: match (enabled, applied_now) {
                (true, true) => "Codex window effects enabled.".to_string(),
                (false, true) => "Codex window effects paused.".to_string(),
                (true, false) => {
                    "Codex window effects will appear the next time a theme runs.".to_string()
                }
                (false, false) => "Codex window effects paused.".to_string(),
            },
        })
    })
    .await
    .map_err(|error| format!("Window effects task failed: {error}"))?
}

#[tauri::command]
pub async fn set_pixel_cat_enabled(
    app: AppHandle,
    enabled: bool,
) -> Result<OperationResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let root = ensure_installed_runtime(&app)?;
        let enabled_arg = if enabled { "true" } else { "false" };
        let output = run_script(&root, "set-pixel-cat-macos.sh", &["--enabled", enabled_arg])?;
        if !output.status.success() {
            return Ok(OperationResult {
                ok: false,
                verified: false,
                status: "error".to_string(),
                message: output_error("set-pixel-cat-macos.sh", &output),
            });
        }
        let snapshot = status_from_script(&app, false)?;
        let applied_now = snapshot.status == "active";
        Ok(OperationResult {
            ok: true,
            verified: true,
            status: snapshot.status,
            message: match (enabled, applied_now) {
                (true, true) => "Pixel cat companion enabled.".to_string(),
                (false, true) => "Pixel cat companion disabled.".to_string(),
                (true, false) => "Pixel cat will appear the next time a theme runs.".to_string(),
                (false, false) => "Pixel cat companion disabled.".to_string(),
            },
        })
    })
    .await
    .map_err(|error| format!("Pixel cat task failed: {error}"))?
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
