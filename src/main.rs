use std::error::Error;

use clap::clap_app;

mod file_watcher;
mod event_handler;

use file_watcher::{FileWatcher, FileEventHandler};
use event_handler::EventHandler;

fn main() -> Result<(), Box<dyn Error>>{
    let matches = clap_app!(fsling =>
        (version: "0.0.1")
        (author: "Ellie Huxtable <e@elm.sh>")
        (about: "fsling - File Sling")

        (@arg exclude: -e --exclude +takes_value +multiple "Regex for paths to ignore")
        (@arg host: -h --host +takes_value +required "Set the remote host for sync - HOST:PORT:/the/path")

        (@arg path: +required "Set the directory to watch")
    )
    .get_matches();



    // Safe to unwrap as this is required above
    let p = matches.value_of("path").unwrap();
    let h = matches.value_of("host").unwrap();

    let excludes = matches.values_of("exclude");

    let mut watcher = FileWatcher::new(p)?;
    let handler = EventHandler::new(h, excludes).unwrap();

    watcher.watch_files(handler)?;

    Ok(())
}
