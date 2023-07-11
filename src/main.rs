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

fn get_bookmarks_path(bookmark_key: Option<&str>) -> io::Result<path::PathBuf> {
    match dirs::home_dir() {
        Some(home_dir) => {
            let mut bookmarks_path = path::PathBuf::new();

            bookmarks_path.push(home_dir);
            bookmarks_path.push(".hmark");
            bookmarks_path.push("bookmarks");

            if let Some(bookmark_key) = bookmark_key {
                bookmarks_path.push(bookmark_key);
            }

            Ok(bookmarks_path)
        },
        None => {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Home directory isn't defined",
            ))
        },
    }
}

fn init_bookmarks_dir() -> io::Result<()> {
    let bookmarks_path = get_bookmarks_path(None)?;
    fs::create_dir_all(bookmarks_path)?;

    Ok(())
}

fn save_bookmark(key: &str, url: &str) -> io::Result<()> {
    init_bookmarks_dir()?;

    let bookmarks_path = get_bookmarks_path(Some(key))?;
    fs::write(&bookmarks_path, url)?;

    Ok(())
}

fn get_bookmark(key: &str) -> io::Result<String> {
    init_bookmarks_dir()?;

    let bookmarks_path = get_bookmarks_path(Some(key))?;
    let bookmark_url = fs::read_to_string(&bookmarks_path)?;

    Ok(bookmark_url)
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
