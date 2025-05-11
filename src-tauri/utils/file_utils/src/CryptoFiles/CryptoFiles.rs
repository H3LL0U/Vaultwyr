
use serde::{Serialize, Deserialize};
use serde_json::map::Iter;
use serde_json::value::Index;
use core::panic;
use std::f32::consts::E;
use std::str::FromStr;
use std::u128;
use sha2::{Sha256, Digest};
use std::ffi::OsStr;
use std::io::{self, BufReader, Read, Seek, Write};
use std::fs::{remove_file, DirEntry, File, OpenOptions, ReadDir};
use std::path::{self, Path, PathBuf};
use encryption_utils::{aes_decrypt_with_key, aes_encrypt_with_key, password_to_key32};
use bincode::{self, Error};
use crate::Parser;
use crate::calculate_file_hash;
use crate::CryptoFiles::Parser::*;
use ParserUtils::*;
use std::io::BufRead;

use super::Parser::*;





 /*pub struct FileChunkIterator<R: Read> {
    reader: BufReader<R> ,
    chunk_size: usize
    
    
}

impl<R: Read> FileChunkIterator<R> {
    fn new(reader: R, chunk_size: usize,) -> Self {
        Self {
            reader: BufReader::new(reader),
            chunk_size,
        }
    }
}

impl<R: Read> Iterator for FileChunkIterator<R> {
    type Item = io::Result<Vec<u8>>;
    //iterating through a real folder
    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = vec![0u8; self.chunk_size];
        match self.reader.read(&mut buffer) {
            Ok(0) => None, // EOF
            Ok(n) => {
                buffer.truncate(n);
                Some(Ok(buffer))
            }
            Err(e) => Some(Err(e)),
        }
    }
}
*/


//files








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

    dbg!(&self.original_path);
    
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

    /*pub fn decrypt_with_password(&self) -> io::Result<()>{
        match &self.data {
            DataSource::Parser(parser) => {
                self.create_original_file()?;
                for data in &self.data{
                    
                    Ok(())
                }
            },
            _ => { return Err(io::Error::new(io::ErrorKind:: Other, "This file was not built from the parser"));

            }
        }
    }
    */


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

    


fn decrypt_all_files(&mut self, password: &str) -> io::Result<()> {
    
    for file in &mut self.files {
        dbg!("file");
        let vaultwyr_folder_file_reader = BufReader::new(OpenOptions::new().read(true).open(&self.new_path)?);
        dbg!("file");
        let mut header = match file.parse_file_header(vaultwyr_folder_file_reader) {
            Some(h) => h,
            None => panic!("Unexpected main header"),
        };
        dbg!("file");
        header.try_restore_with_password(password)?;
        remove_file(&self.new_path)?;
    }
    
    Ok(())
}

}


struct Folder {
    new_path: PathBuf,
    vaultwyr_file: File,
    algo: Option<String>,
    chunk_size: Option<usize>,
    files: ReadDir,
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

        let files = path.read_dir()?;
        path.set_extension("fvaultwyr");
        let vaultwyr_file = Self::create_vaultwyr_file(path.clone())?;

        Ok(Self {
            new_path: path,
            vaultwyr_file,
            algo: None,
            chunk_size: None,
            files,
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

            // Generate file hash (for demonstration purposes, we'll just use the filename length)
            let file_hash = original_path.len().to_string();

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

    pub fn encrypt_to_file(&mut self, password: &str) -> io::Result<()> {
        self.write_header()?;
        self.write_files(password)?;
        Ok(())
    }
}










/// Custom iterator that reads a file in chunks, using the given chunk sizes.
struct ChunkReader {
    reader: BufReader<File>,
    chunk_sizes: Vec<usize>,
    current_index: usize,
}

impl ChunkReader {
    fn new(file_path: &str, chunk_sizes: Vec<usize>) -> io::Result<Self> {
        let file = File::open(file_path)?;
        Ok(Self {
            reader: BufReader::new(file),
            chunk_sizes,
            current_index: 0,
        })
    }
}

impl Iterator for ChunkReader {
    type Item = io::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.chunk_sizes.len() {
            return None;
        }

        let chunk_size = self.chunk_sizes[self.current_index];
        self.current_index += 1;

        let mut buffer = vec![0; chunk_size];
        match self.reader.read_exact(&mut buffer) {
            Ok(_) => Some(Ok(buffer)),
            Err(e) => {
                if e.kind() == io::ErrorKind::UnexpectedEof {
                    if buffer.is_empty() {
                        None
                    } else {
                        Some(Ok(buffer))
                    }
                } else {
                    Some(Err(e))
                }
            }
        }
    }
}













#[cfg(test)]
mod tests{
    use crate::CryptoFiles::Parser::{*};
    use std::{fs::File, io::{self, repeat, BufReader, Read}, path::PathBuf, str::FromStr};
    use super::Folder;
    use std::fs::{self};

