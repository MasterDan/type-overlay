//! macOS Spaces: keeps the overlay in front of the user at all times.
//!
//! Three things are needed on top of tao's `set_visible_on_all_workspaces`
//! (which only sets `CanJoinAllSpaces` and does not cover fullscreen Spaces):
//! - the full behavior mask `CanJoinAllSpaces | FullScreenAuxiliary`, applied
//!   in a single message with the whole mask replaced (conflicting flags such
//!   as `MoveToActiveSpace` cannot survive an OR-merge);
//! - the status window level, above the auto-hiding menu bar;
//! - the `.accessory` activation policy for the duration of the overlay
//!   session: with the default `.regular` policy macOS keeps an app's windows
//!   tied to the app's active Space (the same trick Electron uses to make
//!   `visibleOnAllWorkspaces` work).

use objc2::msg_send;
use objc2::runtime::AnyObject;
use tauri::{ActivationPolicy, AppHandle, Manager, WebviewWindow};

/// NSWindowCollectionBehaviorCanJoinAllSpaces (1 << 0).
const CAN_JOIN_ALL_SPACES: usize = 1 << 0;
/// NSWindowCollectionBehaviorFullScreenAuxiliary (1 << 8): the window may be
/// shown alongside the Spaces created by fullscreen apps.
const FULLSCREEN_AUXILIARY: usize = 1 << 8;
/// NSStatusWindowLevel (25) — above the menu bar (24) and the Dock.
const STATUS_WINDOW_LEVEL: i64 = 25;

const PINNED: usize = CAN_JOIN_ALL_SPACES | FULLSCREEN_AUXILIARY;

fn ns_window(win: &WebviewWindow) -> Option<*mut AnyObject> {
    win.ns_window().ok().map(|p| p as *mut AnyObject)
}

/// Pins one window to all Spaces, including fullscreen ones, and raises it
/// above the menu bar. Must be called on the main thread.
pub fn pin_window(win: &WebviewWindow) {
    let Some(ns) = ns_window(win) else {
        eprintln!("[type-overlay] pin: no ns_window");
        return;
    };
    unsafe {
        let before: usize = msg_send![ns, collectionBehavior];
        // whole-mask replace: clears MoveToActiveSpace & friends that would
        // otherwise fight CanJoinAllSpaces
        let _: () = msg_send![ns, setCollectionBehavior: PINNED];
        let _: () = msg_send![ns, setLevel: STATUS_WINDOW_LEVEL];
        let _: () = msg_send![ns, setHidesOnDeactivate: false];
        // re-evaluate Space membership now that the behavior changed
        let _: () = msg_send![ns, orderFrontRegardless];
        let after: usize = msg_send![ns, collectionBehavior];
        eprintln!("[type-overlay] pin: behavior {before:#x} -> {after:#x} (want {PINNED:#x})");
    }
}

/// Reverses `pin_window`. The window level is restored separately by
/// `set_always_on_top(false)` in the Window-mode branch. Must be called on
/// the main thread.
pub fn unpin_window(win: &WebviewWindow) {
    let Some(ns) = ns_window(win) else {
        eprintln!("[type-overlay] unpin: no ns_window");
        return;
    };
    unsafe {
        let before: usize = msg_send![ns, collectionBehavior];
        let _: () = msg_send![ns, setCollectionBehavior: 0usize];
        let after: usize = msg_send![ns, collectionBehavior];
        eprintln!("[type-overlay] unpin: behavior {before:#x} -> {after:#x}");
    }
}

/// Starts the overlay session: switches the app to the accessory policy and
/// pins the main window. Must be called on the main thread.
pub fn enter_overlay(app: &AppHandle) {
    if let Err(e) = app.set_activation_policy(ActivationPolicy::Accessory) {
        eprintln!("[type-overlay] accessory policy failed: {e}");
    }
    if let Some(win) = app.get_webview_window("main") {
        pin_window(&win);
    }
}

/// Ends the overlay session: unpins the main window and returns the app to
/// the regular policy (Dock icon back). Must be called on the main thread.
pub fn exit_overlay(app: &AppHandle) {
    if let Some(win) = app.get_webview_window("main") {
        unpin_window(&win);
    }
    let _ = app.set_activation_policy(ActivationPolicy::Regular);
}
