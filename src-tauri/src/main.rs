// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, path::{Path, PathBuf}, thread};

use serde::{Deserialize, Serialize};
use tauri::{Window, CustomMenuItem, SystemTrayMenu, SystemTray, SystemTrayEvent, Manager};

#[derive(Serialize, Deserialize, Clone)]
struct Settings {
    install_path: Option<String>,
    current_profile: i16,
    download_sources: Vec<String>,
    minimize_on_launch: Option<bool>,
    minimize_on_close: Option<bool>
}

impl Settings {
    fn save(&self) {
        let data = toml::to_string(&self).unwrap();
        match std::fs::write(Self::settings_path(), data) {
            Ok(()) => {}
            Err(err) => {
                panic!("Failed to save settings: {}", err)
            }
        }
    }

    fn load() -> Self {
        let path = Self::settings_path();
        if !path.exists() {
            let folder = Self::settings_dir();
            if !folder.exists() {
                match fs::create_dir(folder) {
                    Ok(()) => {}
                    Err(err) => panic!("Failed to create config dir: {}", err)
                }
            }
            Settings::default().save();
        }
        let data = std::fs::read_to_string(path).expect("No settings file");
    
        match toml::from_str(&data) {
            Ok(res) => return res,
            Err(err) => panic!("Failed to read settings")
        }
    }

    fn settings_path() -> PathBuf {
        return Self::settings_dir().join(PathBuf::from("settings.toml"))
    }

    fn settings_dir() -> PathBuf {
        return dirs::data_dir().expect("Failed to get config dir").join(PathBuf::from("minlaunch"))
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            install_path: None,
            current_profile: 0,
            download_sources: vec!["Anuken/Mindustry".to_string(), "Anuken/MindustryBuilds".to_string()],
            minimize_on_launch: Some(true),            
            minimize_on_close: Some(true)           
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Profile {
    id: i16,
    name: String
}

#[derive(Debug)]
enum ProfileLoadError {
    NoDataFile,
    InvalidDataFile
}

impl Profile {
    fn load(profile_folder: PathBuf) -> Result<Self, ProfileLoadError> {
        let profiles_data_path = profile_folder.join("profile.toml");
        if !profiles_data_path.exists() { 
            return Err(ProfileLoadError::NoDataFile)
        }
        let data = std::fs::read_to_string(profiles_data_path).expect("Failed to read profile file");

        match toml::from_str(&data) {
            Ok(res) => Ok(res),
            Err(err) => Err(ProfileLoadError::InvalidDataFile)
        }
    }

    fn save(&self) {
        let profile_path = Settings::settings_dir().join("profiles").join(PathBuf::from(self.id.to_string()));
        let profiles_data_path = profile_path.join("profile.toml");
        let data = toml::to_string(&self).unwrap();
        match std::fs::write(profiles_data_path, data) {
            Ok(()) => {}
            Err(err) => {
                panic!("Failed to save profile: {}", err)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Data {
    current_profile: i16,
    profiles: Vec<Profile>,
    install_path: Option<String>,
    settings: Settings
}

#[tauri::command]
fn create_profile(window: Window) {
    let profiles_path = Settings::settings_dir().join("profiles");
    let mut highest_profile_id = -1;
    
    if !profiles_path.exists() {
        match fs::create_dir(&profiles_path) {
            Ok(()) => {}
            Err(err) => panic!("Failed to create profiles dir: {}", err)
        }
    }

    for entry in fs::read_dir(profiles_path.clone()).expect("Failed to iter profiles") {
        let path = entry.expect("").path();
        if path.is_dir() {
            let path_string = path.file_name().unwrap().to_os_string().into_string().expect("Failed to convert path");
            let path_num: i16;
            println!("{}", path_string);
            match path_string.parse() {
                Ok(res) => path_num = res,
                Err(e) => continue
            }
            if path_num > highest_profile_id {
                highest_profile_id = path_num;
            }
        }
    }

    let profile = Profile {
        id: highest_profile_id + 1,
        name: "New Profile".into()
    };

    println!("{}", highest_profile_id);
    let profile_path = profiles_path.join(PathBuf::from(profile.id.to_string()));
    fs::create_dir(&profile_path);
    profile.save();
    window.emit("set_data", load_data()).expect("Failed to emit");
}

#[tauri::command]
fn change_bool_setting(setting: &str, value: bool, window: Window) {
    let mut settings = Settings::load();
    match setting {
        "minimize_on_launch" => settings.minimize_on_launch = Some(value),
        "minimize_on_close" => settings.minimize_on_close = Some(value),
        _ => window.emit("err", "bool setting not found").expect("failed to emit")
    };
    settings.save();
}

#[tauri::command]
fn set_install_path(path: String, window: Window) {
    let mut settings = Settings::load();
    settings.install_path = Some(path);
    settings.save();
    window.emit("set_data", load_data()).expect("Failed to emit");
}

#[tauri::command]
fn get_data() -> Data {
    return load_data();
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
            let res = Profile::load(path).expect("Failed to load profile");
            profiles.push(res);
        }
    }

    Data {
        current_profile: state.current_profile,
        profiles,
        install_path: state.install_path.clone(),
        settings: state
    }
}

#[tauri::command]
fn open_profile_folder(id: i16) {
    let res = open::that(Settings::settings_dir().join("profiles").join(PathBuf::from(id.to_string())));
    match res {
        Ok(()) => {}
        Err(err) => println!("{}", err),
    };
}

#[tauri::command]
fn set_profile(id: i16, window: Window) {
    set_profile_internal(id, &window);
}

fn set_profile_internal(id: i16, window: &Window) {
    let mut settings = Settings::load();
    settings.current_profile = id;
    settings.save();
    window.emit("set_data", load_data()).expect("Failed to emit");
}

#[tauri::command]
fn launch_game(window: Window) {
    let settings = Settings::load();
    
    thread::spawn(move || {
        let path;
        if settings.install_path.is_some() {
            path = settings.install_path.clone().unwrap();
            println!("{}", path);
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
          jar_path.to_string_lossy()).replace("\\", "\\\\"
        );
        fs::write(config_file, data);

        let exe_path = PathBuf::from(path).join("Mindustry.exe");
        let mut child = std::process::Command::new(exe_path)
            .env("MINDUSTRY_DATA_DIR", game_path.into_os_string().into_string().unwrap())
            .spawn()
            .expect("Failed to launch process");
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

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_data, launch_game, set_profile, open_profile_folder, change_bool_setting, set_install_path, create_profile])
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::DoubleClick { position: _, size: _, .. } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
              match id.as_str() {
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
