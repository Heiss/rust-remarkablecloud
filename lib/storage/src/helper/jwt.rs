use crate::{UserFile, UserProfile};
use chrono::{Duration, Utc};
use config::Config;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use uuid::Uuid;

/// Create an jwt from userprofile and claims.
/// It uses HMAC256 for signing.
pub fn create_jwt_from_userprofile<T: UserFile>(config: &Config, user: &UserProfile) -> String {
    let mut scopes = vec![];

    if user.sync15 {
        scopes.push("sync15");
    }

    let expiration = Utc::now() + Duration::days(1);

    let key: Hmac<Sha256> = Hmac::new_from_slice(&config.api.secret_key.as_bytes()).unwrap();

    let mut claims: BTreeMap<&'static str, String> = BTreeMap::new();
    claims.insert("UserID", user.email.0.clone());
    claims.insert("BrowserID", Uuid::new_v4().to_string());
    claims.insert("Email", user.email.0.clone());
    claims.insert("Scopes", scopes.join(" "));
    claims.insert("ExpiresAt", expiration.timestamp().to_string());
    claims.insert("Issuer", "rmCloud WEB".to_string());
    claims.insert("Audience", "web".to_string());

    //user.to_json().as_bytes()

    claims.sign_with_key(&key).unwrap()
}

//fn verify_jwt(){}
