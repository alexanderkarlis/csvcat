use std::io::Result;
use csvcat;

fn main() -> Result<()> {
    let args: csvcat::clap_app::ClapArgs = csvcat::clap_app::new_matches();

    let mut cat_files = csvcat::catfiles::CatFiles::new(&args).expect("could not create new cat object");
    let mut data: Vec<u8> = Vec::new();
    let cols = cat_files.get_cols_from_file_index(&mut data, 0).unwrap();

    let removed_files_index = cat_files.filter_bad_csvs((&cols).to_vec());

    let removed_files: Vec<String> = removed_files_index.iter().map(|x| (args.files[*x].to_string())).collect();
    let removed_file_string = format!("The following files have been omitted because they do not match {}'s columns: {:?}", 
       cat_files.first_file, removed_files);

    if removed_files.len() > 0 {
        println!("{}", removed_file_string);
    }

    cat_files.get_files_bodies();
    let b_vec = &cat_files.total_body.to_vec();
    let body = std::str::from_utf8(b_vec).unwrap();
    let mut outter = cat_files.get_output().unwrap();

    let write_result = outter.write(body.as_bytes());
    Ok(())
}

