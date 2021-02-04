use std::fs;
use std::io;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CatFiles {
    pub file1: String,
    pub file2: String,

    pub file_contents: HashMap<usize, Vec<u8>>,

    pub start_body_index: usize,
}

impl CatFiles {
    pub fn new(files: &Vec<String>) -> Result<CatFiles, io::Error> {
        let mut map: HashMap<usize, Vec<u8>> = HashMap::new();
        for (i, file) in files.iter().enumerate() {
            let contents: Vec<u8> = fs::read(file).expect(&format!("could not read file {}", file));
            map.insert(i, contents);
        }
        Ok(CatFiles{
            file1: "file1".to_string(),
            file2: "file2".to_string(),
            file_contents: map,
            start_body_index: 0,
        }) 
    }

    pub fn get_cols_from_first_file<'a>(
            &mut self, 
            col_vec: &'a mut Vec<u8>
        ) -> Result<&'a Vec<u8>, std::str::Utf8Error> {
        
        let fc: Vec<u8> = (&self.file_contents.get(&0).unwrap()).to_vec();

        for (i, b) in fc.iter().enumerate() {
            if *b as char == '\n' {
                self.start_body_index = i as usize; 
                break;
            }
            col_vec.push(*b);
        }
        Ok(col_vec)
    }

    pub fn get_body<'a>(&self, file_number: usize) -> String {
        // let body: &str;
        // if file_number == 1 {
        //     body = std::str::from_utf8(&self.file_content1).expect("read bytes error");
        // } else {
        //     body = std::str::from_utf8(&self.file_content2).expect("read bytes error");
        // }
        // println!("{}", self.start_body_index);
        // body[self.start_body_index as usize..].to_string()
        todo!();
    }

    /// Removes any csv files whose headers do not match exactly the headers
    /// of the first passed in file. This is the ideal case and that of the cli
    /// version; however, this can be use for any csv headers programmatically. 
    ///
    /// # Arguments 
    ///
    /// * `cols` - Vec of u8 bytes (presumably the col headers to check against)
    /// 
    /// # Returns 
    /// 
    /// * Vec<u8> A vec of indexes that do not match the passed in cols. 
    ///
    pub fn check_equal_cols<'a>(mut self, cols: Vec<u8>) -> Vec<usize> {
        let index = self.start_body_index;
        let mut removed_files: Vec<usize> = Vec::new();

        self.file_contents.retain(|key, file| {
            if !(cols == &file[0..index]) {
                removed_files.push(*key);
            }
            cols == &file[0..index]
        });

        removed_files
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_files_and_get_content() {
        let mut files: Vec<String> = Vec::new();
        files.push("t1.csv".to_string());
        files.push("t2.csv".to_string());

        let mut cat_files = CatFiles::new(&files).expect("could not create new cat object");
        let mut data: Vec<u8> = Vec::new();
        let cols = cat_files.get_cols_from_first_file(&mut data).unwrap();
        println!("{:#?}", cols);
    }
}
