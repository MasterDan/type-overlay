use std::collections::BTreeSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

use tauri::{AppHandle, Emitter};

use crate::models::PressedEvent;

// Source of truth for the pressed-keys highlight. The backend owns the set
// and emits full snapshots ordered by a monotonically increasing `seq`, so
// the frontend stays correct even when the OS swallows key-ups (a global
// hotkey combo is consumed by the OS shortcut handler) or when events from
// different threads are delivered out of order — a dropped or stale
// snapshot can never resurrect a released key.
static KEYS: Mutex<BTreeSet<String>> = Mutex::new(BTreeSet::new());
static SEQ: AtomicU64 = AtomicU64::new(0);

fn emit_snapshot(app: &AppHandle, seq: u64, keys: &BTreeSet<String>) {
    let _ = app.emit(
        "pressed-changed",
        PressedEvent {
            seq,
            codes: keys.iter().cloned().collect(),
        },
    );
}

pub fn set(app: &AppHandle, code: &str, down: bool) {
    let (seq, snapshot) = {
        let mut keys = KEYS.lock().unwrap();
        // Auto-repeat fires key-down over and over; only real transitions
        // bump the sequence and emit.
        let changed = if down {
            keys.insert(code.to_string())
        } else {
            keys.remove(code)
        };
        if !changed {
            return;
        }
        let seq = SEQ.fetch_add(1, Ordering::SeqCst) + 1;
        (seq, keys.clone())
    };
    emit_snapshot(app, seq, &snapshot);
}

pub fn clear(app: &AppHandle) {
    let (seq, snapshot) = {
        let mut keys = KEYS.lock().unwrap();
        if keys.is_empty() {
            return;
        }
        keys.clear();
        let seq = SEQ.fetch_add(1, Ordering::SeqCst) + 1;
        (seq, keys.clone())
    };
    emit_snapshot(app, seq, &snapshot);
}
