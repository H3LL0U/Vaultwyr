


use std::fs::{self, DirEntry, ReadDir};
use std::io;
use std::path::{Path};



/// An iterator that recursively traverses a directory and yields `Result<DirEntry, io::Error>`.
pub struct RecursiveDirIter {
    stack: Vec<ReadDir>,
}

impl RecursiveDirIter {
    /// Creates a new `RecursiveDirIter` for the specified path.
    pub fn new(path: impl AsRef<Path>) -> io::Result<Self> {
        let initial = fs::read_dir(path)?;
        Ok(Self {
            stack: vec![initial],
        })
    }
}

impl Iterator for RecursiveDirIter {
    type Item = Result<DirEntry, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(current) = self.stack.last_mut() {
            match current.next() {
                Some(Ok(entry)) => {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Ok(read_dir) = fs::read_dir(&path) {
                            self.stack.push(read_dir);
                        }
                    }
                    return Some(Ok(entry));
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

