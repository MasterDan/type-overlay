mod actions;
mod commands;
mod hotkeys;
mod keyboard;
mod layout;
mod mode;
mod models;
mod state;

use crate::state::AppState;

#[cfg_attr(mobile, allow(dead_code))]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(AppState::new())
        .setup(|app| {
            let handle = app.handle().clone();
            keyboard::spawn_listener(handle.clone());
            layout::spawn_poller(handle);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::set_app_mode,
            commands::set_overlay_visible,
            commands::register_hotkeys,
            commands::get_layout,
            commands::get_platform,
            commands::check_accessibility,
            commands::get_capture_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
