use thiserror::Error;

use crate::LocalStorageError;

pub trait Storage {}

#[derive(Error, Debug)]
pub enum StoragesError {
    #[error("Error occurred in LocalStorage")]
    LocalStorageError(#[from] LocalStorageError),
}
