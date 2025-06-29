//!This module is mainly used for writing either from .fvaultwyr file or to the vaultwyr file (encrypting and decrypting)




use core::panic;


use std::io::{self, BufReader, Read, Write};
use std::fs::{remove_file, File, OpenOptions};
use std::path::{Path,  PathBuf};
use dialog_lib::responses::{UserResponseSkipRetry, UserResponseTerminateRetry};
use encryption_utils::{aes_decrypt_with_key, aes_encrypt_with_key, password_to_key32, validate_key32};
use crate::file_traversal::calculate_file_hash;
use crate::crypto_files::parser::*;
use crate::file_traversal::RecursiveDirIter;
use crate::file_traversal::calculate_dir_size;
use crate::behaviour::{self, OnErrorBehaviour, VaultwyrError};



use dialog_lib::responses::*;
use dialog_lib::prebuilt_windows::*;
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



pub fn original_path(mut self, new_path:PathBuf) -> Self{
    self.original_path = new_path;
    self
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
            
            let decrypted_chunk = aes_decrypt_with_key(&key, &chunk).map_err(|_| io::Error::new(io::ErrorKind::Other, "error decrypting chunk"))?;
            original_file.write_all(&decrypted_chunk)?;
        }
        Ok(())

            
        }
       



    }
    



///used to represent a .fvaultwyr and .vaultwyr file
pub struct VaultwyrFile{
    pub new_path:PathBuf,
	pub algo : String,
	pub validation: Vec<u8>,

	pub files: VaultwyrFileLinker,

    on_error_behaviour: OnErrorBehaviour,

    restore_into_original_folder: bool, //if set to true will restore the files in the same location as the file is in. Otherwise restores to the original location
    original_path: PathBuf
}




impl VaultwyrFile{

    pub fn restore_into_original_folder(mut self, state:bool) ->Self{
        self.restore_into_original_folder = state;
        self
    } 
    pub fn new(new_path: PathBuf,algo: String,validation:Vec<u8>,files: VaultwyrFileLinker, original_path:PathBuf) -> Self{
        VaultwyrFile { new_path, algo ,
             validation,
            files,
            on_error_behaviour: OnErrorBehaviour::AskUser, //ask user by default
            restore_into_original_folder: true, //restores into the original path by default
            original_path: original_path
        }
    }

    pub fn validate_key(&self,key: &[u8; 32]) -> bool{

        validate_key32(key, &self.validation)
    }
    pub fn validate_password(&self,password: &str) -> bool{
        let key = match password_to_key32(password) {
            Ok(key) => {key},
            Err(_) => {return false;},
        };
    self.validate_key(&key)
    }

