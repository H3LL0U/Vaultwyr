use core::{panic, str};
use std::{error::Error, fmt::format, fs::{File, OpenOptions}, io::{BufRead, Write}, path::{Path, PathBuf}, str::FromStr, vec};
use std::io::{self, BufWriter, BufReader, Seek, SeekFrom, Read};
use encryption_utils::{aes_encrypt_with_key, password_to_key32};
use serde_json::value::Index;
use ParserUtils::{parse_content, vec_to_string, vec_to_usize};
use crate::CryptoFiles::CryptoFiles::{FolderFile};

use super::CryptoFiles::{ VaultWyrFolder};




pub mod ParserUtils{
    use std::io::{self, BufWriter, BufReader, BufRead, Seek, SeekFrom, Read};
    use std::{fs::File};
    pub fn vec_to_usize(vector:Vec<u8>) -> io::Result<usize>{
        
        let header_length = vec_to_string(vector)?;
        
        let header_length:usize = header_length.parse().map_err(|_| io::Error::new(io::ErrorKind::Other, "Could not parse the vector"))?;

        Ok(header_length)
    }

    pub fn vec_to_string(vector:Vec<u8>) -> io::Result<String>{

        

        Ok(String::from_utf8(vector)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Could not convert the vector to string"))?.trim().to_string())
    }
    pub fn read_until_any(reader: &mut BufReader<File>, delimiters: &[u8]) -> io::Result<u8> {
        
        let mut byte = [0; 1];

        while reader.read(&mut byte)? > 0 {
            if delimiters.contains(&byte[0]) {
                break;
            }
            
        }

        Ok(byte[0])
    }

    pub fn parse_length(reader:&mut BufReader<File>) -> usize{
            let mut tmp_buf = Vec::<u8>::new();
            reader.read_until(b' ', &mut tmp_buf);
            tmp_buf.clear();
            reader.read_until(b' ', &mut tmp_buf);
            dbg!(&tmp_buf);
            let length = match vec_to_usize(tmp_buf) {
                Ok(l) => {l},
                Err(_) => { panic!("error parsing the length")},
            };
            length

    }
    pub fn parse_content(mut reader: &mut BufReader<File>,) -> Vec<u8>{
            let length = parse_length(&mut reader);
            let mut content_buf = vec![0u8;length];
            reader.read_exact(&mut content_buf);
            content_buf

    }
    
}



struct FileHeader{
    header_index: u64,
    chunk_indexes: Vec<u64>
}

impl FileHeader{


    pub fn new(header_index:u64) -> Self{
        FileHeader{
            header_index,
            chunk_indexes: Vec::<u64>::new()
        }
    }

    pub fn add_chunk_index(&mut self, index:u64){
        self.chunk_indexes.push(index);
    }

    pub fn parse_to_folder_file(vaultwyr_file: BufReader<File>){
        //pointer location >h length original_path\nfile_hash\n
        
    }
}

pub enum HeaderType {
    
    MainHeader(u64),
    FileHeader(FileHeader)
    

}





impl HeaderType{
    
    pub fn parse_main_header(&self, mut reader:BufReader<File>) -> Option<Vec<String>>{

        
        //returns arguments used for creating a folder 
        match self {
            HeaderType::MainHeader(i) => {
                reader.seek(SeekFrom::Start(*i)).expect("Failed to seek reader");

                let content_str = match vec_to_string(ParserUtils::parse_content(&mut reader)) {
                    Ok(s) => s,
                    Err(_) => panic!("Could not convert the main header's content to string"),
                };

                let content: Vec<String> = content_str
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect();

                if content.len() != 3 {
                    panic!("The main header contains more arguments than expected");
                }

                return Some(content);
            }
        

           ,
        _ => {None}
       }

    }