    use std::env;
    use std::io::{ Write};

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
    fn test_encrypt_to_file_success() {
        let temp_dir = create_temp_dir();

        // Create a test file
        let test_file_path = temp_dir.join("test.txt");
        let mut file = File::create(&test_file_path).unwrap();
        let long_string = "This is a test file to be encrypted.".to_string().repeat(1000); // Repeat the string
        writeln!(file, "{}", long_string).unwrap(); // Write the repeated string to the file

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

        clean_up_test_dir(&temp_dir);
    }
    #[test]
    fn test_parser() -> (){
        
        let path = match PathBuf::from_str("./tempg.fvaultwyr") {
            Ok(p) => {p},
            Err(_) => {panic!("error constructing path")},
        };

        
        let reader = BufReader::new(File::open(&path).unwrap());

        let mut  folder= VaultWyrFileParser::new(VaultwyrFileLinker::from_vaultwyr_file(path).unwrap(), reader).to_folder();

        folder.decrypt_all_files("password").unwrap();

        
    }





    
    /* 
    #[test]
    fn test_folder() -> io::Result<()>{
        let mut  chunks = 0;
        // Handling Result returned by `from_path`
        if let Some(mut a) = Folder::from_path(PathBuf::from_str("./temp").unwrap()) {
            for file in a.files {
                for chunk in file.data{
                    match chunk{
                        Ok(chunk) => {
                            chunks+=1;
                            println!("{:?}", chunk)
                        
                        
                        },
                        Err(_) => {}
                    }
                }

            }

        } else {
            assert!(false);
        }
        let a = match Folder::from_path(PathBuf::from_str("./temp").unwrap()) {
            Some(folder) => {folder},
            None => {return Ok(())},
        };
        a.encrypt_files_into_file_with_password("hello")?;
        
    Ok(())
    }
    */

    /* 
    #[test]
    fn test_writer() -> io::Result<()>{



        
        //create test files
        let mut file1 = OpenOptions::new().create(true).write(true).open("./temp/hello.txt")?;
        let mut file2 = OpenOptions::new().create(true).write(true).open("./temp/hi.txt")?;

        let file1_contents = "hello".repeat(1000); // Now the String is owned and stored
        let file1_contents_buf = file1_contents.as_bytes();
        let file2_contents = "bye".as_bytes();

        file1.write_all(&file1_contents_buf)?;
        file2.write_all(&file2_contents)?;

        if let Some(folder)  = Folder::from_path(PathBuf::from_str("./temp").unwrap()){
            let mut writer = match EncryptedFileWriter::new(folder) {
                Some(w) => {w},
                None => {panic!("Something went wrong when creating a parser");},
            };
            writer.encrypt_to_file("hello")?;


        
        remove_file("./temp/hello.txt")?;
        remove_file("./temp/hi.txt")?;
        let new_path = Folder::from_encrypted_path(PathBuf::from_str("temp.fvaultwyr").unwrap())?;
        

        for file in new_path.files{

            match file {
                Ok(mut f) => {f.try_restore_with_password("hello")?;
            
            },
                Err(e) => { panic!("error decrypting file {}", e)},
            }

        }


    }
    Ok(())

}
    */
}