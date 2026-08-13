use std::sync::Mutex;

use crate::models::AppMode;

pub struct AppState {
    pub mode: Mutex<AppMode>,
    pub overlay_visible: Mutex<bool>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            mode: Mutex::new(AppMode::Window),
            overlay_visible: Mutex::new(true),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
