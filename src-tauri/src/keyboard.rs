use tauri::AppHandle;

#[cfg(target_os = "macos")]
#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn AXIsProcessTrusted() -> bool;
}

pub fn is_trusted() -> bool {
    #[cfg(target_os = "macos")]
    {
        unsafe { AXIsProcessTrusted() }
    }
    #[cfg(not(target_os = "macos"))]
    {
        true
    }
}

pub fn capture_status() -> &'static str {
    #[cfg(target_os = "macos")]
    {
        if mac::installed() {
            "active"
        } else {
            "waiting"
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        "active"
    }
}

pub fn spawn_listener(app: AppHandle) {
    #[cfg(target_os = "macos")]
    {
        crate::keyboard::mac::spawn(app);
    }
    #[cfg(not(target_os = "macos"))]
    {
        crate::keyboard::rdev_listener::spawn(app);
    }
}

// ---------------------------------------------------------------------------
// macOS: a hand-rolled CGEventTap. rdev's macOS backend calls HIToolbox (TSM)
// to resolve key names from inside its event-tap callback, which on modern
// macOS asserts it runs on the main queue and traps (SIGTRAP). We avoid TSM
// entirely and only read the virtual keyCode, scheduling the tap on the main
// run loop alongside Tauri's event loop.
// ---------------------------------------------------------------------------
#[cfg(target_os = "macos")]
mod mac {
    use std::collections::HashSet;
    use std::ffi::c_void;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    use core_foundation::base::TCFType;
    use core_foundation::boolean::CFBoolean;
    use core_foundation::dictionary::CFDictionary;
    use core_foundation::string::CFString;

    use tauri::{AppHandle, Emitter};

    use crate::models::{CaptureStatusEvent, KeyEvent};

    type CFAllocatorRef = *const c_void;
    type CFMachPortRef = *mut c_void;
    type CFRunLoopRef = *mut c_void;
    type CFRunLoopSourceRef = *mut c_void;
    type CGEventRef = *mut c_void;
    type CGEventTapProxy = *mut c_void;

    const K_CG_EVENT_KEY_DOWN: u32 = 10;
    const K_CG_EVENT_KEY_UP: u32 = 11;
    const K_CG_EVENT_FLAGS_CHANGED: u32 = 12;
    const K_CG_SESSION_EVENT_TAP: u32 = 1;
    const K_CG_HEAD_INSERT_EVENT_TAP: u32 = 0;
    // Listen-only tap pairs with Input Monitoring (the privacy category for
    // reading keystrokes). We request that access explicitly below.
    const K_CG_EVENT_TAP_OPTION_LISTEN_ONLY: u32 = 1;
    const K_CG_KEYBOARD_EVENT_KEYCODE: u32 = 9;

    type CGEventTapCallBack = unsafe extern "C" fn(
        proxy: CGEventTapProxy,
        event_type: u32,
        event: CGEventRef,
        user_info: *mut c_void,
    ) -> CGEventRef;

    #[link(name = "ApplicationServices", kind = "framework")]
    extern "C" {
        fn CGEventTapCreate(
            tap: u32,
            place: u32,
            options: u32,
            events_of_interest: u64,
            callback: CGEventTapCallBack,
            user_info: *mut c_void,
        ) -> CFMachPortRef;
        fn CGEventGetIntegerValueField(event: CGEventRef, field: u32) -> i64;
        fn CGRequestListenEventAccess() -> bool;
        fn AXIsProcessTrustedWithOptions(options: *const c_void) -> bool;
        fn CFMachPortCreateRunLoopSource(
            alloc: CFAllocatorRef,
            port: CFMachPortRef,
            order: isize,
        ) -> CFRunLoopSourceRef;
        fn CFRunLoopGetMain() -> CFRunLoopRef;
        fn CFRunLoopAddSource(
            rl: CFRunLoopRef,
            source: CFRunLoopSourceRef,
            mode: *const c_void,
        );
        static kCFRunLoopCommonModes: *const c_void;
    }

