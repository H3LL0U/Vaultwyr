#[cfg(test)]
mod tests{
    
    use std::{fs::File, io::{ BufReader, Read}, path::PathBuf, str::FromStr};
    use file_utils::Parser::*;
    use file_utils::crypto_files::crypto_files::{*};
    use std::fs::{self};

    use std::env;
    use std::io::Write;

fn create_temp_dir() -> PathBuf {
        let current_dir = env::current_dir().unwrap();
        let temp_dir = current_dir.join(".\\tempg");
        
        // Create the temp directory if it doesn't exist
        if !temp_dir.exists() {
            fs::create_dir(&temp_dir).unwrap();
        }
        
        temp_dir
    }

    // Helper function to clean up after the tests
    fn clean_up_test_dir(path: &PathBuf) {
        if path.exists() {
            fs::remove_dir_all(path).unwrap();
        }
    }





    #[test]
    fn test_encryption_and_decryption() {
        let temp_dir = create_temp_dir();

        // Create a test files
        let test_file_path = temp_dir.join("test.txt");
        let mut file = File::create(&test_file_path).unwrap();
        //Create second file
        let test_file_path = temp_dir.join("test1.txt");
        let mut file1 = File::create(test_file_path).unwrap();
        //create third file in the deeper location
        let test_file_path = temp_dir.join("./test/test1.txt");
            //create the structure
            if let Some(parent) = test_file_path.parent() {
            fs::create_dir_all(parent).unwrap();
            }
        let mut file2 = File::create(test_file_path).unwrap();


        let long_string = "This is a test file to be encrypted.".to_string().repeat(1000); // Repeat the string
        writeln!(file, "{}", long_string).unwrap(); // Write the repeated string to the file


        writeln!(file1 ,"this is a test for a smaller file on top of the other file").unwrap();


        write!(file2, "this file is stored deeper in the folder").unwrap();
        // Create folder instance
        let mut folder = Folder::new(temp_dir.clone()).unwrap();
        folder.algo = Some("aes256".to_string());
        folder.chunk_size = Some(1024);

        // Encrypt the files and write to vault file
        folder.encrypt_to_file("password").unwrap();
    
        // Check if the vault file has been created and encrypted
        let vault_file_path = temp_dir.with_extension("fvaultwyr");
        assert!(vault_file_path.exists());

        // You can further check the contents of the vault file to confirm encryption
        let mut vault_file = File::open(vault_file_path).unwrap();
        let mut contents = Vec::new();
        vault_file.read_to_end(&mut contents).unwrap();

        // Check that there is some content written in the vault file
        assert!(contents.len() > 0);

        

        
        test_decryption();
        clean_up_test_dir(&temp_dir);

    }

        
        fn test_decryption () {
        let path = match PathBuf::from_str("./tempg.fvaultwyr") {
            Ok(p) => {p},
            Err(_) => {panic!("error constructing path")},
        };

        
        let reader = BufReader::new(File::open(&path).unwrap());

        let mut  folder= VaultWyrFileParser::new(VaultwyrFileLinker::from_vaultwyr_file(path).unwrap(), reader).to_folder();

        folder.decrypt_all_files("password").unwrap();

        
    }

}