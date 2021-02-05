use crate::Info;

use clap::{Arg, App};

pub fn get_info_from_cli() -> Info {
    let matches = App::new("TFT Search Program")
        .version("0.1.0")
        .author("Tom Chiu <tomchiu@argo.ai>")
        .about("Simple program to look up TFT summoner's history")
        .arg(Arg::with_name("name")
        .short("n")
        .long("name")
        .takes_value(true)
        .required(true)
        .help("Summoner's name to search"))
        .get_matches();
    let summoner_name = matches.value_of("name").unwrap_or_else(|| {"SSomeGuyy"});

    Info::new(summoner_name.to_string())
}
