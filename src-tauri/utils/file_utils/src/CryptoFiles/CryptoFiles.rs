
use serde::{Serialize, Deserialize};
use serde_json::map::Iter;
use serde_json::value::Index;
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
use crate::Parser::EncryptedFileReader;
use crate::calculate_file_hash;
use crate::CryptoFiles::Parser::*;
use ParserUtils::*;
use std::io::BufRead;

use super::Parser;





pub struct FileChunkIterator<R: Read> {
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



//files

pub enum FileType {
    FromEncryptedFolder(EncryptedFileHeaderIterator), //if it is from an encrypted folder you need to get the file based from the index of the file
    FromRegularFolder(ReadDir) // if it is a regular folder you can get the files inside of the folder based on the path
    
}

pub struct FolderFileIter{
    pub file_type: FileType,
	pub folder_path: PathBuf,
	pub reader: Option<BufReader<File>>
}

impl FolderFileIter {
    fn from_folder(path: PathBuf) -> Option<Self> {
        
        if path.extension().and_then(|s| s.to_str()) == Some("fvaultwyr") {
            None //not supported
        } else if path.is_dir() {
            // Handle the Result from read_dir()
            match path.read_dir() {
                Ok(read_dir) => Some(FolderFileIter {
                    file_type: FileType::FromRegularFolder(read_dir),
                    folder_path: path,
                    reader : None
                }),
                Err(_) => None, // If read_dir fails
            }
        } else {
            None 
        }
    }

}


impl Iterator for FolderFileIter {
    type Item = io::Result<FolderFile>;
    
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.file_type {
            FileType::FromRegularFolder(path_reader) => {
                match path_reader.next() {
                    Some(Ok(entry)) => {
                        match FolderFile::from_path(entry.path(), 2048) {
                            Ok(folder_file) => Some(Ok(folder_file)),
                            Err(e) => Some(Err(e)),
                        }
                    }
                    Some(Err(e)) => Some(Err(e)), // Propagate the error
                    None => None,
                }
            }

            FileType::FromEncryptedFolder(headers) => {
                

                for header in headers.into_iter() {
                    match header {
                        Ok(index) => {
                            // Seek to the start of the file header
                            dbg!(index);
                            if let Err(e) = headers.reader.seek(io::SeekFrom::Start(index)) {
                                return Some(Err(io::Error::new(e.kind(), e.to_string())));
                            }

                            let mut header_length: Vec<u8> = vec![];
                            
                            if let Err(e) = headers.reader.read_until(b' ', &mut header_length) {
                                return Some(Err(e));
                            }
                            
                            let header_length = match ParserUtils::vec_to_usize(header_length) {
                                Ok(len) => len,
                                Err(e) => return Some(Err(e)),
                            };
                            

                            let mut buffer = vec![0u8; header_length];
                            if let Err(e) = headers.reader.read_exact(&mut buffer) {
                                return Some(Err(e));
                            }

                            let binding = match vec_to_string(buffer) {
                                Ok(s) => s,
                                Err(e) => return Some(Err(e)),
                            };

                            let buffer_parts: Vec<&str> = binding.split('\n').collect();
                            if buffer_parts.len() != 2 {
                                return Some(Err(io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    "Header format is incorrect",
                                )));
                            }

                            let original_path = PathBuf::from(buffer_parts[0]);
                            let file_hash = buffer_parts[1].to_string();

                            let position = match headers.reader.stream_position() {
                                Ok(i) => i,
                                Err(e) => return Some(Err(e)),
                            };
                            
                            let parser = match FileChunkParser::new(self.folder_path.clone(), position) {
                                Ok(parser) => parser,
                                Err(e) => return Some(Err(e)),
                            };
                            return Some(Ok(FolderFile {
                                original_path,
                                file_hash,
                                validation: vec![],
                                
                                data: DataSource::Parser(parser)
                            }));
                        }
                        Err(e) => return Some(Err(e)),
                    }
                }

                None // Exhausted all headers
            }
        }
    }
}







pub enum DataSource{
    Iterator(FileChunkIterator<File>),
    Parser(FileChunkParser)
}
pub struct FolderFile{
    
    pub original_path: PathBuf,
    
    
    pub validation: Vec<u8>,
    pub file_hash: String,
    pub data: DataSource,
}




