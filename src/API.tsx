import { invoke } from "@tauri-apps/api/core";

async function encryptWithPassword(password: string, file_path: string) {
    try {
      const result = await invoke("encrypt_file_with_password_api", { password, path: file_path })
      console.log("Success:", result);
      return result;
    } catch (err) {
      console.error("Invoke error:", err);
      throw err;
    }
  }

export default encryptWithPassword
