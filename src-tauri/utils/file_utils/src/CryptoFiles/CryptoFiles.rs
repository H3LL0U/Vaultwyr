
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



    fn create_original_file(&self) -> io::Result<File>{
        if self.original_path.exists(){
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, "The file path to the original file already exists"))
        }

        let opened_file = OpenOptions::new().create_new(true).write(true).open(&self.original_path)?;
        
        Ok(opened_file)
    }


    pub fn try_restore_with_password(&mut self, password:&str) -> io::Result<()>{
        let key = password_to_key32(password)?;
        let mut  original_file = self.create_original_file()?;
        for chunk in &mut self.data{
            let decrypted_chunk = aes_decrypt_with_key(key, &chunk).map_err(|_| io::Error::new(io::ErrorKind::Other, "error decrypting chunk"))?;
            original_file.write_all(&decrypted_chunk);
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
        let vaultwyr_folder_file_reader = BufReader::new(OpenOptions::new().read(true).open(&self.new_path)?);

        let mut header = match file.parse_file_header(vaultwyr_folder_file_reader) {
            Some(h) => h,
            None => panic!("Unexpected main header"),
        };

        header.try_restore_with_password(password)?;
    }
    Ok(())
}

}


struct Folder{
    path: PathBuf
}


#[cfg(test)]
mod tests{
    use crate::CryptoFiles::CryptoFiles::{*};
    use std::{io::repeat, path::PathBuf, str::FromStr};


    #[test]
    fn test_parser() -> io::Result<()>{

        let path = match PathBuf::from_str("./temp.fvaultwyr") {
            Ok(p) => {p},
            Err(_) => {panic!("error constructing path")},
        };

        
        let reader = BufReader::new(File::open(&path)?);

        let mut  folder= VaultWyrFileParser::new(VaultwyrFileLinker::from_vaultwyr_file(path)?, reader).to_folder();

        folder.decrypt_all_files("hello")?;

        Ok(())
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