use std::collections::HashMap;

use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::actions::run_action;
use crate::models::HotkeyAction;

pub fn register(app: &AppHandle, hotkeys: &HashMap<HotkeyAction, String>) {
    let gs = app.global_shortcut();
    let _ = gs.unregister_all();

    for (action, combo) in hotkeys {
        let action = *action;
        match gs.on_shortcut(combo.as_str(), move |handle, _sc, event| {
            if event.state == ShortcutState::Pressed {
                run_action(handle, action);
            }
        }) {
            Ok(_) => {}
            Err(e) => eprintln!("[hotkeys] failed to register '{combo}': {e}"),
        }
    }
}
