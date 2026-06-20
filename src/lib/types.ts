export interface GenreLink {
  name: string;
  slug: string;
}

export interface AnimeCard {
  title: string;
  slug: string;
  thumbnail: string | null;
  episode: string | null;
  day: string | null;
  rating: string | null;
  date: string | null;
}

export interface EpisodeLink {
  title: string;
  slug: string;
  date?: string | null;
}

export interface DownloadLink {
  host: string;
  url: string;
}

export interface DownloadQuality {
  format: string;
  quality: string;
  size: string | null;
  links: DownloadLink[];
}

export interface Pagination {
  current_page: number;
  last_page: number;
  has_next: boolean;
}

export interface AnimeGridResponse {
  anime_list: AnimeCard[];
  pagination: Pagination;
}

export interface HomeResponse {
  ongoing: AnimeCard[];
  complete: AnimeCard[];
}

export interface AnimeListGroups {
  [letter: string]: EpisodeLink[];
}

export interface JadwalRilisResponse {
  [day: string]: EpisodeLink[];
}

export interface AnimeDetail {
  title: string;
  japanese_title: string | null;
  thumbnail: string | null;
  score: string | null;
  producer: string | null;
  type: string | null;
  status: string | null;
  total_episode: string | null;
  duration: string | null;
  release_date: string | null;
  studio: string | null;
  genres: GenreLink[];
  synopsis: string | null;
  episode_list: EpisodeLink[];
  batch: EpisodeLink | null;
  recommendations: { title: string; slug: string; thumbnail: string | null }[];
}

export interface MirrorHost {
  host: string;
  data_content: string;
}

export interface EpisodeDetail {
  title: string;
  stream_url: string | null;
  mirrors: Record<string, MirrorHost[]>;
  downloads: DownloadQuality[];
  episode_list: EpisodeLink[];
  prev_episode: string | null;
  anime_slug: string | null;
  info: Record<string, unknown>;
}

export interface BatchDetail {
  title: string;
  thumbnail: string | null;
  anime_slug: string | null;
  info: Record<string, unknown>;
  downloads: DownloadQuality[];
}

export interface SearchResult {
  title: string;
  slug: string;
  thumbnail: string | null;
  genres: GenreLink[];
  status: string | null;
  rating: string | null;
}

export interface GenreDetailItem {
  title: string;
  slug: string;
  studio: string | null;
  episodes: string | null;
  rating: string | null;
  genres: GenreLink[];
  thumbnail: string | null;
  synopsis: string | null;
  season: string | null;
}

export interface GenreDetailResponse {
  genre_title: string;
  anime_list: GenreDetailItem[];
  pagination: Pagination;
}
