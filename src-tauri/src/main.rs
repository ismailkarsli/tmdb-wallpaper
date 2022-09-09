#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod config;
mod lib;

use clokwerk::{Scheduler, TimeUnits};
use core::panic;
use lib::tmdb::Image;
use lib::tmdb::Tmdb;
use rand::seq::SliceRandom;
use reqwest;
use std::time::Duration;
use std::{fs, path::PathBuf};
use tauri::api::process;
use tauri::async_runtime::block_on;
use tauri::{
    AppHandle, CustomMenuItem, Manager, RunEvent, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, WindowEvent,
};
use wallpaper;

pub static APP_NAME: &str = "WallpaperFlix";

#[tauri::command]
fn restart_app(app_handle: AppHandle) {
    process::restart(&app_handle.env());
}

#[tauri::command]
async fn manual_fetch() {
    fetch_wallpaper().await.ok();
}

#[tauri::command]
fn save_settings(settings: &str) -> Result<String, String> {
    let config: config::Config = serde_json::from_str(settings).unwrap();
    if !config.movies || !config.tv {
        return Err("You must select at least one of movies or tv".to_string());
    }
    confy::store(APP_NAME, None, config).unwrap();
    Ok(format!("Settings saved!"))
}

#[tauri::command]
fn get_settings() -> String {
    let config: config::Config = confy::load(APP_NAME, None).unwrap();
    serde_json::to_string(&config).unwrap()
}

#[tauri::command]
async fn create_request_token() -> Result<String, String> {
    let config: config::Config = confy::load(APP_NAME, None).unwrap();
    let tmdb_api_key = config.tmdb_api_key.unwrap();
    let tmdb = Tmdb::new(tmdb_api_key, None);

    let request_token = tmdb.create_request_token().await;

    match request_token {
        Ok(token) => Ok(token),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn create_session_id(request_token: String) -> Result<String, String> {
    let mut config: config::Config = confy::load(APP_NAME, None).unwrap();
    let tmdb_api_key = config.tmdb_api_key.clone().unwrap();
    let mut tmdb = Tmdb::new(tmdb_api_key, None);
    let session_id = tmdb.create_session_id(&request_token).await;
    match session_id {
        Ok(session_id) => {
            config.session_id = Some(session_id.clone());
            confy::store(APP_NAME, None, config).unwrap();
            Ok("Session ID created!".to_string())
        }
        Err(_) => Err("Error creating session ID. Make sure you approve the link.".to_string()),
    }
}

#[tokio::main]
async fn main() {
    let config: config::Config = confy::load(APP_NAME, None).unwrap();

    let period = config.fetch_period.clone();

    let mut scheduler = Scheduler::new();

    let period = match period.as_str() {
        "every minute" => scheduler.every(1.minutes()),
        "every half hour" => scheduler.every(30.minutes()),
        "hourly" => scheduler.every(1.hours()),
        "half day" => scheduler.every(12.hours()),
        "daily" => scheduler.every(1.days()),
        "weekly" => scheduler.every(1.weeks()),
        "monthly" => scheduler.every(30.days()),
        _ => scheduler.every(1.days()),
    };

    period.run(|| {
        block_on(fetch_wallpaper()).ok();
    });

    let _thread_handle = scheduler.watch_thread(Duration::from_secs(30));

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let toggle_visibility = CustomMenuItem::new("toggle_visibility".to_string(), "Show/Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(toggle_visibility)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let tray = SystemTray::new().with_menu(tray_menu);

    let app = tauri::Builder::default()
        .setup(move |app| {
            if config.tmdb_api_key.is_none() || config.session_id.is_none() {
                app.get_window("main").unwrap().show().unwrap();
            }
            Ok(())
        })
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "toggle_visibility" => {
                    let window = app.get_window("main").unwrap();
                    let is_visible = window.is_visible().unwrap();
                    if is_visible {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                    }
                }
                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            restart_app,
            save_settings,
            get_settings,
            create_request_token,
            create_session_id,
            manual_fetch
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|app, event| match event {
        RunEvent::ExitRequested { api, .. } => api.prevent_exit(),
        RunEvent::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                let window = app.get_window("main").unwrap();
                window.hide().unwrap();
            }
            _ => {}
        },
        _ => (),
    });
}

