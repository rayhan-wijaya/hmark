use std::path::Path;
use clap::{command, Arg, ArgMatches};

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
    let dotfolder_path = Path::new("~/.hmark");

    if dotfolder_path.exists() {
        return Ok(());
    }

    std::fs::create_dir_all(dotfolder_path)
}

fn save_bookmark(key: String, url: String) -> std::io::Result<()> {
    todo!()
}

fn main() {
    todo!();
}
