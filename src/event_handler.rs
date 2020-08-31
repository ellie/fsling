use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::PathBuf;

use anyhow::{Result, Error};
use clap::Values;
use globset::{Glob, GlobMatcher};
use notify::DebouncedEvent;
use ssh2::Session;

pub struct EventHandler {
    remote: String,
    session: Session,
    globs: Vec<GlobMatcher>,
}
// Take a string such as my.host:22:/home/me/src, and return
// (my.host:22, /home/me/src)
fn parse_host(host: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = host.split(":").collect();

    match parts[..] {
        [hostname, path] => Ok((format!("{}:22", hostname), path.to_string())),
        [hostname, port, path] => Ok((format!("{}:{}", hostname, port), path.to_string())),
        _ => Err(Error::msg("Failed to parse host input"))
    }
}

impl EventHandler {
    pub fn new(host: &str, excludes: Option<Values>) -> Result<EventHandler> {
        let globs: Vec<GlobMatcher> = excludes
            .unwrap_or(Values::default())
            .map(|v| Glob::new(v).unwrap().compile_matcher())
            .collect();

        let (host, remote) = parse_host(host)?;

        let tcp = TcpStream::connect(host).unwrap();
        let mut session = Session::new().unwrap();

        session.set_tcp_stream(tcp);
        session.handshake().unwrap();
        session.userauth_agent("ellie").unwrap();

        Ok(EventHandler {
            remote,
            session,
            globs,
        })
    }


    fn ignore(&self, path: &str) -> bool {
        self.globs.iter().map(|g| g.is_match(path)).any(|x| x)
    }

    // Take a local filepath, and write the contents to the remote
    fn send_file(&self, source: &mut dyn Read, dest: &mut dyn Write) -> Result<()> {
        std::io::copy(source, dest)?;
        dest.flush()?;

        Ok(())
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
        code.push(&self.remote);
        code.push(path);

        println!("sending: {:?}", code);

        let mut source = File::open(path)?;

        let mut remote_file = self
            .session
            .scp_send(code.as_path(), 0o644, source.metadata()?.len(), None)
            .unwrap();

        self.send_file(&mut source, &mut remote_file)?;

        Ok(())
    }
}
