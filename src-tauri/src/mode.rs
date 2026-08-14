use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize};

use crate::models::{AppMode, ModeEvent};
use crate::state::AppState;

const WINDOW_W: u32 = 1180;
const WINDOW_H: u32 = 720;
const WINDOW_MIN_W: u32 = 980;
const WINDOW_MIN_H: u32 = 620;

/// Overlay fills ~90% of the screen width and keeps the keyboard aspect ratio
/// (BASE_W:BASE_H = 878:403), horizontally centered and biased toward the
/// top of the monitor (top margin = 1/4 of the leftover vertical space).
const OVERLAY_WIDTH_RATIO: f64 = 0.9;
const KEY_ASPECT: f64 = 403.0 / 878.0;

fn main_window(app: &AppHandle) -> tauri::WebviewWindow {
    app.get_webview_window("main")
        .expect("main window not found")
}

pub fn apply_mode(app: &AppHandle, mode: AppMode) -> tauri::Result<()> {
    let win = main_window(app);
    match mode {
        AppMode::Window => {
            win.set_always_on_top(false)?;
            win.set_skip_taskbar(false)?;
            win.set_ignore_cursor_events(false)?;
            win.set_resizable(true)?;
            win.set_decorations(true)?;
            win.set_shadow(true)?;
            win.set_min_size(Some(PhysicalSize::new(WINDOW_MIN_W, WINDOW_MIN_H)))?;
            win.set_size(PhysicalSize::new(WINDOW_W, WINDOW_H))?;
            win.center()?;
            win.show()?;
            win.set_focus()?;
        }
        AppMode::Overlay => {
            win.set_always_on_top(true)?;
            win.set_resizable(false)?;
            win.set_decorations(false)?;
            // no DWM shadow/rounded frame around the transparent overlay
            win.set_shadow(false)?;
            win.set_skip_taskbar(true)?;
            // clear min size so the short overlay height is not clamped
            win.set_min_size::<PhysicalSize<u32>>(None)?;
            if let Ok(Some(monitor)) = win.current_monitor() {
                let mon_size = monitor.size();
                let mon_pos = monitor.position();
                let mon_w = mon_size.width as i32;
                let mon_h = mon_size.height as i32;
                let w = ((mon_w as f64) * OVERLAY_WIDTH_RATIO).round() as i32;
                let h = ((w as f64) * KEY_ASPECT).round() as i32;
                let x = mon_pos.x + (mon_w - w) / 2;
                let y = mon_pos.y + (mon_h - h) / 4;
                win.set_size(PhysicalSize::new(w as u32, h as u32))?;
                win.set_position(PhysicalPosition::new(x, y))?;
            }
            win.set_ignore_cursor_events(true)?;
            win.show()?;
            win.set_focus()?;
        }
    }

    let st = app.state::<AppState>();
    *st.mode.lock().unwrap() = mode;
    if mode == AppMode::Overlay {
        *st.overlay_visible.lock().unwrap() = true;
    }

    let _ = app.emit("mode-changed", ModeEvent { mode });
    Ok(())
}

pub fn set_overlay_visible(app: &AppHandle, visible: bool) -> tauri::Result<()> {
    let win = main_window(app);
    if visible {
        win.show()?;
        win.set_focus()?;
    } else {
        win.hide()?;
    }
    let st = app.state::<AppState>();
    *st.overlay_visible.lock().unwrap() = visible;
    Ok(())
}
