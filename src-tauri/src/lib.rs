mod macos_bridge;

use std::{
    collections::VecDeque,
    path::Path,
    sync::{Mutex, OnceLock},
};
use tauri::Emitter;

use macos_bridge::{
    apply_theme, delete_theme, export_theme_creation_guide, get_runtime_status,
    import_codextheme_package, import_codextheme_path, import_theme_folder,
    initialize_theme_library, inspect_codextheme_package, open_codex, open_project_home,
    open_theme_gallery, open_themes_folder, restore_original, set_pixel_cat_enabled,
    set_window_border_enabled, set_window_effects_enabled, update_theme_settings,
};

static PENDING_CODEXTHEME_PATHS: OnceLock<Mutex<VecDeque<String>>> = OnceLock::new();

fn pending_codextheme_paths() -> &'static Mutex<VecDeque<String>> {
    PENDING_CODEXTHEME_PATHS.get_or_init(|| Mutex::new(VecDeque::new()))
}

fn enqueue_codextheme_path(path: impl AsRef<Path>) {
    let path = path.as_ref();
    if path.extension().and_then(|value| value.to_str()) != Some("codextheme") {
        return;
    }
    if let Ok(mut pending) = pending_codextheme_paths().lock() {
        let value = path.to_string_lossy().to_string();
        if !pending.contains(&value) {
            pending.push_back(value);
        }
    }
}

#[tauri::command]
fn pending_codextheme_path() -> Option<String> {
    pending_codextheme_paths().lock().ok()?.pop_front()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            for value in std::env::args().skip(1) {
                enqueue_codextheme_path(value);
            }
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_runtime_status,
            apply_theme,
            update_theme_settings,
            restore_original,
            open_codex,
            set_window_border_enabled,
            set_window_effects_enabled,
            set_pixel_cat_enabled,
            initialize_theme_library,
            import_theme_folder,
            import_codextheme_package,
            inspect_codextheme_package,
            import_codextheme_path,
            pending_codextheme_path,
            delete_theme,
            open_themes_folder,
            export_theme_creation_guide,
            open_theme_gallery,
            open_project_home
        ])
        .build(tauri::generate_context!())
        .expect("error while building Codex Themes")
        .run(|app, event| {
            if let tauri::RunEvent::Opened { urls } = event {
                for url in urls {
                    if let Ok(path) = url.to_file_path() {
                        if path.extension().and_then(|value| value.to_str()) == Some("codextheme") {
                            enqueue_codextheme_path(&path);
                            let _ = app.emit("codextheme-open-requested", ());
                        }
                    }
                }
            }
        });
}
