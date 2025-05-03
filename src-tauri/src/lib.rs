use file_utils::EncryptionOptions;
use std::path::PathBuf;

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
        Err(e) => return format!("Error locating the file\n{}", e),
    };

    match selected_file.lock_file_with_password(password) {
        Ok(_) => return "File has been encrypted with your selected password".to_string(),
        Err(e) => return format!("There was an error when encrypting your file:\n{}", e),
    };

    
}

#[tauri::command]
fn decrypt_file_with_password_api(path: &str, password: &str) -> String {
    let normalized_path = path.replace("\\", "/");
    let path_buf = PathBuf::from(&normalized_path);

    let mut selected_file = match  EncryptionOptions::from_file(path_buf) {
        Ok(t) => t,
        Err(e) => return format!("Error locating the file\n{}", e),
        
    };

    match selected_file.unlock_file_with_password(password) {
        Ok(_) => return "File has been decrypted successfully".to_string(),
        Err(e) => return format!("There was an error when decrypting your file:\n{}", e),
    };
    


}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            encrypt_file_with_password_api,
            decrypt_file_with_password_api
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
