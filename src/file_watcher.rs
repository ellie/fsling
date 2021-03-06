use std::error::Error;
use std::sync::mpsc::{Receiver, channel};
use std::time::Duration;

use anyhow::Result;

use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};

pub trait FileEventHandler{
    fn handle(&self, event: DebouncedEvent) -> Result<()>;
}

pub struct FileWatcher {
    path: String,
    watcher: RecommendedWatcher,
    rx: Receiver<DebouncedEvent>,
}

impl FileWatcher {
    pub fn new(path: &str) -> Result<FileWatcher> {
        let (tx, rx) = channel();
        let watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(250))?;

        Ok(FileWatcher {
            path: path.to_string(),
            watcher,
            rx,
        })
    }

    pub fn watch_files(
        &mut self,
        event_handler: impl FileEventHandler,
    ) -> Result<()> {
        self.watcher
            .watch(self.path.as_str(), RecursiveMode::Recursive)?;

        loop {
            match self.rx.recv() {
                Ok(event) => event_handler.handle(event)?,
                Err(e) => {},
            }
        }
    }
}
