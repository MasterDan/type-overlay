use std::collections::HashMap;

use tauri::AppHandle;

use crate::hotkeys;
use crate::keyboard;
use crate::layout;
use crate::mode;
use crate::models::{AppMode, HotkeyAction};

#[tauri::command]
pub fn set_app_mode(app: AppHandle, mode: AppMode) -> Result<(), String> {
    mode::apply_mode(&app, mode).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_overlay_visible(app: AppHandle, visible: bool) -> Result<(), String> {
    mode::set_overlay_visible(&app, visible).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn register_hotkeys(
    app: AppHandle,
    hotkeys: HashMap<HotkeyAction, String>,
) -> Result<(), String> {
    hotkeys::register(&app, &hotkeys);
    Ok(())
}

#[tauri::command]
pub async fn get_layout(app: AppHandle) -> Result<String, String> {
    let (tx, rx) = std::sync::mpsc::channel();
    let _ = app
        .run_on_main_thread(move || {
            let _ = tx.send(layout::current_layout());
        })
        .map_err(|e| e.to_string());
    Ok(rx.recv().unwrap_or_else(|_| "en".to_string()))
}

#[tauri::command]
pub fn get_platform() -> String {
    std::env::consts::OS.to_string()
}

#[tauri::command]
pub fn check_accessibility() -> bool {
    #[cfg(target_os = "macos")]
    {
        keyboard::is_trusted()
    }
    #[cfg(not(target_os = "macos"))]
    {
        true
    }
}

#[tauri::command]
pub fn get_capture_status() -> String {
    keyboard::capture_status().to_string()
}
