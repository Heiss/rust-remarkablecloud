use std::env;
use thiserror::Error;
use toml::Value;

use crate::TomlError;

#[derive(Error, Debug)]
pub enum CommonError {
    #[error("error in common toml config")]
    TomlError(#[from] TomlError),
}

/// Represents all configs for admin UI
#[derive(Debug)]
pub struct Common {
    pub port: u16,
    pub loglevel: String,
    pub socket: u16,
}

impl Common {
    /// Creates the UI config struct and checks for required and optional fields
    pub fn create(yaml: &Value) -> Result<Self, CommonError> {
        let common_config = yaml.get("COMMON").ok_or(TomlError::KeyNotFound("COMMON"))?;

        let port = common_config
            .get("PORT")
            .ok_or(TomlError::KeyNotFound("COMMON.PORT"))?
            .as_integer()
            .ok_or(TomlError::WrongType("COMMON.PORT", "Integer"))?
            .try_into()
            .map_err(|e| {
                println!("error in config while trying to read port: {:?}", e);
                TomlError::WrongType("COMMON.PORT", "Integer")
            })?;

        let loglevel = common_config
            .get("LOGLEVEL")
            .ok_or(TomlError::KeyNotFound("COMMON.LOGLEVEL"))?
            .as_str()
            .ok_or(TomlError::WrongType("COMMON.LOGLEVEL", "String"))?
            .to_string();

        let socket = common_config
            .get("SOCKET")
            .ok_or(TomlError::KeyNotFound("COMMON.SOCKET"))?
            .as_integer()
            .ok_or(TomlError::WrongType("COMMON.SOCKET", "Integer"))?
            .try_into()
            .map_err(|e| {
                println!("error in config while trying to read socketport: {:?}", e);
                TomlError::WrongType("COMMON.SOCKET", "Integer")
            })?;

        Ok(Self {
            port,
            loglevel,
            socket,
        })
    }
}

impl Default for Common {
    fn default() -> Self {
        Self {
            // this will be used, when no config file should be used.
            port: env::var("PORT")
                .expect("PORT not found in environment variables.")
                .parse::<u16>()
                .expect("PORT not valid number."),
            loglevel: env::var("LOGLEVEL").expect("LOGLEVEL not found in environment variables."),
            socket: env::var("SOCKET_PORT")
                .expect("SOCKET_PORT not found in environment variables.")
                .parse::<u16>()
                .expect("SOCKET_PORT not valid number."),
        }
    }
}
