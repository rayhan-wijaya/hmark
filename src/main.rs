extern crate dirs;

use clap::{Parser, Subcommand};
use std::{fs, io};
use std::path::PathBuf;

#[derive(Subcommand, Clone)]
enum Commands {
    Set {
        #[arg(short, long)]
        key: String,

        #[arg(short, long)]
        url: String,
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn init_dotfolder() -> io::Result<()> {
    if let Some(home_dir) = dirs::home_dir() {
        let mut dotfolder_path = PathBuf::new();

        dotfolder_path.push(home_dir);
        dotfolder_path.push(".hmark");

        return fs::create_dir_all(dotfolder_path)
    }

    Err(io::Error::new(
        io::ErrorKind::Other,
        "Home directory isn't defined",
    ))
}

fn save_bookmark(key: String, url: String) -> std::io::Result<()> {
    todo!()
}

fn main() {
    todo!()
}
