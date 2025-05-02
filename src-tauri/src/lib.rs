use file_utils::EncryptionOptions;
use std::path::{self, PathBuf};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn encrypt_file_with_password_api(path: &str, password: &str) -> String {
    let normalized_path = path.replace("\\", "/");
    let path_buf = PathBuf::from(&normalized_path);

    let mut selected_file = match EncryptionOptions::new(path_buf, None, None) {
        Ok(t) => t,
        Err(_) => return "Error locating a file".to_string(),
    };

    match selected_file.lock_file_with_password(password) {
        Ok(_) => return "File has been encrypted with your selected password".to_string(),
        Err(_) => format!("There was an error when encrypting your file:\n"),
    };

    "Successfully ecrypted a file".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            encrypt_file_with_password_api
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
