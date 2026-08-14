use tauri::{
    AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize, WebviewUrl,
    WebviewWindowBuilder,
};

use crate::models::{AppMode, ModeEvent};
use crate::state::AppState;

/// Label prefix of the secondary overlay windows (one per extra monitor).
const OVERLAY_LABEL_PREFIX: &str = "overlay-";

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

/// Geometry (x, y, w, h) of the overlay for a given monitor.
fn overlay_geometry(mon: &tauri::Monitor) -> (i32, i32, u32, u32) {
    let mon_w = mon.size().width as i32;
    let mon_h = mon.size().height as i32;
    let w = ((mon_w as f64) * OVERLAY_WIDTH_RATIO).round() as i32;
    let h = ((w as f64) * KEY_ASPECT).round() as i32;
    let x = mon.position().x + (mon_w - w) / 2;
    let y = mon.position().y + (mon_h - h) / 4;
    (x, y, w as u32, h as u32)
}

fn close_overlay_windows(app: &AppHandle) {
    for (label, win) in app.webview_windows() {
        if label.starts_with(OVERLAY_LABEL_PREFIX) {
            let _ = win.close();
        }
    }
}

/// Spawns a secondary overlay window on every monitor except the one covered
/// by the main overlay window (`skip_pos` = position of that monitor).
fn spawn_overlay_windows(app: &AppHandle, skip_pos: PhysicalPosition<i32>) {
    let Ok(monitors) = app.available_monitors() else {
        return;
    };
    for (i, mon) in monitors.iter().enumerate() {
        if *mon.position() == skip_pos {
            continue;
        }
        let label = format!("{OVERLAY_LABEL_PREFIX}{i}");
        let (x, y, w, h) = overlay_geometry(mon);
        let app = app.clone();
        let _ = app.clone().run_on_main_thread(move || {
            if let Some(win) = app.get_webview_window(&label) {
                let _ = win.set_position(PhysicalPosition::new(x, y));
                let _ = win.set_size(PhysicalSize::new(w, h));
                let _ = win.show();
                return;
            }
            let Ok(win) = WebviewWindowBuilder::new(&app, &label, WebviewUrl::default())
                .title("Type Overlay")
                .transparent(true)
                .decorations(false)
                .shadow(false)
                .resizable(false)
                .skip_taskbar(true)
                .always_on_top(true)
                .focused(false)
                .visible(false)
                .position(x as f64, y as f64)
                .inner_size(w as f64, h as f64)
                .build()
            else {
                return;
            };
            let _ = win.set_ignore_cursor_events(true);
            let _ = win.show();
        });
    }
}

pub fn apply_mode(app: &AppHandle, mode: AppMode) -> tauri::Result<()> {
    let win = main_window(app);
    match mode {
        AppMode::Window => {
            close_overlay_windows(app);
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
            let mut skip_pos = PhysicalPosition::new(0, 0);
            if let Ok(Some(monitor)) = win.current_monitor() {
                let (x, y, w, h) = overlay_geometry(&monitor);
                skip_pos = *monitor.position();
                win.set_size(PhysicalSize::new(w, h))?;
                win.set_position(PhysicalPosition::new(x, y))?;
            }
            spawn_overlay_windows(app, skip_pos);
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
    for (label, overlay) in app.webview_windows() {
        if !label.starts_with(OVERLAY_LABEL_PREFIX) {
            continue;
        }
        if visible {
            // no focus: secondary overlays stay click-through
            let _ = overlay.show();
        } else {
            let _ = overlay.hide();
        }
    }
    let st = app.state::<AppState>();
    *st.overlay_visible.lock().unwrap() = visible;
    Ok(())
}
