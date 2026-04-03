use tauri::{command, AppHandle, Emitter};

use crate::domain::player::{PLAYER, PlayMode};
use crate::{app_info};

// 发送播放器状态变更事件
fn emit_state_change(app: &AppHandle) {
    let player = PLAYER.lock().unwrap();
    let state = player.get_state();
    let _ = app.emit("player-state-changed", state);
}

// 播放指定路径的歌曲
#[command]
pub fn push_play(file_path: String, app: AppHandle) -> Result<(), String> {
    let mut player = PLAYER.lock().map_err(|e| format!("get player failed: {}", e))?;
    player.play(&file_path)?;
    drop(player);
    emit_state_change(&app);
    Ok(())
}

// 切换播放/暂停状态
#[command]
pub fn toggle_play(app: AppHandle) {
    let mut player = PLAYER.lock().unwrap();
    player.toggle_play();
    drop(player);
    emit_state_change(&app);
}

// 获取播放状态
#[command]
pub fn get_play_status() -> bool {
    let player = PLAYER.lock().unwrap();
    player.is_playing()
}

// 播放上一首（使用队列）
#[command]
pub fn push_prev(app: AppHandle) -> Result<(), String> {
    let mut player = PLAYER.lock().map_err(|e| format!("get player failed: {}", e))?;
    player.play_prev()?;
    drop(player);
    app_info!("上一曲");
    emit_state_change(&app);
    Ok(())
}

// 播放下一首（使用队列）
#[command]
pub fn push_next(app: AppHandle) -> Result<(), String> {
    let mut player = PLAYER.lock().map_err(|e| format!("get player failed: {}", e))?;
    player.play_next()?;
    drop(player);
    app_info!("下一曲");
    emit_state_change(&app);
    Ok(())
}

// 设置播放模式
#[command]
pub fn push_mode(mode: u8, app: AppHandle) {
    let mut player = PLAYER.lock().unwrap();
    player.set_play_mode(PlayMode::from(mode));
    drop(player);
    app_info!("switch the play mode to: {}", mode);
    emit_state_change(&app);
}

// 获取当前播放模式
#[command]
pub fn get_play_mode() -> u8 {
    let player = PLAYER.lock().unwrap();
    match player.get_play_mode() {
        PlayMode::Sequential => 0,
        PlayMode::SingleLoop => 1,
        PlayMode::Random => 2,
    }
}

// 停止播放
#[command]
pub fn push_stop(app: AppHandle) {
    let mut player = PLAYER.lock().unwrap();
    player.stop();
    drop(player);
    emit_state_change(&app);
}

// 跳转到指定时间
#[command]
pub fn seek_to(time: f32, app: AppHandle) {
    let mut player = PLAYER.lock().unwrap();
    player.seek_to(time);
    drop(player);
    emit_state_change(&app);
}

// 获取歌曲总时长（秒）
#[command]
pub fn get_duration() -> f32 {
    let player = PLAYER.lock().unwrap();
    player.get_duration()
}

// 获取当前播放位置（秒）
#[command]
pub fn get_pos() -> u64 {
    let player = PLAYER.lock().unwrap();
    player.get_current_second()
}

// 获取播放器完整状态
#[command]
pub fn get_player_state() -> Result<crate::domain::player::PlayerState, String> {
    let player = PLAYER.lock().map_err(|e| format!("get player failed: {}", e))?;
    Ok(player.get_state())
}
