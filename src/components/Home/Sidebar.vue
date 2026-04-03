<template>
  <aside class="sidebar">
    <!-- 固定 logo -->
    <div class="sidebar__header">
      <div class="sidebar__logo">
        <img :src="getCoverUrl('@/assets/vue.svg')" alt="Better Music Icon" />
        <span>Better Music</span>
      </div>
    </div>

    <!-- sidebar 内容 -->
    <div class="sidebar__content">
      <nav class="sidebar__nav">
        <ul>
          <li
            v-for="item in navItems"
            :key="item.id"
            :class="{ active: activeNav === item.id }"
            @click="activeNav = item.id"
          >
            <span class="icon">{{ item.icon }}</span>
            <span class="label">{{ item.label }}</span>
            <span v-if="item.badge" class="badge">{{ item.badge }}</span>
          </li>
        </ul>
      </nav>

      <div class="sidebar__divider"></div>

      <div class="sidebar__section">
        <h4>我的</h4>
        <ul>
          <li 
            v-for="item in myItems" 
            :key="item.id"
            :class="{ active: route.path === item.route }"
            @click="handleMyItemClick(item)"
          >
            <span class="icon">{{ item.icon }}</span>
            <span class="label">{{ item.label }}</span>
          </li>
        </ul>
      </div>

      <div class="sidebar__divider"></div>

      <div class="sidebar__section">
        <div class="section-header">
          <h4>创建的歌单</h4>
          <button class="add-btn">+</button>
        </div>
        <ul>
          <li v-for="item in createdPlaylists" :key="item.id">
            <img :src="getCoverUrl(item.cover)" alt="" class="playlist-cover" />
            <span class="label">{{ item.label }}</span>
          </li>
        </ul>
      </div>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useRoute, useRouter } from "vue-router";

const route = useRoute();
const router = useRouter();
const activeNav = ref("recommend");

const navItems = [
  { id: "recommend", icon: "🏠", label: "推荐", badge: "" },
  { id: "精选", icon: "🎵", label: "精选", badge: "" },
  { id: "播客", icon: "🎙️", label: "播客", badge: "" },
  { id: "漫游", icon: "🌏", label: "漫游", badge: "" },
  { id: "关注", icon: "👀", label: "关注", badge: "" },
];

const myItems = [
  { id: "like", icon: "❤️", label: "我的音乐", route: "/my-songs" },
  { id: "recent", icon: "⏯️", label: "最近播放", route: "/recent" },
  { id: "podcast", icon: "🎙️", label: "我的播客", route: "/podcast" },
];

// 处理"我的"菜单点击
const handleMyItemClick = (item: typeof myItems[0]) => {
  if (item.route) {
    router.push(item.route);
  }
};

// 模拟多歌单（测试滚动）
const createdPlaylists = [
  { id: "pl1", cover: "@/assets/cover-1.webp", label: "千禧年" },
  { id: "pl2", cover: "@/assets/cover-2.webp", label: "Best Music 2021 - 年度最佳" },
  { id: "pl3", cover: "@/assets/vue.svg", label: "Best Music 2020 - ..." },
  { id: "pl4", cover: "@/assets/vue.svg", label: "复古disco" },
  { id: "pl5", cover: "@/assets/vue.svg", label: "深夜治愈" },
  { id: "pl6", cover: "@/assets/vue.svg", label: "运动燃曲" },
  { id: "pl7", cover: "@/assets/vue.svg", label: "自习专用" },
  { id: "pl8", cover: "@/assets/vue.svg", label: "开车必听" },
  { id: "pl9", cover: "@/assets/vue.svg", label: "粤语经典" },
];

// 统一处理 cover 路径
const getCoverUrl = (cover: string) => {
  if (cover.startsWith("http")) return cover;
  try {
    const modules = import.meta.glob("@/assets/*", { eager: true, as: "url" });
    const localPath = cover.replace("@/", "/src/");
    return modules[localPath] || "https://p1.music.126.net/6y-UleORITEDbvrOLV0Q8A==/5639395138885805.jpg";
  } catch (err) {
    return "https://p1.music.126.net/6y-UleORITEDbvrOLV0Q8A==/5639395138885805.jpg";
  }
};
</script>

<style scoped>
/* 外层 sidebar：垂直 flex，高度100% */
.sidebar {
  width: 200px;
  background: #f8d7e9;
  padding: 0; /* 去掉外层 padding，避免滚动区间距问题 */
  display: flex;
  flex-direction: column;
  height: 100%; /* 占满父容器高度 */
}

/* 固定头部（logo）：不滚动，单独加 padding */
.sidebar__header {
  padding: 16px; /* 只给 logo 加 padding */
  flex-shrink: 0; /* 关键：固定高度，不被挤压 */
}

.sidebar__logo {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 18px;
  font-weight: bold;
  color: #d43c33;
}

.sidebar__logo img {
  width: 32px;
  height: 32px;
  object-fit: cover;
}

/* 可滚动内容区：logo 下方所有内容 */
.sidebar__content {
  flex: 1; /* 占满剩余高度 */
  overflow-y: auto; /* 内容超出时滚动 */
  padding: 0 16px 16px; /* 左右+底部 padding，顶部和 logo 间距通过 gap 控制 */
  scrollbar-width: none; /* Firefox 隐藏滚动条 */
  -ms-overflow-style: none; /* IE/Edge 隐藏滚动条 */
}

/* 隐藏 webkit 内核滚动条 */
.sidebar__content::-webkit-scrollbar {
  display: none !important;
}

/* 以下样式不变，微调间距 */
.sidebar__nav ul,
.sidebar__section ul {
  list-style: none;
  padding: 0;
  margin: 0;
}

.sidebar__nav li,
.sidebar__section li {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 8px;
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.2s;
}

.sidebar__nav li.active {
  background: #d43c33;
  color: white;
}

.sidebar__divider {
  height: 1px;
  background: rgba(0, 0, 0, 0.1);
  margin: 16px 0; /* 分隔线上下间距 */
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.add-btn {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  border: none;
  background: #d43c33;
  color: white;
  font-size: 14px;
  cursor: pointer;
}

.playlist-cover {
  width: 32px;
  height: 32px;
  border-radius: 4px;
  object-fit: cover;
  background-color: #f0f0f0;
}
</style>