use csvcat;

fn main() -> std::io::Result<()> {
    let args = csvcat::clap_app::new_matches();
    let boxed_args = Box::new(args);
    let arg: &'static csvcat::clap_app::ClapArgs = Box::leak(boxed_args);
    let mut cat_files = csvcat::catfiles::CatFiles::new(arg).unwrap();
    let mut data: Vec<u8> = Vec::new();
    let _ = cat_files.get_cols_from_file_index(&mut data).unwrap();

    cat_files.check_files_concurrently();
    Ok(())
}
