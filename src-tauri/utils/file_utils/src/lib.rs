mod CryptoFiles;

use serde::{Serialize, Deserialize};
use serde_json::value::Index;
use core::panic;
use std::u128;
use sha2::{Sha256, Digest};
use std::ffi::OsStr;
use std::io::{self, Write, Read, BufReader};
use std::fs::{remove_file, DirEntry, File, OpenOptions, ReadDir};
use std::path::{self, Path, PathBuf};
use encryption_utils::{aes_decrypt_with_key, aes_encrypt_with_key, password_to_key32};
use bincode::{self, Error};

use CryptoFiles::CryptoFiles::{*};

use crate::CryptoFiles::{*};
#[derive(Serialize,Deserialize)]
pub struct EncryptionOptions {
    //#[serde(skip_serializing)]
    pub new_path: PathBuf,
    pub original_path: PathBuf,
    pub chunk_size: i128,
    pub algo: String,
    pub validation: Vec<u8>,
    pub file_hash: String,
    pub data: Vec<u8>,
}


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

impl EncryptionOptions{

    pub fn from_file(path:PathBuf) ->  std::io::Result<EncryptionOptions> {
            
            let file = File::open(path)?;
            let decoded: EncryptionOptions = bincode::deserialize_from(file).unwrap();
            Ok(decoded)
        
    }

    pub fn new(original_path:PathBuf,chunk_size:Option<i128>,algo:Option<String>) -> io::Result<EncryptionOptions>{
        let chunk_size = chunk_size.unwrap_or(4096);
        
        let algo = algo.unwrap_or("aes256".to_string());
        
        let mut options = EncryptionOptions{
            original_path : original_path,
            new_path : PathBuf::from(""),
            chunk_size : chunk_size,
            algo : algo,
            validation : vec![0u8; 32],
            file_hash : "".to_string(),
            data : Vec::<u8>::new(),
            
        };
        options.init_values()?;
        Ok(options)
    }

    fn init_values (&mut self) -> io::Result<()>{
        let mut converted_path = self.original_path.clone();
        if converted_path.extension().is_none() {
            panic!("No file extension present");
        }
        self.file_hash = calculate_file_hash(&self.original_path)?;
        converted_path.set_extension("vaultwyr");
        self.new_path = converted_path;
        Ok(())
    }
    pub fn get_data_as_utf(&self) -> std::io::Result<String> {
        String::from_utf8(self.data.clone())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
    
    
    fn create_locked_file(&mut self) -> io::Result<()> {
            let mut opened_file = OpenOptions::new()
                .truncate(true)
                .write(true)
                .read(true)
                .create(true)
                .open(&self.new_path)?;
    
            // Serialize self (excluding new_path) using bincode
            let serialized =  bincode::serialize(&self)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    
            // Write binary data directly
            opened_file.write_all(&serialized)?;
    
            Ok(())
        }
    fn restore_file(&self) -> io::Result<()>{
        let mut opened_file = OpenOptions::new()
        .truncate(true)
        .write(true)
        .read(true)
        .create(true)
        .open(&self.original_path)?;
        
        
        opened_file.write_all(&self.data)?;
        Ok(())
    }
    pub fn lock_file_with_password(&mut self, password:&str) -> io::Result<()> {

        
        
        
        if self.validation != vec![0u8;32]{
            return Err(io::Error::new(io::ErrorKind::Other, "File already Encrypted"))
        }
        
        let extension: &OsStr = OsStr::new("vaultwyr");

        if self.new_path.extension().unwrap_or_default() != extension  {
            return Err(io::Error::new(io::ErrorKind::Other, "new path extension was not set"))
        }
        let mut original_file = File::open(&self.original_path)?;
        let mut original_file_contents = String::new();
        original_file.read_to_string(&mut original_file_contents)?;
        let key = password_to_key32(password)?;
        
        self.validation = match aes_encrypt_with_key(key, &self.validation){
            Ok(n) => n,
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Could not encrypt file contents"))
        };
        //encrypt main data
        self.data = match aes_encrypt_with_key(key, &original_file_contents.as_bytes()){
            Ok(n) => n,
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Could not encrypt file contents")),
            
        };
    
        self.create_locked_file()?;
        
        remove_file(&self.original_path)?;
        Ok(())
    }
    pub fn unlock_file_with_password(&mut self, password:&str) -> io::Result<()>{
        if self.validation == vec![0u8;32]{
            return Err(io::Error::new(io::ErrorKind::Other, "File already Decrypted"))
        }
        
        let extension: &OsStr = OsStr::new("vaultwyr");

        if self.new_path.extension().unwrap_or_default() != extension  {
            return Err(io::Error::new(io::ErrorKind::Other, "new path extension was not set"))
        }

        let key = password_to_key32(password)?;
        
        //decrypt validation vector
        self.validation = match aes_decrypt_with_key(key, &self.validation){
            Ok(n) => n,
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Could not encrypt validation contents"))
        };
        //decrypt main data
        self.data = match aes_decrypt_with_key(key, &self.data){
            Ok(n) => n,
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Could not encrypt file contents")),
            
        };
    
        self.restore_file()?;
        remove_file(&self.new_path)?;
        Ok(())
    }
    
}
    

pub fn calculate_file_hash<P: AsRef<Path>>(path: P) -> io::Result<String> {
    
    let mut file = File::open(path)?;

    
    let mut hasher = Sha256::new();

    // Buffer to read chunks of the file (e.g., 4 KB at a time)
    let mut buffer = [0; 4096]; 
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // End of file
        }
        // Update the hasher with the chunk of data
        hasher.update(&buffer[..bytes_read]);
    }

    
    let hash = hasher.finalize();
    Ok(format!("{:x}", hash)) 
}


//chunk iterator



