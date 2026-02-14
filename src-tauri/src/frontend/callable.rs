use crate::{app_info, app_error, app_warning, app_debug};

#[tauri::command]
pub fn greet(name: &str) -> String {
    if name.is_empty() {
        app_info!("Called the greet with empty name.");
        "You're Nobody?".to_string()
    } else {
        app_info!("Called the greet with name {}.", name);
        format!("Hello, {}! This is the App from the Tauri!", name)
    }
}