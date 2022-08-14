use regex::Regex;
use thiserror::Error;

pub fn validate_email(email: &str) -> bool {
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    email_regex.is_match(email)
}
#[derive(Debug, Clone)]
pub struct EMail(pub String);

impl EMail {
    pub fn create(email: &str) -> Result<Self, EMailError> {
        if validate_email(email) {
            Ok(Self(email.to_string()))
        } else {
            Err(EMailError::EmailInvalid)
        }
    }
}

#[derive(Error, Debug)]
pub enum EMailError {
    #[error("User already exists")]
    EmailInvalid,
}
