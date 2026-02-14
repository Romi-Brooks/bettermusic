// src/router/index.ts
import { createRouter, createWebHashHistory, RouteRecordRaw } from "vue-router";
// 导入你的首页组件
import HomeView from "@/views/HomeView.vue";

// 路由规则：根路径 "/" 对应 HomeView
const routes: Array<RouteRecordRaw> = [
  {
    path: "/",          // 访问根路径（http://localhost:1420/）
    name: "home",
    component: HomeView // 渲染 HomeView 组件
  }
  // 其他路由可后续添加，比如歌单详情页
  // { path: "/playlist/:id", name: "playlist", component: () => import("@/views/PlaylistView.vue") }
];

// 创建路由实例（Tauri 必须用 Hash 路由，避免刷新404）
const router = createRouter({
  history: createWebHashHistory(), // 核心：Hash 模式
  routes
});

export default router;