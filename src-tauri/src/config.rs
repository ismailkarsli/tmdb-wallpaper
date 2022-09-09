use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub tmdb_api_key: Option<String>,
    pub session_id: Option<String>,
    pub movies: bool,
    pub tv: bool,
    pub list_type: String,
    pub fetch_period: String,
    pub filter_photos_with_text: bool,
    pub language_of_photos: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tmdb_api_key: None,
            session_id: None,
            movies: true,
            tv: true,
            list_type: "watchlist".to_string(),
            fetch_period: "daily".to_string(),
            filter_photos_with_text: true,
            language_of_photos: Some("en".to_string()),
            width: Some(1280),
            height: Some(720),
        }
    }
}
