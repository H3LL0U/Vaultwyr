use core::str;
use std::{error::Error, fmt::format, fs::{File, OpenOptions}, io::{BufRead, Write}, path::{Path, PathBuf}, str::FromStr, vec};
use std::io::{self, BufWriter, BufReader, Seek, SeekFrom, Read};
use encryption_utils::{aes_encrypt_with_key, password_to_key32};
use ParserUtils::{vec_to_string, vec_to_usize};
use crate::CryptoFiles::CryptoFiles::DataSource;

use super::CryptoFiles::{Folder, FolderFileIter};




pub mod ParserUtils{
    use std::io;
    pub fn vec_to_usize(vector:Vec<u8>) -> io::Result<usize>{
        
        let header_length = vec_to_string(vector)?;
        
        let header_length:usize = header_length.parse().map_err(|_| io::Error::new(io::ErrorKind::Other, "Could not parse the vector"))?;

        Ok(header_length)
    }

    pub fn vec_to_string(vector:Vec<u8>) -> io::Result<String>{

        

        Ok(String::from_utf8(vector)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Could not convert the vector to string"))?.trim().to_string())
    }

    
}


pub enum HeaderType {
    MainHeader(u64),
    FileHeader(u64),
    FileChunk(u64),
    
}


pub struct FileChunkParser{
    path: PathBuf,
    index: u64,
    reader: BufReader<File>

}

impl FileChunkParser{
    pub fn new(path: PathBuf,index: u64) -> io::Result<Self>{
        let mut reader = BufReader::new(File::open(&path)?);
        reader.seek(SeekFrom::Start(index))?;
        Ok(FileChunkParser{
            reader: reader,
            path,
            index,
            
        })
    }
}

impl Iterator for FileChunkParser{
    type Item = io::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {

        /*the pointer is located here >c chunk_len chunk */
        let mut length_buf = Vec::<u8>::new();
        
        // Read until the first space (assumed to be the length part)
        if let Err(e) = self.reader.read_until(b' ', &mut length_buf) {
            return Some(Err(e));
        }
        // check if the next chunk exists by checking for chunk specifier (c)
        if *match length_buf.get(0) {
            Some(b) => {b},
            None => {return None},
        } != b'c'{
        
        return  None;
        
        }
        
        length_buf.clear();
        if let Err(e) = self.reader.read_until(b' ', &mut length_buf) {
            return Some(Err(e));
        }
        // Convert length buffer to a usize
        let length = match ParserUtils::vec_to_usize(length_buf) {
            Ok(len) => len,
            Err(e) => return Some(Err(e)),
        };
        
        let mut chunk_buf = vec![0u8; length];
        if let Err(e) = self.reader.read_exact(&mut chunk_buf) {
            return Some(Err(e));
        }
        


        Some(Ok(chunk_buf))
    }
}
pub struct EncryptedFileHeaderIterator {
    pub reader: BufReader<File>,
}

impl EncryptedFileHeaderIterator {
    pub fn new(file: File) -> Self {
        Self {
            reader: BufReader::new(file),
            
        }
    }

    /// Custom method to read until any of the specified delimiters.
    fn read_until_header(&mut self, buf: &mut Vec<u8>) -> io::Result<()> {
        let mut byte = [0u8; 1];
        
        let delimeters = [b'h'];


        loop {
            match self.reader.read_exact(&mut byte) {
                Ok(_) => {
                    buf.push(byte[0]);
                    if delimeters.contains(&byte[0]) {
                        break;
                    }
                }

                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
    

}


impl Iterator for EncryptedFileHeaderIterator{
    type Item = io::Result<u64>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut  tmp_buf:  Vec<u8> = vec![] ;
        match self.read_until_header(&mut tmp_buf) {
            Ok(_) => {
                    return Some(Ok(match self.reader.stream_position() {
                        Ok(i) => {i+1},
                        Err(_) => {return Some(Err(io::Error::new(io::ErrorKind::Other, "could not get the reader pos")))},
                    }))
                }
                


            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                    return None;}
            Err(e) => {return Some(Err(e))},
        };
    }
}

pub struct EncryptedFileReader{
    vaultwyr_path:PathBuf,
    reader: BufReader<File>
}





impl EncryptedFileReader{

    pub fn new(vaultwyr_path:PathBuf) -> io::Result<(Self)>{




        let vaultwyr_file = File::open(&vaultwyr_path)?;

        Ok(EncryptedFileReader{
            vaultwyr_path: vaultwyr_path,
            reader: BufReader::new(vaultwyr_file)  
        })

        
    }

