use serde::{Deserialize, Serialize};
use std::io::Read;

const CONFIG_FILE: &str = "AppConfig.toml";

#[derive(Serialize, Deserialize)]
struct Config {
    tmdb_api_key: Option<String>,
    session_id: Option<String>,
    latest_fetch: Option<String>,
}

pub fn get(key: &str) -> Option<String> {
    let mut file = std::fs::File::open(CONFIG_FILE).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let config: Config = toml::from_str(&contents).unwrap();
    match key {
        "tmdb_api_key" => config.tmdb_api_key,
        "session_id" => config.session_id,
        "latest_fetch" => config.latest_fetch,
        _ => panic!("No such key"),
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
        "latest_fetch" => config.latest_fetch = Some(value),
        _ => panic!("No such key"),
    }
    let contents = toml::to_string(&config).unwrap();
    std::fs::write(CONFIG_FILE, contents).unwrap();
}
