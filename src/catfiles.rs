use crate::clap_app;

use std::fs;
use std::io::{self, Write};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CatFiles<'a> {
    /// file_contents is a hash-map of usize and Vec<u8> where key 
    /// is the order in which the files were passed in through the 
    /// command line.
    pub file_contents: HashMap<usize, Vec<u8>>,

    /// start_body_index is the index where the first '\n' is to 
    /// deliniate the headers vs the body of the csv.
    pub start_body_index: usize,

    /// total_body is all the bodies of the input csvs formed into
    /// one `Vec<u8>`
    pub total_body: Vec<u8>,

    /// Outfile is the file that gets written out to.
    pub output: OutFile<'a>,

    /// TODO: add functionality to override this from cli.
    /// first file passed into cli 
    pub first_file: String,
}

#[derive(Debug)]
pub enum OutFile<'a> {
    Stdout(&'a str),
    FileOut(PathBuf),
}

impl<'a> CatFiles<'a>{
    pub fn new(args: &clap_app::ClapArgs) -> Result<CatFiles, io::Error> {
        let mut map: HashMap<usize, Vec<u8>> = HashMap::new();
        for (i, file) in args.files.iter().enumerate() {
            let contents: Vec<u8> = fs::read(file).expect(&format!("could not read file {}", file));
            map.insert(i, contents);
        };

        let mut out_arg: OutFile = OutFile::FileOut(PathBuf::from(&args.outfile));
        if args.outfile == "" || args.outfile.to_uppercase() == "STDOUT" {
            out_arg = OutFile::Stdout(&args.outfile);
        };

        Ok(CatFiles{
            file_contents: map,
            start_body_index: 0,
            total_body: Vec::new(),
            output: out_arg,
            first_file: args.files.get(0).unwrap().to_string(),
        }) 
    }

    pub fn get_cols_from_first_file(
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
    
    /// Internal tool for quick converting Vec<u8> to string. Useful for
    /// reading from files.
    fn u8_array_to_string(&self, array: &'a Vec<u8>) -> &'a str {
        std::str::from_utf8(&array).unwrap()
    }

    /// This function is called after `filter_bad_csvs` in order to remove
    /// all the csv files who's headers do not match our desired col headers 
    /// from the first file. Does not sort the results.
    ///
    /// # Arugments
    ///
    /// * none
    ///
    /// # Returns 
    /// 
    /// * &str - all the bodies appended into one big csv. 
    ///
    pub fn get_files_bodies<'b>(&mut self) {
        let first = &self.file_contents.get(&0).unwrap();
        for byte in &mut first[..&self.start_body_index+1].iter() {
            &self.total_body.push(*byte);
        }
        for file_map_index in self.file_contents.keys() {
            let good_bytes = &self.file_contents
                .get(&file_map_index)
                .unwrap()
                .get(&self.start_body_index+1..)
                .unwrap();
            for b in good_bytes.iter() {
                &self.total_body.push(*b);
            }
        }
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
    /// * Vec<u8> - A vec of indexes that do not match the passed in cols. 
    ///
    pub fn filter_bad_csvs(&mut self, cols: Vec<u8>) -> Vec<usize> {
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

    /// Writes the contents to which ever stream (file or stdout) is specified by 
    /// the user.
    ///
    /// # Arguments
    /// 
    /// * `string` - String argument to be written
    ///
    /// # Returns 
    ///
    /// * io::Result<()>, based on if the write was successful
    ///
    pub fn get_output(&self) -> Result<Box<dyn Write>, io::Error> {
        todo!()
        // if self.output == String::from("STDOUT")
        //     Ok(Box::new(io::stdout()))
        // }
        // fs::File::create(self.output as std::path::Path).map(|f| Box::new(f) as Box<dyn Write>)
        // match self.output {
        //     Some(ref path) => 
        //     None => 
        // }
    }
    // 
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
