<template>
  <div class="my-songs-view">
    <Sidebar />
    <main class="main-content">
      <TopBar />
      
      <div class="content-scroll">
        <!-- 加载状态 -->
        <div v-if="loading" class="loading-state">
          <div class="loading-spinner"></div>
          <p>加载中...</p>
        </div>

        <!-- 错误状态 -->
        <div v-else-if="error" class="error-state">
          <p>{{ error }}</p>
          <button class="btn btn-primary" @click="fetchData">重新加载</button>
        </div>

        <!-- 内容 -->
        <template v-else>
          <!-- 我的音乐头部信息 -->
          <div class="playlist-header">
            <div class="playlist-cover">
              <img :src="getCoverUrl(mySongsInfo.cover)" alt="歌单封面" />
              <div class="play-count-overlay">
                <span class="play-icon">▶</span>
                <span>{{ formatPlayCount(mySongsInfo.playCount) }}</span>
              </div>
              <div class="heart-overlay">❤️</div>
            </div>
            <div class="playlist-info">
              <h1 class="playlist-title">{{ mySongsInfo.title }}</h1>
              <div class="playlist-creator">
                <span class="create-date">{{ mySongsInfo.songCount }}首歌曲</span>
              </div>
              <div class="playlist-actions">
                <button class="btn btn-primary" @click="playAll">
                  <span class="btn-icon">▶</span>
                  播放全部
                </button>
                <button class="btn btn-secondary" @click="downloadAll">
                  <span class="btn-icon">⬇</span>
                  下载
                </button>
                <button class="btn btn-secondary">
                  <span class="btn-icon">⋯</span>
                </button>
              </div>
            </div>
          </div>

          <!-- 标签页 -->
          <div class="tabs">
            <div 
              v-for="tab in tabs" 
              :key="tab.id"
              :class="['tab-item', { active: activeTab === tab.id }]"
              @click="activeTab = tab.id"
            >
              {{ tab.label }}
              <span v-if="tab.count !== undefined" class="tab-count">{{ tab.count }}</span>
            </div>
          </div>

          <!-- 歌曲列表 -->
          <div class="songs-list">
            <div class="list-header">
              <div class="col-index">#</div>
              <div class="col-title">标题</div>
              <div class="col-album">专辑</div>
              <div class="col-like">喜欢</div>
              <div class="col-duration">时长</div>
            </div>
            
            <div 
              v-for="(song, index) in songs" 
              :key="song.id"
              :class="['song-item', { 'is-playing': currentPlayingId === song.id }]"
              @mouseenter="hoveredSong = song.id"
              @mouseleave="hoveredSong = null"
              @dblclick="playSong(song)"
            >
              <div class="col-index">
                <span v-if="hoveredSong !== song.id && currentPlayingId !== song.id" class="song-number">{{ String(index + 1).padStart(2, '0') }}</span>
                <span v-else-if="currentPlayingId === song.id && isPlaying" class="playing-indicator">
                  <span class="bar"></span>
                  <span class="bar"></span>
                  <span class="bar"></span>
                </span>
                <span v-else class="play-btn" @click.stop="playSong(song)">▶</span>
              </div>
              <div class="col-title">
                <div class="song-info">
                  <div class="song-name">{{ song.name }}</div>
                  <div class="song-artist">{{ song.artist }}</div>
                </div>
              </div>
              <div class="col-album">{{ song.album }}</div>
              <div class="col-like">
                <span class="heart-icon">❤️</span>
              </div>
              <div class="col-duration">{{ song.duration }}</div>
            </div>
          </div>

          <!-- 空状态 -->
          <div v-if="songs.length === 0" class="empty-state">
            <p>暂无歌曲</p>
          </div>
        </template>
      </div>
    </main>
    <PlayerBar />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import Sidebar from "@/components/Home/Sidebar.vue";
import TopBar from "@/components/Home/TopBar.vue";
import PlayerBar from "@/components/Home/PlayerBar.vue";
import { getCoverUrl } from "@/utils/asset";
import type { Song, MySongs, GetMySongsResponse, PlayerState } from "@/types/music";

