use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use tauri::Window;

use crate::load_data;

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub install_path: Option<String>,
    pub current_profile: i16,
    pub download_sources: Vec<String>,
    pub minimize_on_launch: Option<bool>,
    pub minimize_on_close: Option<bool>
}

impl Settings {
    pub fn save(&self) {
        let data = toml::to_string(&self).unwrap();
        match std::fs::write(Self::settings_path(), data) {
            Ok(()) => {}
            Err(err) => {
                panic!("Failed to save settings: {}", err)
            }
        }
    }

    pub fn load() -> Self {
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

    pub fn settings_path() -> PathBuf {
        return Self::settings_dir().join(PathBuf::from("settings.toml"))
    }

    pub fn settings_dir() -> PathBuf {
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

#[tauri::command]
pub fn change_bool_setting(setting: &str, value: bool, window: Window) {
    let mut settings = Settings::load();
    match setting {
        "minimize_on_launch" => settings.minimize_on_launch = Some(value),
        "minimize_on_close" => settings.minimize_on_close = Some(value),
        _ => window.emit("err", "bool setting not found").expect("failed to emit")
    };
    settings.save();
}

#[tauri::command]
pub fn set_install_path(path: String, window: Window) {
    let mut settings = Settings::load();
    settings.install_path = Some(path);
    settings.save();
    window.emit("set_data", load_data()).expect("Failed to emit");
}