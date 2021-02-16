use crate::clap_app;

use crossbeam;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
// use std::ops::{Deref, DerefMut};
use std::{thread, time::Duration};

use crate::file_deets;

#[derive(Debug, Clone)]
pub struct CatFiles {
    /// file_contents is a hash-map of usize and Vec<u8> where key
    /// is the order in which the files were passed in through the
    /// command line.
    pub file_contents: HashMap<usize, Vec<u8>>,

    /// all files
    pub all_files: file_deets::AllCatFiles,

    /// start_body_index is the index where the first '\n' is to
    /// deliniate the headers vs the body of the csv.
    pub start_body_index: usize,

    /// use_cols is the vec of u8 of the columns to check the
    /// rest of the files with.
    pub use_cols: Vec<u8>,

    /// total_body is all the bodies of the input csvs formed into
    /// one `Vec<u8>`
    pub total_body: Vec<u8>,

    /// Outfile is the file that gets written out to.
    pub output: OutFileType,

    /// TODO: add functionality to override this from cli.
    /// first file passed into cli
    pub first_file: String,
}

#[derive(Debug, Clone)]
pub enum OutFileType {
    Stdout(String),
    FileOut(PathBuf),
}

impl<'a> CatFiles {
    /// New function creates a new instance of CatFiles. Takes an argument of ClapArgs and
    /// this function checks and sees which are active and which aren't.
    ///
    /// # Arguments
    ///
    /// * `args`: type ClapArgs
    ///
    /// # Returns
    ///
    /// * `Result<CatFiles, Err>`, returns error if it cannot read file contents.
    ///
    pub fn new(args: &clap_app::ClapArgs) -> Result<CatFiles, io::Error> {
        let map: HashMap<usize, Vec<u8>> = HashMap::new();

        let veccy: Vec<file_deets::CatFileDetails> = args
            .files
            .iter()
            .enumerate()
            .map(|(i, file)| file_deets::CatFileDetails::new(i, (*file).to_string()).clone())
            .collect();

        let af = file_deets::AllCatFiles::new(veccy);
        let mut out_arg: OutFileType = OutFileType::FileOut(PathBuf::from(&args.outfile));
        let cloned_str = { args.outfile.clone() };
        if &args.outfile == &"" || &args.outfile.to_uppercase() == "STDOUT" {
            out_arg = OutFileType::Stdout(cloned_str);
        };
        // let first_file = args.clone();
        Ok(CatFiles {
            file_contents: map,
            all_files: af,
            start_body_index: 0,
            use_cols: Vec::new(),
            total_body: Vec::new(),
            output: out_arg,
            first_file: args.files.get(0).unwrap().to_string(),
        })
    }

    /// Gets the columns from the first file to check against other file's cols
    ///
    /// # Arguments
    ///
    /// * Vec<u8> is the vector of u8's the columns with be added to.
    ///
    /// * usize is the location of the file that is passed into the program (starts
    /// at 0 index).
    ///
    /// # Returns
    ///
    /// * Result<Vec<u8>, Utf8Error> is the vector that is passed into this function and
    /// therefore filled with the columns.
    pub fn get_cols_from_file_index(
        &mut self,
        col_vec: &'a mut Vec<u8>,
        index: usize,
    ) -> Result<&'a Vec<u8>, std::str::Utf8Error> {
        let selected_file_details = self.all_files.files.get(index).unwrap();
        let contents: Vec<u8> = fs::read(&selected_file_details.f_dir).expect(&format!(
            "could not read file {:#?}",
            selected_file_details.f_dir.to_str()
        ));

        for (i, b) in &mut contents.iter().enumerate() {
            if *b == b'\n' {
                self.start_body_index = i as usize;
                break;
            }
            col_vec.push(*b);
        }
        self.use_cols = col_vec.to_vec();
        self.total_body.append(&mut col_vec.to_vec());
        let f_str = std::str::from_utf8(&self.use_cols).unwrap();
        println!("columns::: {:#?}", f_str);
        Ok(col_vec)
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
    pub fn get_files_bodies(&mut self) {
        let first = &self.file_contents.get(&0).unwrap();
        for byte in &mut first[..&self.start_body_index + 1].iter() {
            &self.total_body.push(*byte);
        }
        for file_map_index in self.file_contents.keys() {
            let good_bytes = &self
                .file_contents
                .get(&file_map_index)
                .unwrap()
                .get(&self.start_body_index + 1..)
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
    /// * Self
    ///
    /// # Returns
    ///
    /// * io::Result<()>, based on if the write was successful
    /// * Result<Box<dyn Write>, io::Error> where the body can be written to either stdout or file
    ///
    pub fn get_output_type(&self) -> Result<Box<dyn Write>, io::Error> {
        match &self.output {
            OutFileType::Stdout(_) => Ok(Box::new(io::stdout()) as Box<dyn Write>),
            OutFileType::FileOut(filename) => Ok(Box::new(
                std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(filename)
                    .unwrap(),
            )),
        }
    }

    /// check files column heaers to see if they match up with the desired ones.
    ///
    /// # Arguments
    ///
    /// * i -> index of usize, get this index of files passed into the program
    ///
    /// # Returns
    ///
    /// * none
    ///
    fn are_file_headers_are_ok(&self, i: usize) -> bool {
        let selected_file_details = self.all_files.files.get(i).unwrap();
        let contents: Vec<u8> = fs::read(&selected_file_details.f_dir).expect(&format!(
            "could not read file {:#?}",
            selected_file_details.f_dir.to_str()
        ));
        contents[0..self.start_body_index] == self.use_cols[..]
    }

    /// Concurrently reads files and checks the columns. If the columns do not match
    /// the control file's columns, it is ignored.
    pub fn check_files_concurrently(&mut self) {
        let local_self = &self.clone();
        let body_all = Arc::new(Mutex::new(self));
        let mut outter = local_self.get_output_type().unwrap();
        outter.write(&local_self.use_cols).unwrap();
        // outter.write(&['\n' as u8]).unwrap();
        crossbeam::scope(|thread_scope| {
            for (idx, file) in local_self.all_files.files.iter().enumerate() {
                let data_mutex_clone = Arc::clone(&body_all);
                thread_scope.spawn(move |_| {
                    let data = data_mutex_clone.lock().unwrap();
                    let is_ok = local_self.are_file_headers_are_ok(idx);
                    if is_ok {
                        let mut contents: Vec<u8> = fs::read(&file.f_dir).unwrap();
                        contents = contents[data.start_body_index..contents.len() -1].to_vec();

                        let f_str = std::str::from_utf8(&contents).unwrap();

                        println!("{:#?}", &f_str);
                        let mut outter = data.get_output_type().unwrap();
                        let _ = outter.write(&contents);
                    }
                });
            }
        })
        .expect("A child thread panicked");
        // let unlocked = &body_all.lock().unwrap().total_body;
        // let f_str = std::str::from_utf8(&unlocked).unwrap();
        // println!("{:#?}", &f_str);
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
    }
}
