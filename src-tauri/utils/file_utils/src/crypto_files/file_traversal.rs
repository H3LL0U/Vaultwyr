


use std::fs::{self, File, ReadDir};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use sha2::{Sha256, Digest};


/// An iterator that recursively traverses a directory and yields `Result<PathBuf, io::Error>`.


enum PathType{
    Folder(Vec<ReadDir>),
    File(Option<PathBuf>)
}

impl PathType{
    fn push(&mut self, read_dir:ReadDir){
        match self {
            PathType::Folder(a) => {a.push(read_dir);},
            _ => return (),
        }
    }

    fn pop(&mut self) -> Option<ReadDir>{
        match self {
            PathType::Folder(f) => {f.pop()},
            _ => None
        }
    }


}

pub struct RecursiveDirIter {
    stack: PathType,
}

impl RecursiveDirIter {
    /// Creates a new `RecursiveDirIter` for the specified path.
    pub fn new(path: impl AsRef<Path>) -> io::Result<Self> {
        
        if path.as_ref().is_file(){
            return Ok(Self {
                stack: PathType::File(Some(path.as_ref().to_path_buf()))
            })
        }

        let initial = fs::read_dir(path)?;
        Ok(Self {
            stack: PathType::Folder(vec![initial]),
        })
    }
}

impl Iterator for RecursiveDirIter {
    type Item = Result<PathBuf, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(current) = match &mut self.stack {
            PathType::File(f) => {return f.take().and_then(|f: PathBuf| Some(Ok(f)))},
            PathType::Folder(f) => f
        }
        .last_mut() {
            match current.next() {
                Some(Ok(entry)) => {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Ok(read_dir) = fs::read_dir(&path) {
                            self.stack.push(read_dir);
                        }
                    }
                    return Some(Ok(entry.path()));
                }
                Some(Err(err)) => return Some(Err(err)),
                None => {
                    // Remove empty iterator
                    self.stack.pop();
                }
            }
        }
        None
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

pub fn calculate_dir_size(path: &Path) -> io::Result<u64> {
    let mut total_size = 0;

    if path.is_dir() {
        // Iterate through the directory
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                // Recurse into subdirectories
                total_size += calculate_dir_size(&entry_path)?;
            } else {
                // Add file size
                total_size += entry.metadata()?.len();
            }
        }

    }
    else{
        total_size += fs::metadata(path)?.len();
    }
    Ok(total_size)
}