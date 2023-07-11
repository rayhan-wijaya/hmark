extern crate dirs;

use clap::{Parser, Subcommand, ValueEnum};
use std::{fs, io, path};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ListType {
    Key,
    Url,
}

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

fn get_bookmark_path(bookmark_key: Option<&str>) -> io::Result<path::PathBuf> {
    match dirs::home_dir() {
        Some(home_dir) => {
            let mut bookmark_path = path::PathBuf::new();

            bookmark_path.push(home_dir);
            bookmark_path.push(".hmark");
            bookmark_path.push("bookmarks");

            if let Some(bookmark_key) = bookmark_key {
                bookmark_path.push(bookmark_key);
            }

            Ok(bookmark_path)
        },
        None => {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Home directory isn't defined",
            ))
        },
    }
}

fn init_dotfolder() -> io::Result<()> {
    let dotfolder_path = get_bookmark_path(None)?;
    fs::create_dir_all(dotfolder_path)
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

fn get_bookmark(key: &str) -> io::Result<String> {
    init_dotfolder()?;

    if let Some(home_dir) = dirs::home_dir() {
        let mut bookmark_path = path::PathBuf::new();

        bookmark_path.push(home_dir);
        bookmark_path.push(".hmark");
        bookmark_path.push("bookmarks");
        bookmark_path.push(key);

        let bookmark_url = fs::read_to_string(&bookmark_path)?;

        return Ok(bookmark_url);
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
        Some(Commands::View { key }) => {
            println!("{}", get_bookmark(key)?);
            Ok(())
        },
        None => Ok(()),
    }
}
