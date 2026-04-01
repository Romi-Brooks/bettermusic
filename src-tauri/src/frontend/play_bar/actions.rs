use tauri::command;

use crate::domain::player::PLAYER;
use crate::{app_info};

#[command]
pub fn push_play(file_path: String) -> Result<(), String> {
    let mut _player = PLAYER.lock().unwrap();
    _player.play(&file_path)?;
    Ok(())
}

#[command]
pub fn toggle_play() {
    let mut _player = PLAYER.lock().unwrap();
    if _player.is_playing() {
        _player.pause();
    } else {
        _player.resume();
    };
}

#[command]
pub fn get_play_status() -> bool {
    let _player = PLAYER.lock().unwrap();
    _player.is_playing()
}

#[command]
pub fn push_prev() {
    app_info!("上一曲");
}

#[command]
pub fn push_next() {
    app_info!("下一曲");
}

#[command]
pub fn push_mode(mode: u8) {
    let mut _player = PLAYER.lock().unwrap();
    app_info!("switch the play mode to: {}", mode);
}

#[command]
pub fn is_playing() -> bool {
    let _player = PLAYER.lock().unwrap();
    _player.is_playing()
}

#[command]
pub fn push_stop() {
    let mut _player = PLAYER.lock().unwrap();
    _player.stop();
}

#[command]
pub fn seek_to(time: f32) {
    let mut _player = PLAYER.lock().unwrap();
    _player.seek_to(time);
}

#[command]
pub fn get_duration() -> f32 {
    let mut _player = PLAYER.lock().unwrap();
    _player.get_duration()
}

#[command]
pub fn get_pos() -> u64 {
    let mut _player = PLAYER.lock().unwrap();
    _player.get_current_second()
}