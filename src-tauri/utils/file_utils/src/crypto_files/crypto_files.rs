//!This module is mainly used for writing either from .fvaultwyr file or to the vaultwyr file (encrypting and decrypting)




use core::panic;

use std::fs;

use std::io::{self, BufReader, Read, Write};
use std::fs::{remove_file, File, OpenOptions};
use std::path:: PathBuf;
use encryption_utils::{aes_decrypt_with_key, aes_encrypt_with_key, password_to_key32};
use crate::calculate_file_hash;
use crate::crypto_files::parser::*;
use crate::file_traversal::RecursiveDirIter;
use crate::calculate_dir_size;

///Used for getting the file chunks from some file and decryption
pub struct FolderFile{
    pub original_path: PathBuf,
    pub file_hash: String,
    pub data: FileChunkIterator,
}




impl FolderFile{

    pub fn new(original_path:PathBuf,file_hash:String, data:FileChunkIterator) -> Self{
        FolderFile { original_path, file_hash , data, }
    }



fn create_original_file(&self) -> io::Result<File> {
    let parent_dir = self.original_path.parent().unwrap();
    
    // Create missing directories (if any)
    if !parent_dir.exists() {
        std::fs::create_dir_all(parent_dir)?;
    }

    // Check if file already exists
    if self.original_path.exists() {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "The file path to the original file already exists"));
    }

    
    
    // Attempt to create and open the file
    let opened_file = OpenOptions::new().create(true).write(true).open(&self.original_path)?;

    Ok(opened_file)
}


    pub fn try_restore_with_password(&mut self, password:&str) -> io::Result<()>{
        let key = password_to_key32(password)?;

        let mut  original_file = self.create_original_file()?;
        for chunk in &mut self.data{
            
            let decrypted_chunk = aes_decrypt_with_key(key, &chunk).map_err(|_| io::Error::new(io::ErrorKind::Other, "error decrypting chunk"))?;
            original_file.write_all(&decrypted_chunk)?;
        }
        Ok(())

            
        }
       

    }



///used to represent a .fvaultwyr file
pub struct VaultWyrFolder{
    pub new_path:PathBuf,
	pub algo : String,
	pub chunk_size: usize,

	pub files: VaultwyrFileLinker
}




impl VaultWyrFolder{


    pub fn new(new_path: PathBuf,algo: String,chunk_size:usize,files: VaultwyrFileLinker) -> Self{
        VaultWyrFolder { new_path, algo , chunk_size, files}
    }

    


pub fn decrypt_all_files(&mut self, password: &str) -> io::Result<()> {
    
    for file in &mut self.files {
        
        let vaultwyr_folder_file_reader = BufReader::new(OpenOptions::new().read(true).open(&self.new_path)?);
        
        let mut header = match file.parse_file_header(vaultwyr_folder_file_reader) {
            Some(h) => h,
            None => panic!("Unexpected main header"),
        };
        
        header.try_restore_with_password(password)?;
        
    }
    remove_file(&self.new_path)?;
    Ok(())
}

}

///This struct is used to represent a regular folder containing files
///You can use it to create a fvaultwyr file
pub struct Folder {
    pub new_path: PathBuf,
    pub vaultwyr_file: File,
    pub algo: Option<String>,
    pub chunk_size: Option<usize>,
    pub files: RecursiveDirIter,
    pub max_size: usize
}

impl Folder {
    fn create_vaultwyr_file(path: PathBuf) -> io::Result<File> {
        
        OpenOptions::new().create_new(true).write(true).read(true).open(path)
    }

    pub fn new(mut path: PathBuf) -> io::Result<Self> {
        if !path.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "The provided path is not a directory",
            ));
        }

        let files = RecursiveDirIter::new(&path)?;
        path.set_extension("fvaultwyr");
        let vaultwyr_file = Self::create_vaultwyr_file(path.clone())?;

        Ok(Self {
            new_path: path,
            vaultwyr_file,
            algo: None,
            chunk_size: None,
            files,
            max_size: 53_687_091_200 //50 GB default max size
        })
    }

    fn write_header(&mut self) -> io::Result<()> {
        let new_path = self
            .new_path
            .to_str()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid path"))?;

        let algo = self.algo.as_deref().unwrap_or("aes256");
        let chunk_size = self.chunk_size.unwrap_or(2048).to_string();

        let buffer = format!("{}\n{}\n{}", new_path, algo, chunk_size);
        let final_buffer = format!("m {} {}", buffer.len(), buffer);

        self.vaultwyr_file.write_all(final_buffer.as_bytes())?;
        Ok(())
    }

    fn write_files(&mut self, password: &str) -> io::Result<()> {
        let key = password_to_key32(password)?;

        for file_result in &mut self.files {
            let file_entry = match file_result {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let file_path = file_entry.path();

            // Skip if it is not a regular file
            if !file_path.is_file() {
                continue;
            }

            let original_path = file_path
                .to_str()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid file path"))?;

            
            let file_hash = calculate_file_hash(&file_path)?;

            // Header for the file
            let mut header = format!("{}\n{}", original_path, file_hash);
            header = format!("h {} {}", header.len(), header);
            self.vaultwyr_file.write_all(header.as_bytes())?;

            // Read and write chunks
            let mut file = File::open(&file_path)?;
            let mut buffer = vec![0; self.chunk_size.unwrap_or(2048)];

            loop {
                let bytes_read = file.read(&mut buffer)?;
                if bytes_read == 0 {
                    break;
                }

                let mut chunk = buffer[..bytes_read].to_vec();

                match self.algo.as_deref().unwrap_or("aes256") {
                    "aes256" => {
                        chunk = aes_encrypt_with_key(key, &chunk).map_err(|_| io::Error::new(io::ErrorKind::Other,"Error encrypting file"))?;
                    }
                    _ => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Unsupported algorithm",
                        ));
                    }
                }

                let chunk_len = chunk.len().to_string();
                self.vaultwyr_file.write_all(b"c ")?;
                self.vaultwyr_file.write_all(chunk_len.as_bytes())?;
                self.vaultwyr_file.write_all(b" ")?;
                self.vaultwyr_file.write_all(&chunk)?;
            }
        }

        Ok(())
    }
    fn clear_self(self) -> io::Result<()>{
        let mut path = self.new_path;
        path.set_extension("");
        if calculate_dir_size(&path)? > self.max_size as u64{
            return Err(io::Error::new(io::ErrorKind::FileTooLarge , "The folder is too big to delete please update the max size"))
        }
        
        fs::remove_dir_all(path)
        
    }
    ///Used to encrypt all the contents into the file when the file is encrypted the folder gets consumed since it shouldn't exist anymore
    pub fn encrypt_to_file(mut self, password: &str) -> io::Result<()> {
        self.write_header()?;
        self.write_files(password)?;
        self.clear_self()?;
        Ok(())
    }
}
