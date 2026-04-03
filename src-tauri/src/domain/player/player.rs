use std::fs::File;
use std::path::Path;
use std::sync::Mutex;
use std::time::Duration;

use lazy_static::lazy_static;
use rodio::{Decoder, Source, Player, MixerDeviceSink};
use serde::{Deserialize, Serialize};

use crate::{app_info, app_error, app_warning};
use crate::domain::song_list::Song;

// Player Modes
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlayMode {
    Sequential = 0,  // 顺序播放
    SingleLoop = 1,  // 单曲循环
    Random = 2,      // 随机播放
}

impl Default for PlayMode {
    fn default() -> Self {
        PlayMode::Sequential
    }
}

impl From<u8> for PlayMode {
    fn from(value: u8) -> Self {
        match value {
            1 => PlayMode::SingleLoop,
            2 => PlayMode::Random,
            _ => PlayMode::Sequential,
        }
    }
}

// Player State
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerState {
    pub is_playing: bool,
    pub current_song: Option<Song>,
    pub current_index: Option<usize>,
    pub progress: f32,       // 0-100
    pub duration_sec: u64,
    pub current_sec: u64,
    pub play_mode: PlayMode,
    pub queue_length: usize,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            is_playing: false,
            current_song: None,
            current_index: None,
            progress: 0.0,
            duration_sec: 0,
            current_sec: 0,
            play_mode: PlayMode::default(),
            queue_length: 0,
        }
    }
}

// Global Player Instance
lazy_static! {
    pub static ref PLAYER: Mutex<MusicPlayer> = Mutex::new(MusicPlayer::new());
}

// Player

pub struct MusicPlayer {
    // rodio
    sink: Option<MixerDeviceSink>,
    decoder: Option<Decoder<File>>,
    player: Option<Player>,

    // Player state
    current_path: Option<String>,
    is_playing: bool,
    duration: Option<Duration>,

    // player queue
    queue: Vec<Song>,
    current_index: Option<usize>,
    play_mode: PlayMode,
}

impl MusicPlayer {
    pub fn new() -> Self {
        let sink = rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
        let player = Player::connect_new(&sink.mixer());
        app_info!("Music Player Created");

        Self {
            sink: Some(sink),
            decoder: None,
            player: Some(player),
            current_path: None,
            is_playing: false,
            duration: None,
            queue: Vec::new(),
            current_index: None,
            play_mode: PlayMode::default(),
        }
    }

    // play a song
    pub fn play(&mut self, file_path: &str) -> Result<(), String> {
        // pre-check
        if !Path::new(file_path).exists() {
            return Err(format!("the file doesn't exist: {}", file_path));
        }

        // stop it, if there are another song is playing
        if let Some(player) = self.player.as_ref() {
            player.stop();
        }

        // open the file
        let file = File::open(file_path).map_err(|e| format!("error when opening the file: {}", e))?;
        let len = file.metadata().map_err(|e| format!("get the metadata error: {}", e))?.len();

        // creat the decoder
        let decoder = Decoder::builder()
            .with_data(file)
            .with_byte_len(len)
            .with_seekable(true)
            .build()
            .map_err(|e| format!("error when creating the decoder: {:?}", e))?;

        // get the song duration
        self.duration = decoder.total_duration();
        if let Some(dur) = self.duration {
            app_info!("song duration: {}s", dur.as_secs());
        }

        self.decoder = Some(decoder);

        // play it
        if let Some(player) = &self.player {
            player.append(self.decoder.take().unwrap());
            player.play();
            self.is_playing = true;
            self.current_path = Some(file_path.to_string());
            app_info!("playing: {}", file_path);
        }

        Ok(())
    }

    // pause
    pub fn pause(&mut self) {
        if let Some(player) = &self.player {
            player.pause();
            self.is_playing = false;
        }
        app_info!("paused");
    }

    // resume
    pub fn resume(&mut self) {
        if let Some(player) = &self.player {
            player.play();
            self.is_playing = true;
        }
        app_info!("playing resumed");
    }

    // stop
    pub fn stop(&mut self) {
        if let Some(player) = &self.player {
            player.stop();
            self.is_playing = false;
        }
        app_info!("playing stopped");
    }

    // toggle play
    pub fn toggle_play(&mut self) {
        if self.is_playing {
            self.pause();
        } else {
            self.resume();
        }
    }

    // set queue
    pub fn set_queue(&mut self, songs: Vec<Song>) {
        self.queue = songs;
        app_info!("queue set: {} songs", self.queue.len());
    }

    // get queue
    pub fn get_queue(&self) -> &[Song] {
        &self.queue
    }

    // clear queue
    pub fn clear_queue(&mut self) {
        self.queue.clear();
        self.current_index = None;
        app_info!("queue cleared");
    }

