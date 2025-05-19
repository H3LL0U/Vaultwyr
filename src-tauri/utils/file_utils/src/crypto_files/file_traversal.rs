


use std::fs::{self, ReadDir};
use std::io;
use std::path::{Path, PathBuf};



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