    struct TapState {
        app: AppHandle,
        mods_down: Mutex<HashSet<u64>>,
    }

    // Raw pointers are not Send; wrap the leaked state pointer so it can cross
    // into the main-thread closure.
    #[derive(Clone, Copy)]
    struct SendPtr(*mut c_void);
    unsafe impl Send for SendPtr {}

    unsafe extern "C" fn tap_callback(
        _proxy: CGEventTapProxy,
        event_type: u32,
        event: CGEventRef,
        user_info: *mut c_void,
    ) -> CGEventRef {
        let state = &*(user_info as *const TapState);
        let keycode = CGEventGetIntegerValueField(event, K_CG_KEYBOARD_EVENT_KEYCODE) as u64;

        let (name, pressed) = match event_type {
            K_CG_EVENT_KEY_DOWN => ("key-down", true),
            K_CG_EVENT_KEY_UP => ("key-up", false),
            K_CG_EVENT_FLAGS_CHANGED => {
                let mut set = state.mods_down.lock().unwrap();
                let was_down = set.contains(&keycode);
                if was_down {
                    set.remove(&keycode);
                } else {
                    set.insert(keycode);
                }
                (if was_down { "key-up" } else { "key-down" }, !was_down)
            }
            _ => return event,
        };

        let _ = pressed;
        if let Some(code) = keycode_to_code(keycode) {
            let _ = state
                .app
                .emit(name, KeyEvent { code: code.to_string() });
        }
        event
    }

