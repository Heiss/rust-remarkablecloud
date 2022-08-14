use anyhow::{Ok, Result};
use config::read_config;
use config::Config;
use std::path::PathBuf;
use storage::CodeStorage;
use storage::Storages;
use storage::UserStorage;

pub struct ServerBuilder<U, C>
where
    U: UserStorage,
    C: CodeStorage,
{
    path: PathBuf,
    storage: Storages<U, C>,
}

impl<U: UserStorage, C: CodeStorage> std::fmt::Display for ServerBuilder<U, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "config path: {}", self.path.display())
    }
}

impl<U: UserStorage, C: CodeStorage> ServerBuilder<U, C> {
    pub fn new(path: PathBuf, storage: Storages<U, C>) -> Self {
        ServerBuilder { path, storage }
    }

    pub fn build(self) -> Result<Server<U, C>> {
        println!("Creating server with the following arguments.\n{}\n", self);
        Ok(Server {
            config: read_config(&self.path)?,
            storages: self.storage,
        })
    }
}

pub struct Server<U, C>
where
    U: UserStorage,
    C: CodeStorage,
{
    config: Config,
    storages: Storages<U, C>,
}

impl<U: UserStorage, C: CodeStorage> Server<U, C> {
    pub fn execute(self) -> Result<()> {
        server::run(self.config, self.storages);
        Ok(())
    }
}
