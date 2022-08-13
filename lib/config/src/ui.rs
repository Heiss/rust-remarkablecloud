use serde_yaml::Value;
use std::env;
use thiserror::Error;

use crate::YamlError;

#[derive(Error, Debug)]
pub enum UiError {
    #[error("error in ui yaml config")]
    YamlError(#[from] YamlError),
    #[error("UI.URL must not contain protocol like http")]
    UrlContainsProtocol,
}

/// Represents all configs for admin UI
#[derive(Debug)]
pub struct Ui {
    pub url: String,
}

impl Ui {
    /// Creates the UI config struct and checks for required and optional fields
    pub fn create(yaml: &Value) -> Result<Self, UiError> {
        let ui_config = yaml.get("UI").ok_or(YamlError::KeyNotFound("UI.URL"))?;
        let url = ui_config
            .get("URL")
            .ok_or(YamlError::KeyNotFound("UI.URL"))?
            .as_str()
            .ok_or(YamlError::WrongType("UI.URL", "String"))?
            .to_string();

        if url.contains("://") {
            return Err(UiError::UrlContainsProtocol);
        }
        Ok(Self { url })
    }
}

impl Default for Ui {
    fn default() -> Self {
        Self {
            // this will be used, when no config file should be used.
            url: env::var("UI_URL").expect("UI_URL not found in environment variables."),
        }
    }
}
