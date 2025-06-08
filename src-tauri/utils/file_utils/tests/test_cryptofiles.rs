#[cfg(test)]
mod tests{
    
    use std::{fs::File, io::{ Read}, path::PathBuf, str::FromStr};
    use encryption_utils::{aes_decrypt_with_key, password_to_key};
    use file_utils::{behaviour, parser::*};
    use file_utils::crypto_files::crypto_files::{*};
    use std::fs::{self};

    use std::env;
    use std::io::Write;

    use serial_test::serial;
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
    #[serial]
    fn test_encryption_and_decryption() {
        //running multipple times just in case
        for _ in 1..9{
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
        let mut folder = EncryptionPath::new(temp_dir.clone()).unwrap().on_error_behaviour(behaviour::OnErrorBehaviour::TerminateOnError);
        folder.algo = Some("aes256".to_string());
        folder.chunk_size = Some(1024);

        // Encrypt the files and write to vault file
        match folder.encrypt_to_file("password") {
            Some(e) => {panic!("{:?}",e)},
            None => {},
        };
    
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
    }
        
        fn test_decryption () {
        let path = match PathBuf::from_str("./tempg.fvaultwyr") {
            Ok(p) => {p},
            Err(_) => {panic!("error constructing path")},
        };

        
        //let _reader = BufReader::new(File::open(&path).unwrap());

        let mut folder = VaultWyrFileParser::from_path(&path).unwrap().to_folder();

        folder.decrypt_all_files("password").unwrap();

        clean_up_test_dir(&path);
    }

    #[test]
    #[serial]
    fn test_one_file_encryption() {
        //running multipple times just in case
        for _ in 1..9{
        let temp_dir = create_temp_dir();

        // Create a test files
        let test_file_path = temp_dir.join("test.txt");
        let mut file = File::create(&test_file_path).unwrap();
        let long_string = "This is a test file to be encrypted.".to_string().repeat(1000); // Repeat the string
        writeln!(file, "{}", long_string).unwrap(); // Write the repeated string to the file

        let file_to_encrypt = EncryptionPath::new(test_file_path).unwrap().on_error_behaviour(behaviour::OnErrorBehaviour::TerminateOnError);

        match file_to_encrypt.encrypt_to_file("123") {
            Some(e) => {panic!("{:?}",e)},
            None => {},
        };

        let new_path = temp_dir.join("test.vaultwyr");
        
        let encrypted_file = VaultWyrFileParser::from_path(&new_path).unwrap();

        let mut folder = encrypted_file.to_folder();
        folder.decrypt_all_files("123").unwrap();
        clean_up_test_dir(&temp_dir);
        }
        ()

        

        




        
    }
    #[test]
    fn test_decrypt_validation(){
        let validation: Vec<u8> = vec![
    169,
    201,
    204,
    207,
    41,
    226,
    49,
    143,
    139,
    21,
    53,
    122,
    120,
    91,
    243,
    211,
    91,
    247,
    179,
    110,
    139,
    95,
    234,
    146,
    115,
    167,
    81,
    194,
    244,
    89,
    186,
    32,
    88,
    203,
    37,
    158,
    25,
    216,
    98,
    185,
    205,
    101,
    178,
    29,
    24,
    90,
    60,
    153,
    239,
    208,
    208,
    168,
    227,
    118,
    255,
    7,
    200,
    223,
    108,
    117,
];
    let key = password_to_key("password").unwrap();
    let a = aes_decrypt_with_key(&key, &validation).unwrap();
    dbg!(&a);
    }

}