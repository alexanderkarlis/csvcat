extern crate clap;
use clap::{App, Arg, ArgMatches};

#[derive(Debug)]
pub struct ClapArgs{
    pub files: Vec<String>,
    pub outfile: String,
}

impl From<ArgMatches> for ClapArgs {
    fn from(m: ArgMatches) -> Self {
        ClapArgs {
            files: m.values_of("files").unwrap().map(|x| x.to_owned().to_string()).collect(),
            outfile: m.value_of("outfile").unwrap_or_else(|| &"").to_string(),
        }
    }
}

pub fn new_matches() -> ClapArgs {
    let matches = App::new("csvcat")
        .version("0.1")
        .about("takes in a list of csv's and concats them together")
        .arg(Arg::new("files")
            .long("files")
            .short('f')
            .takes_value(true)
            .multiple(true)
            .about("can use `ls` piped to `xargs` as files arg"))
        .arg(Arg::new("outfile")
            .long("outfile")
            .short('o')
            .takes_value(true))
        .get_matches();
    ClapArgs::from(matches)
}