const activeTab = ref("songs");
const hoveredSong = ref<string | null>(null);
const loading = ref(false);
const error = ref<string | null>(null);

// 当前播放的歌曲ID和播放状态
const currentPlayingId = ref<string | null>(null);
const isPlaying = ref(false);

// 我的音乐信息
const mySongsInfo = ref<MySongs>({
  id: "my-songs",
  title: "我喜欢的音乐",
  cover: "",
  playCount: 0,
  songCount: 0,
});

// 歌曲列表
const songs = ref<Song[]>([]);

// 标签页
const tabs = computed(() => [
  { id: "songs", label: "歌曲", count: mySongsInfo.value.songCount },
  { id: "comments", label: "评论" },
  { id: "collectors", label: "收藏者" },
]);

const fetchData = async () => {
  loading.value = true;
  error.value = null;

  try {
    const result = await invoke<GetMySongsResponse>("get_my_songs");

    mySongsInfo.value = result.mySongs;
    songs.value = result.songs;
    
    // 如果没有封面，使用第一首歌的封面或默认封面
    if (!mySongsInfo.value.cover && songs.value.length > 0) {
      mySongsInfo.value.cover = songs.value[0].cover;
    }
  } catch (err) {
    console.error("获取歌曲列表失败:", err);
    error.value = "获取歌曲列表失败，请稍后重试";
  } finally {
    loading.value = false;
  }
};

const updateStateFromData = (state: PlayerState) => {
  isPlaying.value = state.isPlaying;
  if (state.currentSong) {
    currentPlayingId.value = state.currentSong.id;
  } else {
    currentPlayingId.value = null;
  }
};

const syncPlayerState = async () => {
  try {
    const state = await invoke<PlayerState>('get_player_state');
    updateStateFromData(state);
  } catch (err) {
    console.error('同步播放器状态失败:', err);
  }
};

const playAll = async () => {
  if (songs.value.length === 0) return;
  
  try {
    await invoke("play_all_from_index", {
      songIds: songs.value.map(s => s.id),
      startIndex: 0,
    });
    console.log("开始播放全部歌曲");
    // 同步状态
    await syncPlayerState();
  } catch (err) {
    console.error("播放全部失败:", err);
  }
};

const downloadAll = async () => {
  try {
    await invoke("download_songs", {
      songIds: songs.value.map(s => s.id),
    });
  } catch (err) {
    console.error("下载失败:", err);
  }
};

const playSong = async (song: Song) => {
  try {
    await invoke("play_song_from_list", { songId: song.id });
    console.log("开始播放:", song.name);
    // 同步状态
    await syncPlayerState();
  } catch (err) {
    console.error("播放歌曲失败:", err);
  }
};


// 格式化播放次数

function formatPlayCount(count: number) {
  if (count >= 10000) {
    return `${(count / 10000).toFixed(1)}万`;
  }
  return count.toString();
}

let unlistenStateChange: (() => void) | null = null;

onMounted(async () => {
  fetchData();
  syncPlayerState();
  
  // 监听播放状态变更事件
  unlistenStateChange = await listen<PlayerState>('player-state-changed', (event) => {
    updateStateFromData(event.payload);
  });
});

onUnmounted(() => {
  if (unlistenStateChange) {
    unlistenStateChange();
  }
});
</script>

<style scoped>
.my-songs-view {
  display: flex;
  height: 100vh;
  background: #fef0f6;
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.content-scroll {
  flex: 1;
  overflow-y: auto;
  padding: 0 24px 24px;
}

/* 加载状态 */
.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 0;
  color: #999;
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 3px solid #f3f3f3;
  border-top: 3px solid #d43c33;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 16px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

/* 错误状态 */
.error-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 0;
  color: #666;
}

.error-state p {
  margin-bottom: 16px;
}

/* 空状态 */
.empty-state {
  text-align: center;
  padding: 60px 0;
  color: #999;
}

/* 歌单头部 */
.playlist-header {
  display: flex;
  gap: 24px;
  margin-bottom: 32px;
}

