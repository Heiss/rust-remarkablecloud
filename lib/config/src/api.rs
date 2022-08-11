use crate::YamlError;
use serde_yaml::Value;
use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("error in api yaml config")]
    YamlError(#[from] YamlError),
}

/// Represents the config for API which communicates with the remarkable tablets
#[derive(Debug)]
pub struct Api {
    pub url: String,
    pub secret_key: String,
    pub data_dir: String,
    pub hwr: Option<HWR>,
    pub smtp: Option<SMTP>,
}

/// Represents all config for HWR functionalities
#[derive(Debug)]
pub struct HWR {
    pub app_key: String,
    pub hmac: String,
}

/// Represents all config for SMTP functionalities
#[derive(Debug)]
pub struct SMTP {
    pub server: String,
    pub username: String,
    pub password: String,
}

impl HWR {
    fn create(yaml: &Value) -> Result<Self, ApiError> {
        let hwr = yaml
            .get("HWR")
            .ok_or_else(|| YamlError::KeyNotFound("API.HWR"))?;

        let app_key = hwr
            .get("APPLICATIONKEY")
            .ok_or_else(|| YamlError::KeyNotFound("API.HWR.APPLICATIONKEY"))?
            .as_str()
            .ok_or_else(|| YamlError::WrongType("API.HWR.APPLICATIONKEY", "String"))?
            .to_string();

        let hmac = hwr
            .get("HMAC")
            .ok_or_else(|| YamlError::KeyNotFound("API.HWR.HMAC"))?
            .as_str()
            .ok_or_else(|| YamlError::WrongType("API.HWR.HMAC", "String"))?
            .to_string();

        Ok(Self { app_key, hmac })
    }
}

impl SMTP {
    fn create(yaml: &Value) -> Result<Self, ApiError> {
        let smtp = yaml
            .get("SMTP")
            .ok_or_else(|| YamlError::KeyNotFound("API.SMTP"))?;

        let server = smtp
            .get("SERVER")
            .ok_or_else(|| YamlError::KeyNotFound("API.SMTP.SERVER"))?
            .as_str()
            .ok_or_else(|| YamlError::WrongType("API.SMTP.SERVER", "String"))?
            .to_string();

        let username = smtp
            .get("USERNAME")
            .ok_or_else(|| YamlError::KeyNotFound("API.SMTP.USERNAME"))?
            .as_str()
            .ok_or_else(|| YamlError::WrongType("API.SMTP.USERNAME", "String"))?
            .to_string();

        let password = smtp
            .get("PASSWORD")
            .ok_or_else(|| YamlError::KeyNotFound("API.SMTP.PASSWORD"))?
            .as_str()
            .ok_or_else(|| YamlError::WrongType("API.SMTP.PASSWORD", "String"))?
            .to_string();

        Ok(Self {
            server,
            username,
            password,
        })
    }
}

impl Default for Api {
    fn default() -> Self {
        Self {
            url: env::var("API_URL").expect("API_URL not found in environment variables."),
            secret_key: env::var("API_SECRET_KEY")
                .expect("API_SECRET_KEY not found in environment variables."),
            data_dir: env::var("API_DATA_DIR")
                .expect("API_DATA_DIR not found in environment variables."),
            hwr: None,
            smtp: None,
        }
    }
}

impl Api {
    /// Creates the Api config struct and checks for required and optional fields
    pub fn create(yaml: &Value) -> Result<Self, ApiError> {
        let api = yaml
            .get("API")
            .ok_or_else(|| YamlError::KeyNotFound("API.API"))?;

        let url = api
            .get("URL")
            .ok_or_else(|| YamlError::KeyNotFound("API.URL"))?
            .as_str()
            .ok_or_else(|| YamlError::WrongType("API.URL", "String"))?
            .to_string();

        let secret_key = api
            .get("SECRET_KEY")
            .ok_or_else(|| YamlError::KeyNotFound("API.SECRET_KEY"))?
            .as_str()
            .ok_or_else(|| YamlError::WrongType("API.SECRET_KEY", "String"))?
            .to_string();

        let data_dir = api
            .get("DATADIR")
            .ok_or_else(|| YamlError::KeyNotFound("API.DATADIR"))?
            .as_str()
            .ok_or_else(|| YamlError::WrongType("API.DATADIR", "String"))?
            .to_string();

        let smtp = match SMTP::create(api) {
            Err(ApiError::YamlError(YamlError::KeyNotFound("API.SMTP"))) => None,
            v => Some(v?),
        };

        let hwr = match HWR::create(api) {
            Err(ApiError::YamlError(YamlError::KeyNotFound("API.HWR"))) => None,
            v => Some(v?),
        };

        Ok(Self {
            url,
            secret_key,
            data_dir,
            smtp,
            hwr,
        })
    }
}
