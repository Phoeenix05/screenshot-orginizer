use std::path::PathBuf;

use clap::{arg, Parser};
use colored::*;
use notify::{Error, Event, RecursiveMode::NonRecursive, Watcher};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of the folder this program watches for changes.
    /// Default path is $HOME/Screenshots
    #[arg(long)]
    folder: Option<String>,
}

fn get_path(folder: Option<String>) -> PathBuf {
    if folder.is_some() {
        PathBuf::from(folder.unwrap())
    } else {
        use dirs::home_dir;

        let dir = if let Some(pb) = home_dir() {
            pb
        } else {
            let msg = "Home directory couldn't be determined".bold().red();
            println!("{}", msg);
            std::process::exit(0);
        };

        PathBuf::from(dir)
    }
}

fn event_handler(res: Result<Event, Error>) {
    match res {
        Ok(event) => println!("event: {:?}", event),
        Err(e) => println!("watch error: {:?}", e),
    }
}

fn main() -> ! {
    let args = Args::parse();
    // println!("{:#?}", args);

    // Get folder path to watch for changes
    let path = get_path(args.folder);

    // Validate that the path is a folder
    if !path.is_dir() {
        let msg = "Path given is a file not a directory".bold().red();
        println!("{}", msg);
        std::process::exit(0);
    }
    
    // Get absolute path for relative paths. Doesn't do anything if path is already a absolute path.
    let absolute_path = path.canonicalize().unwrap();
    let msg = format!("Watching path {:?}", absolute_path).bold();
    println!("{}", msg);

    // Create watcher
    let mut watcher = notify::recommended_watcher(event_handler).unwrap();
    watcher.watch(&absolute_path, NonRecursive).unwrap();

    // Keep the program running indefinetely
    loop {}
}