.playlist-cover {
  position: relative;
  width: 200px;
  height: 200px;
  border-radius: 8px;
  overflow: hidden;
  flex-shrink: 0;
}

.playlist-cover img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.play-count-overlay {
  position: absolute;
  top: 8px;
  right: 8px;
  display: flex;
  align-items: center;
  gap: 4px;
  background: rgba(0, 0, 0, 0.6);
  color: white;
  font-size: 12px;
  padding: 4px 8px;
  border-radius: 12px;
}

.play-icon {
  font-size: 10px;
}

.heart-overlay {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  font-size: 48px;
  opacity: 0.9;
}

.playlist-info {
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 16px;
}

.playlist-title {
  font-size: 24px;
  font-weight: bold;
  margin: 0;
  color: #333;
}

.playlist-creator {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: #666;
}

.creator-avatar {
  width: 24px;
  height: 24px;
  border-radius: 50%;
}

.creator-name {
  color: #d43c33;
  cursor: pointer;
}

.create-date {
  color: #999;
}

.playlist-actions {
  display: flex;
  gap: 12px;
}

.btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  border-radius: 20px;
  border: none;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary {
  background: #d43c33;
  color: white;
}

.btn-primary:hover {
  background: #c0352d;
}

.btn-secondary {
  background: white;
  color: #333;
  border: 1px solid #ddd;
}

.btn-secondary:hover {
  background: #f5f5f5;
}

.btn-icon {
  font-size: 12px;
}

/* 标签页 */
.tabs {
  display: flex;
  gap: 24px;
  margin-bottom: 16px;
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
}

.tab-item {
  padding: 12px 0;
  font-size: 14px;
  color: #666;
  cursor: pointer;
  position: relative;
  transition: color 0.2s;
}

.tab-item:hover {
  color: #333;
}

.tab-item.active {
  color: #333;
  font-weight: 500;
}

.tab-item.active::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 2px;
  background: #d43c33;
}

.tab-count {
  font-size: 12px;
  color: #999;
  margin-left: 4px;
}

/* 歌曲列表 */
.songs-list {
  background: transparent;
}

.list-header {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  font-size: 12px;
  color: #999;
  border-bottom: 1px solid rgba(0, 0, 0, 0.05);
}

.song-item {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.2s;
}

.song-item:hover {
  background: rgba(0, 0, 0, 0.03);
}

.song-item.is-playing {
  background: rgba(212, 60, 51, 0.05);
}

.col-index {
  width: 40px;
  text-align: center;
  font-size: 12px;
  color: #999;
}

.song-number {
  display: block;
}

.play-btn {
  display: block;
  color: #d43c33;
  font-size: 12px;
  cursor: pointer;
}

/* 正在播放的动画指示器 */
.playing-indicator {
  display: flex;
  align-items: flex-end;
  justify-content: center;
  gap: 2px;
  height: 16px;
}

.playing-indicator .bar {
  width: 3px;
  background: #d43c33;
  animation: sound-bar 0.8s ease-in-out infinite;
}

.playing-indicator .bar:nth-child(1) {
  height: 8px;
  animation-delay: 0s;
}

.playing-indicator .bar:nth-child(2) {
  height: 14px;
  animation-delay: 0.2s;
}

.playing-indicator .bar:nth-child(3) {
  height: 10px;
  animation-delay: 0.4s;
}

@keyframes sound-bar {
  0%, 100% { transform: scaleY(0.5); }
  50% { transform: scaleY(1); }
}

.col-title {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 0;
}

.song-info {
  min-width: 0;
}

.song-name {
  font-size: 14px;
  color: #333;
  margin-bottom: 4px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.song-item.is-playing .song-name {
  color: #d43c33;
}

.song-artist {
  font-size: 12px;
  color: #999;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.col-album {
  width: 200px;
  font-size: 12px;
  color: #666;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.col-like {
  width: 60px;
  text-align: center;
}

.heart-icon {
  color: #d43c33;
  font-size: 14px;
}

.col-duration {
  width: 60px;
  text-align: right;
  font-size: 12px;
  color: #999;
}
</style>
