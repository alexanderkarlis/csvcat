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
    println!("hello clap app");
    let matches = App::new("csvcat")
        .version("1.0")
        .about("Does awesome things")
        .arg(Arg::new("files")
            .long("files")
            .short('f')
            .takes_value(true)
            .multiple(true)
            .about("hello world!"))
        .arg(Arg::new("outfile")
            .long("outfile")
            .short('o')
            .takes_value(true))
        .get_matches();
    ClapArgs::from(matches)
}
