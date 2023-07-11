extern crate dirs;

use clap::{Parser, Subcommand};
use std::{fs, io};
use std::io::BufRead;
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

fn save_bookmark(key: String, url: String) -> io::Result<()> {
    init_dotfolder()?;

    if let Some(home_dir) = dirs::home_dir() {
        let mut bookmark_path = PathBuf::new();

        bookmark_path.push(home_dir);
        bookmark_path.push(".hmark");
        bookmark_path.push("bookmarks");
        bookmark_path.push(&key);

        fs::write(&bookmark_path, url)?;

        return Ok(())
    }

    Err(io::Error::new(
        io::ErrorKind::Other,
        "Home directory isn't defined",
    ))
}

fn main() {
    todo!()
}
