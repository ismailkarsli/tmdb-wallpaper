use serde::{Deserialize, Serialize};
use std::io::Read;

const CONFIG_FILE: &str = "AppConfig.toml";

#[derive(Serialize, Deserialize)]
pub struct Config {
    tmdb_api_key: Option<String>,
    session_id: Option<String>,
    movies: bool,
    tv: bool,
    list_type: String,
    fetch_period: String,
    filter_photos_with_text: bool,
    language_of_photos: String,
    width: Option<u32>,
    height: Option<u32>,
}

pub fn get(key: &str) -> Option<String> {
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
        "language_of_photos" => Some(config.language_of_photos.clone()),
        "width" => Some(config.width.unwrap_or(0).to_string()),
        "height" => Some(config.height.unwrap_or(0).to_string()),
        _ => None,
    }
}

pub fn set(key: &str, value: String) {
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
        "language_of_photos" => config.language_of_photos = value,
        "width" => config.width = Some(value.parse::<u32>().unwrap()),
        "height" => config.height = Some(value.parse::<u32>().unwrap()),
        _ => (),
    }
    let contents = toml::to_string(&config).unwrap();
    std::fs::write(CONFIG_FILE, contents).unwrap();
}

pub fn set_all(config: Config) {
    let contents = toml::to_string(&config).unwrap();
    std::fs::write(CONFIG_FILE, contents).unwrap();
}

pub fn get_all() -> Config {
    let mut file = std::fs::File::open(CONFIG_FILE).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    toml::from_str(&contents).unwrap()
}
