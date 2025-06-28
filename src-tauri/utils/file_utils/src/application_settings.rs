use serde::{Deserialize, Serialize};
use tauri::App;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::PathBuf;
use directories::ProjectDirs;
use std::{default, fs};
#[derive(Serialize, Deserialize)]
pub struct AppSettings {
    pub MaxDeletionSize: usize,
    pub RestoreToOriginalFolder: bool
}

impl AppSettings{
    pub fn default() -> Self{
        Self{
            MaxDeletionSize: 53_687_091_200, //50GB
            RestoreToOriginalFolder: true
        }
    }
}

fn get_settings_location() -> Option<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "H3LL_0U", "Vaultwyr") {
        let config_dir = proj_dirs.config_dir();

        // Ensure the config directory exists
        if let Err(e) = fs::create_dir_all(config_dir) {
            eprintln!("Failed to create config directory: {}", e);
            return None;
        }
        
        Some(config_dir.join("settings.svaultwyr"))
    } else {
        None
    }
}




pub fn update_settings(settings: &AppSettings) -> io::Result<()> {

    let settings_location = match get_settings_location() {
        Some(k) => {k},
        None => {return Err(io::Error::new(io::ErrorKind::NotADirectory, "Could not locate the settings location"));},
    };
    let mut file = File::create(settings_location)?;
    let json = serde_json::to_string_pretty(settings)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

//tries to get the settings if it cant find them creates them in their default configuration
pub fn get_settings() -> io::Result<AppSettings> {
    let settings_location = match get_settings_location() {
        Some(k) => {k},
        None => {return Err(io::Error::new(io::ErrorKind::NotADirectory, "Could not locate the settings location"));},
    };
    
    let file = match File::open(&settings_location) {
        Ok(k) => {k},
        Err(_) => {let _ = File::create(settings_location)?; //create the settings path if it doesnt exist before returning the default
        let default_settings = AppSettings::default();    
        update_settings(&default_settings)?;
        return Ok(default_settings);
        
        
        }, 
    };
    let reader = BufReader::new(file);
    let settings: AppSettings = match serde_json::from_reader(reader) {
        Ok(k) => {k},
        Err(_) => {
            let default_settings = AppSettings::default();
            update_settings(&default_settings)?;
            default_settings
            
        },
    };
    Ok(settings)
}
