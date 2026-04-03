use tauri::command;

use crate::domain::song_list::{SONG_LIST_MANAGER, GetMySongsResponse};
use crate::{app_info, app_error};

/// 获取我的音乐列表
/// 
/// 前端调用: invoke('get_my_songs')
/// 返回: { my_songs: MySongs, songs: Song[] }
#[command]
pub fn get_my_songs() -> Result<GetMySongsResponse, String> {
    app_info!("get my songs response");
    
    let manager = SONG_LIST_MANAGER.lock().map_err(|e| {
        let err_msg = format!("get song list manager failed: {}", e);
        app_error!("{}", err_msg);
        err_msg
    })?;
    
    let response = manager.get_my_songs_response();
    app_info!("return {} songs", response.songs.len());
    
    Ok(response)
}

/// 下载歌曲
/// 
/// 前端调用: invoke('download_songs', { songIds: string[] })
#[command]
pub fn download_songs(song_ids: Vec<String>) -> Result<(), String> {
    app_info!("download {} songs", song_ids.len());
    
    if song_ids.is_empty() {
        return Err("song ids is empty".to_string());
    }
    
    // 下载功能暂未实现
    app_info!("download songs not implemented");
    
    Ok(())
}

/// 重新扫描音乐目录
/// 
/// 前端调用: invoke('rescan_music_directory')
#[command]
pub fn rescan_music_directory() -> Result<(), String> {
    app_info!("rescan music directory");
    
    let mut manager = SONG_LIST_MANAGER.lock().map_err(|e| {
        format!("get song list manager failed: {}", e)
    })?;
    
    manager.scan_music_directory()
}