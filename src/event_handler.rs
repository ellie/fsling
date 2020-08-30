use clap::Values;
use regex::Regex;

use notify::DebouncedEvent;

pub struct EventHandler {
    excludes: Vec<Regex>,
}

impl EventHandler {
    pub fn new(excludes: Option<Values>) -> Option<EventHandler> {
        let excludes: Vec<Regex> = excludes
            .unwrap_or(Values::default())
            .map(|i| Regex::new(i).unwrap())
            .collect();

        Some(EventHandler { excludes })
    }

    fn ignore(&self, path: &str) -> bool {
        self.excludes.iter().map(|r| r.is_match(path)).any(|x| x)
    }
}

impl crate::FileEventHandler for EventHandler {
    fn handle(&self, event: DebouncedEvent) {
        let path = match event {
            DebouncedEvent::Write(p) => p,
            DebouncedEvent::Create(p) => p,
            DebouncedEvent::Remove(p) => p,

            _ => {
                return;
            }
        };

        let path = match path.into_os_string().into_string() {
            Ok(p) => p,
            Err(_) => {
                return;
            }
        };

        match self.ignore(path.as_str()) {
            true => {}
            false => println!("{:?}", path),
        }
    }
}
