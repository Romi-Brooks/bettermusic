<template>
  <svg
    class="search-icon"
    xmlns="http://www.w3.org/2000/svg"
    :width="width"
    :height="height"
    viewBox="0 0 24 24"
    fill="none"
    :stroke="color"
    :stroke-width="isHover ? 2.5 : 1.5"
    stroke-linecap="round"
    stroke-linejoin="round"
    @mouseenter="isHover = true"
    @mouseleave="isHover = false"
  >
    <circle cx="11" cy="11" r="8"/>
    <line x1="21" y1="21" x2="16.65" y2="16.65"/>
  </svg>
</template>

<script setup lang="ts">
import { ref } from "vue";

defineProps({
  width: { type: String, default: "16px" },
  height: { type: String, default: "16px" },
  color: { type: String, default: "#999" }
});

// 定义悬浮状态
const isHover = ref(false);
</script>

<style scoped>
.search-icon {
  cursor: pointer; /* 鼠标移上去显示手型 */
  transition: stroke-width 0.2s ease; /* 加粗动画过渡 */
}
</style>