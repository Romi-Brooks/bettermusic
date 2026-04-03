<template>
  <header class="top-bar">
    <div class="search-bar">
      <SearchIcon />
      <input type="text" placeholder="搜索歌曲、歌手、专辑" />
      <button class="voice-btn"></button>
    </div>
    <div class="user-info">
      <img src="@/assets/avatar.jpg" alt="头像" class="avatar" />
      <span class="username">RomiBrooks</span>
      <span class="vip-badge">VIP·贰</span>
    </div>
  </header>
</template>

<script setup lang="ts">
import SearchIcon from "@/components/Icon/SearchIcon.vue";
</script>

<style scoped>
.top-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 24px;
  background: #fef0f6;
  position: sticky;
  top: 0;
  z-index: 100;
}
.search-bar {
  display: flex;
  align-items: center;
  background: white;
  border-radius: 20px;
  padding: 8px 16px;
  width: 400px;
}
.search-bar input {
  border: none;
  outline: none;
  flex: 1;
  background: transparent;
}
.voice-btn {
  background: none;
  border: none;
  cursor: pointer;
}
.user-info {
  display: flex;
  align-items: center;
  gap: 12px;
}
.avatar {
  width: 32px;
  height: 32px;
  border-radius: 50%;
}
.vip-badge {
  font-size: 12px;
  color: #d43c33;
  border: 1px solid #d43c33;
  border-radius: 4px;
  padding: 2px 6px;
}
</style>