    fn keycode_to_code(keycode: u64) -> Option<&'static str> {
        Some(match keycode {
            53 => "Escape",
            122 => "F1",
            120 => "F2",
            99 => "F3",
            118 => "F4",
            96 => "F5",
            97 => "F6",
            98 => "F7",
            100 => "F8",
            101 => "F9",
            109 => "F10",
            103 => "F11",
            111 => "F12",
            50 => "Backquote",
            18 => "Digit1",
            19 => "Digit2",
            20 => "Digit3",
            21 => "Digit4",
            23 => "Digit5",
            22 => "Digit6",
            26 => "Digit7",
            28 => "Digit8",
            25 => "Digit9",
            29 => "Digit0",
            27 => "Minus",
            24 => "Equal",
            51 => "Backspace",
            114 => "Insert",
            115 => "Home",
            119 => "End",
            116 => "PageUp",
            121 => "PageDown",
            117 => "Delete",
            48 => "Tab",
            12 => "KeyQ",
            13 => "KeyW",
            14 => "KeyE",
            15 => "KeyR",
            17 => "KeyT",
            16 => "KeyY",
            32 => "KeyU",
            34 => "KeyI",
            31 => "KeyO",
            35 => "KeyP",
            33 => "BracketLeft",
            30 => "BracketRight",
            42 => "Backslash",
            57 => "CapsLock",
            0 => "KeyA",
            1 => "KeyS",
            2 => "KeyD",
            3 => "KeyF",
            5 => "KeyG",
            4 => "KeyH",
            38 => "KeyJ",
            40 => "KeyK",
            37 => "KeyL",
            41 => "Semicolon",
            39 => "Quote",
            36 => "Enter",
            56 => "ShiftLeft",
            60 => "ShiftRight",
            6 => "KeyZ",
            7 => "KeyX",
            8 => "KeyC",
            9 => "KeyV",
            11 => "KeyB",
            45 => "KeyN",
            46 => "KeyM",
            43 => "Comma",
            47 => "Period",
            44 => "Slash",
            126 => "ArrowUp",
            59 => "ControlLeft",
            62 => "ControlRight",
            55 => "MetaLeft",
            54 => "MetaRight",
            58 => "AltLeft",
            61 => "AltRight",
            63 => "Fn",
            49 => "Space",
            123 => "ArrowLeft",
            125 => "ArrowDown",
            124 => "ArrowRight",
            _ => return None,
        })
    }

    const STATUS_WAITING: &str = "waiting";
    const STATUS_ACTIVE: &str = "active";

    static TAP_INSTALLED: AtomicBool = AtomicBool::new(false);

    pub(super) fn installed() -> bool {
        TAP_INSTALLED.load(Ordering::Acquire)
    }

    // Fires the Accessibility system prompt when `prompt` is true and reports
    // the current trust state. The OS only honours the prompt while a decision
    // is still pending, so the caller enables it exactly once.
    unsafe fn request_accessibility(prompt: bool) -> bool {
        let key = CFString::new("AXTrustedCheckOptionPrompt");
        let value = CFBoolean::from(prompt);
        let options = CFDictionary::from_CFType_pairs(&[(key, value)]);
        AXIsProcessTrustedWithOptions(options.as_CFTypeRef())
    }

    pub fn spawn(app: AppHandle) {
        let state = Box::new(TapState {
            app: app.clone(),
            mods_down: Mutex::new(HashSet::new()),
        });
        let state_ptr = SendPtr(Box::into_raw(state) as *mut c_void);

        let mask: u64 = (1u64 << K_CG_EVENT_KEY_DOWN)
            | (1u64 << K_CG_EVENT_KEY_UP)
            | (1u64 << K_CG_EVENT_FLAGS_CHANGED);

        // The permission prompts fire asynchronously and return `false` right
        // away on first launch, so a single install attempt always fails until
        // the user grants permission. Retrying on a timer lets the tap come up
        // (and the UI unblock) without a manual restart.
        let last_status: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
        let should_prompt = Arc::new(AtomicBool::new(true));

        std::thread::spawn(move || loop {
            if installed() {
                return;
            }
            let last_status = last_status.clone();
            let should_prompt = should_prompt.clone();
            let emit_app = app.clone();
            let _ = app.run_on_main_thread(move || {
                let prompt = should_prompt.swap(false, Ordering::SeqCst);
                let status = unsafe { try_install(state_ptr, mask, prompt) };
                if status == STATUS_ACTIVE {
                    TAP_INSTALLED.store(true, Ordering::Release);
                }
                let mut last = last_status.lock().unwrap();
                if last.as_deref() != Some(status) {
                    *last = Some(status.to_string());
                    drop(last);
                    let _ = emit_app.emit(
                        "capture-status",
                        CaptureStatusEvent {
                            status: status.to_string(),
                        },
                    );
                }
            });
            std::thread::sleep(Duration::from_millis(1500));
        });
    }

    unsafe fn try_install(state_ptr: SendPtr, mask: u64, prompt: bool) -> &'static str {
        // Accessibility — required by CGEventTapCreate even for listen-only taps.
        if !request_accessibility(prompt) {
            return STATUS_WAITING;
        }
        // Input Monitoring — the privacy category for reading keystrokes.
        if !CGRequestListenEventAccess() {
            return STATUS_WAITING;
        }
        let port = CGEventTapCreate(
            K_CG_SESSION_EVENT_TAP,
            K_CG_HEAD_INSERT_EVENT_TAP,
            K_CG_EVENT_TAP_OPTION_LISTEN_ONLY,
            mask,
            tap_callback,
            state_ptr.0,
        );
        if port.is_null() {
            return STATUS_WAITING;
        }
        let source = CFMachPortCreateRunLoopSource(std::ptr::null(), port, 0);
        if source.is_null() {
            eprintln!("[keyboard] could not create run loop source");
            return STATUS_WAITING;
        }
        CFRunLoopAddSource(CFRunLoopGetMain(), source, kCFRunLoopCommonModes);
        STATUS_ACTIVE
    }
}

