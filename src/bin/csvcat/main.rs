use std::io::Result;

use csvcat;

fn main() -> Result<()> {
    let args: csvcat::clap_app::ClapArgs = csvcat::clap_app::new_matches();
    println!("{:#?}", args);

    let mut cat_files = csvcat::catfiles::CatFiles::new(&args).expect("could not create new cat object");
    let mut data: Vec<u8> = Vec::new();
    let cols = cat_files.get_cols_from_first_file(&mut data).unwrap();

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
    println!("{}", body);
    let mut outter = cat_files.get_output().unwrap();
    // l{et mut out_opt = match cat_files.get_output() {
    //     Ok(n) => n,
    //     Err(_) =>  
    // };

    let write_result = outter.write(b"hello world");
    println!("{:#?}", write_result);
    // match out_opt {
    //     Ok(mut n) => n.write(b"hello world").unwrap(),
    //     Err(_) => {
    //         println!("{:#?}", "error occured on write"); 
    //         0 as usize
    //     },
    // };
    Ok(())
}