    pub fn on_error_behaviour(mut self, behaviour: OnErrorBehaviour) -> Self{
        self.on_error_behaviour = behaviour;
        self
    }

fn get_new_file_path(original_root: &Path, new_root: &Path, file_path: &Path) -> PathBuf {
    let original_root = original_root.parent().unwrap_or_else(|| Path::new(""));
    let new_root = new_root.parent().unwrap_or_else(|| Path::new(""));
    let relative_path = file_path.strip_prefix(original_root).unwrap_or(file_path);
    new_root.join(relative_path)
}
pub fn decrypt_all_files(mut self, password: &str) -> Option<VaultwyrError>   {

    match self.validate_password(password) {
        true => {},
        false => {
            match self.on_error_behaviour {
                OnErrorBehaviour::AskUser => {close_popup("Invalid password", "The password you provided is either incorrect or invalid");
                return Some(VaultwyrError::BadPassword);
            },
            OnErrorBehaviour::TerminateOnError => return Some(VaultwyrError::BadPassword)

                
            }
            
    }
}


    //parsing the vaultwyr file



    'file_loop :for file in &mut self.files {
        


    let vaultwyr_folder_file_reader: BufReader<File>;

    loop {
        vaultwyr_folder_file_reader = BufReader::new(match OpenOptions::new().read(true).open(&self.new_path) {
            Ok(f) => {f},
            Err(_) => {
                match self.on_error_behaviour {
                    OnErrorBehaviour::AskUser => {match ask_terminate_retry("Unable to open the vaultwyr file", "The file you selected could not be opened. Make sure that it is not used by any other program") {
                        Some(UserResponseTerminateRetry::Retry) => {continue;},
                        None| Some(UserResponseTerminateRetry::Terminate) => {return Some(VaultwyrError::FileReadError);},
                    }},
                    OnErrorBehaviour::TerminateOnError =>{ return Some(VaultwyrError::FileReadError);}
                }
            },
        });
        break;
    }


        let mut header = match file.parse_file_header(vaultwyr_folder_file_reader) {
            Some(h) => h,
            None => panic!("Unexpected main header"),
        };


        // edit the header's path to a new path if restore to original path is set false (restore in the same location as the vaultwyr file)
        match self.restore_into_original_folder {
            true => {},
            false =>{
                header.original_path = Self::get_new_file_path(&self.original_path, &self.new_path, &header.original_path);
                
            }
        };


        loop {
            
        
        match header.try_restore_with_password(password) {
            Ok(_) => {break;},
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists=> {

                match self.on_error_behaviour {
                    OnErrorBehaviour::AskUser => {match ask_skip_retry("The path already exists", format!("An error occurred when decrypting a file into the following location:\n{:?}\nThis location already contains a file.\nWhat should be done?", &header.original_path)) {
                    Some(UserResponseSkipRetry::Skip) => { continue 'file_loop;},
                    Some(UserResponseSkipRetry::Retry)|None => {continue;}}
                    },
                    OnErrorBehaviour::TerminateOnError => return Some(VaultwyrError::DecryptionError)


                    
                }
                
            },
            Err(_) => {
                match self.on_error_behaviour {
                    OnErrorBehaviour::AskUser => {match ask_skip_retry("Unknown error", format!("An error occurred when decrypting a file into the following location:\n{:?}\nWhat should be done?", &header.original_path)) {
                    Some(UserResponseSkipRetry::Skip) => { continue 'file_loop;},
                    Some(UserResponseSkipRetry::Retry)|None => {continue;}}
                    },
                    OnErrorBehaviour::TerminateOnError => return Some(VaultwyrError::DecryptionError)


                    
                }
            }
        };
        
        }
        
    }
    loop {
        
    
    match remove_file(&self.new_path) {
        Ok(_) => {break;},
        Err(_) => {
            match self.on_error_behaviour {
                OnErrorBehaviour::AskUser => {
                    match ask_terminate_retry("Error deleting file", "Could not delete the vaultwyr file") {
                        Some(UserResponseTerminateRetry::Retry) => {continue;},
                        Some(UserResponseTerminateRetry::Terminate)|None => {return Some(VaultwyrError::FileDeletionError);},
                    }
                },
                OnErrorBehaviour::TerminateOnError => {
                    return Some(VaultwyrError::FileDeletionError);
                }
            }
        },
    };
}
    None
}

}

///This struct is used to represent a regular folder containing files
///You can use it to create a fvaultwyr/vaultwyr file
pub enum PathType{
    File(RecursiveDirIter, PathBuf),
    Folder(RecursiveDirIter)
}


pub struct EncryptionPath {
    pub new_path: PathBuf,
    pub vaultwyr_file: File,
    pub algo: Option<String>,
    pub chunk_size: Option<usize>,
    pub files: PathType,
    validation: Vec<u8>,
    pub max_size: usize,
    on_error_behaviour: behaviour::OnErrorBehaviour,
    paths: Vec<PathBuf>
}

impl EncryptionPath {
    pub fn max_size(mut self, max_size:usize) -> Self{
        self.max_size = max_size;
        self
    }
    pub fn on_error_behaviour(mut self,behaviour:OnErrorBehaviour) -> Self{
        self.on_error_behaviour = behaviour;
        self
    }
    pub fn chunk_size(mut self, chunk_size:usize) -> Self{
        self.chunk_size = Some(chunk_size);
        self
    }
    fn create_vaultwyr_file(path: &PathBuf) -> io::Result<File> {
        
        OpenOptions::new().create_new(true).write(true).read(true).open(path)
    }

