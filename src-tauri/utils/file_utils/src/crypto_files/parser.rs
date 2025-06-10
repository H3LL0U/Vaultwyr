
use core::panic;
use std::{ fs::File, path:: PathBuf, str::FromStr, };
use std::io::{self, BufReader, Seek, SeekFrom };

use parser_utils::{parse_content, vec_to_string, };
use crate::crypto_files::crypto_files::FolderFile;

use super::crypto_files::VaultwyrFile;




pub mod parser_utils{
    use std::io::{self,  BufReader, BufRead, Read};
    use std::fs::File;
    pub fn vec_to_usize(vector:Vec<u8>) -> io::Result<usize>{
        
        let header_length = vec_to_string(vector)?;
        
        let header_length:usize = header_length.parse().map_err(|_| io::Error::new(io::ErrorKind::Other, "Could not parse the vector"))?;

        Ok(header_length)
    }

pub fn vec_to_string(vector: Vec<u8>) -> io::Result<String> {
    String::from_utf8(vector)
        .map(|s| s.trim().to_string()) // Trim and convert to String
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Could not convert the vector to string"))
}
    pub fn read_until_any(reader: &mut BufReader<File>, delimiters: &[u8]) -> io::Result<u8> {
        
        let mut byte = [0; 1];

        while reader.read(&mut byte)? > 0 {
            if delimiters.contains(&byte[0]) {
                return Ok(byte[0])
            }
            
        }
        Err(io::Error::new(io::ErrorKind::UnexpectedEof, "end of file"))
        
    }

    pub fn parse_length(reader:&mut BufReader<File>) -> usize{
            let mut tmp_buf = Vec::<u8>::new();
            reader.read_until(b' ', &mut tmp_buf).unwrap();
            tmp_buf.clear();
            reader.read_until(b' ', &mut tmp_buf).unwrap();
            
            let length = match vec_to_usize(tmp_buf) {
                Ok(l) => {l},
                Err(_) => { panic!("error parsing the length")},
            };
            length

    }
    pub fn parse_content(mut reader: &mut BufReader<File>,) -> Vec<u8>{
            let length = parse_length(&mut reader);
            
            let mut content_buf = vec![0u8;length];
            reader.read_exact(&mut content_buf).unwrap();
            content_buf

    }

pub fn split_into_chunks(data: Vec<u8>, delimiter: u8, num_chunks: usize) -> Vec<Vec<u8>> {
    let mut splits = data.split(|&byte| byte == delimiter);
    let mut chunks = Vec::with_capacity(num_chunks);

    for _ in 0..num_chunks - 1 {
        if let Some(chunk) = splits.next() {
            chunks.push(chunk.to_vec());
        }
    }

    
    let mut remaining = Vec::new();
    for part in splits {
        remaining.extend_from_slice(part);
        remaining.push(delimiter); 
    }

    
    if let Some(last) = remaining.last() {
        if *last == delimiter {
            remaining.pop();
        }
    }

    chunks.push(remaining);
    chunks
}

    
}


#[derive(Debug)]
pub struct FileHeader{
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

}

pub enum HeaderType {
    
    MainHeader(u64),
    FileHeader(FileHeader)
    

}





impl HeaderType{
    
    pub fn parse_main_header(&self, mut reader:BufReader<File>) -> Option<Vec<Vec<u8>>>{

        
        //returns arguments used for creating a folder 
        match self {
            HeaderType::MainHeader(i) => {
                reader.seek(SeekFrom::Start(*i)).expect("Failed to seek reader");

                let content =  parser_utils::split_into_chunks(parser_utils::parse_content(&mut reader),
                b'\n',
                3);

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
                
                match reader.seek(SeekFrom::Start(fileheader.header_index)) {
                    Ok(_) => {},
                    Err(_) => {panic!("error updating the pointer")},
                };
                let content = parser_utils::parse_content(&mut reader);
                
                let content_str = match vec_to_string(content) {
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
    pub fn from_vaultwyr_file(path:&PathBuf) -> io::Result<Self>{
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
        let header_length = parser_utils::parse_length(&mut self.vaultwyr_file_reader);
        match self.vaultwyr_file_reader.seek(SeekFrom::Current(header_length as i64)) {
            Ok(_) => {},
            Err(_) => {panic!("Could not advance to the next header")},
        };
        header_start_index
    }


}

impl Iterator for VaultwyrFileLinker{
    type Item = HeaderType;

    fn next(&mut self) -> Option<Self::Item> {
        loop{
        let delimeters = [b'm', b'h', b'c'];


        let header_representation = match parser_utils::read_until_any(&mut self.vaultwyr_file_reader, &delimeters) {
            Ok(h) => {h},
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof && self.cur_fileheader.is_none() => {
            return None;
            
            },

            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof =>{
                let old_fileheader = self.cur_fileheader.take();
                
                return Some(HeaderType::FileHeader(old_fileheader.unwrap()));
            }

            _ => panic!("Error when reading untill some delimter")
        };
        
        
            match header_representation {

                //finding the main header
                b'm' => {
                    
                    return Some(HeaderType::MainHeader(self.seek_header_end()));
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
                        
                            continue;
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
        self.reader.seek(SeekFrom::Start(*chunk_index)).unwrap();
        
        self.chunk_indexes.pop();
        Some(parse_content(&mut self.reader))

        
    }

}



pub struct VaultWyrFileParser{
    linker: VaultwyrFileLinker,
    reader: BufReader<File>,
    path: PathBuf
}
impl VaultWyrFileParser{

    pub fn from_path(path:&PathBuf) -> io::Result<Self> {
        let linker = VaultwyrFileLinker::from_vaultwyr_file(path)?;
        let reader = BufReader::new(File::open(path)?);
        let path = path.clone();

        Ok(Self{
            linker,
            reader,
            path
        })
    }


    pub fn to_folder(mut self) -> VaultwyrFile{
        let header_type = match self.linker.next() {
            Some(header) => {header},
            None => {panic!("Could not find the main header")},
        };
        let mut args = match header_type.parse_main_header(self.reader) {
            Some(args) => {args},
            None => {panic!("The first header is not the main header")},
        };


        
        let validation =  args.pop().expect("could not get the validation string from the main header");
        let algo =  parser_utils::vec_to_string(args.pop().expect("could not get the algo from the main header")).expect("could not convert the algorythm type to string");
        let new_path = self.path;
            
            
        let files = self.linker;


        //goes in reverse order since args are stored like this : new_path , algo, validation <(sequential pop starts from here)
        VaultwyrFile::new(new_path, algo, validation, files)


    }
}