    /// 播放队列中的指定歌曲
    pub fn play_at_index(&mut self, index: usize) -> Result<(), String> {
        if index >= self.queue.len() {
            return Err(format!("index out of range: {} >= {}", index, self.queue.len()));
        }

        // 先克隆需要的数据，避免借用冲突
        let song = self.queue[index].clone();
        let file_path = song.file_path.clone();
        let artist = song.artist.clone();
        let name = song.name.clone();
        
        self.play(&file_path)?;
        self.current_index = Some(index);
        
        app_info!("playing song: {} - {}", artist, name);
        Ok(())
    }

    pub fn play_song_with_queue(&mut self, song: Song, queue: Vec<Song>) -> Result<(), String> {
        // 找到歌曲在队列中的索引
        let index = queue.iter().position(|s| s.id == song.id);
        
        self.set_queue(queue);
        
        if let Some(idx) = index {
            self.play_at_index(idx)
        } else {
            // 如果歌曲不在队列中，添加到队列开头并播放
            self.queue.insert(0, song.clone());
            self.play_at_index(0)
        }
    }

    // play next
    pub fn play_next(&mut self) -> Result<(), String> {
        if self.queue.is_empty() {
            return Err("播放队列为空".to_string());
        }

        let next_index = match self.play_mode {
            PlayMode::Sequential => {
                match self.current_index {
                    Some(idx) if idx + 1 < self.queue.len() => idx + 1,
                    _ => 0, // 循环到第一首
                }
            }
            PlayMode::SingleLoop => {
                // 单曲循环：重新播放当前歌曲
                self.current_index.unwrap_or(0)
            }
            PlayMode::Random => {
                // 随机播放
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                use std::time::{SystemTime, UNIX_EPOCH};
                
                let seed = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64;
                let mut hasher = DefaultHasher::new();
                seed.hash(&mut hasher);
                (hasher.finish() as usize) % self.queue.len()
            }
        };

        self.play_at_index(next_index)
    }

    // play prev
    pub fn play_prev(&mut self) -> Result<(), String> {
        if self.queue.is_empty() {
            return Err("播放队列为空".to_string());
        }

        let prev_index = match self.current_index {
            Some(0) => self.queue.len() - 1, // 第一首的上一首是最后一首
            Some(idx) => idx - 1,
            None => 0,
        };

        self.play_at_index(prev_index)
    }

    pub fn get_current_song(&self) -> Option<&Song> {
        self.current_index.and_then(|idx| self.queue.get(idx))
    }

    // get next song without play
    pub fn get_next_song(&self) -> Option<&Song> {
        if self.queue.is_empty() {
            return None;
        }

        let next_index = match self.current_index {
            Some(idx) if idx + 1 < self.queue.len() => idx + 1,
            _ => 0,
        };

        self.queue.get(next_index)
    }

    // get prev song without play
    pub fn get_prev_song(&self) -> Option<&Song> {
        if self.queue.is_empty() {
            return None;
        }

        let prev_index = match self.current_index {
            Some(0) => self.queue.len() - 1,
            Some(idx) => idx - 1,
            None => 0,
        };

        self.queue.get(prev_index)
    }

    // set play mode
    pub fn set_play_mode(&mut self, mode: PlayMode) {
        self.play_mode = mode;
        app_info!("switch the play mode to: {:?}", mode);
    }

    // get play mode
    pub fn get_play_mode(&self) -> PlayMode {
        self.play_mode
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn current_path(&self) -> Option<&str> {
        self.current_path.as_deref()
    }

    pub fn get_duration(&self) -> f32 {
        self.duration.as_ref().map(|d| d.as_secs() as f32).unwrap_or(0.0)
    }

    pub fn get_current_second(&self) -> u64 {
        self.player.as_ref().map(|p| p.get_pos().as_secs()).unwrap_or(0)
    }

    pub fn seek_to(&mut self, second: f32) {
        let Some(player) = &self.player else {
            app_warning!("player was not be created");
            return;
        };
        if self.is_playing == false {
            app_warning!("no music is playing");
            return;
        }

        let seek_time = Duration::from_secs(second as u64);
        match player.try_seek(seek_time) {
            Ok(_) => {
                app_info!("seeking to {} s", second);
            }
            Err(e) => {
                app_error!("error when seeking :{}", e);
            }
        }
    }

    // get player state
    pub fn get_state(&self) -> PlayerState {
        let current_song = self.get_current_song().cloned();
        let duration_sec = self.duration.as_ref().map(|d| d.as_secs()).unwrap_or(0);
        let current_sec = self.get_current_second();
        let progress = if duration_sec > 0 {
            (current_sec as f32 / duration_sec as f32) * 100.0
        } else {
            0.0
        };

        PlayerState {
            is_playing: self.is_playing,
            current_song,
            current_index: self.current_index,
            progress,
            duration_sec,
            current_sec,
            play_mode: self.play_mode,
            queue_length: self.queue.len(),
        }
    }
}
