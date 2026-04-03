use tauri::{command, Emitter, AppHandle};

use crate::domain::player::{PLAYER, PlayMode};
use crate::domain::song_list::{SONG_LIST_MANAGER, Song};
use crate::{app_info, app_error};

// 播放指定歌曲（从歌曲列表）
// 前端调用: invoke('play_song_from_list', { songId: string })
// 功能：双击歌曲时播放该歌曲，并设置播放列表为全部歌曲
#[command]
pub fn play_song_from_list(app: AppHandle, song_id: String) -> Result<(), String> {
    app_info!("play song from list: {}", song_id);
    
    // 获取歌曲信息和全部歌曲列表
    let manager = SONG_LIST_MANAGER.lock().map_err(|e| {
        let err_msg = format!("get song list manager failed: {}", e);
        app_error!("{}", err_msg);
        err_msg
    })?;
    
    let song = manager.get_song_by_id(&song_id).ok_or_else(|| {
        let err_msg = format!("song not found: {}", song_id);
        app_error!("{}", err_msg);
        err_msg
    })?;
    
    // 获取全部歌曲作为播放队列
    let all_songs = manager.get_my_songs();
    
    // 释放 manager 锁
    drop(manager);
    
    // 调用播放器播放（使用队列功能）
    let mut player = PLAYER.lock().map_err(|e| {
        let err_msg = format!("get player failed: {}", e);
        app_error!("{}", err_msg);
        err_msg
    })?;
    
    player.play_song_with_queue(song, all_songs)?;
    
    // 发送播放状态变更事件
    let state = player.get_state();
    let _ = app.emit("player-state-changed", state);
    
    Ok(())
}

// 播放指定歌曲（通过文件路径） 
// 前端调用: invoke('play_song_by_path', { filePath: string })
// 功能：直接通过文件路径播放歌曲（不设置队列）
#[command]
pub fn play_song_by_path(file_path: String, app: AppHandle) -> Result<(), String> {
    app_info!("play song by path: {}", file_path);
    
    let mut player = PLAYER.lock().map_err(|e| {
        let err_msg = format!("get player failed: {}", e);
        app_error!("{}", err_msg);
        err_msg
    })?;
    
    player.play(&file_path)?;
    
    // 发送播放状态变更事件
    let state = player.get_state();
    let _ = app.emit("player-state-changed", state);
    
    Ok(())
}

// 播放全部歌曲并指定起始歌曲
// 前端调用: invoke('play_all_from_index', { songIds: string[], startIndex: number })
// 功能：播放全部歌曲，从指定索引开始
#[command]
pub fn play_all_from_index(app: AppHandle, song_ids: Vec<String>, start_index: usize) -> Result<(), String> {
    app_info!("play all songs from index {} to index {} with {} songs in total", start_index, song_ids.len(), song_ids.len());
    
    if song_ids.is_empty() {
        return Err("song ids is empty".to_string());
    }
    
    if start_index >= song_ids.len() {
        return Err("start index out of range".to_string());
    }
    
    // 获取歌曲列表管理器
    let manager = SONG_LIST_MANAGER.lock().map_err(|e| {
        let err_msg = format!("get song list manager failed: {}", e);
        app_error!("{}", err_msg);
        err_msg
    })?;
    
    // 根据 ID 列表获取完整的歌曲列表（保持顺序）
    let mut songs: Vec<Song> = Vec::new();
    for id in &song_ids {
        if let Some(song) = manager.get_song_by_id(id) {
            songs.push(song);
        }
    }
    
    // 释放 manager 锁
    drop(manager);
    
    if songs.is_empty() {
        return Err("no valid songs".to_string());
    }
    
    // 播放指定索引的歌曲
    let mut player = PLAYER.lock().map_err(|e| {
        let err_msg = format!("get player failed: {}", e);
        app_error!("{}", err_msg);
        err_msg
    })?;
    
    player.set_queue(songs);
    player.play_at_index(start_index)?;
    
    // 发送播放状态变更事件
    let state = player.get_state();
    let _ = app.emit("player-state-changed", state);
    
    Ok(())
}

// 获取当前播放的歌曲信息
// 前端调用: invoke('get_current_song')
// 返回: Song 对象或 null
#[command]
pub fn get_current_song() -> Result<Option<Song>, String> {
    let player = PLAYER.lock().map_err(|e| {
        format!("get player failed: {}", e)
    })?;
    
    Ok(player.get_current_song().cloned())
}

// 获取下一首歌曲（不播放）
// 前端调用: invoke('get_next_song')
// 返回: Song 对象或 null
#[command]
pub fn get_next_song() -> Result<Option<Song>, String> {
    let player = PLAYER.lock().map_err(|e| {
        format!("get player failed: {}", e)
    })?;
    
    Ok(player.get_next_song().cloned())
}

// 获取上一首歌曲（不播放）
// 前端调用: invoke('get_prev_song')
// 返回: Song 对象或 null
#[command]
pub fn get_prev_song() -> Result<Option<Song>, String> {
    let player = PLAYER.lock().map_err(|e| {
        format!("get player failed: {}", e)
    })?;
    
    Ok(player.get_prev_song().cloned())
}

// 播放下一首歌曲
// 前端调用: invoke('play_next_song')
#[command]
pub fn play_next_song(app: AppHandle) -> Result<(), String> {
    let mut player = PLAYER.lock().map_err(|e| {
        format!("get player failed: {}", e)
    })?;
    
    player.play_next()?;
    
    // 发送播放状态变更事件
    let state = player.get_state();
    let _ = app.emit("player-state-changed", state);
    
    Ok(())
}

// 播放上一首歌曲
// 前端调用: invoke('play_prev_song')
#[command]
pub fn play_prev_song(app: AppHandle) -> Result<(), String> {
    let mut player = PLAYER.lock().map_err(|e| {
        format!("get player failed: {}", e)
    })?;
    
    player.play_prev()?;
    
    // 发送播放状态变更事件
    let state = player.get_state();
    let _ = app.emit("player-state-changed", state);
    
    Ok(())
}

// 设置播放模式
// 前端调用: invoke('set_play_mode', { mode: number })
// mode: 0=顺序播放, 1=单曲循环, 2=随机播放
#[command]
pub fn set_play_mode(mode: u8, app: AppHandle) -> Result<(), String> {
    let mut player = PLAYER.lock().map_err(|e| {
        format!("get player failed: {}", e)
    })?;
    
    player.set_play_mode(PlayMode::from(mode));
    
    // 发送播放状态变更事件
    let state = player.get_state();
    let _ = app.emit("player-state-changed", state);
    
    Ok(())
}
