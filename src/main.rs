extern crate dirs;

use clap::{command, Arg, ArgMatches};
use std::path::PathBuf;

fn get_matches() -> ArgMatches {
    let set_arg = Arg::new("set")
        .short('s')
        .long("set")
        .value_name("key=url")
        .help("Sets a bookmark using a provided key, and url.");

    command!()
        .arg(set_arg)
        .get_matches()
}

fn init_dotfolder() -> std::io::Result<()> {
    if let Some(home_dir) = dirs::home_dir() {
        let mut dotfolder_path = PathBuf::new();

        dotfolder_path.push(home_dir);
        dotfolder_path.push(".hmark");

        return std::fs::create_dir_all(dotfolder_path)
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Home directory isn't defined",
    ))
}

fn save_bookmark(key: String, url: String) -> std::io::Result<()> {
    todo!()
}

fn main() {
    todo!()
}
