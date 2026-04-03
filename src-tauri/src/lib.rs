pub mod infra {
    pub mod logger;
    pub mod metadata;
}
pub use infra::*;

pub mod frontend {
    pub mod play_bar;
    pub mod my_songs;
    pub mod my_songs_play;
}
pub use frontend::*;

pub mod domain {
    pub mod player;
    pub mod song_list;
}
pub use domain::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    logger::init_logger("info")
        .expect("Failed to initialize logger");

    app_info!("Starting BetterMusic Tauri app...");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            // play bar
            play_bar::actions::push_play,
            play_bar::actions::push_next,
            play_bar::actions::push_prev,
            play_bar::actions::push_mode,
            play_bar::actions::get_play_mode,
            play_bar::actions::toggle_play,
            play_bar::actions::get_duration,
            play_bar::actions::get_pos,
            play_bar::actions::seek_to,
            play_bar::actions::get_player_state,
            // my songs
            my_songs::actions::get_my_songs,
            my_songs::actions::download_songs,
            my_songs::actions::rescan_music_directory,
            // my songs play
            my_songs_play::actions::play_song_from_list,
            my_songs_play::actions::play_song_by_path,
            my_songs_play::actions::play_all_from_index,
            my_songs_play::actions::get_current_song,
            my_songs_play::actions::get_next_song,
            my_songs_play::actions::get_prev_song,
            my_songs_play::actions::play_next_song,
            my_songs_play::actions::play_prev_song,
            my_songs_play::actions::set_play_mode,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
