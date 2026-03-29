use tauri::command;
use crate::domain::player::PLAYER;
use crate::{app_info, app_error, app_warning};

#[command]
pub fn push_play(file_path: String) -> Result<(), String> {
    let mut player = PLAYER.lock().unwrap();
    player.play(&file_path)?;
    app_info!("播放: {}", file_path);
    Ok(())
}

#[command]
pub fn toggle_play() {
    let mut player = PLAYER.lock().unwrap();
    let status = if player.is_playing() {
        // 当前正在播放，执行暂停
        player.pause();
        "暂停"
    } else {
        // 当前已暂停，执行恢复播放
        player.resume();
        "恢复播放"
    };
    app_info!("{}", status);
}

#[command]
pub fn get_play_status() -> bool {
    let player = PLAYER.lock().unwrap();
    player.is_playing()
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
    let mut player = PLAYER.lock().unwrap();
    app_info!("切换播放模式: {}", mode);
}

#[command]
pub fn push_stop() {
    let mut player = PLAYER.lock().unwrap() ;
    player.stop();
    app_info!("停止播放");
}