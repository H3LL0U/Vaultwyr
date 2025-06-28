import { invoke } from "@tauri-apps/api/core";

async function encryptWithPassword(password: string, file_path: string) {
    try {
      const result = await invoke("encrypt_path_with_password_api", { password, path: file_path })
      
      return result;
    } catch (err) {
      console.error("Invoke error:", err);
      throw err;
    }
  }

async function decryptWithPassword(password: string, file_path: string) {

      try {
      const result = await invoke("decrypt_path_with_password_api", { password, path: file_path })
      return result;
    } catch (err) {
      console.error("Invoke error:", err);
      throw err;
    }
}

async function pathExists(path:string) {
      try {
      const result = await invoke("path_exists", {path})
      return result;
    } catch (err) {
      console.error("Invoke error:", err);
      throw err;
    }
}

async function getAppArgs(): Promise<string[]> {
  try {
    const result = await invoke<string[]>("get_app_args");
    
    return result;
  } catch (err) {
    alert(err)
    console.error("Invoke error:", err);
    throw err;
  }
}
//all of the settings parameters should be stored here!

export interface AppSettings {
  MaxDeletionSize: number;
  RestoreToOriginalFolder: boolean;
}

async function applySettings(settings: AppSettings): Promise<string> {
  try {
    let new_settings: AppSettings = { ...settings }; //taking a copy of the 
    new_settings.MaxDeletionSize = Math.floor(new_settings.MaxDeletionSize*1_073_741_824) //convert to bytes
    
    const result = await invoke("apply_settings", { settings: new_settings });
    alert("Settings successfully applied!")
    return result as string
  } catch (err) {
    alert("Error applying settings" + err)
    console.error("Invoke error:", err);
    throw err;
  }
}

async function getSettings(): Promise<AppSettings> {
  try {
    const result = await invoke("get_settings") as AppSettings;
    result.MaxDeletionSize = Math.floor(result.MaxDeletionSize/ 1_073_741_824) //convert to gigabytes
    return result
  } catch (err) {
    alert(err)
    console.error("Invoke error:", err);
    throw err;
  }
}




export default {
  encryptWithPassword,
  decryptWithPassword,
  pathExists,
  getAppArgs,
  applySettings,
  getSettings,
};

