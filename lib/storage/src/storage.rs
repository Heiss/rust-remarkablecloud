use std::path::PathBuf;

use thiserror::Error;

use crate::{CodeStorage, LocalStorageError, UserStorage};

pub trait Storage {}

#[derive(Debug)]
pub struct Storages<U: UserStorage, C: CodeStorage> {
    pub user_storage: U,
    pub code_storage: C,
}

#[derive(Error, Debug)]
pub enum StoragesError {
    #[error("Error occurred in LocalStorage")]
    LocalStorageError(#[from] LocalStorageError),
}

impl<U: UserStorage, C: CodeStorage> Storages<U, C> {
    pub fn create(config_path: &PathBuf) -> Result<Self, StoragesError> {
        let user_storage = U::create(&config_path)?;
        let code_storage = C::create(&config_path)?;
        Ok(Self {
            user_storage,
            code_storage,
        })
    }
}
