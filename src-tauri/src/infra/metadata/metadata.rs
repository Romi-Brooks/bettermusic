use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use lofty::config::ParseOptions;
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::picture::PictureType;
use lofty::prelude::*;
use lofty::probe::Probe;
use lofty::tag::ItemValue;
use serde::{Deserialize, Serialize};

use crate::{app_debug, app_error, app_info, app_warning};

// 将图片数据转换为 Base64 Data URL
fn image_to_base64(data: &[u8]) -> String {
    let base64 = base64::encode(data);
    // 检测图片格式
    let mime_type = if data.starts_with(&[0xFF, 0xD8, 0xFF]) {
        "image/jpeg"
    } else if data.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
        "image/png"
    } else if data.starts_with(&[0x47, 0x49, 0x46]) {
        "image/gif"
    } else if data.starts_with(&[0x42, 0x4D]) {
        "image/bmp"
    } else {
        "image/jpeg" // 默认
    };
    format!("data:{};base64,{}" , mime_type, base64)
}

// 音乐文件元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicMetadata {
    /// 歌曲标题
    pub title: Option<String>,
    /// 艺术家
    pub artist: Option<String>,
    /// 专辑
    pub album: Option<String>,
    /// 专辑艺术家
    pub album_artist: Option<String>,
    /// 时长（秒）
    pub duration_sec: u64,
    /// 时长（格式化字符串 mm:ss）
    pub duration: String,
    /// 封面图片路径（本地缓存路径或URL）
    pub cover_path: Option<String>,
    /// 封面图片 Base64 Data URL
    pub cover_data: Option<String>,
    /// 音轨号
    pub track_number: Option<u32>,
    /// 年份
    pub year: Option<u32>,
    /// 流派
    pub genre: Option<String>,
    /// 歌词
    pub lyrics: Option<String>,
    /// 文件路径
    pub file_path: String,
    /// 文件格式
    pub format: String,
    /// 比特率（kbps）
    pub bitrate: Option<u32>,
    /// 采样率（Hz）
    pub sample_rate: Option<u32>,
}

impl Default for MusicMetadata {
    fn default() -> Self {
        Self {
            title: None,
            artist: None,
            album: None,
            album_artist: None,
            duration_sec: 0,
            duration: "00:00".to_string(),
            cover_path: None,
            cover_data: None,
            track_number: None,
            year: None,
            genre: None,
            lyrics: None,
            file_path: String::new(),
            format: String::new(),
            bitrate: None,
            sample_rate: None,
        }
    }
}

// 封面缓存管理器
pub struct CoverCache {
    // 缓存目录路径
    cache_dir: PathBuf,
}

impl CoverCache {
    // 创建封面缓存管理器
    pub fn new() -> Result<Self, String> {
        let cache_dir = Self::get_cache_dir()?;
        
        // 确保缓存目录存在
        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir)
                .map_err(|e| format!("failed to create cache cache directory: {}", e))?;
        }
        
        app_debug!("cache cache directory: {:?}", cache_dir);
        
        Ok(Self { cache_dir })
    }
    
    // 获取缓存目录路径
    fn get_cache_dir() -> Result<PathBuf, String> {
        let app_data_dir = dirs::data_dir()
            .ok_or_else(|| "failed to get application data directory".to_string())?;
        Ok(app_data_dir.join("BetterMusic").join("cover_cache"))
    }
    
    // 获取封面缓存路径
    fn get_cover_cache_path(&self, song_id: &str) -> PathBuf {
        self.cache_dir.join(format!("{}.jpg", song_id))
    }
    
    // 检查封面是否已缓存
    pub fn is_cached(&self, song_id: &str) -> bool {
        self.get_cover_cache_path(song_id).exists()
    }
    
    // 获取封面路径（如果已缓存）
    pub fn get_cover_path(&self, song_id: &str) -> Option<String> {
        let path = self.get_cover_cache_path(song_id);
        if path.exists() {
            path.to_str().map(|s| s.to_string())
        } else {
            None
        }
    }
    
    // 保存封面到缓存
    pub fn save_cover(&self, song_id: &str, cover_data: &[u8]) -> Result<String, String> {
        let cover_path = self.get_cover_cache_path(song_id);
        
        let mut file = fs::File::create(&cover_path)
            .map_err(|e| format!("failed to create cache cache file: {}", e))?;
        
        file.write_all(cover_data)
            .map_err(|e| format!("failed to write cache cache data: {}", e))?;
        
        app_debug!("cached cover: {:?}", cover_path);
        
        cover_path.to_str()
            .map(|s| s.to_string())
            .ok_or_else(|| "invalid path".to_string())
    }
    
    // 清理过期缓存
    pub fn clean_expired_cache(&self, _max_age_days: u32) -> Result<(), String> {
        // todo
        app_debug!("clean expired cache");
        Ok(())
    }
}

