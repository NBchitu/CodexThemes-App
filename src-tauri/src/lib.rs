mod macos_bridge;

use macos_bridge::{
    apply_theme, delete_theme, export_theme_creation_guide, get_runtime_status,
    import_theme_folder, initialize_theme_library, open_codex, open_theme_gallery,
    open_themes_folder, restore_original,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
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
            restore_original,
            open_codex,
            initialize_theme_library,
            import_theme_folder,
            delete_theme,
            open_themes_folder,
            export_theme_creation_guide,
            open_theme_gallery
        ])
        .run(tauri::generate_context!())
        .expect("error while running Codex Themes");
}
