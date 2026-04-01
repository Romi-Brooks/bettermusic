<template>
  <div class="progress-bar">
    <div class="progress-bg">
      <div class="progress-current" :style="{ width: progress + '%' }"></div>
    </div>
    <!-- clickable progress bar -->
    <div class="progress-click" @click="handleProgressClick"></div>
  </div>

  <div class="player-bar">
    <div class="player-track">
      <img :src="currentTrack.cover" alt="" class="track-cover" />
      <div class="track-info">
        <span class="title">{{ currentTrack.title }}</span>
        <span class="artist">{{ currentTrack.artist }}</span>
      </div>
      <div class="track-actions">
        <button class="action-btn">❤️</button>
      </div>
    </div>

    <div class="player-controls">

    <!-- 播放方式 -->
    <button class="control-btn" @click="togglePlayMode">
    <!-- 顺序 -->
    <svg v-if="playMode === 0" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="svg-icon">
      <path d="M4.70710678,18 L6.85355339,20.1464466 C7.04881554,20.3417088 7.04881554,20.6582912 6.85355339,20.8535534 C6.65829124,21.0488155 6.34170876,21.0488155 6.14644661,20.8535534 L3.14644661,17.8535534 C2.95118446,17.6582912 2.95118446,17.3417088 3.14644661,17.1464466 L6.14644661,14.1464466 C6.34170876,13.9511845 6.65829124,13.9511845 6.85355339,14.1464466 C7.04881554,14.3417088 7.04881554,14.6582912 6.85355339,14.8535534 L4.70710678,17 L18.5,17 C19.3284271,17 20,16.3284271 20,15.5 L20,12.5 C20,12.2238576 20.2238576,12 20.5,12 C20.7761424,12 21,12.2238576 21,12.5 L21,15.5 C21,16.8807119 19.8807119,18 18.5,18 L4.70710678,18 Z M19.2928932,7 L17.1464466,4.85355339 C16.9511845,4.65829124 16.9511845,4.34170876 17.1464466,4.14644661 C17.3417088,3.95118446 17.6582912,3.95118446 17.8535534,4.14644661 L20.8535534,7.14644661 C21.0488155,7.34170876 21.0488155,7.65829124 20.8535534,7.85355339 L17.8535534,10.8535534 C17.6582912,11.0488155 17.3417088,11.0488155 17.1464466,10.8535534 C16.9511845,10.6582912 16.9511845,10.3417088 17.1464466,10.1464466 L19.2928932,8 L5.5,8 C4.67157288,8 4,8.67157288 4,9.5 L4,12.5 C4,12.7761424 3.77614237,13 3.5,13 C3.22385763,13 3,12.7761424 3,12.5 L3,9.5 C3,8.11928813 4.11928813,7 5.5,7 L19.2928932,7 Z"/>
    </svg>

    <!-- 单曲循环 -->
    <svg v-else-if="playMode === 1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="svg-icon">
      <path d="M4.70710678,18 L6.85355339,20.1464466 C7.04881554,20.3417088 7.04881554,20.6582912 6.85355339,20.8535534 C6.65829124,21.0488155 6.34170876,21.0488155 6.14644661,20.8535534 L3.14644661,17.8535534 C2.95118446,17.6582912 2.95118446,17.3417088 3.14644661,17.1464466 L6.14644661,14.1464466 C6.34170876,13.9511845 6.65829124,13.9511845 6.85355339,14.1464466 C7.04881554,14.3417088 7.04881554,14.6582912 6.85355339,14.8535534 L4.70710678,17 L18.5,17 C19.3284271,17 20,16.3284271 20,15.5 L20,12.5 C20,12.2238576 20.2238576,12 20.5,12 C20.7761424,12 21,12.2238576 21,12.5 L21,15.5 C21,16.8807119 19.8807119,18 18.5,18 L4.70710678,18 Z M19.2928932,7 L17.1464466,4.85355339 C16.9511845,4.65829124 16.9511845,4.34170876 17.1464466,4.14644661 C17.3417088,3.95118446 17.6582912,3.95118446 17.8535534,4.14644661 L20.8535534,7.14644661 C21.0488155,7.34170876 21.0488155,7.65829124 20.8535534,7.85355339 L17.8535534,10.8535534 C17.6582912,11.0488155 17.3417088,11.0488155 17.1464466,10.8535534 C16.9511845,10.6582912 16.9511845,10.3417088 17.1464466,10.1464466 L19.2928932,8 L5.5,8 C4.67157288,8 4,8.67157288 4,9.5 L4,12.5 C4,12.7761424 3.77614237,13 3.5,13 C3.22385763,13 3,12.7761424 3,12.5 L3,9.5 C3,8.11928813 4.11928813,7 5.5,7 L19.2928932,7 Z"/>
      <path
        d="M11.5 9.5
           L12.8 8.8
           L12.8 15.2
           L13.6 15.2
           L13.6 16
           L10.8 16
           L10.8 15.2
           L11.6 15.2
           Z"
        fill="currentColor"
      />
    </svg>

    <svg v-else-if="playMode === 2"  xmlns="http://www.w3.org/2000/svg" viewBox="0 0 48 48" class="svg-icon" fill="none" stroke="currentColor" stroke-width="2">
      <g>
        <path d="m30.67 7 6 6h-22.17c-5.51 0-10 4.49-10 10v2h4v-2c0-3.31 2.69-6 6-6h22.17l-6 6 2.83 2.83 10.83-10.83-10.83-10.83z"/>
        <path d="m39.5 25c0 3.31-2.69 6-6 6h-22.17l6-6-2.83-2.83-10.83 10.83 10.83 10.83 2.83-2.83-6-6h22.17c5.51 0 10-4.49 10-10v-2h-4z"/>
      </g>
    </svg>
    </button>

      <!-- 上一曲/下一曲/播放/暂停按钮 -->
        <!-- 上一曲 -->
        <button class="control-btn" @click="playPrev">
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polygon points="19 20 9 12 19 4 19 20"/>
            <line x1="5" y1="19" x2="5" y2="5"/>
          </svg>
        </button>

        <!-- 播放/暂停切换 -->
        <button class="play-btn" @click="togglePlayPause">
          <template v-if="!isPlaying">
            <!-- 播放 -->
            <svg
              v-if="!isPlaying"
              class="play-svg play-icon"
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <polygon points="5 3 19 12 5 21" />
            </svg>
          </template>
          <template v-else>
            <!-- 暂停 -->
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"
              stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="play-svg pause-icon">
              <rect x="6" y="4" width="4" height="16"/>
              <rect x="14" y="4" width="4" height="16"/>
            </svg>
          </template>
        </button>

        <!-- 下一曲 -->
        <button class="control-btn" @click="playNext">
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"
            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polygon points="5 4 15 12 5 20 5 4"/>
            <line x1="19" y1="5" x2="19" y2="19"/>
          </svg>
        </button>

      <!-- 右侧控件 -->
      <!-- 歌单 -->
      <button class="control-btn">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"
          stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <line x1="4" y1="6" x2="20" y2="6"/>
          <line x1="4" y1="12" x2="20" y2="12"/>
          <line x1="4" y1="18" x2="20" y2="18"/>
        </svg>
      </button>
    </div>

    <div class="player-extra">
      <span class="quality">极高</span>
      <!-- 收藏 -->
      <button class="action-btn">
        <svg viewBox="0 0 24 24" class="svg-icon" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M20.8 4.6a5.5 5.5 0 0 0-7.8 0L12 5.6l-1-1a5.5 5.5 0 0 0-7.8 7.8l1 1L12 21l7.8-7.6 1-1a5.5 5.5 0 0 0 0-7.8z"/>
        </svg>
      </button>

      <!-- 播放列表 -->
      <button class="control-btn">
        <svg viewBox="0 0 24 24" class="svg-icon" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="4" y1="6" x2="20" y2="6"/>
          <line x1="4" y1="12" x2="20" y2="12"/>
          <line x1="4" y1="18" x2="20" y2="18"/>
        </svg>
      </button>

      <!-- 音量 -->
      <button class="extra-btn">
        <svg viewBox="0 0 24 24" class="svg-icon" fill="none" stroke="currentColor" stroke-width="2">
          <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"/>
          <path d="M15 9a4 4 0 0 1 0 6"/>
        </svg>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { ref, onMounted, onUnmounted } from "vue";
  import type { Track } from "@/types/music";
  const progress = ref(0);
  const playMode = ref(0);
  const isPlaying = ref(false);
  let progressTimer: number | null = null; // 进度定时器

  const currentTrack = ref<Track>({
    id: "t1",
    title: "素颜",
    artist: "许嵩,何曼婷",
    cover: "https://p1.music.126.net/6y-UleORITEDbvrOLV0Q8A==/5639395138885805.jpg",
    time: 0, // total second
    filePath: "E:/Media/Music/本兮/蓝色的海-本兮.mp3",
    hasPlaying: false,
  });

  const getPos = async () => {
    try {
      // get the second now
      const currentSecond = await invoke<number>('get_pos');
      
      // total second
      const totalSecond = currentTrack.value.time || 1;
      if (totalSecond <= 0) return;

      // get percent
      const percent = (currentSecond / totalSecond) * 100;
      progress.value = Math.min(100, Math.max(0, percent));
    } catch (err) {
      console.error('get the progress error:', err);
    }
  };

  // update the progress with 0.5s
  const startProgressSync = () => {
    if (progressTimer) clearInterval(progressTimer);
    progressTimer = window.setInterval(() => {
      if (isPlaying.value) {
        getPos();
      }
    }, 500);
  };

  // stop the progress update
  const stopProgressSync = () => {
    if (progressTimer) {
      clearInterval(progressTimer);
      progressTimer = null;
    }
  };

  // play a song
  const pushPlay = async (track?: Track) => {
    try {
      const targetTrack = track || currentTrack.value;
      if (!targetTrack.filePath) {
        console.error('passed an error path');
        return;
      }

      await invoke('push_play', { filePath: targetTrack.filePath });

      currentTrack.value.time = await invoke('get_duration');
      currentTrack.value = targetTrack;
      currentTrack.value.hasPlaying = true;
    
      isPlaying.value = true;

      progress.value = 0; // reset the progress
      startProgressSync(); // start the progress update

      
      console.log('playing:', targetTrack.title);
    } catch (err) {
      console.error('error when playing:', err);
    }
  };

  // resume/pause
  const togglePlayPause = async () => {
    if (!currentTrack.value.filePath || !currentTrack.value.hasPlaying) {
      return pushPlay();
    }

    try {
      await invoke('toggle_play');
      isPlaying.value = !isPlaying.value;
      
      // 暂停时也继续同步进度，播放时继续刷新
      if (isPlaying.value) startProgressSync();
      console.log('已切换播放状态:', isPlaying.value ? '播放中' : '已暂停');
    } catch (err) {
      console.error('切换播放状态失败:', err);
    }
  };

  //  prev song
  const playPrev = async () => {
    try {
      await invoke('push_prev');
      progress.value = 0;
      console.log('previous track triggered');
    } catch (err) {
      console.error('error when play the prev track:', err);
    }
  };

  // next song
  const playNext = async () => {
    try {
      await invoke('push_next');
      progress.value = 0;
      console.log('next track triggered');
    } catch (err) {
      console.error('error when play the next track:', err);
    }
  };

  // set the play mode
  const togglePlayMode = async () => {
    try {
      playMode.value = (playMode.value + 1) % 3;
      await invoke('push_mode', { mode: playMode.value });
      console.log('set playmode to:', playMode.value);
    } catch (err) {
      console.error('error to set playmode:', err);
    }
  };

  // stop the play
  const stopPlay = async () => {
    try {
      await invoke('push_stop');
      isPlaying.value = false;
      progress.value = 0;
      stopProgressSync();
      console.log('stopped');
    } catch (err) {
      console.error('error when stop:', err);
    }
  };

  // seek by click the progress bar
  const handleProgressClick = async (e: MouseEvent) => {
    const bar = e.currentTarget as HTMLElement;
    const rect = bar.getBoundingClientRect();
    const clickX = e.clientX - rect.left;
    const percent = clickX / rect.width;
    const targetTime = percent * (currentTrack.value.time || 0);
    
    await seekTo(targetTime);
    progress.value = percent * 100;
  };

  const seekTo = async (time: number) => {
    try {
      await invoke('seek_to', { time });
      console.info('seek to:', time, 's');
    } catch (err) {
      console.error('error when seek:', err);
    }
  };

  // life time
  onMounted(() => {
    startProgressSync();
  });

  onUnmounted(() => {
    stopProgressSync();
  });
