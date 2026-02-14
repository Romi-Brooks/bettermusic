export interface Playlist {
  id: string;
  title: string;
  cover: string;
  playCount?: number;
  description?: string;
}

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

export interface Track {
  id: string;
  title: string;
  artist: string;
  cover: string;
  isVip?: boolean;
}