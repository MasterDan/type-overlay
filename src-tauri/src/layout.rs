use std::sync::{Arc, Mutex};
use std::time::Duration;

use tauri::{AppHandle, Emitter};

use crate::models::LayoutEvent;

#[cfg(target_os = "macos")]
mod platform {
    use core_foundation::base::{CFTypeRef, TCFType};
    use core_foundation::string::{CFString, CFStringRef};

    type TISInputSourceRef = *mut std::ffi::c_void;

    #[link(name = "Carbon", kind = "framework")]
    extern "C" {
        fn TISCopyCurrentKeyboardInputSource() -> TISInputSourceRef;
        fn TISGetInputSourceProperty(
            source: TISInputSourceRef,
            key: CFStringRef,
        ) -> CFTypeRef;
    }

    fn input_source_id() -> Option<String> {
        unsafe {
            let source = TISCopyCurrentKeyboardInputSource();
            if source.is_null() {
                return None;
            }
            let key = CFString::new("TISPropertyInputSourceID");
            let prop = TISGetInputSourceProperty(source, key.as_concrete_TypeRef());
            if prop.is_null() {
                return None;
            }
            let cf_string = CFString::wrap_under_get_rule(prop as CFStringRef);
            Some(cf_string.to_string())
        }
    }

    pub fn current_layout() -> String {
        match input_source_id() {
            Some(id) if id.to_ascii_lowercase().contains("russian") => "ru".into(),
            _ => "en".into(),
        }
    }
}

#[cfg(target_os = "windows")]
mod platform {
    #[link(name = "user32")]
    extern "system" {
        fn GetForegroundWindow() -> isize;
        fn GetWindowThreadProcessId(hwnd: isize, process_id: *mut u32) -> u32;
        fn GetKeyboardLayout(thread_id: u32) -> isize;
    }

    /// 0x0419 = Russian (Russia). Anything else is treated as English.
    pub fn current_layout() -> String {
        unsafe {
            let hwnd = GetForegroundWindow();
            if hwnd == 0 {
                return "en".into();
            }
            let thread_id = GetWindowThreadProcessId(hwnd, std::ptr::null_mut());
            let hkl = GetKeyboardLayout(thread_id);
            let language_id = (hkl as u32) & 0xffff;
            if language_id == 0x0419 {
                "ru".into()
            } else {
                "en".into()
            }
        }
    }
}

#[cfg(target_os = "linux")]
mod platform {
    /// On Wayland there is no portable way to read the *active* layout, so we
    /// fall back to the configured default layout. Runtime layout switching is
    /// handled by the manual toggle in the UI.
    pub fn current_layout() -> String {
        let configured = std::env::var("XKB_DEFAULT_LAYOUT").unwrap_or_default();
        if configured.to_ascii_lowercase().contains("ru") {
            "ru".into()
        } else {
            "en".into()
        }
    }
}

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
mod platform {
    pub fn current_layout() -> String {
        "en".into()
    }
}

pub fn current_layout() -> String {
    platform::current_layout()
}

pub fn spawn_poller(app: AppHandle) {
    std::thread::spawn(move || {
        let last: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
        loop {
            std::thread::sleep(Duration::from_millis(800));
            let app_for_closure = app.clone();
            let last = last.clone();
            let _ = app.run_on_main_thread(move || {
                let current = current_layout();
                let mut prev = last.lock().unwrap();
                if current != *prev {
                    *prev = current.clone();
                    drop(prev);
                    let _ = app_for_closure.emit("layout-changed", LayoutEvent { layout: current });
                }
            });
        }
    });
}
