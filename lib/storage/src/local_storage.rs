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
    #[error("Code was not valid")]
    CodeNotValid,
    #[error("Code already expired")]
    CodeExpired,
}

pub trait UserStorage: Storage + Send + Sync + 'static + std::fmt::Debug {
    fn create(config_file: &PathBuf) -> Result<Box<Self>, LocalStorageError>
    where
        Self: Sized;
    fn get_user(&self, email: &EMail) -> Result<Box<dyn UserFile>, LocalStorageError>;
    fn delete_user(&self, email: &EMail) -> Result<(), LocalStorageError>;
    fn create_user(
        &self,
        email: &EMail,
        password: &str,
        is_admin: &bool,
        sync15: &bool,
    ) -> Result<Box<dyn UserFile>, LocalStorageError>;

    fn edit_user(
        &self,
        email: &EMail,
        password: &str,
        is_admin: &bool,
        sync15: &bool,
    ) -> Result<(), LocalStorageError>;
}

pub trait CodeStorage: Storage + Send + Sync + 'static + std::fmt::Debug {
    fn create(config_file: &PathBuf) -> Result<Box<Self>, LocalStorageError>
    where
        Self: Sized;
    fn validate_code(&self, email: &EMail, code: &str) -> Result<(), LocalStorageError>;
    fn create_code(&mut self, email: &EMail) -> Result<Box<String>, LocalStorageError>;
    fn remove_code(&mut self, email: &EMail, code: &str) -> Result<(), LocalStorageError>;
    fn clean_codes(&mut self) -> Result<(), LocalStorageError>;
}
