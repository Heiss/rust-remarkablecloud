use crate::{Api, Ui};
use serde_yaml::Value;
use std::{env, path::PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]

pub enum YamlError {
    #[error("Required `{0}` field in yaml file is missing")]
    KeyNotFound(&'static str),
    #[error("Required `{0}` field in yaml file is not the correct type: {1}")]
    WrongType(&'static str, &'static str),
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Required PORT field cannot be found in yaml file")]
    PortKeyNotFound,
    #[error("Required PORT field in yaml file is not a number")]
    PortKeyNotNumber,
    #[error("Required PORT field in yaml file is not a valid port: Between 1-65535")]
    PortKeyExceedsPortRange,
    #[error("Required LOGLEVEL field cannot be found in yaml file")]
    LogLevelKeyNotFound,
    #[error("Required LOGLEVEL field in yaml file is not a string")]
    LogLevelKeyNotString,
    #[error("UI config is not valid")]
    UiError(#[from] crate::ui::UiError),
    #[error("API config is not valid")]
    ApiError(#[from] crate::api::ApiError),
    #[error("Given yaml string was not valid")]
    NotValidYaml(#[from] serde_yaml::Error),
    #[error("There was an io error")]
    IoError(#[from] std::io::Error),
}

/// Represents the global config struct, which holds all configuration
#[derive(Debug)]
pub struct Config {
    pub api: Api,
    pub ui: Ui,
    pub port: u16,
    pub loglevel: String,
}

impl Config {
    /// Creates the config object from the given yaml object.
    pub fn create(yaml_str: &str) -> Result<Self, ConfigError> {
        let yaml: Value =
            serde_yaml::from_str(yaml_str).map_err(|v| ConfigError::NotValidYaml(v))?;

        let loglevel = yaml
            .get("LOGLEVEL")
            .ok_or(ConfigError::LogLevelKeyNotFound)?
            .as_str()
            .ok_or(ConfigError::LogLevelKeyNotString)?
            .to_string();

        let port_u32 = yaml
            .get("PORT")
            .ok_or(ConfigError::PortKeyNotFound)?
            .as_u64()
            .ok_or(ConfigError::PortKeyNotNumber)?;

        let port = u16::try_from(port_u32)
            .map_err(|_| ConfigError::PortKeyExceedsPortRange)
            .and_then(|v| {
                (v > 0)
                    .then(|| v)
                    .ok_or(ConfigError::PortKeyExceedsPortRange)
            })?;

        Ok(Self {
            ui: Ui::create(&yaml)?,
            api: Api::create(&yaml)?,
            port,
            loglevel,
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api: Default::default(),
            ui: Default::default(),
            port: env::var("PORT")
                .and_then(|x| {
                    Ok(x.parse::<u16>()
                        .expect("PORT value in environment variable is not a valid number."))
                })
                .unwrap_or(80),
            loglevel: env::var("LOGLEVEL").unwrap_or("info".to_string()),
        }
    }
}

pub fn read_config(path: &PathBuf) -> Result<Config, ConfigError> {
    Config::create(&std::fs::read_to_string(path)?)
}
