use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::{app_info, app_error, app_warning};
use crate::infra::metadata::{MetadataExtractor, MusicMetadata};

// 歌曲信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    pub id: String,
    pub name: String,        // 歌曲名称
    pub artist: String,      // 艺术家
    pub album: String,       // 专辑
    pub duration: String,    // 时长，格式: "03:53"
    #[serde(rename = "durationSec")]
    pub duration_sec: u64,   // 时长（秒）
    pub cover: String,       // 封面图片路径
    #[serde(rename = "filePath")]
    pub file_path: String,   // 本地文件路径
}

/// 我的音乐信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MySongs {
    pub id: String,
    pub title: String,       // 固定为 "我喜欢的音乐"
    pub cover: String,       // 封面图片路径
    #[serde(rename = "songCount")]
    pub song_count: u32,     // 收藏的歌曲数量
    #[serde(rename = "playCount")]
    pub play_count: u32,     // 播放次数
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistInfo {
    pub id: String,
    pub title: String,
    pub cover: String,
    #[serde(rename = "playCount")]
    pub play_count: u32,
    #[serde(rename = "creatorName")]
    pub creator_name: String,
    #[serde(rename = "creatorAvatar")]
    pub creator_avatar: String,
    #[serde(rename = "createDate")]
    pub create_date: String, // 格式: "2018-02-11"
    #[serde(rename = "songCount")]
    pub song_count: u32,
    pub description: Option<String>,
}

// 获取我的歌曲响应
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMySongsResponse {
    #[serde(rename = "mySongs")]
    pub my_songs: MySongs,
    pub songs: Vec<Song>,
}

// 获取歌单详情响应
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPlaylistResponse {
    pub playlist: PlaylistInfo,
    pub songs: Vec<Song>,
}

// Global Song List Manager
lazy_static! {
    /// 全局歌曲列表管理器
    pub static ref SONG_LIST_MANAGER: Mutex<SongListManager> = Mutex::new(SongListManager::new());
}

// 音乐文件夹路径
const MUSIC_PATH: &str = "E:/Media/Music";

// 临时封面资源
const DEFAULT_COVER: &str = "https://p1.music.126.net/6y-UleORITEDbvrOLV0Q8A==/5639395138885805.jpg";


pub struct SongListManager {
    // 我的音乐列表
    my_songs: Vec<Song>,
    // 用户创建的歌单
    playlists: Vec<PlaylistInfo>,
    // 最后扫描时间
    last_scan_time: Option<u64>,
}

impl SongListManager {
    pub fn new() -> Self {
        let mut manager = Self {
            my_songs: Vec::new(),
            playlists: Vec::new(),
            last_scan_time: None,
        };

        // 初始化时扫描音乐目录
        if let Err(e) = manager.scan_music_directory() {
            app_error!("scan music directory failed: {}", e);
        }

        manager
    }

    // 扫描音乐目录，构建歌曲索引
    pub fn scan_music_directory(&mut self) -> Result<(), String> {
        app_info!("scan music directory from: {}", MUSIC_PATH);

        let music_dir = Path::new(MUSIC_PATH);
        if !music_dir.exists() {
            return Err(format!("music directory not found: {}", MUSIC_PATH));
        }

        self.my_songs.clear();
        self.scan_directory_recursive(music_dir)?;

        // 更新时间戳
        self.last_scan_time = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );

