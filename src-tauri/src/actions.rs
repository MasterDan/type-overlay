use tauri::{AppHandle, Manager};

use crate::mode;
use crate::models::{AppMode, HotkeyAction};
use crate::state::AppState;

pub fn run_action(app: &AppHandle, action: HotkeyAction) {
    let st = app.state::<AppState>();
    match action {
        HotkeyAction::ToggleMode => {
            let next = match *st.mode.lock().unwrap() {
                AppMode::Window => AppMode::Overlay,
                AppMode::Overlay => AppMode::Window,
            };
            let _ = mode::apply_mode(app, next);
        }
        HotkeyAction::ToggleOverlay => {
            let current = *st.mode.lock().unwrap();
            if current == AppMode::Window {
                let _ = mode::apply_mode(app, AppMode::Overlay);
            } else {
                let visible = !*st.overlay_visible.lock().unwrap();
                let _ = mode::set_overlay_visible(app, visible);
            }
        }
    }
}
