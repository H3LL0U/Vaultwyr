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
    console.error("Invoke error:", err);
    throw err;
  }
}



export default {
  encryptWithPassword,
  decryptWithPassword,
  pathExists,
  getAppArgs
};