        app_info!("scan music directory completed, found {} songs", self.my_songs.len());
        Ok(())
    }

    // 递归扫描目录
    fn scan_directory_recursive(&mut self, dir: &Path) -> Result<(), String> {
        let entries = fs::read_dir(dir).map_err(|e| format!("read directory failed: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("read directory entry failed: {}", e))?;
            let path = entry.path();

            if path.is_dir() {
                // 递归扫描子目录
                self.scan_directory_recursive(&path)?;
            } else if path.is_file() {
                // 检查是否是音乐文件
                if let Some(ext) = path.extension() {
                    let ext = ext.to_string_lossy().to_lowercase();
                    if matches!(ext.as_str(), "mp3" | "flac" | "wav" | "m4a" | "ogg") {
                        if let Some(song) = self.parse_song_from_path(&path) {
                            self.my_songs.push(song);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    // get the song information from file path
    fn parse_song_from_path(&self, path: &Path) -> Option<Song> {
        let file_path = path.to_string_lossy().to_string();
        
        // 生成唯一 ID（使用文件路径的哈希）
        let id = format!("{:x}", md5::compute(&file_path));

        // 使用 metadata 提取器读取元数据
        match MetadataExtractor::new() {
            Ok(extractor) => {
                match extractor.extract(&file_path, &id) {
                    Ok(metadata) => {
                        // 优先使用 Base64 Data URL，如果没有则使用本地路径或默认封面
                        let cover = metadata.cover_data
                            .or(metadata.cover_path)
                            .unwrap_or_else(|| DEFAULT_COVER.to_string());
                        
                        Some(Song {
                            id,
                            name: metadata.title.unwrap_or_else(|| {
                                path.file_stem()
                                    .and_then(|s| s.to_str())
                                    .unwrap_or("未知歌曲")
                                    .to_string()
                            }),
                            artist: metadata.artist.unwrap_or_else(|| "未知艺术家".to_string()),
                            album: metadata.album.unwrap_or_else(|| "未知专辑".to_string()),
                            duration: metadata.duration,
                            duration_sec: metadata.duration_sec,
                            cover,
                            file_path,
                        })
                    }
                    Err(e) => {
                        app_warning!("extract metadata failed [{}]: {}", file_path, e);
                        // 降级处理：使用文件名解析
                        self.parse_song_fallback(path, &id, &file_path)
                    }
                }
            }
            Err(e) => {
                app_warning!("create metadata extractor failed: {}", e);
                // 降级处理：使用文件名解析
                self.parse_song_fallback(path, &id, &file_path)
            }
        }
    }

    // 降级处理：从文件名和路径推断信息
    fn parse_song_fallback(&self, path: &Path, id: &str, file_path: &str) -> Option<Song> {
        let file_name = path.file_stem()?.to_string_lossy();

        // 尝试从文件名解析 "艺术家 - 歌曲名" 格式
        let (artist, name) = if let Some(pos) = file_name.find(" - ") {
            let artist = file_name[..pos].trim().to_string();
            let name = file_name[pos + 3..].trim().to_string();
            (artist, name)
        } else {
            // 如果没有分隔符，使用文件名作为歌曲名，目录名作为艺术家
            let name = file_name.to_string();
            let artist = path
                .parent()
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "未知艺术家".to_string());
            (artist, name)
        };

        // 从父目录名推断专辑名
        let album = path
            .parent()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "未知专辑".to_string());

        Some(Song {
            id: id.to_string(),
            name,
            artist,
            album,
            duration: "00:00".to_string(),
            duration_sec: 0,
            cover: DEFAULT_COVER.to_string(),
            file_path: file_path.to_string(),
        })
    }

    // 获取我的音乐信息
    pub fn get_my_songs_info(&self) -> MySongs {
        MySongs {
            id: "my-songs".to_string(),
            title: "我喜欢的音乐".to_string(),
            cover: DEFAULT_COVER.to_string(),
            song_count: self.my_songs.len() as u32,
            play_count: 0, // 后续可从数据库或配置文件读取
        }
    }

    // 获取我的音乐歌曲列表
    pub fn get_my_songs(&self) -> Vec<Song> {
        self.my_songs.clone()
    }

    // 获取我的音乐完整响应
    pub fn get_my_songs_response(&self) -> GetMySongsResponse {
        GetMySongsResponse {
            my_songs: self.get_my_songs_info(),
            songs: self.get_my_songs(),
        }
    }

    // 根据 ID 获取歌曲
    pub fn get_song_by_id(&self, song_id: &str) -> Option<Song> {
        self.my_songs.iter().find(|s| s.id == song_id).cloned()
    }

    // 获取所有歌曲 ID 列表
    pub fn get_all_song_ids(&self) -> Vec<String> {
        self.my_songs.iter().map(|s| s.id.clone()).collect()
    }

    // 获取创建的歌单列表
    pub fn get_playlists(&self) -> Vec<PlaylistInfo> {
        // 返回空列表，后续实现
        Vec::new()
    }

    // 根据 ID 获取歌单详情
    pub fn get_playlist_by_id(&self, _playlist_id: &str) -> Option<GetPlaylistResponse> {
        // 返回 None，后续实现
        None
    }
}

// 格式化时长（秒 -> "mm:ss"）
pub fn format_duration(seconds: u64) -> String {
    let mins = seconds / 60;
    let secs = seconds % 60;
    format!("{:02}:{:02}", mins, secs)
}

// 解析时长（"mm:ss" -> 秒）
pub fn parse_duration(duration: &str) -> u64 {
    let parts: Vec<&str> = duration.split(':').collect();
    if parts.len() == 2 {
        if let (Ok(mins), Ok(secs)) = (parts[0].parse::<u64>(), parts[1].parse::<u64>()) {
            return mins * 60 + secs;
        }
    }
    0
}
