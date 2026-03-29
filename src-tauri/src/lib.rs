pub mod infra {
    pub mod logger;

}
pub use infra::*;

pub mod frontend{
    pub mod play_bar;
}
pub use frontend::*;

pub mod domain {
    pub mod player;
}
pub use domain::*;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    logger::init_logger("info")
        .expect("Failed to initialize logger");

    app_info!("Starting BetterMusic Tauri app...");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            play_bar::actions::push_play,
            play_bar::actions::push_next,
            play_bar::actions::push_prev,
            play_bar::actions::push_mode,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}