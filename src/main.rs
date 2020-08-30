use std::error::Error;

use clap::clap_app;
use notify::DebouncedEvent;

mod filewatcher;

use filewatcher::FileWatcher;

fn event_handler(event: DebouncedEvent) {
    println!("{:?}", event);
}

fn main() -> Result<(), Box<dyn Error>>{
    let matches = clap_app!(fsling =>
        (version: "0.0.1")
        (author: "Ellie Huxtable <e@elm.sh>")
        (about: "fsling - File Sling")
        (@arg config: -c --config +takes_value "Sets a custom config file")
        (@arg path: +required "Set the directory to watch")
    )
    .get_matches();

    // Safe to unwrap as this is required above
    let p = matches.value_of("path").unwrap();

    let mut watcher = FileWatcher::new(p)?;

    watcher.watch_files(event_handler)?;

    Ok(())
}
