// 1. 预加载 assets 目录的资源
const assetModules = import.meta.glob("@/assets/*", {
  eager: true,
  query: "?url",
  import: "default",
}) as Record<string, string>;

/**
 * 统一处理封面路径（支持本地资源 + 远程 URL）
 * @param cover - "@/assets/xxx.webp" | "https://xxx.jpg"
 * @returns 可直接用于 img/src 或 background-image 的 URL
 */
export const getCoverUrl = (cover: string): string => {
  if (!cover) return "";

  // 1. 远程 URL 直接返回
  if (/^https?:\/\//.test(cover)) {
    return cover;
  }

  // 2. 本地资源：@/assets/xxx.webp → /src/assets/xxx.webp
  const localPath = cover.replace("@/", "/src/");
  return assetModules[localPath] || cover;
};
