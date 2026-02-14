<template>
  <section class="discover-section">
    <div class="discover-cards">
      <div
        v-for="card in discoverCards"
        :key="card.id"
        class="discover-card"
        :style="{ backgroundImage: `url(${getCoverUrl(card.cover)})` }"
      >
        <div class="card-content">
          <h3>{{ card.title }}</h3>
          <p>{{ card.subtitle }}</p>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import type { DiscoverCard } from "@/types/music";
import { getCoverUrl } from "@/utils/asset";

const discoverCards: DiscoverCard[] = [
  {
    id: "d1",
    title: "每日推荐",
    cover: "@/assets/cover-1.webp",
    subtitle: "每日推荐 | 从「Sleepless Nights」...",
  },
  {
    id: "d2",
    title: "私人雷达",
    cover: "https://p1.music.126.net/6y-UleORITEDbvrOLV0Q8A==/5639395138885805.jpg",
    subtitle: "今天从《Sugar》听起",
  },
  {
    id: "d3",
    title: "私人漫游",
    cover: "https://p1.music.126.net/6y-UleORITEDbvrOLV0Q8A==/5639395138885805.jpg",
    subtitle: "从「Runnin」开启无限漫游",
  },
];
</script>

<style scoped>
.discover-cards {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
  gap: 16px;
  margin-bottom: 32px;
}
.discover-card {
  position: relative;
  height: 180px;
  border-radius: 8px;
  background-size: cover;
  background-position: center;
  overflow: hidden;
  cursor: pointer;
}
.discover-card::before {
  content: "";
  position: absolute;
  inset: 0;
  background: linear-gradient(transparent, rgba(0, 0, 0, 0.6));
}
.card-content {
  position: absolute;
  bottom: 12px;
  left: 12px;
  color: white;
  z-index: 1;
}
.card-content h3 {
  margin: 0 0 4px;
  font-size: 16px;
}
.card-content p {
  margin: 0;
  font-size: 12px;
  opacity: 0.9;
}
</style>