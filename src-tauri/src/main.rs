// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::Write;
use std::{fs, path::PathBuf, thread};

use profile::set_profile_internal;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use settings::Settings;
use tauri::{Window, CustomMenuItem, SystemTrayMenu, SystemTray, SystemTrayEvent, Manager};

use crate::settings::{set_install_path, change_bool_setting};
use crate::profile::{create_profile, set_profile, open_profile_folder, update_profile, Profile};

mod settings;
mod profile;

#[derive(Serialize, Deserialize, Clone)]
struct Data {
    profiles: Vec<Profile>,
    settings: Settings
}

#[tauri::command]
fn get_data() -> Data {
    return load_data();
}

#[tauri::command]
fn get_data_async(window: Window) {
    window.emit("set_data", load_data()).expect("Failed to emit");
}

fn load_data() -> Data {
    let state = Settings::load();
    let mut profiles = vec![];
    let profiles_path = Settings::settings_dir().join("profiles");
    
    if !profiles_path.exists() {
        match fs::create_dir(&profiles_path) {
            Ok(()) => {}
            Err(err) => panic!("Failed to create profiles dir: {}", err)
        }
    }

    for entry in fs::read_dir(profiles_path).expect("Failed to iter profiles") {
        let path = entry.expect("").path();
        if path.is_dir() {
            let res = Profile::load_folder(path).expect("Failed to load profile");
            profiles.push(res);
        }
    }

    Data {
        profiles,
        settings: state
    }
}

#[tauri::command]
fn launch_game(window: Window) {
    let settings = Settings::load();
    
    thread::spawn(move || {
        let path;
        if settings.install_path.is_some() {
            path = settings.install_path.clone().unwrap();
        } else {
            window.emit("launch_err", "Install path not defined").expect("Failed to emit");
            return
        }

        let config_file = PathBuf::from(&path).join("Mindustry.json");
        if !config_file.exists() {
            window.emit("launch_err", "Invalid install location").expect("Failed to emit");
            return
        }

        let profile_id = settings.current_profile;
        let profile_path = Settings::settings_dir().join("profiles").join(PathBuf::from(profile_id.to_string()));
        let jar_path = PathBuf::from(&profile_path).join("desktop.jar");
        let game_path = PathBuf::from(profile_path).join("game");

        if !game_path.exists() {
            fs::create_dir(&game_path);
        }

        let data = format!(
            "{{
            \"jrePath\": \"jre\",
            \"classPath\": [
              \"{}\"
            ],
            \"mainClass\": \"mindustry.desktop.DesktopLauncher\",
            \"useZgcIfSupportedOs\": false,
            \"vmArgs\": [
              \"-Dhttps.protocols=TLSv1.2,TLSv1.1,TLSv1\",
              \"-XX:+ShowCodeDetailsInExceptionMessages\"
            ]
          }}", 
          jar_path.to_string_lossy()
        ).replace("\\", "\\\\");
        fs::write(config_file, data);

        let mut exe_path = PathBuf::from(path.clone()).join("Mindustry.exe");
        if !exe_path.exists() {
            exe_path = PathBuf::from(path).join("Mindustry");
        }
        let mut child = match std::process::Command::new(exe_path)
            .env("MINDUSTRY_DATA_DIR", game_path.into_os_string().into_string().unwrap())
            .spawn() {
                Ok(child) => child,
                Err(err) => {
                    window.emit("err", "Failed to launch process").expect("Failed to emit");
                    return;
                }
            };
        window.emit("start", profile_id).expect("Failed to emit");

        if settings.minimize_on_launch.is_none() || settings.minimize_on_launch.unwrap() {
            window.hide().unwrap();
        }

        let exit_code = child.wait().expect("Failed to wait on child");

        window.show().unwrap();
        set_profile_internal(profile_id, &window);

        window.emit("stop", ExitData {
            exit_code: exit_code.code().expect("Failed to get exit code"),
            profile_id: profile_id
        }).expect("Failed to emit");
    });
}

#[derive(Serialize, Deserialize, Clone)]
struct ExitData {
    exit_code: i32,
    profile_id: i16
}

#[derive(Serialize, Deserialize)]
struct VersionsResponse {
    url: String,
    assets_url: String,
    upload_url: String,
    html_url: String,
    id: i32,
    #[serde(skip)]
    author: String, // Not actually a string, but skipped anyways and not used
    node_id: String,
    tag_name: String,
    target_commitish: String,
    name: String,
    draft: bool,
    prerelease: bool,
    created_at: String,
    published_at: String,
    assets: Vec<AssetResponse>,
    tarball_url: String,
    zipball_url: String,
    body: String,
    #[serde(skip)]
    reactions: String // Not actually a string, but skipped anyways and not used
}