</script>

<style scoped>
.progress-bar {
  position: fixed;
  left: 0;
  right: 0;
  bottom: 72px;
  height: 4px;
  z-index: 1000;
  cursor: pointer;
}

/* 点击区域 */
.progress-click {
  position: absolute;
  top: -6px;
  left: 0;
  width: 100%;
  height: 16px;
}

.progress-bg {
  width: 100%;
  height: 100%;
  background: #f1f1f1;
}

/* 已播放进度：红色 */
.progress-current {
  height: 100%;
  background: linear-gradient(90deg, #ff5a5f, #d43c33);
  transition: width 0.2s linear;
}

/* 播放器主体 */
.player-bar {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  height: 72px;
  background: #fff;
  border-top: 1px solid #eee;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 28px;
  z-index: 999;
}

/* 左侧歌曲信息 */
.player-track {
  display: flex;
  align-items: center;
  gap: 14px;
  min-width: 220px;
}

.track-cover {
  width: 46px;
  height: 46px;
  border-radius: 6px;
  object-fit: cover;
  box-shadow: 0 4px 10px rgba(0,0,0,.08);
}

.track-info {
  display: flex;
  flex-direction: column;
  line-height: 1.2;
}

.title {
  font-size: 15px;
  font-weight: 600;
  color: #222;
}

.artist {
  font-size: 12px;
  color: #888;
  margin-top: 2px;
}

.vip-tag {
  display: inline-block;
  font-size: 10px;
  color: #d43c33;
  border: 1px solid #d43c33;
  border-radius: 3px;
  padding: 0 4px;
  margin-left: 6px;
  transform: scale(0.9);
}

/* 控制区 */
.player-controls,
.player-extra {
  display: flex;
  align-items: center;
  gap: 16px;
}

/* 普通按钮 */
.control-btn,
.action-btn,
.extra-btn {
  background: none;
  border: none;
  cursor: pointer;
  color: #666;
  padding: 6px;
  border-radius: 6px;
  transition: all .15s ease;
}

.control-btn:hover,
.action-btn:hover,
.extra-btn:hover {
  color: #d43c33;
  background: rgba(212,60,51,.08);
}

.control-btn:active,
.action-btn:active,
.extra-btn:active {
  transform: scale(0.9);
}

/* 主播放按钮 */
.play-btn {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: linear-gradient(135deg, #ff5a5f, #d43c33);
  color: white;
  border: none;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 6px 16px rgba(212,60,51,.35);
  transition: all .15s ease;
}

.play-btn:hover {
  transform: scale(1.06);
}

.play-btn:active {
  transform: scale(0.94);
}

.play-icon {
  transform: translateX(1px);
}

.pause-icon {
  transform: translateX(-1px);
}

/* icon 尺寸 */
.svg-icon {
  width: 20px;
  height: 20px;
  display: block;
}

.play-svg {
  width: 22px;
  height: 22px;
  color: #fff;
  margin-left: 3px;
}

/* 右侧 */
.quality {
  font-size: 12px;
  color: #999;
  margin-right: 6px;
}
</style>