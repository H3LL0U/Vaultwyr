

use file_utils::{behaviour::VaultwyrError, crypto_files::crypto_files::*, parser::VaultWyrFileParser};
use std::{path::{Path, PathBuf}, str::FromStr};
use tauri::{AppHandle, Emitter};
use std::env;
use dialog_lib;
use file_utils::application_settings;
use crate::application_settings::AppSettings;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn encrypt_path_with_password_api(handle: AppHandle, path: &str, password: &str) -> String {
    

    let settings = match application_settings::get_settings() {
        Ok(k) => {k},
        Err(_) => {return "Error locating your settings file".to_string()},
    };

    let path = PathBuf::from_str(path).unwrap();
    let path_to_encrypt = match EncryptionPath::new(path,None) {
        Ok(k) => {k},
        Err(_) => { return "Encryption Cancelled".to_string();},
    }.max_size(settings.MaxDeletionSize);

    match path_to_encrypt.encrypt_to_file(password) {
        None => {"File encrypted Successfully".to_string()},
        Some(_) => {"Error encrypting file".to_string()},
    }
}

#[tauri::command]
fn decrypt_path_with_password_api(path: &str, password: &str) -> String {
    let settings = match application_settings::get_settings() {
        Ok(k) => {k},
        Err(_) => {return "Error locating your settings file".to_string()},
    };




    let path = PathBuf::from_str(path).unwrap();
    let encrypted_file = VaultWyrFileParser::from_path(&path).unwrap().to_folder().restore_into_original_folder(settings.RestoreToOriginalFolder);
    
    match encrypted_file.decrypt_all_files(password) {
        None=> {"decrypted file successfully".to_string()},
        Some(_)=> {"Wrong password".to_string()},
    }
}
#[tauri::command]
fn path_exists(path:String) -> bool{
    let path = PathBuf::from(path);
    path.exists()
}

#[tauri::command]
fn get_app_args() -> Vec<String>{
    env::args().collect()
}

#[tauri::command]
fn get_settings() -> application_settings::AppSettings{
    match application_settings::get_settings() {
        Ok(k) => {k},
        Err(_) => {
            return AppSettings::default();
        },
    }
}
#[tauri::command]
fn apply_settings(settings: AppSettings) -> String{
    match application_settings::update_settings(&settings) {
        Ok(k) => {return "Your settings were updated Sucessfully".to_string();},
        Err(_) => {return "Something went wrong when updating the settings. Restart the application and try again".to_string();},
    }
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            encrypt_path_with_password_api,
            decrypt_path_with_password_api,
            path_exists,
            get_app_args,
            get_settings,
            apply_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