    pub fn parse_file_header(self, mut reader:BufReader<File>,) -> Option<FolderFile>{
        match self {
            HeaderType::FileHeader(fileheader) => {

                reader.seek(SeekFrom::Start(fileheader.header_index));
                let content = ParserUtils::parse_content(&mut reader);
                
                let content_str = match vec_to_string(ParserUtils::parse_content(&mut reader)) {
                    Ok(s) => s,
                    Err(_) => panic!("Could not convert the file header's content to string"),
                };

            let content: Vec<String> = content_str
                .lines()
                .map(|s| s.to_string())
                .collect();

            let [path_str, algo] = &content[..2] else {
                panic!("The file header contains more arguments than expected");
            };

            Some(FolderFile::new(
                PathBuf::from_str(path_str).expect("Error converting the argument into path"),
                algo.clone(),
                FileChunkIterator::new(reader, fileheader.chunk_indexes),
            ))

            },
            _ => None
        }
       }
}


pub struct VaultwyrFileLinker{
    pub vaultwyr_file_reader: BufReader<File>,
    cur_fileheader: Option<FileHeader>
    
}



impl VaultwyrFileLinker{
    pub fn from_vaultwyr_file(path:PathBuf) -> io::Result<Self>{
        let file = File::open(path)?;
        Ok(Self { vaultwyr_file_reader: BufReader::new(file), cur_fileheader: None })
    }
    fn seek_header_end(&mut self) -> u64{
        //returns the beginning index of the current header
        let header_start_index = match self.vaultwyr_file_reader.stream_position() {
        Ok(i) => {i},
        Err(_) => {panic!("Error getting the position of the pointer")},
        };
                //pointer at >h header_length ...
        let header_length = ParserUtils::parse_length(&mut self.vaultwyr_file_reader);
        match self.vaultwyr_file_reader.seek(SeekFrom::Current(header_length as i64)) {
            Ok(index) => {},
            Err(_) => {panic!("Could not advance to the next header")},
        };
        header_start_index
    }


}

impl Iterator for VaultwyrFileLinker{
    type Item = HeaderType;

    fn next(&mut self) -> Option<Self::Item> {
        
        let delimeters = [b'm', b'h', b'c'];


        let header_representation = match ParserUtils::read_until_any(&mut self.vaultwyr_file_reader, &delimeters) {
            Ok(h) => {h},
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
            return None;
            
            }
            _ => panic!("Error when reading untill some delimter")
        };
        loop{
            match header_representation {

                //finding the main header
                b'm' => {
                    HeaderType::MainHeader(self.seek_header_end());
                }
                b'h' => {
                    // Compute the new FileHeader without borrowing `self` twice
                    let new_fileheader = FileHeader::new(self.seek_header_end());

                    // Replace the current FileHeader and get the old one
                    let old_fileheader = self.cur_fileheader.replace(new_fileheader);

                    match old_fileheader {
                        Some(f) => {
                            if f.chunk_indexes.is_empty() {
                                panic!("There were no chunks in the fileheader");
                            }
                            return Some(HeaderType::FileHeader(f));
                        }
                        None => {
                            return None;
                        }
                    }
                }
                b'c' => {
                    let cur_header_pos = self.seek_header_end();
                    match &mut self.cur_fileheader {
                        Some(fileheader) => {
                            fileheader.add_chunk_index(cur_header_pos);
                        },
                        None => {panic!("A file chunk appeared without the file header")},
                    }
                    
                }
                _ => panic!("This header type is not implemented")
                
                ,
            }
        }





        None


    }
}

pub struct FileChunkIterator{
    reader:BufReader<File>,
    chunk_indexes:Vec<u64> //should be reversed
}

impl FileChunkIterator {
    pub fn new(reader: BufReader<File>,mut chunk_indexes: Vec<u64>) -> Self{
        chunk_indexes.reverse();
        Self { reader: reader, chunk_indexes: chunk_indexes}
    }

}
impl Iterator for FileChunkIterator{
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk_index = match self.chunk_indexes.last() {
            Some(i) => {i},
            None => {return  None;},
        };
        self.reader.seek(SeekFrom::Start(*chunk_index));
        self.chunk_indexes.pop();
        Some(parse_content(&mut self.reader))

        
    }

}



pub struct VaultWyrFileParser{
    linker: VaultwyrFileLinker,
    reader: BufReader<File>
}
impl VaultWyrFileParser{
    pub fn new(linker: VaultwyrFileLinker, reader: BufReader<File>) -> Self{
        
    VaultWyrFileParser { 
        linker, 
        reader }
    }
    pub fn to_folder(mut self) -> VaultWyrFolder{
        let header_type = match self.linker.next() {
            Some(header) => {header},
            None => {panic!("Could not find the main header")},
        };
        let args = match header_type.parse_main_header(self.reader) {
            Some(args) => {args},
            None => {panic!("The first header is not the main header")},
        };
        let [new_path, algo, chunk_size] = &args[..3] else {
            panic!("Expected exactly 3 arguments: new_path, algo, chunk_size");
        };

        VaultWyrFolder { 
            new_path: PathBuf::from_str(new_path).expect("Error converting the path from string"),
            algo: algo.clone(),
            chunk_size: chunk_size.parse().expect("Could not convert chunk_size into int"),
            files: self.linker,
        }
    }
}

struct EncryptedFileReader{
    linker:VaultwyrFileLinker
    
}




/* 
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

        buffer = format!("m {} {}", buffer.len(), buffer);
        
        self.vaultwyr_file.write_all(&buffer.as_bytes())?;
        Ok(())

    }

    fn write_files(&mut self, password: &str) -> io::Result<()> {
        let key = password_to_key32(password)?;
    
        for file_result in &mut self.folder.files{

    
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

*/


