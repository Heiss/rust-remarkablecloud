use anyhow::{Ok, Result};
use config::read_config;
use config::Config;
use std::path::PathBuf;

#[derive(Default)]
pub struct ServerBuilder {
    path: PathBuf,
}

impl std::fmt::Display for ServerBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "config path: {}", self.path.display())
    }
}

impl ServerBuilder {
    pub fn new(path: PathBuf) -> Self {
        ServerBuilder { path }
    }

    pub fn build(self) -> Result<Server> {
        println!("Creating server with the following arguments.\n{}\n", self);
        Ok(Server {
            config: read_config(&self.path)?,
        })
    }
}

pub struct Server {
    config: Config,
}

impl Server {
    pub fn execute(self) -> Result<()> {
        server::run(self.config);
        Ok(())
    }
}
