import { convertFileSrc } from '@tauri-apps/api/core';

// 1. 预加载 assets 目录的资源
const assetModules = import.meta.glob("@/assets/*", {
  eager: true,
  query: "?url",
  import: "default",
}) as Record<string, string>;

// 默认封面 URL
const DEFAULT_COVER = 'https://p1.music.126.net/6y-UleORITEDbvrOLV0Q8A==/5639395138885805.jpg';

/**
 * 统一处理封面路径（支持本地资源 + 远程 URL + 本地文件路径 + Base64 Data URL）
 * @param cover - "@/assets/xxx.webp" | "https://xxx.jpg" | "C:\\Users\\...\\cover.jpg" | "data:image/jpeg;base64,..."
 * @returns 可直接用于 img/src 或 background-image 的 URL
 */
export const getCoverUrl = (cover: string): string => {
  if (!cover) {
    return DEFAULT_COVER;
  }

  // 1. Base64 Data URL 直接返回
  if (cover.startsWith('data:')) {
    return cover;
  }

  // 2. 远程 URL 直接返回
  if (/^https?:\/\//.test(cover)) {
    return cover;
  }

  // 3. 本地文件路径（Windows 盘符路径或 Unix 绝对路径）
  // 支持格式: "C:\\Users\\...", "/home/user/...", "file://..."
  if (cover.startsWith('file://') || cover.includes(':\\') || 
      (cover.startsWith('/') && !cover.startsWith('/src/'))) {
    try {
      return convertFileSrc(cover);
    } catch (e) {
      console.error('转换本地文件路径失败:', e);
      return DEFAULT_COVER;
    }
  }

  // 4. 本地资源：@/assets/xxx.webp → /src/assets/xxx.webp
  if (cover.startsWith('@/')) {
    const localPath = cover.replace("@/", "/src/");
    return assetModules[localPath] || DEFAULT_COVER;
  }

  return DEFAULT_COVER;
};

/**
 * 检查是否为本地文件路径
 */
export const isLocalFilePath = (path: string): boolean => {
  if (!path) return false;
  return path.startsWith('file://') || 
         path.includes(':\\') || 
         (path.startsWith('/') && !path.startsWith('/src/') && !path.startsWith('http'));
};

/**
 * 获取默认封面 URL
 */
export const getDefaultCover = (): string => {
  return 'https://p1.music.126.net/6y-UleORITEDbvrOLV0Q8A==/5639395138885805.jpg';
};
