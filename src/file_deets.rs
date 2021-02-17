use std::vec::Vec;
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, Clone)]
pub struct CatFileDetails {
    /// file index in order passed in to program
    pub f_index: usize,

    /// file directory
    pub f_dir: PathBuf,
}
#[derive(Debug, Clone)]
pub struct AllCatFiles {
    pub files: Vec<CatFileDetails>,
}

pub struct Inspector<'a> {
    pub iter: std::slice::Iter<'a, CatFileDetails>,
}

impl<'a> AllCatFiles {
    pub fn new(fs: Vec<CatFileDetails>) -> AllCatFiles {
        AllCatFiles { files: fs }
    }

    pub fn values(&self) -> Inspector {
        Inspector {
            iter: self.files.iter(),
        }
    }
}

impl<'a> Iterator for Inspector<'a> {
    type Item = &'a CatFileDetails;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|cf| cf)
    }
}

impl CatFileDetails {
    pub fn new(i: usize, dir: String) -> Self {
        Self {
            f_index: i,
            f_dir: PathBuf::from_str(&dir).unwrap(),
        }
    }
}
