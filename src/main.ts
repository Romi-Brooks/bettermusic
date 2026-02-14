import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";

// 隐藏滑动条
import "./styles/global.css";

// 挂载应用到主页
createApp(App)
  .use(router) // 启用路由
  .mount("#app");