//! Wayland wlr-layer-shell: overlay surfaces that stay in front of the user
//! on every workspace, above fullscreen windows (Hyprland, Sway, ...).
//!
//! A layer surface is bound to an output, so one hidden window per monitor is
//! created and turned into a layer surface **before it is mapped**
//! (`gtk_layer_init_for_window` refuses already-mapped windows). The regular
//! xdg windows (macOS path / X11 sticky) are used as a fallback when the
//! compositor has no layer-shell support (e.g. GNOME's Mutter).

use std::sync::OnceLock;

use gtk_layer_shell::{Edge, Layer, LayerShell};
use tauri::{AppHandle, Manager, Monitor, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

use crate::mode::{overlay_geometry, OVERLAY_LABEL_PREFIX};

static SUPPORTED: OnceLock<bool> = OnceLock::new();

/// Probes layer-shell support. Must be called once from the main thread
/// during app setup (may block on a Wayland roundtrip).
pub fn probe_support() {
    let _ = SUPPORTED.set(gtk_layer_shell::is_supported());
}

/// Whether the current session supports layer-shell surfaces.
pub fn supported() -> bool {
    SUPPORTED.get().copied().unwrap_or(false)
}

/// Creates one overlay layer window per monitor (the main window is hidden
/// by the caller). All gtk work runs on the main thread.
pub fn spawn_overlays(app: &AppHandle) {
    let Ok(monitors) = app.available_monitors() else {
        return;
    };
    let app = app.clone();
    let _ = app.run_on_main_thread(move || {
        for (i, mon) in monitors.iter().enumerate() {
            let label = format!("{OVERLAY_LABEL_PREFIX}{i}");
            if let Some(win) = app.get_webview_window(&label) {
                let _ = win.show();
                continue;
            }
            let (_, _, w, h) = overlay_geometry(mon);
            let scale = mon.scale_factor();
            let Ok(win) = WebviewWindowBuilder::new(&app, &label, WebviewUrl::default())
                .title("Type Overlay")
                .transparent(true)
                .decorations(false)
                .shadow(false)
                .resizable(false)
                .skip_taskbar(true)
                .focused(false)
                // hidden: must stay unmapped until it becomes a layer surface
                .visible(false)
                .inner_size(w as f64 / scale, h as f64 / scale)
                .build()
            else {
                continue;
            };
            init_surface(&win, mon);
            let _ = win.set_ignore_cursor_events(true);
            let _ = win.show();
        }
    });
}

/// Turns a not-yet-mapped window into an overlay layer surface on `mon`:
/// stretched across ~90% of the width, keyboard aspect ratio, biased to the
/// top (the same geometry `overlay_geometry` computes for regular windows).
fn init_surface(win: &WebviewWindow, mon: &Monitor) {
    let Ok(gtk_win) = win.gtk_window() else {
        return;
    };
    gtk_win.init_layer_shell();
    gtk_win.set_layer(Layer::Overlay);
    gtk_win.set_namespace("type-overlay");
    // visible above fullscreen surfaces, does not reserve screen space;
    // keyboard interactivity stays off (the default) — the overlay is
    // click-through and keys come from the global listener
    gtk_win.set_exclusive_zone(-1);

    let (_, _, w, h) = overlay_geometry(mon);
    let size = mon.size();
    let scale = mon.scale_factor();
    let side = (((size.width as i32 - w as i32) / 2) as f64 / scale).round() as i32;
    let top = (((size.height as i32 - h as i32) / 4) as f64 / scale).round() as i32;

    gtk_win.set_anchor(Edge::Top, true);
    gtk_win.set_anchor(Edge::Left, true);
    gtk_win.set_anchor(Edge::Right, true);
    gtk_win.set_layer_shell_margin(Edge::Top, top);
    gtk_win.set_layer_shell_margin(Edge::Left, side);
    gtk_win.set_layer_shell_margin(Edge::Right, side);

    if let Some(gmon) = find_monitor(mon) {
        gtk_win.set_monitor(&gmon);
    }
}

/// Finds the gdk monitor matching a tauri one by comparing physical geometry.
fn find_monitor(mon: &Monitor) -> Option<gdk::Monitor> {
    let display = gdk::Display::default()?;
    let pos = mon.position();
    let size = mon.size();
    for i in 0..display.n_monitors() {
        let Some(m) = display.monitor(i) else {
            continue;
        };
        let geom = m.geometry();
        let scale = m.scale_factor();
        if geom.x() * scale == pos.x
            && geom.y() * scale == pos.y
            && geom.width() * scale == size.width as i32
            && geom.height() * scale == size.height as i32
        {
            return Some(m);
        }
    }
    None
}
