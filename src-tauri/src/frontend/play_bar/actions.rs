use crate::{app_info, app_error, app_warning};

#[tauri::command]
pub fn push_play() {
    app_warning!("You pushed pause btn!");
}

#[tauri::command]
pub fn push_next() {
    app_warning!("You pushed next btn!");
}

#[tauri::command]
pub fn push_prev() {
    app_warning!("You pushed previous btn!");
}

#[tauri::command]
pub fn push_status() {
    app_warning!("You pushed status btn!");
}