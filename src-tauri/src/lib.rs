use file_utils::{crypto_files::crypto_files::*, parser::VaultWyrFileParser};
use std::{path::{Path, PathBuf}, str::FromStr};
use tauri::AppHandle;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn encrypt_path_with_password_api(handle: AppHandle, path: &str, password: &str) -> String {
    let path = PathBuf::from_str(path).unwrap();
    let path_to_encrypt = EncryptionPath::new(path).unwrap();

    match path_to_encrypt.encrypt_to_file(password) {
        None => {"File encrypted Successfully".to_string()},
        Some(_) => {"Error encrypting file".to_string()},
    }

    
}

#[tauri::command]
fn decrypt_path_with_password_api(handle: AppHandle, path: &str, password: &str) -> String {




    let path = PathBuf::from_str(path).unwrap();


    let mut encrypted_file = VaultWyrFileParser::from_path(&path).unwrap().to_folder();




    match encrypted_file.decrypt_all_files(password) {
        Ok(_) => {"decrypted file successfully".to_string()},
        Err(_) => {"Wrong password".to_string()},
    }



}
#[tauri::command]
fn path_exists(path:String) -> bool{
    let path = PathBuf::from(path);
    path.exists()
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
            path_exists
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