    pub fn new(mut path: PathBuf, optional_on_error_behaviour: Option<OnErrorBehaviour>) -> io::Result<Self> {

        let on_error_behaviour = optional_on_error_behaviour.unwrap_or(OnErrorBehaviour::AskUser);
        let files ;

        if !path.is_dir() {
            
            files = PathType::File(RecursiveDirIter::new(&path)?,path.clone());
            path.set_extension("vaultwyr");
        }
        else {
            
            files = PathType::Folder(RecursiveDirIter::new(&path)?);
            path.set_extension("fvaultwyr");
        }
        let vaultwyr_file:File;
        loop {
            
        
        match Self::create_vaultwyr_file(&path.clone()) {
            Ok(f) => {vaultwyr_file = f;
            break;},
            Err(_) => {
                match on_error_behaviour {
                    OnErrorBehaviour::AskUser => {
                        //TODO: Add a rename pop-up that asks a user if they want to rename the vaultwyr file to some other name instead of using the default
                        match ask_replace_terminate_retry("Error creating the initial Vaultwyr file", format!("There was an error when creating the vaultwyr file\nPlease check if you have the file created at the following location:\n{:?}",&path).as_str()) {
                            Some(UserResponseReplaceTerminateRetry::Terminate) |None => {return Err(io::Error::new(io::ErrorKind::AlreadyExists, "The Vaultwyr path already exists"))},
                            Some(UserResponseReplaceTerminateRetry::Retry) => {continue;}
                            Some(UserResponseReplaceTerminateRetry::Replace) =>{
                                match remove_file(&path) {
                                    Ok(_) => {continue;},
                                    Err(_) => {

                                        let _ = close_popup("Could not replace the file", "An error occurred when trying to replace the file");
                                        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "The Vaultwyr path already exists"))
                                    },
                                }
                            }
                            
                        }
                    },
                    OnErrorBehaviour::TerminateOnError => {
                       return Err(io::Error::new(io::ErrorKind::AlreadyExists, "The Vaultwyr path already exists")) 
                    }
                }



            },
        };
        }
        Ok(Self {
            new_path: path,
            vaultwyr_file,
            algo: None,
            chunk_size: Some(2048),
            files,
            validation: vec![0u8;32],
            max_size: 53_687_091_200, //50 GB default max size
            on_error_behaviour: on_error_behaviour, //ask user by default
            paths: Vec::new() // used to store the paths that were successfully encrypted

        })
    }

    fn write_header(&mut self,key:&[u8; 32]) -> Option<VaultwyrError> {
        self.encrypt_validation(key);
        let new_path = match self
                    .new_path
                    .to_str()
                    .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid path")) {
            Ok(new_path) => {new_path},
            Err(_) => {let _ = close_popup("The new vaultwyr path isnt correct", "The path created by vaultwyr is invalid");
                return Some(VaultwyrError::BadPath);},
        };

        let algo = self.algo.as_deref().unwrap_or("aes256").as_bytes();
        let mut buffer = Vec::<u8>::new();
        //let buffer = format!("{}\n{}\n{}", new_path, algo, validation);
        buffer.extend(new_path.as_bytes());
        buffer.push(b'\n');
        buffer.extend(algo);
        buffer.push(b'\n');
        
        buffer.extend(&self.validation);
        
        //let final_buffer = format!("m {} {}", buffer.len(), buffer);
        
        let mut final_buffer = Vec::<u8>::new();
        final_buffer.extend("m ".as_bytes());
        final_buffer.extend(buffer.len().to_string().as_bytes());
        final_buffer.push(b' ');
        final_buffer.extend(buffer);

        loop {
            
        
        match self.vaultwyr_file.write_all(&final_buffer) {
            Ok(_) => {break;},
            Err(_) => {
                
            match self.on_error_behaviour {
                //in case the behaviour is set to ask user you ask if the retry should be initiated otherwise it returns an error
                OnErrorBehaviour::AskUser =>{
                    match ask_terminate_retry("Error when writing to the file",
                    format!("Error when writing the header to the vaultwyr file\nMake sure that this path: \"{} is not opened by any other program\nDo you want to retry?", new_path)) {
                        Some(UserResponseTerminateRetry::Retry) => {
                            continue;
                        },
                        _ => { return Some(VaultwyrError::FileWriteError) ;},
            }},
            // if the behaviour is to terminate return the error immediately
            OnErrorBehaviour::TerminateOnError =>{
                return Some(VaultwyrError::FileWriteError);
            }


                    }
                }

        };
        }
        None
    }

    fn write_files(&mut self, key:&[u8; 32]) -> Option<VaultwyrError> {
        

        'file_loop: for file_result in match &mut self.files {
            PathType::Folder(f) => {f},
            PathType::File(f, _) => f
        } {
            let file_path = match file_result {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            

            // Skip if it is not a regular file
            if !file_path.is_file() {
                continue;
            }

            let original_path = match file_path
                            .to_str() {
                Some(s) => {s},
                None => {continue;} //continue if it is an empty string
            };

            
            
            //trying to calculate the hash
            let file_hash: String;
            loop {
                match calculate_file_hash(&file_path) {
                    Ok(hash) => {
                        file_hash = hash;
                        break; // Exit retry loop
                    }
                    Err(_) => {
                        match self.on_error_behaviour {

                            OnErrorBehaviour::AskUser => {
                                match ask_skip_retry(
                                    "Error calculating file hash",
                                    format!(
                                        "An error occurred when calculating the file hash at:\n\"{file_path:?}\"\n\
                                        Make sure this file is not being accessed by another program.\n\
                                        Do you want to skip this file or retry?"
                                    ),
                                ) {
                                    Some(UserResponseSkipRetry::Retry) | None => continue, // Retry loop
                                    Some(UserResponseSkipRetry::Skip) => continue 'file_loop, // Skip current file
                                }
                            }
                            OnErrorBehaviour::TerminateOnError => {
                                return Some(VaultwyrError::FileHashError);
                            }
                        }
                    }
                }
            }

            // Header for the file
            let mut header = format!("{}\n{}", original_path, file_hash);
            header = format!("h {} {}", header.len(), header);
            match self.vaultwyr_file.write_all(header.as_bytes()) {
                Ok(k) => {k},
                Err(_) => {
                    return Some(VaultwyrError::handle_file_write_error(&self.on_error_behaviour, "Error writing the file header", "An error occurred when writing the file header"))

                },
            };
            // add the file for later deletion
            self.paths.push(file_path.clone());
            // Read and write chunks
            let mut file = match File::open(&file_path) {
                Ok(f) => {f},
                Err(_) => {
                    return Some(VaultwyrError::handle_generic_error(&self.on_error_behaviour, "Could not open the file", "Could not open a vaultwyr file", VaultwyrError::FileOpenError))
                },
            };
            let mut buffer = vec![0; self.chunk_size.unwrap_or(2048)];

            loop {
                let bytes_read = match file.read(&mut buffer) {
                    Ok(b) => {b},
                    Err(_) => {
                        return Some(VaultwyrError::handle_generic_error(&self.on_error_behaviour, "Could not read the chunk", "Could not read from a vaultwyr file", VaultwyrError::FileOpenError));
                    },
                };
                if bytes_read == 0 {
                    break;
                }

                let mut chunk = buffer[..bytes_read].to_vec();

                match self.algo.as_deref().unwrap_or("aes256") {
                    "aes256" => {
                        chunk = match aes_encrypt_with_key(key, &chunk).map_err(|_| io::Error::new(io::ErrorKind::Other,"Error encrypting file")) {
                            Ok(c) => {c},
                            Err(_) => {
                                return Some(VaultwyrError::handle_generic_error(&self.on_error_behaviour, "Error encrypting chunk using aes256", "Could not encrypt chunk", VaultwyrError::EncryptionError));
                            },
                        };
                    }
                    _ => {
                        return Some(VaultwyrError::handle_generic_error(&self.on_error_behaviour, "Unsupported encryption algorythm", "this algorythm is not supported", VaultwyrError::NotImplemented))
                    }
                }



                fn write_chunk<W: Write>(writer: &mut W, chunk: &Vec<u8>) -> io::Result<()> {
                    let chunk_len = chunk.len().to_string();
                    writer.write_all(b"c ")?;
                    writer.write_all(chunk_len.as_bytes())?;
                    writer.write_all(b" ")?;
                    writer.write_all(chunk)?;
                    Ok(())
                }


                match write_chunk(&mut self.vaultwyr_file, &chunk) {
                    Ok(_) => {},
                    Err(_) => {return Some(VaultwyrError::handle_generic_error(&self.on_error_behaviour, "Error when writing chunks", format!("could not write the chunks of the file in the following location:\n{:?}", &file_path).as_str(), VaultwyrError::EncryptionError))},
                }
                
                
            }

            
        }

        None
    }

    fn encrypt_validation(&mut self, key:&[u8; 32]) {
        let unencrypted_vec = vec![0u8;32];

        if self.validation != unencrypted_vec {
            panic!("the vector appears to already be encrypted")
        }
        
        self.validation = aes_encrypt_with_key(key, &self.validation).expect("could not encrypt the validation");

    }

    fn clear_self(self) -> Option<VaultwyrError>{
        let mut path = self.new_path;
        match self.files {
            PathType::File(_,p) => {path = p},
            _ => {path.set_extension(""); ()}
        };

        let path_size = match calculate_dir_size(&path) {
            Ok(s) => {s},
            Err(_) => { return Some(VaultwyrError::handle_generic_error(&self.on_error_behaviour, "Error calculating the path size", "The files will not be deleted", VaultwyrError::PathSizeError))},
        };

        if path_size > self.max_size as u64{
            return Some(VaultwyrError::handle_generic_error(&self.on_error_behaviour, "The path is too large", format!("The path that you provided for deletion is too large!\n({} bytes > {} bytes)\nIf you want to be able to delete larger files after they have been encrypted please update the settings", path_size, &self.max_size).as_str(), VaultwyrError::PathSizeError))
        }
        
        for path in self.paths{
            
            loop {
                
            
            match remove_file(&path) {
                Ok(_) => {break;},
                Err(_) => {
                    match self.on_error_behaviour {
                        OnErrorBehaviour::AskUser => {
                            match ask_skip_retry("Error deleting file", format!("Couldn't delete the file at the following location:\n{:?}\nPlease make sure that the current file is not in use by other programs", &path)) {
                                Some(UserResponseSkipRetry::Retry) => {continue;},
                                Some(UserResponseSkipRetry::Skip) => {break;}
                                None => {continue;},
                            }
                        },
                        OnErrorBehaviour::TerminateOnError => {
                            break;
                        }
                    }
                },
            }
            }
        }

        None
        
        
    }


    ///Used to encrypt all the contents into the file when the file is encrypted the folder gets consumed since it shouldn't exist anymore
     pub fn encrypt_to_file(mut self, password: &str) -> Option<VaultwyrError> {
        
        let key = match password_to_key32(password) {
            Ok(key) => {key},
            Err(_) => {
                return Some(VaultwyrError::handle_generic_error(&self.on_error_behaviour, "Error converting password into key","The password you provided is invalid", VaultwyrError::BadPassword))
        }};

        match self.write_header(&key) {
            Some(e) => {return Some(e);},
            None => {},
        };
        match self.write_files(&key) {
            Some(e) => {return Some(e);},
            None => {},
        };
        
        self.clear_self()?;
        None
    }
}
