use csvcat;
use std::io::Result;

fn main() -> Result<()> {
    let args = csvcat::clap_app::new_matches();
    let boxed_args = Box::new(args);
    let arg: &'static csvcat::clap_app::ClapArgs = Box::leak(boxed_args);
    let mut cat_files = csvcat::catfiles::CatFiles::new(arg).unwrap();
    let mut data: Vec<u8> = Vec::new();
    let cols = cat_files.get_cols_from_file_index(&mut data, 0).unwrap();

    let removed_files_index = cat_files.filter_bad_csvs((&cols).to_vec());

    let removed_files: Vec<String> = removed_files_index
        .iter()
        .map(|x| (arg.files[*x].to_string()))
        .collect();
    let removed_file_string = format!(
        "The following files have been omitted because they do not match {}'s columns: {:?}",
        cat_files.first_file, removed_files
    );

    // if removed_files.len() > 0 {
    //     println!("{}", removed_file_string);
    // }

    // cat_files.get_files_bodies();
    let b_vec = &cat_files.total_body.to_vec();
    let body = std::str::from_utf8(b_vec).unwrap();

    // let mut outter = cat_files.get_output_type().unwrap();
    // let _ = outter.write(body.as_bytes());

    cat_files.check_files_concurrently();
    Ok(())
}
