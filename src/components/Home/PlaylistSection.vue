<template>
  <section class="playlist-section">
    <div class="section-header">
      <h2>推荐歌单</h2>
      <a href="#" class="more">更多 &gt;</a>
    </div>
    <div class="playlist-grid">
      <div v-for="pl in playlists" :key="pl.id" class="playlist-card">
        <div class="cover-wrapper">
          <img :src="getCoverUrl(pl.cover)" alt="" />
          <span v-if="pl.playCount" class="play-count">
            {{ formatPlayCount(pl.playCount) }}
          </span>
        </div>
        <h4 class="title">{{ pl.title }}</h4>
        <p class="desc">{{ pl.description }}</p>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import type { Playlist } from "@/types/music";
import { getCoverUrl } from "@/utils/asset";

const playlists: Playlist[] = [
  {
    id: "p1",
    title: "【电音】全球200首最佳电子舞曲",
    cover: "@/assets/cover-1.webp",
    playCount: 1260000,
    description: "你与天空皆晴朗",
  },
  {
    id: "p2",
    title: "ColorBass彩虹贝斯",
    cover: "@/assets/cover-2.webp",
    playCount: 624400,
    description: "这里聚集了Revealed和其他各大厂牌的高质量Bass House音乐，用这张歌单来开启你的周末吧！",
  },
  // ...更多歌单
];

function formatPlayCount(count: number) {
  if (count >= 10000) {
    return `${(count / 10000).toFixed(1)}万`;
  }
  return count.toString();
}
</script>

<style scoped>
.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}
.playlist-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
  gap: 16px;
}
.playlist-card {
  cursor: pointer;
}
.cover-wrapper {
  position: relative;
  width: 100%;
  aspect-ratio: 1;
  border-radius: 8px;
  overflow: hidden;
  margin-bottom: 8px;
}
.cover-wrapper img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}
.play-count {
  position: absolute;
  bottom: 4px;
  right: 4px;
  background: rgba(0, 0, 0, 0.6);
  color: white;
  font-size: 12px;
  padding: 2px 6px;
  border-radius: 4px;
}
.title {
  font-size: 14px;
  margin: 0 0 4px;
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
.desc {
  font-size: 12px;
  color: #666;
  margin: 0;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>