async fn fetch_wallpaper() -> Result<String, String> {
    let config: config::Config = confy::load(APP_NAME, None).unwrap();

    if config.tmdb_api_key.is_none() || config.session_id.is_none() {
        return Err("No TMDB API key or session ID".to_string());
    }

    let mut tmdb = Tmdb::new(config.tmdb_api_key.unwrap(), config.session_id);

    let mut options = vec![];
    if config.movies {
        options.push("movie");
    };
    if config.tv {
        options.push("tv");
    }
    let movie_or_tv = options.choose(&mut rand::thread_rng()).unwrap();

    struct MovieOrTV {
        // name: String,
        id: i32,
    }
    let selected_item;
    match movie_or_tv {
        &"movie" => {
            let watchlist;
            if config.list_type == "watchlist" {
                watchlist = tmdb.get_movie_watchlist().await
            } else {
                watchlist = tmdb.get_movie_favorites().await
            }
            if watchlist.results.len() == 0 {
                return Err(format!("No movies in {}.", config.list_type));
            }
            let random_movie = watchlist.results.choose(&mut rand::thread_rng()).unwrap();
            selected_item = Some(MovieOrTV {
                // name: random_movie.title.clone(),
                id: random_movie.id,
            });
        }
        &"tv" => {
            let watchlist;
            if config.list_type == "watchlist" {
                watchlist = tmdb.get_tv_watchlist().await
            } else {
                watchlist = tmdb.get_tv_favorites().await
            }
            if watchlist.results.len() == 0 {
                return Err(format!("No TV shows in {}.", config.list_type));
            }
            let random_tv = watchlist.results.choose(&mut rand::thread_rng()).unwrap();
            selected_item = Some(MovieOrTV {
                // name: random_tv.name.clone(),
                id: random_tv.id,
            });
        }
        _ => {
            panic!("Invalid movie or tv selection");
        }
    }

    let selected_item = selected_item.unwrap();
    let language = match config.filter_photos_with_text {
        true => config.language_of_photos,
        false => None,
    };

    let images = match movie_or_tv {
        &"movie" => tmdb.get_movie_images(selected_item.id, language).await,
        &"tv" => tmdb.get_tv_images(selected_item.id, language).await,
        _ => return Err("Invalid movie or tv selection".to_string()),
    };

    let usable_images: Vec<Image> = images
        .backdrops
        .into_iter()
        .filter(|image| {
            image.height >= config.height.unwrap()
                && image.width >= config.width.unwrap()
                && image.aspect_ratio > 1.75
        })
        .collect();

    if usable_images.len() == 0 {
        return Err(format!(
            "No usable images found with {}x{} resolution and aspect ratio greater than 1.75",
            config.width.unwrap(),
            config.height.unwrap()
        ));
    }

    let selected_image = usable_images.choose(&mut rand::thread_rng()).unwrap();

    let image_url = format!(
        "https://image.tmdb.org/t/p/original{}",
        selected_image.file_path
    );

    // Download the image to a file located in pictures folder
    let wallpaper_target: PathBuf = dirs::picture_dir()
        .unwrap_or(dirs::home_dir().unwrap())
        .join("wallpaper.jpg");
    let resp = reqwest::get(&image_url).await.unwrap();
    fs::write(&wallpaper_target, resp.bytes().await.unwrap()).unwrap();

    // Set the wallpaper
    wallpaper::set_from_path(&wallpaper_target.to_str().unwrap()).unwrap();
    wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();

    Ok("Wallpaper successfully set.".to_string())
}