// ---------------------------------------------------------------------------
// Windows / Linux: rdev works fine here (no HIToolbox main-queue trap).
// ---------------------------------------------------------------------------
#[cfg(not(target_os = "macos"))]
mod rdev_listener {
    use rdev::{listen, EventType, Key};
    use tauri::{AppHandle, Emitter};

    use crate::models::KeyEvent;

    fn key_to_code(key: Key) -> Option<&'static str> {
        use Key::*;
        Some(match key {
            Alt => "AltLeft",
            AltGr => "AltRight",
            Backspace => "Backspace",
            CapsLock => "CapsLock",
            ControlLeft => "ControlLeft",
            ControlRight => "ControlRight",
            Delete => "Delete",
            DownArrow => "ArrowDown",
            End => "End",
            Escape => "Escape",
            Home => "Home",
            Insert => "Insert",
            LeftArrow => "ArrowLeft",
            RightArrow => "ArrowRight",
            UpArrow => "ArrowUp",
            PageDown => "PageDown",
            PageUp => "PageUp",
            Pause => "Pause",
            PrintScreen => "PrintScreen",
            ScrollLock => "ScrollLock",
            NumLock => "NumLock",
            Return => "Enter",
            Space => "Space",
            Tab => "Tab",
            Function => "Fn",
            F1 => "F1",
            F2 => "F2",
            F3 => "F3",
            F4 => "F4",
            F5 => "F5",
            F6 => "F6",
            F7 => "F7",
            F8 => "F8",
            F9 => "F9",
            F10 => "F10",
            F11 => "F11",
            F12 => "F12",
            BackQuote => "Backquote",
            Minus => "Minus",
            Equal => "Equal",
            LeftBracket => "BracketLeft",
            RightBracket => "BracketRight",
            BackSlash => "Backslash",
            SemiColon => "Semicolon",
            Quote => "Quote",
            Comma => "Comma",
            Dot => "Period",
            Slash => "Slash",
            Num0 => "Digit0",
            Num1 => "Digit1",
            Num2 => "Digit2",
            Num3 => "Digit3",
            Num4 => "Digit4",
            Num5 => "Digit5",
            Num6 => "Digit6",
            Num7 => "Digit7",
            Num8 => "Digit8",
            Num9 => "Digit9",
            KeyA => "KeyA",
            KeyB => "KeyB",
            KeyC => "KeyC",
            KeyD => "KeyD",
            KeyE => "KeyE",
            KeyF => "KeyF",
            KeyG => "KeyG",
            KeyH => "KeyH",
            KeyI => "KeyI",
            KeyJ => "KeyJ",
            KeyK => "KeyK",
            KeyL => "KeyL",
            KeyM => "KeyM",
            KeyN => "KeyN",
            KeyO => "KeyO",
            KeyP => "KeyP",
            KeyQ => "KeyQ",
            KeyR => "KeyR",
            KeyS => "KeyS",
            KeyT => "KeyT",
            KeyU => "KeyU",
            KeyV => "KeyV",
            KeyW => "KeyW",
            KeyX => "KeyX",
            KeyY => "KeyY",
            KeyZ => "KeyZ",
            MetaLeft => "MetaLeft",
            MetaRight => "MetaRight",
            ShiftLeft => "ShiftLeft",
            ShiftRight => "ShiftRight",
            _ => return None,
        })
    }

    fn emit_key(app: &AppHandle, name: &str, key: Key) {
        if let Some(code) = key_to_code(key) {
            let _ = app.emit(name, KeyEvent { code: code.to_string() });
        }
    }

    pub fn spawn(app: AppHandle) {
        std::thread::spawn(move || {
            let callback = move |event: rdev::Event| match event.event_type {
                EventType::KeyPress(k) => emit_key(&app, "key-down", k),
                EventType::KeyRelease(k) => emit_key(&app, "key-up", k),
                _ => {}
            };
            if let Err(e) = listen(callback) {
                eprintln!("[keyboard] global listen failed: {e:?}");
            }
        });
    }
}
