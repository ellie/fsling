use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::sync::mpsc::{Receiver, channel};
use std::time::Duration;

pub struct FileWatcher {
    path: String,
    watcher: RecommendedWatcher,
    rx: Receiver<DebouncedEvent>,
}

impl FileWatcher {
    pub fn new(path: &str) -> Result<FileWatcher> {
        let (tx, rx) = channel();
        let watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

        Ok(FileWatcher {
            path: path.to_string(),
            watcher,
            rx,
        })
    }

    pub fn watch_files(
        &mut self,
        event_handler: impl Fn(DebouncedEvent),
    ) -> Result<()> {
        self.watcher
            .watch(self.path.as_str(), RecursiveMode::Recursive)?;

        loop {
            match self.rx.recv() {
                Ok(event) => event_handler(event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    }
}
