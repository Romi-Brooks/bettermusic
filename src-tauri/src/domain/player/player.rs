use std::fs::File;
use std::path::Path;
use std::sync::Mutex;
use std::time::Duration;

use lazy_static::lazy_static;
use rodio::{Decoder, Source, Player, MixerDeviceSink};

use crate::{app_info, app_error, app_warning};

// Global Static Player Entity
lazy_static! {
    pub static ref PLAYER: Mutex<MusicPlayer> = Mutex::new(MusicPlayer::new());
}

    // player
    pub struct MusicPlayer {
        // rodio settings
        sink: Option<MixerDeviceSink>,
        decoder: Option<Decoder<File>>,
        player: Option<Player>,

        // player settings
        current_path: Option<String>,
        is_playing: bool,
        duration: Option<Duration>,
    }

    impl MusicPlayer {
        pub fn new() -> Self {
            let sink = rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
            let player = Player::connect_new(&sink.mixer());
            app_info!("Music Player Created");

            Self {
                sink: Some(sink),
                decoder: None,
                player:Some(player),
                current_path: None,
                is_playing: false,
                duration: None,
            }
        }

        // play a song by path
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
            // resume from a paused player
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

    // get the playing status
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    // get the file path witch are playing
    pub fn current_path(&self) -> Option<&str> {
        self.current_path.as_deref()
    }

    // get the song duration
    pub fn get_duration(&mut self) -> f32 {
        self.duration.as_ref().unwrap().as_secs() as f32
    }

    // get the song pos
    pub fn get_current_second(&self) -> u64 {
        self.player.as_ref().unwrap().get_pos().as_secs()
    }

    // seek to the pos
    pub fn seek_to(&mut self, second: f32) {
        // pre-check
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
}