// 元数据提取器
pub struct MetadataExtractor {
    cover_cache: CoverCache,
}

impl MetadataExtractor {
    // 创建元数据提取器
    pub fn new() -> Result<Self, String> {
        let cover_cache = CoverCache::new()?;
        Ok(Self { cover_cache })
    }
    
    // 从文件路径提取元数据
    pub fn extract(&self, file_path: &str, song_id: &str) -> Result<MusicMetadata, String> {
        let path = Path::new(file_path);
        
        if !path.exists() {
            return Err(format!("file not found: {}", file_path));
        }
        
        // 使用 lofty 读取文件
        let tagged_file = Probe::open(path)
            .map_err(|e| format!("failed to open file: {}", e))?
            .options(ParseOptions::new())
            .read()
            .map_err(|e| format!("failed to read file: {}", e))?;
        
        // 获取文件属性
        let properties = tagged_file.properties();
        let duration = properties.duration();
        let duration_sec = duration.as_secs();
        
        // 获取标签
        let tag = tagged_file.primary_tag();
        
        let mut metadata = MusicMetadata {
            duration_sec,
            duration: format_duration(duration_sec),
            file_path: file_path.to_string(),
            format: path.extension()
                .and_then(|e| e.to_str())
                .unwrap_or("unknown")
                .to_lowercase(),
            bitrate: properties.audio_bitrate(),
            sample_rate: properties.sample_rate(),
            ..Default::default()
        };
        
        // 提取标签信息
        if let Some(tag) = tag {
            metadata.title = tag.title().map(|t| t.into_owned());
            metadata.artist = tag.artist().map(|a| a.into_owned());
            metadata.album = tag.album().map(|a| a.into_owned());

            // 提取更详细的标签信息
            for item in tag.items() {
                let key = item.key();
                let value = match item.value() {
                    ItemValue::Text(text) => text.to_string(),
                    ItemValue::Locator(loc) => loc.to_string(),
                    _ => continue, // 跳过二进制等其他类型
                };
                match key {
                    ItemKey::TrackArtist => metadata.artist = Some(value),
                    ItemKey::TrackTitle => metadata.title = Some(value),
                    ItemKey::AlbumTitle => metadata.album = Some(value),
                    ItemKey::AlbumArtist => metadata.album_artist = Some(value),
                    ItemKey::TrackNumber => {
                        metadata.track_number = value.parse().ok();
                    }
                    ItemKey::Year => {
                        metadata.year = value.parse().ok();
                    }
                    ItemKey::Genre => metadata.genre = Some(value),
                    ItemKey::Lyrics => metadata.lyrics = Some(value),
                    _ => {}
                }
            }
            
            // 提取封面图片
            if let Some(picture) = tag.get_picture_type(PictureType::CoverFront) {
                let pic_data = picture.data();
                
                // 转换为 Base64 Data URL
                metadata.cover_data = Some(image_to_base64(pic_data));
                
                // 同时保存到缓存（可选，用于其他用途）
                match self.cover_cache.save_cover(song_id, pic_data) {
                    Ok(cover_path) => {
                        metadata.cover_path = Some(cover_path);
                    }
                    Err(e) => {
                        app_warning!("failed to save cover: {}", e);
                    }
                }
            }
        }
        
        // 如果元数据中没有标题，使用文件名
        if metadata.title.is_none() {
            metadata.title = path.file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string());
        }
        
