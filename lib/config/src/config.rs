use crate::{Api, Common, Ui};
use std::{env, path::PathBuf};
use thiserror::Error;
use toml::Value;

#[derive(Error, Debug)]

pub enum TomlError {
    #[error("Required `{0}` field in toml file is missing")]
    KeyNotFound(&'static str),
    #[error("Required `{0}` field in toml file is not the correct type: {1}")]
    WrongType(&'static str, &'static str),
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Required PORT field cannot be found in toml file")]
    PortKeyNotFound,
    #[error("Required PORT field in toml file is not a number")]
    PortKeyNotNumber,
    #[error("Required PORT field in toml file is not a valid port: Between 1-65535")]
    PortKeyExceedsPortRange,
    #[error("Required LOGLEVEL field cannot be found in toml file")]
    LogLevelKeyNotFound,
    #[error("Required LOGLEVEL field in toml file is not a string")]
    LogLevelKeyNotString,
    #[error("UI config is not valid")]
    UiError(#[from] crate::ui::UiError),
    #[error("API config is not valid")]
    ApiError(#[from] crate::api::ApiError),
    #[error("Common config is not valid")]
    CommonError(#[from] crate::common::CommonError),
    #[error("Given toml string was not valid")]
    NotValidToml(#[from] toml::de::Error),
    #[error("There was an io error")]
    IoError(#[from] std::io::Error),
}

/// Represents the global config struct, which holds all configuration
#[derive(Debug)]
pub struct Config {
    pub api: Api,
    pub ui: Ui,
    pub common: Common,
}

impl Config {
    /// Creates the config object from the given toml object.
    pub fn create(toml_str: &str) -> Result<Self, ConfigError> {
        let toml: Value = toml::from_str(toml_str).map_err(|v| ConfigError::NotValidToml(v))?;

        Ok(Self {
            ui: Ui::create(&toml)?,
            api: Api::create(&toml)?,
            common: Common::create(&toml)?,
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api: Default::default(),
            ui: Default::default(),
            common: Default::default(),
        }
    }
}

pub fn read_config(path: &PathBuf) -> Result<Config, ConfigError> {
    Config::create(&std::fs::read_to_string(path)?)
}
