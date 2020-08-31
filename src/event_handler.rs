use std::env;
use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::Values;
use notify::DebouncedEvent;
use regex::Regex;
use ssh2::Session;
use globset::{Glob, GlobMatcher};

pub struct EventHandler {
    session: Session,
    globs: Vec<GlobMatcher>,
}

impl EventHandler {
    pub fn new(excludes: Option<Values>) -> Option<EventHandler> {
        let globs: Vec<GlobMatcher> = excludes.unwrap_or(Values::default())
                    .map(|v|{
                        Glob::new(v).unwrap().compile_matcher()
                    })
                    .collect();

        let tcp = TcpStream::connect("ellie.tv:22").unwrap();
        let mut session = Session::new().unwrap();

        session.set_tcp_stream(tcp);
        session.handshake().unwrap();
        session.userauth_agent("ellie").unwrap();

        Some(EventHandler { session, globs })
    }

    fn ignore(&self, path: &str) -> bool {
        self.globs.iter().map(|g| g.is_match(path)).any(|x|x)
    }
}

impl crate::FileEventHandler for EventHandler {
    fn handle(&self, event: DebouncedEvent) -> Result<()> {
        let path = match event {
            DebouncedEvent::Write(p) => p,
            DebouncedEvent::Create(p) => p,
            DebouncedEvent::Remove(p) => p,

            _ => {
                return Ok(());
            }
        };

        let path = match fs::canonicalize(path.clone()) {
            Ok(path) => path,
            Err(_) => path,
        };

        let current = env::current_dir().unwrap();

        let path = path.strip_prefix(env::current_dir()?)?;
        let path = path.to_str().unwrap();

        match self.ignore(path) {
            true => {
                return Ok(());
            }

            false => {
                println!("{:?}", path);
            }
        }

        let mut code = PathBuf::new();
        code.push("/home/ellie/src");
        code.push(path);

        println!("sending: {:?}", code);

        let mut remote_file = self
            .session
            .scp_send(code.as_path(), 0o644, 10, None)
            .unwrap();
        remote_file.write(b"1234567890").unwrap();

        Ok(())
    }
}