        app_debug!("extract metadata success: {} - {:?}", file_path, metadata.title);
        
        Ok(metadata)
    }
    
    // 快速提取（仅基本信息，不提取封面）
    pub fn extract_basic(&self, file_path: &str) -> Result<MusicMetadata, String> {
        let path = Path::new(file_path);
        
        if !path.exists() {
            return Err(format!("file not found: {}", file_path));
        }
        
        let tagged_file = Probe::open(path)
            .map_err(|e| format!("failed to open file: {}", e))?
            .options(ParseOptions::new())
            .read()
            .map_err(|e| format!("failed to read file: {}", e))?;
        
        let properties = tagged_file.properties();
        let duration_sec = properties.duration().as_secs();
        
        let tag = tagged_file.primary_tag();
        
        let mut metadata = MusicMetadata {
            duration_sec,
            duration: format_duration(duration_sec),
            file_path: file_path.to_string(),
            format: path.extension()
                .and_then(|e| e.to_str())
                .unwrap_or("unknown")
                .to_lowercase(),
            ..Default::default()
        };
        
        if let Some(tag) = tag {
            metadata.title = tag.title().map(|t| t.into_owned());
            metadata.artist = tag.artist().map(|a| a.into_owned());
            metadata.album = tag.album().map(|a| a.into_owned());
        }
        
        // 如果元数据中没有标题，使用文件名
        if metadata.title.is_none() {
            metadata.title = path.file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string());
        }
        
        Ok(metadata)
    }
    
    // 获取默认封面路径
    pub fn get_default_cover() -> String {
        "https://p1.music.126.net/6y-UleORITEDbvrOLV0Q8A==/5639395138885805.jpg".to_string()
    }
    
    // 获取封面路径（优先使用缓存，否则使用默认）
    pub fn get_cover_or_default(&self, song_id: &str) -> String {
        self.cover_cache.get_cover_path(song_id)
            .unwrap_or_else(|| Self::get_default_cover())
    }
}

// 格式化时长（秒 -> "mm:ss"）
fn format_duration(seconds: u64) -> String {
    let mins = seconds / 60;
    let secs = seconds % 60;
    format!("{:02}:{:02}", mins, secs)
}

// 从文件路径提取元数据（便捷函数）
pub fn extract_metadata(file_path: &str, song_id: &str) -> Result<MusicMetadata, String> {
    let extractor = MetadataExtractor::new()?;
    extractor.extract(file_path, song_id)
}

// 批量提取元数据
pub fn batch_extract_metadata(files: &[(String, String)]) -> Vec<MusicMetadata> {
    let extractor = match MetadataExtractor::new() {
        Ok(e) => e,
        Err(e) => {
            app_error!("failed to create metadata extractor: {}", e);
            return Vec::new();
        }
    };
    
    let mut results = Vec::new();
    
    for (file_path, song_id) in files {
        match extractor.extract(file_path, song_id) {
            Ok(metadata) => results.push(metadata),
            Err(e) => {
                app_warning!("extract metadata failed [{}]: {}", file_path, e);
            }
        }
    }
    
    app_info!("batch extract metadata success: {}/{} success", results.len(), files.len());
    results
}

// 
pub fn get_audio_duration(file_path: &str) -> Result<u64, String> {
    let path = Path::new(file_path);
    
    if !path.exists() {
        return Err(format!("file not found: {}", file_path));
    }
    
    let tagged_file = Probe::open(path)
        .map_err(|e| format!("failed to open file: {}", e))?
        .options(ParseOptions::new())
        .read()
        .map_err(|e| format!("failed to read file: {}", e))?;

    Ok(tagged_file.properties().duration().as_secs())
}

// 检查文件是否为支持的音频格式
pub fn is_supported_audio_format(file_path: &str) -> bool {
    let path = Path::new(file_path);
    
    if let Some(ext) = path.extension() {
        let ext = ext.to_string_lossy().to_lowercase();
        matches!(ext.as_str(), "mp3" | "flac" | "wav" | "m4a" | "ogg" | "aac" | "wma" | "ape")
    } else {
        false
    }
}
