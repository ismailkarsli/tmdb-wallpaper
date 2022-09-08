use serde::{Deserialize, Serialize};
use std::io::Read;

const CONFIG_FILE: &str = "AppConfig.toml";
static mut IS_CONFIG_FILE_READY: bool = false;

#[derive(Serialize, Deserialize)]
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

pub fn get(key: &str) -> Option<String> {
    check_config_file();
    let mut file = std::fs::File::open(CONFIG_FILE).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let config: Config = toml::from_str(&contents).unwrap();

    match key {
        "tmdb_api_key" => config.tmdb_api_key.clone(),
        "session_id" => config.session_id.clone(),
        "movies" => Some(config.movies.to_string()),
        "tv" => Some(config.tv.to_string()),
        "list_type" => Some(config.list_type.clone()),
        "fetch_period" => Some(config.fetch_period.clone()),
        "filter_photos_with_text" => Some(config.filter_photos_with_text.to_string()),
        "language_of_photos" => config.language_of_photos.clone(),
        "width" => Some(config.width.unwrap_or(0).to_string()),
        "height" => Some(config.height.unwrap_or(0).to_string()),
        _ => None,
    }
}

pub fn set(key: &str, value: String) {
    check_config_file();
    let mut file = std::fs::File::open(CONFIG_FILE).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut config: Config = toml::from_str(&contents).unwrap();
    match key {
        "tmdb_api_key" => config.tmdb_api_key = Some(value),
        "session_id" => config.session_id = Some(value),
        "movies" => config.movies = value == "true",
        "tv" => config.tv = value == "true",
        "list_type" => config.list_type = value,
        "fetch_period" => config.fetch_period = value,
        "filter_photos_with_text" => config.filter_photos_with_text = value == "true",
        "language_of_photos" => config.language_of_photos = Some(value),
        "width" => config.width = Some(value.parse::<u32>().unwrap()),
        "height" => config.height = Some(value.parse::<u32>().unwrap()),
        _ => (),
    }
    let contents = toml::to_string(&config).unwrap();
    std::fs::write(CONFIG_FILE, contents).unwrap();
}

pub fn set_all(config: Config) {
    check_config_file();
    let contents = toml::to_string(&config).unwrap();
    std::fs::write(CONFIG_FILE, contents).unwrap();
}

pub fn get_all() -> Config {
    check_config_file();
    let mut file = std::fs::File::open(CONFIG_FILE).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    toml::from_str(&contents).unwrap()
}

fn check_config_file() {
    if unsafe { !IS_CONFIG_FILE_READY } {
        if !std::path::Path::new(CONFIG_FILE).exists() {
            let config = Config {
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
            };
            let contents = toml::to_string(&config).unwrap();
            std::fs::write(CONFIG_FILE, contents).unwrap();
        }
        unsafe {
            IS_CONFIG_FILE_READY = true;
        }
    }
}
