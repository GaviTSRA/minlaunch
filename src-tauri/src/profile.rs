use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use tauri::Window;

use crate::{load_data, settings::Settings};

#[derive(Serialize, Deserialize, Clone)]
pub struct Profile {
    settings: ProfileSettings,
    has_jar: bool
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProfileSettings {
    id: i16,
    name: String
}

#[derive(Debug)]
pub enum ProfileLoadError {
    NoDataFile,
    InvalidDataFile
}

impl Profile {
    pub fn load_folder(profile_folder: PathBuf) -> Result<Self, ProfileLoadError> {
        let profiles_data_path = profile_folder.join("profile.toml");
        if !profiles_data_path.exists() { 
            return Err(ProfileLoadError::NoDataFile)
        }
        let data = std::fs::read_to_string(profiles_data_path).expect("Failed to read profile file");

        let settings: ProfileSettings = match toml::from_str(&data) {
            Ok(res) => res,
            Err(err) => return Err(ProfileLoadError::InvalidDataFile)
        };
        return Ok(Profile {
            settings,
            has_jar: profile_folder.join("desktop.jar").exists()
        })
    }

    pub fn load_id(id: i16) -> Result<Self, ProfileLoadError> {
        let profile_folder = Settings::settings_dir().join("profiles").join(id.to_string());
        Profile::load_folder(profile_folder)
    }

    pub fn save(&self) {
        let profile_path = Settings::settings_dir().join("profiles").join(PathBuf::from(self.settings.id.to_string()));
        let profiles_data_path = profile_path.join("profile.toml");
        let data = toml::to_string(&self.settings).unwrap();
        match std::fs::write(profiles_data_path, data) {
            Ok(()) => {}
            Err(err) => {
                panic!("Failed to save profile: {}", err)
            }
        }
    }
}

#[tauri::command]
pub fn create_profile(window: Window) {
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
        settings: ProfileSettings {
            id: highest_profile_id + 1,
            name: "New Profile".into()
        },
        has_jar: false
    };

    let profile_path = profiles_path.join(PathBuf::from(profile.settings.id.to_string()));
    fs::create_dir(&profile_path);
    profile.save();
    window.emit("set_data", load_data()).expect("Failed to emit");
}

#[tauri::command]
pub fn open_profile_folder(id: i16) {
    let res = open::that(Settings::settings_dir().join("profiles").join(PathBuf::from(id.to_string())));
    match res {
        Ok(()) => {}
        Err(err) => println!("{}", err),
    };
}

#[tauri::command]
pub fn update_profile(id: i16, name: Option<String>) {
    let mut profile = Profile::load_id(id).expect("failed to load profile");
    if name.is_some() {
        profile.settings.name = name.unwrap();
    }
    profile.save();
}

#[tauri::command]
pub fn set_profile(id: i16, window: Window) {
    set_profile_internal(id, &window);
}

pub fn set_profile_internal(id: i16, window: &Window) {
    let mut settings = Settings::load();
    settings.current_profile = id;
    settings.save();
    window.emit("set_data", load_data()).expect("Failed to emit");
}