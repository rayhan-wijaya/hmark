extern crate dirs;

use clap::{Parser, Subcommand};
use std::{fs, io};
use std::io::{BufRead, BufReader};
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

fn is_bookmark_exist(key: String, reader: io::BufReader<fs::File>) -> bool {
    todo!()
}

fn save_bookmark(key: String, url: String) -> io::Result<()> {
    init_dotfolder()?;

    if let Some(home_dir) = dirs::home_dir() {
        let mut bookmarks_path = PathBuf::new();

        bookmarks_path.push(home_dir);
        bookmarks_path.push(".hmark");
        bookmarks_path.push("bookmarks");

        // Actual Writing
        //
        // fs::write(bookmarks_path, "...")?;

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
