use crate::{EMail, EMailError};
use serde_yaml::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserProfileError {
    #[error("Missing key in userprofile. key: {0}")]
    MissingKey(&'static str),
    #[error("Invalid value in userprofile. key: {0}, expected type: {1}")]
    InvalidType(&'static str, &'static str),
    #[error("Yaml error occurred")]
    YamlError(#[from] serde_yaml::Error),
    #[error("email error occurred")]
    EMailError(#[from] EMailError),
}

pub trait UserFile {
    fn new(email: EMail, password: String, is_admin: bool, sync15: bool) -> Self;
    fn to_yaml(&self) -> String;
    fn to_json(&self) -> String;
    fn from_yaml(yaml: Value) -> Result<Self, UserProfileError>
    where
        Self: Sized;
}

pub trait UserLocalFile: UserFile {}

#[derive(Debug)]
pub struct UserProfile {
    pub email: EMail,
    pub password: String,
    pub is_admin: bool,
    pub sync15: bool,
}

impl UserFile for UserProfile {
    fn new(email: EMail, password: String, is_admin: bool, sync15: bool) -> Self {
        Self {
            email,
            password,
            is_admin,
            sync15,
        }
    }

    fn to_yaml(&self) -> String {
        format!(
            "email: {}\npassword: {}\nis_admin: {}\nsync15: {}",
            self.email.0, self.password, self.is_admin, self.sync15
        )
    }

    fn to_json(&self) -> String {
        format!(
            "{{\"email\":{},\"password\":{},\"is_admin\":{},\"sync15\":{}}}",
            self.email.0, self.password, self.is_admin, self.sync15
        )
    }

    fn from_yaml(yaml: Value) -> Result<Self, UserProfileError> {
        let email = yaml
            .get("email")
            .ok_or_else(|| UserProfileError::MissingKey("email"))?
            .as_str()
            .ok_or_else(|| UserProfileError::InvalidType("email", "String"))?
            .to_string();

        let password = yaml
            .get("password")
            .ok_or_else(|| UserProfileError::MissingKey("password"))?
            .as_str()
            .ok_or_else(|| UserProfileError::InvalidType("password", "String"))?
            .to_string();

        let is_admin = yaml
            .get("is_admin")
            .ok_or_else(|| UserProfileError::MissingKey("is_admin"))?
            .as_bool()
            .ok_or_else(|| UserProfileError::InvalidType("is_admin", "Boolean"))?;

        let sync15 = yaml
            .get("sync15")
            .ok_or_else(|| UserProfileError::MissingKey("sync15"))?
            .as_bool()
            .ok_or_else(|| UserProfileError::InvalidType("sync15", "String"))?;

        Ok(Self {
            email: EMail::create(&email)?,
            password,
            is_admin,
            sync15,
        })
    }
}
impl UserLocalFile for UserProfile {}