    pub fn into_folder(mut self) -> io::Result<Folder>{
        //parsing the main header
        let mut header_length: Vec<u8> = vec![];

        self.reader.read_until(b' ', &mut header_length)?;

        let header_length = ParserUtils::vec_to_usize(header_length)?;

        let mut buffer = vec![0u8; header_length];
        self.reader.read_exact(&mut buffer)?;

        let binding = vec_to_string(buffer)?;
        let buffer_parts: Vec<&str> = binding.split("\n").collect();

        if buffer_parts.len() != 3{
            return Err(io::Error::new(io::ErrorKind::Other,"Main header contains more parameters than it should"))
        }

        let new_path = PathBuf::from_str(buffer_parts[0]).map_err(|_| io::Error::new(io::ErrorKind::Other, "Error parsing new path into pathbuf"))?;
        Ok(Folder{
            new_path: new_path.clone(),
            algo : buffer_parts[1].to_string(),
            chunk_size: buffer_parts[2].parse().map_err(|_| io::Error::new(io::ErrorKind::Other, "Error parsing chunk size into pathbuf"))?,
            files: FolderFileIter {
                file_type: super::CryptoFiles::FileType::FromEncryptedFolder(EncryptedFileHeaderIterator{
                    reader: self.reader
                }),
                folder_path: new_path,
                reader: None
            }
        })
        
        

        

        





    }
}


pub struct EncryptedFileWriter{
    folder:Folder,
    vaultwyr_file: File

}








impl EncryptedFileWriter{

    fn write_header(&mut self) -> io::Result<()>{

        let new_path = match self.folder.new_path.to_str() {
            Some(s) => {s},
            None => {return Err(io::Error::new(io::ErrorKind::InvalidData, "Could not convert path to str"))},
        };
        let mut buffer = format!("{}\n{}\n{}", new_path, self.folder.algo,self.folder.chunk_size);

        buffer = format!("{} {}", buffer.len(), buffer);
        
        self.vaultwyr_file.write_all(&buffer.as_bytes())?;
        Ok(())

    }

    fn write_files(&mut self, password: &str) -> io::Result<()> {
        let key = password_to_key32(password)?;
    
        for file_result in &mut self.folder.files {
            let mut file = match file_result {
                Ok(file) => file,
                Err(_) => continue, // Skip on error
            };
    
            // Creating the file header
            let original_path = match file.original_path.to_str() {
                Some(s) => s,
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Could not convert path to str",
                    ));
                }
            };
    
            let mut header = format!("{}\n{}", original_path, &file.file_hash);
            header = format!("h {} {}", header.len(), header);
            self.vaultwyr_file.write_all(header.as_bytes())?;
    
            // Write out the chunks
            if let DataSource::Iterator(ref mut data) = file.data {
                for chunk_result in data {
                    let mut chunk = match chunk_result {
                        Ok(chunk) => chunk,
                        Err(_) => continue, // Skip the chunk on error
                    };
    
                    match self.folder.algo.as_str() {
                        "aes256" => {
                            chunk = match aes_encrypt_with_key(key, &chunk) {
                                Ok(encrypted_chunk) => encrypted_chunk,
                                Err(_) => {
                                    return Err(io::Error::new(
                                        io::ErrorKind::InvalidData,
                                        "Could not encrypt chunk",
                                    ));
                                }
                            };
    
                            // Write in the format: "c <chunk_len> <chunk_data>"
                            self.vaultwyr_file.write_all(b"c ")?;
                            self.vaultwyr_file.write_all(chunk.len().to_string().as_bytes())?;
                            self.vaultwyr_file.write_all(b" ")?;
                            self.vaultwyr_file.write_all(&chunk)?;
                        }
                        _ => {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                "No support for such algorithm",
                            ));
                        }
                    }
                }
            }
        }
    
        Ok(())
    }
        

        pub fn encrypt_to_file(&mut self, password:&str) -> io::Result<()>{

            self.write_header()?;
            self.write_files(password)?;
            Ok(())
        }
        pub fn new(folder:Folder) -> Option<Self>{
            //check if the file exists first
            if folder
            .new_path
            .extension()
            .and_then(|ext| ext.to_str())
            != Some("fvaultwyr") || folder.new_path.exists()
        {
            return None;
        }
            let vaultwyr_file = match OpenOptions::new().write(true).create_new(true).open(&folder.new_path) {
                Ok(f) => {f},
                Err(_) => {return None},
            };
    
            Some(EncryptedFileWriter { folder: folder, vaultwyr_file })
        }
    

    }




