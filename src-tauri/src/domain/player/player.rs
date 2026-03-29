use lazy_static::lazy_static;
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use std::convert::TryFrom;
use std::fs::File;
use std::path::Path;
use std::sync::Mutex;

// 全局单例播放器
lazy_static! {
    pub static ref PLAYER: Mutex<MusicPlayer> = Mutex::new(MusicPlayer::new());
}

/// 音乐播放器
pub struct MusicPlayer {
    sink: Option<Sink>,
    stream: Option<OutputStream>, // 保存音频流，防止被提前drop导致无声
    current_path: Option<String>,
    is_playing: bool,
}

impl MusicPlayer {
    pub fn new() -> Self {
        Self {
            sink: None,
            stream: None,
            current_path: None,
            is_playing: false,
        }
    }

    /// 播放本地音乐
    pub fn play(&mut self, file_path: &str) -> Result<(), String> {
        // 检查文件是否存在
        if !Path::new(file_path).exists() {
            return Err(format!("文件不存在：{}", file_path));
        }

        // 停止之前的音乐
        self.stop();

        // 创建默认音频流
        let stream = match OutputStreamBuilder::open_default_stream() {
            Ok(s) => s,
            Err(e) => return Err(format!("未找到音频输出设备：{}", e)),
        };

        // 通过混音器创建Sink
        let sink = Sink::connect_new(stream.mixer());
        let file = File::open(file_path).map_err(|e| format!("打开文件失败：{}", e))?;

        // Decoder使用TryFrom转换
        let decoder = Decoder::try_from(file).map_err(|e| format!("解码失败：{}", e))?;
        sink.append(decoder);

        // 必须同时保存stream，否则流会被drop导致无声
        self.sink = Some(sink);
        self.stream = Some(stream);
        self.current_path = Some(file_path.to_string());
        self.is_playing = true;

        Ok(())
    }

    /// 暂停
    pub fn pause(&mut self) {
        if let Some(sink) = &self.sink {
            sink.pause();
            self.is_playing = false;
        }
    }

    /// 继续播放
    pub fn resume(&mut self) {
        if let Some(sink) = &self.sink {
            sink.play();
            self.is_playing = true;
        }
    }

    /// 停止
    pub fn stop(&mut self) {
        if let Some(sink) = &self.sink {
            sink.stop();
        }
        // 同时清空流，关闭音频输出
        self.sink = None;
        self.stream = None;
        self.is_playing = false;
    }

    /// 获取播放状态
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    /// 获取当前播放文件
    pub fn current_path(&self) -> Option<&str> {
        self.current_path.as_deref()
    }
}