extern crate dirs;

use clap::{Parser, Subcommand, ValueEnum};
use std::{fs, io, path};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ListType {
    Key,
    Value,
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
        value: String,
    },
    List {
        #[arg(short, long, value_enum, default_value_t = ListType::Key)]
        list: ListType,
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
    let bookmarks_path = get_bookmark_path(None)?;
    fs::create_dir_all(bookmarks_path)?;

    Ok(())
}

fn save_bookmark(key: &str, value: &str) -> io::Result<()> {
    init_bookmarks_dir()?;

    let bookmark_path = get_bookmark_path(Some(key))?;
    fs::write(&bookmark_path, value)?;

    Ok(())
}

fn get_bookmark(key: &str) -> io::Result<String> {
    init_bookmarks_dir()?;

    let bookmark_path = get_bookmark_path(Some(key))?;
    let bookmark_value = fs::read_to_string(&bookmark_path)?;

    Ok(bookmark_value)
}

fn get_all_bookmarks(list_type: &ListType) -> io::Result<Vec<Option<String>>> {
    init_bookmarks_dir()?;

    let mut bookmarks = Vec::new();

    let bookmarks_path = get_bookmark_path(None)?;
    let bookmark_files = fs::read_dir(bookmarks_path)?;

    for bookmark_file_result in bookmark_files.into_iter() {
        let bookmark_file = bookmark_file_result?;
        let bookmark = match list_type {
            ListType::Key => bookmark_file.file_name().into_string().ok(),
            ListType::Value => fs::read_to_string(bookmark_file.path()).ok(),
        };

        bookmarks.push(bookmark);
    }

    Ok(bookmarks)
}

// TODO: There should be better error handling in main(). Rather than it
// returning io::Result.

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Set { key, value }) => {
            save_bookmark(key, value)?;
            Ok(())
        },
        Some(Commands::View { key }) => {
            println!("{}", get_bookmark(key)?);
            Ok(())
        },
        Some(Commands::List { list }) => {
            get_all_bookmarks(list)?.into_iter().for_each(|bookmark_option| {
                let bookmark = match bookmark_option {
                    Some(bookmark) => bookmark,
                    None => String::from("(?)"),
                };

                println!("{}", bookmark);
            });

            Ok(())
        },
        None => Ok(()),
    }
}
