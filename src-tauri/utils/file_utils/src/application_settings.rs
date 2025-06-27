use serde::{Deserialize, Serialize};
use tauri::App;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct AppSettings {
    pub MaxDeletionSize: usize,
}

impl AppSettings{
    pub fn default() -> Self{
        Self{
            MaxDeletionSize: 53_687_091_200 //5GB
        }
    }
}

fn get_settings_location() -> io::Result<PathBuf> {
    let exe_path = std::env::current_exe()?;
    match exe_path.parent() {
        Some(dir) => Ok(dir.join("settings.svaultwyr")),
        None => Err(io::Error::new(io::ErrorKind::Other, "Could not determine executable directory")),
    }
}




pub fn update_settings(settings: &AppSettings) -> io::Result<()> {

    let settings_location = get_settings_location()?;
    let mut file = File::create(settings_location)?;
    let json = serde_json::to_string_pretty(settings)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn get_settings() -> io::Result<AppSettings> {
    let settings_location = get_settings_location()?;
    
    let file = match File::open(&settings_location) {
        Ok(k) => {k},
        Err(_) => {let f = File::create(settings_location)?; //create the settings path if it doesnt exist before returning the default
        let default_settings = AppSettings::default();    
        update_settings(&default_settings)?;
        return Ok(default_settings);
        
        
        }, 
    };
    let reader = BufReader::new(file);
    let settings = serde_json::from_reader(reader)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(settings)
}
