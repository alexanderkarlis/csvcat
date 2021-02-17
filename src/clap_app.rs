extern crate clap;
use clap::{App, Arg, ArgMatches};

#[derive(Debug)]
pub struct ClapArgs {
    pub files: Vec<String>,
    pub outfile: String,
    pub use_file_index: usize,
}

impl From<ArgMatches> for ClapArgs {
    fn from(m: ArgMatches) -> Self {
        let incoming_files = m
            .values_of("files")
            .unwrap()
            .map(|x| x.to_owned().to_string())
            .collect::<Vec<_>>();
        let incoming_file_name = m.value_of("use-file-name").unwrap_or_else(|| "");
        let incoming_file_index = m.value_of("use-file-index").unwrap_or_else(|| "");

        let mut use_file = 0 as usize;
        if incoming_file_index != "" {
            use_file = incoming_file_index.parse::<usize>().unwrap();
        } else if incoming_file_name != "" {
            let files_clone = incoming_files.clone();
            use_file = files_clone.into_iter().position(|x| x == incoming_file_name).unwrap();
        };
        Self {
            files: incoming_files,
            outfile: m.value_of("outfile").unwrap_or_else(|| &"").to_string(),
            use_file_index: use_file,
        }
    }
}

pub fn new_matches() -> ClapArgs {
    let matches = App::new("csvcat")
        .version("0.1")
        .about("takes in a list of csv's and concats them together.\n\nExample: ls ../../rc/address-splitter/CMS_SPLIT/*split*/part*.csv | xargs cargo run -- -o cms.csv --files")
        .arg(
            Arg::new("files")
                .long("files")
                .short('f')
                .takes_value(true)
                .multiple(true)
                .about("can use `ls` piped to `xargs` as files arg"),
        )
        .arg(
            Arg::new("outfile")
                .long("outfile")
                .short('o')
                .takes_value(true)
                .about("out-file name. Can be file or STDOUT"),
        )
        .arg(
            Arg::new("use-file-index")
                .long("use_file_index")
                .short('i')
                .takes_value(true)
                .conflicts_with("use-file-name")
                .about("the index of file to for the default columns"),
        )
        .arg(
            Arg::new("use-file-name")
                .long("use_file_name")
                .short('n')
                .takes_value(true)
                .conflicts_with("use-file-index")
                .about("the name of file to for the default columns"),
        )
        .get_matches();
    ClapArgs::from(matches)
}