impl FolderFile{
    pub fn from_path(original_path:PathBuf, chunk_size:usize) -> io::Result<FolderFile>{
        let file_hash = calculate_file_hash(&original_path)?;
        
        Ok(FolderFile{
            data : DataSource::Iterator(FileChunkIterator::new(File::open(&original_path)?, chunk_size)),
            original_path : original_path,
            validation : vec![0u8; 32],
            file_hash : file_hash, 
            
            
        })
    }


    fn create_original_file(&self) -> io::Result<File>{
        if self.original_path.exists(){
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, "The file path to the original file already exists"))
        }

        let opened_file = OpenOptions::new().create_new(true).write(true).open(&self.original_path)?;
        
        Ok(opened_file)
    }


    pub fn try_restore_with_password(&mut self, password:&str) -> io::Result<()>{

        let mut  original_file = self.create_original_file()?;

        

            match &mut self.data {
                
                DataSource::Iterator(_) => {

                    return Err(io::Error::new(io::ErrorKind::Unsupported, "DataSource Iterator not supported, the file should be decrypted"))


                },
                DataSource::Parser(p) =>{
                    let key = password_to_key32(password).map_err(|_| io::Error::new(io::ErrorKind::Other,"error converting password to key"))?;
                    for chunk in p{

                        match chunk {
                            Ok(chunk) => {
                                
                                //decrypt using aes256 into file
                                
                                let decrypted_chunk = aes_decrypt_with_key(key, &chunk).map_err(|_| io::Error::new(io::ErrorKind::Other, "Error decrypting chunk"))?;

                                original_file.write_all(&decrypted_chunk)?;



                            },
                        Err(e) => { 
                            return Err(io::Error::new(
                                io::ErrorKind::Other,
                                format!("Error retrieving chunk: {}", e),
                            ));
                            }
                        }


                    }

                }
            };
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


pub struct Folder{
    pub new_path:PathBuf,
	pub algo : String,
	pub chunk_size: usize,

	pub files: FolderFileIter
}




impl Folder{
    pub fn from_path(path: PathBuf) -> Option<Self> {
        let mut folder = Folder {
            new_path:PathBuf::new(),
            algo: "aes256".to_string(),
            chunk_size: 2048,
            
            files: match FolderFileIter::from_folder(path) {
                Some(folder) => folder,
                None => return None,
            },
        };
        folder.new_path = folder.files.folder_path.clone();
        folder.new_path.set_extension("fvaultwyr");
        Some(folder)
    }


    fn from_encrypted_path(vaultwyr_path:PathBuf) -> io::Result<Self>{
        
        EncryptedFileReader::new(vaultwyr_path)?.into_folder()


    }
    fn decrypt_all_files(&self){
        
    }
    /* 
    pub fn encrypt_files_into_file_with_password(mut self ,password:&str) -> io::Result<()>{
        let mut  chunks = 0;
        // Handling Result returned by `from_path`
        self.new_path = self.files.folder_path.clone();
        self.new_path.set_extension("fvaultwyr");

        
        if self.new_path.exists(){
                return Err(io::Error::new(io::ErrorKind::AlreadyExists, "The file already exists"))
            }
        let mut  opened_file = OpenOptions::new().create_new(true).write(true).read(true).open(&self.new_path)?;


        let key = password_to_key32(password)?;


        
        for file in self.files {
            //todo specify that this is the begining of the file

            for chunk in file.data{
                match chunk{
                    Ok(chunk) => {

                        let encrypted_chunk = match aes_encrypt_with_key(key, &chunk) {
                            Ok(encrypted) => {encrypted},
                            Err(_) => { return Err(io::Error::new(io::ErrorKind::InvalidData, "Could not encrypt "))},
                        };
                        encrypted_chunk.len();
                        
                        opened_file.write_all(&encrypted_chunk)?;
                        
                        
                        
                    },
                        Err(_) => {}
                    }
                }
            }
        Ok(())
    }
    



    fn print_paths(&mut self) {
        for file in &mut self.files{
            println!("{}" ,file.original_path.to_str().unwrap())
        }
    }
    */
    pub fn algo(mut self,algo:&str) -> Self{
        self.algo = algo.to_string();
        self
    }

    pub fn chunk_size(mut self ,chunk_size:usize) -> Self{
        self.chunk_size = chunk_size;
        self
    }

    








}


#[cfg(test)]
mod tests{
    use crate::CryptoFiles::CryptoFiles::{*};
    use std::{io::repeat, path::PathBuf, str::FromStr};
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
}