#[derive(Serialize, Deserialize, Clone)]
struct AssetResponse {
    url: String,
    id: i32,
    node_id: String,
    name: String,
    label: Option<String>,
    #[serde(skip)]
    uploader: String, // Not actually a string, but skipped anyways and not used
    content_type: String,
    state: String,
    size: i32,
    download_count: i32,
    created_at: String,
    updated_at: String,
    browser_download_url: String
}

#[derive(Serialize, Deserialize, Clone)]
struct Version {
    name: String,
    asset_name: String,
    asset_url: String,
    asset_size: i32
}

#[derive(Serialize, Deserialize)]
struct VersionCache {
    source: String,
    versions: Vec<Version>,
    timestamp: i64
}

#[tauri::command]
async fn get_source_versions(source: String) -> Vec<Version> {
    // Load cache data and return result, if it's not too old
    let cache_path = Settings::settings_dir().join("version_cache.json");
    let mut cache_data: Vec<VersionCache> = vec![];
    if cache_path.exists() {
        let cache_data_string = match fs::read_to_string(&cache_path) {
            Ok(res) => res,
            Err(err) => panic!("Failed to read cache file: {}", err)
        };
        cache_data = match serde_json::from_str(&cache_data_string) {
            Ok(res) => res,
            Err(err) => panic!("Failed to parse version cache: {}", err)
        };
        for cache in &cache_data {
            if cache.source == source {
                let time: DateTime<Utc> = DateTime::from_timestamp(cache.timestamp, 0).expect("Failed to parse timestamp");
                let current_time = Utc::now();
                if current_time.signed_duration_since(time).num_hours() < 3 {
                    return cache.versions.clone();
                }
            }
        }
    }

    // Fetch the version from the github api
    let url = "https://api.github.com/repos/".to_owned() + &source + "/releases?per_page=100";
    let client = reqwest::Client::new();
    let res = client.get(url).header(USER_AGENT, "minlaunch").send().await;
    let body = match res {
        Ok(res) => res.json::<Vec<VersionsResponse>>().await,
        Err(err) => panic!("Failed to get versions: {}", err)
    };
    let parsed = match body {
        Ok(res) => res,
        Err(err) => panic!("Failed to parse body: {}", err)
    };
    let mut versions = vec![];
    for version in parsed {
        let mut asset: Option<AssetResponse> = None;
        for version_asset in version.assets {
            if version_asset.name == "Mindustry.jar" ||version_asset.name.contains("Desktop") { asset = Some(version_asset) }
        }
        if asset.is_none() { continue; }
        let final_asset = asset.unwrap();
        versions.push(Version {
            name: version.name, 
            asset_name: final_asset.clone().name, 
            asset_url: final_asset.browser_download_url,
            asset_size: final_asset.size
        })
    }

    // Save the result to the cache
    let mut new_cache_data = vec![];
    for cache in cache_data {
        if cache.source != source {
            new_cache_data.push(cache);
        }
    }
    new_cache_data.push(VersionCache {
        source,
        versions: versions.clone(),
        timestamp: Utc::now().timestamp()
    });
    let cache_data_string = serde_json::to_string(&new_cache_data).expect("Failed to convert new cache data to json");
    fs::write(cache_path, cache_data_string).expect("Failed to write cache file");

    return versions;
}

#[tauri::command]
async fn install_version_jar(profile_id: i16, url: String, size: i32, window: Window) {
    let profile = Profile::load_id(profile_id).expect("Failed to load profile");
    let client = reqwest::Client::new();
    let mut response = client.get(url).send().await.expect("get failed");
    let mut file = File::create(profile.get_folder().join("desktop.jar")).expect("create file failed");
    let mut downloaded_size = 0;

    while let Some(chunk) = response.chunk().await.expect("Failed to chunk") {
        file.write_all(&chunk).expect("Failed to write to file");
        downloaded_size += chunk.len() as u64;
        let progress = (downloaded_size as f64 / size as f64) * 100.0;
        window.emit("downloadProgress", progress.round());
    }
    window.emit("downloadDone", "");
}

fn main() {
    let launch = CustomMenuItem::new("launch".to_string(), "Launch");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(launch).add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_data, launch_game, set_profile, open_profile_folder, change_bool_setting, set_install_path, create_profile, update_profile, get_source_versions, install_version_jar, get_data_async])
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::DoubleClick { position: _, size: _, .. } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
              match id.as_str() {
                "launch" => {
                    launch_game(app.get_window("main").unwrap());
                }
                "quit" => {
                  std::process::exit(0);
                }
                _ => {}
              }
            }
            _ => {}
          })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                let settings = Settings::load();
                if settings.minimize_on_close.is_none() || settings.minimize_on_close.unwrap() {
                    event.window().hide().unwrap();
                    api.prevent_close();
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
