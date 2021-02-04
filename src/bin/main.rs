use std::env;
use std::io::{self, BufRead};
use csvcat;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        args.push(line.unwrap());
    }
    args.remove(0);
    println!("{:#?}", args);

    let mut cat_files = csvcat::CatFiles::new(&args).expect("could not create new cat object");
    let mut data: Vec<u8> = Vec::new();
    let cols = cat_files.get_cols_from_first_file(&mut data).unwrap();

    let removed_files_index = cat_files.check_equal_cols((&cols).to_vec());

    let removed_files: Vec<&String> = removed_files_index.iter().map(|x| (&args[*x])).collect();
    let first_file = args.get(0).unwrap();
    let removed_file_string = format!("The following files do not match {}'s columns: {:?}", 
       first_file, removed_files);
    println!("{}", removed_file_string);
}
