#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod config;
mod lib;

use clokwerk::{Scheduler, TimeUnits};
use lib::tmdb::Tmdb;
use rand::seq::{IteratorRandom, SliceRandom};
use reqwest;
use std::time::Duration;
use std::{fs, path::PathBuf};
use tauri::api::process;
use tauri::{
    AppHandle, CustomMenuItem, Manager, RunEvent, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, WindowEvent,
};
use wallpaper;

#[tauri::command]
fn restart_app(app_handle: AppHandle) {
    process::restart(&app_handle.env());
}

#[tauri::command]
fn save_settings(settings: &str) -> String {
    let settings = serde_json::from_str(settings).unwrap();
    config::set_all(settings);
    format!("Settings saved!")
}

#[tauri::command]
fn get_settings() -> String {
    serde_json::to_string(&config::get_all()).unwrap()
}

fn main() {
    println!("Starting background thread");

    let mut scheduler = Scheduler::new();

    let period = config::get("fetch_period").unwrap();

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
        let _ = fetch_wallpaper();
    });

    let _ = scheduler.watch_thread(Duration::from_secs(30));

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let toggle_visibility = CustomMenuItem::new("toggle_visibility".to_string(), "Show/Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(toggle_visibility)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let tray = SystemTray::new().with_menu(tray_menu);

    let app = tauri::Builder::default()
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
            get_settings
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

async fn fetch_wallpaper() {
    let tmdb_api_key: String = config::get("tmdb_api_key").unwrap();
    let session_id: Option<String> = config::get("session_id");

    let mut tmdb = Tmdb::new(tmdb_api_key, session_id);

    if tmdb.session_id.is_none() {
        let request_url: String = tmdb.create_request_url().await;
        open::that(&request_url).ok();
        println!(
            "Please approve the request token at {}. Once you approve, click enter here.",
            request_url
        );
        loop {
            let _i: i32 = text_io::read!("{}\n");
            let session_id = tmdb.create_session_id(&request_url).await;
            match session_id {
                Ok(session_id) => {
                    config::set("session_id", session_id);
                    break;
                }
                Err(e) => {
                    println!("Make sure you approved the request token. Error: {}", e);
                }
            }
        }
    }

    let options = vec!["movie", "tv"];
    let movie_or_tv = options.choose(&mut rand::thread_rng()).unwrap();

    struct MovieOrTV {
        name: String,
        id: i32,
    }
    let selected_item;
    match movie_or_tv {
        &"movie" => {
            let watchlist = tmdb.get_movie_watchlist().await;
            let random_movie = watchlist.results.choose(&mut rand::thread_rng()).unwrap();
            selected_item = Some(MovieOrTV {
                name: random_movie.title.clone(),
                id: random_movie.id,
            });
        }
        &"tv" => {
            let watchlist = tmdb.get_tv_watchlist().await;
            let random_tv = watchlist.results.choose(&mut rand::thread_rng()).unwrap();
            selected_item = Some(MovieOrTV {
                name: random_tv.name.clone(),
                id: random_tv.id,
            });
        }
        _ => {
            panic!("Invalid movie or tv selection");
        }
    }

    let selected_item = selected_item.unwrap();
    println!("Selected {}: {}", movie_or_tv, selected_item.name);

    let images = match movie_or_tv {
        &"movie" => tmdb.get_movie_images(selected_item.id).await,
        &"tv" => tmdb.get_tv_images(selected_item.id).await,
        _ => {
            panic!("Invalid movie or tv selection");
        }
    };

    let usable_images = images
        .backdrops
        .into_iter()
        .filter(|image| image.height > 1920 && image.width > 1080 && image.aspect_ratio > 1.75);

    let selected_image = usable_images.choose(&mut rand::thread_rng()).unwrap();

    let image_url = format!(
        "https://image.tmdb.org/t/p/original{}",
        selected_image.file_path
    );

    // Download the image to a file located in pictures folder
    let wallpaper_target: PathBuf = dirs::picture_dir().unwrap().join("wallpaper.jpg");
    let resp = reqwest::get(&image_url).await.unwrap();
    fs::write(&wallpaper_target, resp.bytes().await.unwrap()).unwrap();

    // Set the wallpaper
    wallpaper::set_from_path(&wallpaper_target.to_str().unwrap()).unwrap();
    wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
    std::process::exit(0);
}
