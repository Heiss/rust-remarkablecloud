use std::path::PathBuf;

use crate::userprofile::UserProfileError;
use crate::Storage;
use crate::UserFile;
use crate::{EMail, EMailError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LocalStorageError {
    #[error("ConfigParser error occurred")]
    ConfigError(#[from] config::ConfigError),
    #[error("Io error occurred")]
    IoError(#[from] std::io::Error),
    #[error("Given user email not found")]
    UserNotFound,
    #[error("Given email was invalid")]
    UserAlreadyExists,
    #[error("Yaml error occurred")]
    YamlError(#[from] serde_yaml::Error),
    #[error("Yaml error occurred")]
    UserProfileError(#[from] UserProfileError),
    #[error("EMail error occurred")]
    EMailError(#[from] EMailError),
}

pub trait UserStorage: Storage {
    fn create(config_file: &PathBuf) -> Result<Self, LocalStorageError>
    where
        Self: Sized;

    fn get_user<T>(&self, email: &EMail) -> Result<T, LocalStorageError>
    where
        T: UserFile;

    fn delete_user<T>(&self, email: &EMail) -> Result<(), LocalStorageError>
    where
        T: UserFile;

    fn create_user<T>(
        &self,
        email: &EMail,
        password: &str,
        is_admin: &bool,
        sync15: &bool,
    ) -> Result<T, LocalStorageError>
    where
        T: UserFile;

    fn edit_user<T>(
        &self,
        email: &EMail,
        password: &str,
        is_admin: &bool,
        sync15: &bool,
    ) -> Result<(), LocalStorageError>
    where
        T: UserFile;
}
