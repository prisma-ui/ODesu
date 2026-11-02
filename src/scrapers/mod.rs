//! Scraper modules for parsing HTML from otakudesu.blog

pub mod anime_detail;
pub mod anime_grid;
pub mod anime_list;
pub mod batch;
pub mod episode;
pub mod genres;
pub mod home;
pub mod jadwal_rilis;
pub mod search;

use crate::types::*;
use crate::parsing::*;

// Re-export all parse functions
pub use anime_detail::parse_anime_detail;
pub use anime_grid::{parse_anime_grid, parse_pagination};
pub use anime_list::parse_anime_list;
pub use batch::parse_batch;
pub use episode::parse_episode;
pub use genres::{parse_genre_detail, parse_genre_list};
pub use home::parse_home;
pub use jadwal_rilis::parse_jadwal_rilis;
pub use search::parse_search;