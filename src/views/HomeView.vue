<template>
  <div class="home-view">
    <Sidebar />
    <main class="main-content">
      <TopBar />
      
      <div class="content-scroll">
        <DiscoverSection />
        <PlaylistSection />
        <ActivitySection />
      </div>
    </main>
    <PlayerBar />
  </div>
</template>

<script setup lang="ts">
import Sidebar from "@/components/Home/Sidebar.vue";
import TopBar from "@/components/Home/TopBar.vue";
import DiscoverSection from "@/components/Home/DiscoverSection.vue";
import PlaylistSection from "@/components/Home/PlaylistSection.vue";
import ActivitySection from "@/components/Home/ActivitySection.vue";
import PlayerBar from "@/components/Home/PlayerBar.vue";
</script>

<style scoped>
.home-view {
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
</style>