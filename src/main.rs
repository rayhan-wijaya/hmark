extern crate dirs;

use clap::{Parser, Subcommand};
use std::{fs, io, path};

#[derive(Subcommand, Clone)]
enum Commands {
    View {
        #[arg(short, long)]
        key: String,
    },
    Set {
        #[arg(short, long)]
        key: String,

        #[arg(short, long)]
        url: String,
    },
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn init_dotfolder() -> io::Result<()> {
    if let Some(home_dir) = dirs::home_dir() {
        let mut dotfolder_path = path::PathBuf::new();

        dotfolder_path.push(home_dir);
        dotfolder_path.push(".hmark");
        dotfolder_path.push("bookmarks");

        return fs::create_dir_all(dotfolder_path)
    }

    Err(io::Error::new(
        io::ErrorKind::Other,
        "Home directory isn't defined",
    ))
}

fn save_bookmark(key: &str, url: &str) -> io::Result<()> {
    init_dotfolder()?;

    if let Some(home_dir) = dirs::home_dir() {
        let mut bookmark_path = path::PathBuf::new();

        bookmark_path.push(home_dir);
        bookmark_path.push(".hmark");
        bookmark_path.push("bookmarks");
        bookmark_path.push(key);

        fs::write(&bookmark_path, url)?;

        return Ok(())
    }

    Err(io::Error::new(
        io::ErrorKind::Other,
        "Home directory isn't defined",
    ))
}

// TODO: There should be better error handling in main(). Rather than it
// returning io::Result.

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Set { key, url }) => save_bookmark(key, url),
        None => Ok(()),
    }
}
