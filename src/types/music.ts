// ==================== 基础歌曲类型 ====================

/**
 * 歌曲类型 - 统一使用此类型表示所有歌曲
 * 用于：播放列表、我的歌曲、搜索结果等
 */
export interface Song {
  id: string;
  name: string;           // 歌曲名称
  artist: string;         // 艺术家
  album: string;          // 专辑
  duration: string;       // 时长，格式: "03:53"
  durationSec: number;    // 时长（秒），用于播放器计算
  cover: string;          // 封面图片路径/URL
  filePath?: string;      // 本地文件路径（可选，用于本地播放）
}

// ==================== 歌单相关类型 ====================

/**
 * 歌单基础信息 - 用于展示歌单卡片、列表项
 */
export interface Playlist {
  id: string;
  title: string;
  cover: string;
  playCount?: number;
  description?: string;
}

/**
 * "我的音乐" 类型 - 用于 Sidebar 中的"我的音乐"项
 * 这是用户的个人音乐收藏入口
 */
export interface MySongs {
  id: string;
  title: string;          // 固定为 "我喜欢的音乐"
  cover: string;
  songCount: number;      // 收藏的歌曲数量
  playCount: number;      // 播放次数
}

/**
 * 创建的歌单信息 - 用于用户自己创建的歌单
 * 与 MySongs 区分，这是用户主动创建的歌单
 */
export interface PlaylistInfo {
  id: string;
  title: string;
  cover: string;
  playCount: number;
  creatorName: string;
  creatorAvatar: string;
  createDate: string;     // 格式: "2018-02-11"
  songCount: number;
  description?: string;   // 歌单描述（可选）
}

// ==================== 发现页相关类型 ====================

export interface DiscoverCard {
  id: string;
  title: string;
  cover: string;
  subtitle: string;
}

export interface Activity {
  id: string;
  title: string;
  cover: string;
}

// ==================== 播放器相关类型 ====================

/**
 * 播放模式
 */
export enum PlayMode {
  Sequential = 0,        // 顺序播放
  SingleLoop = 1,        // 单曲循环
  Random = 2,            // 随机播放
}

/**
 * 播放器状态 - 从后端获取的完整状态
 */
export interface PlayerState {
  isPlaying: boolean;           // 是否正在播放
  currentSong: Song | null;     // 当前播放的歌曲
  currentIndex: number | null;  // 当前播放索引
  progress: number;             // 播放进度 0-100
  durationSec: number;          // 总时长（秒）
  currentSec: number;           // 当前播放位置（秒）
  playMode: PlayMode;           // 播放模式
  queueLength: number;          // 播放队列长度
}

/**
 * 播放器状态（旧版，兼容用）
 */
export interface PlayerStateOld {
  isPlaying: boolean;
  currentSong: Song | null;
  progress: number;       // 0-100
  volume: number;         // 0-100
  playMode: PlayMode;
}

// ==================== API 响应类型 ====================

/**
 * 获取我的歌曲响应
 */
export interface GetMySongsResponse {
  mySongs: MySongs;       // 我的音乐信息
  songs: Song[];          // 歌曲列表
}

/**
 * 获取创建的歌单详情响应
 */
export interface GetPlaylistResponse {
  playlist: PlaylistInfo;
  songs: Song[];
}
