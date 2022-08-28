use crate::{
    helper::{create_jwt_from_userprofile, verify_and_get_claims},
    State, StateCodeStorage, StateUserStorage,
};
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{any, get, post},
    Extension, Json, Router,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use config::Config;
use std::sync::Arc;
use std::{sync::atomic::Ordering, vec};
use storage::EMail;

pub async fn api_handler(
    Extension(state): Extension<Arc<State>>,
    //    Extension(config): Extension<Arc<Config>>,
) -> Html<String> {
    state.api_requests.fetch_add(1, Ordering::SeqCst);
    tracing::debug! {"got api request"}
    Html(format!(
        "api: {}",
        state.api_requests.load(Ordering::SeqCst)
    ))
}

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct About<'a> {
    api_version: String,
    servername: String,
    hostname: String,
    software: Vec<&'a str>,
}

#[derive(Deserialize, Debug)]
struct Login {
    code: String,
    email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JWT {
    pub jwt: String,
}

// TODO add a "CODE FORGOTTEN" endpoint and link

async fn login_handler(
    Extension(config): Extension<Arc<Config>>,
    user_storage: Extension<StateUserStorage>,
    code_storage: Extension<StateCodeStorage>,
    Json(payload): Json<Login>,
) -> Result<impl IntoResponse, StatusCode> {
    tracing::debug! {?payload, "Got code for login exchange"};
    let email = match EMail::create(&payload.email) {
        Ok(v) => v,
        Err(e) => {
            tracing::debug! {?e, "email not created"};
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    match code_storage
        .read()
        .unwrap()
        .validate_code(&email, &payload.code)
    {
        Ok(_) => {
            let jwt = create_jwt_from_userprofile(
                config.as_ref(),
                user_storage
                    .read()
                    .unwrap()
                    .get_user(&email)
                    .unwrap()
                    .as_ref(),
            );
            return Ok(Json(JWT { jwt }));
        }
        Err(v) => tracing::debug! {?v, "got error"},
    };
    Err(StatusCode::UNAUTHORIZED)
}

pub async fn jwt_handler(
    Extension(config): Extension<Arc<Config>>,
    user_storage: Extension<StateUserStorage>,
    Json(payload): Json<JWT>,
) -> impl IntoResponse {
    let claims = verify_and_get_claims(&payload.jwt, &config).map_err(|v| {
        tracing::debug! {?v, "Error in jwt verification"};
        StatusCode::UNAUTHORIZED
    })?;
    let email = match EMail::create(&claims.get("UserID").expect("UserID was not in signature.")) {
        Ok(v) => v,
        Err(e) => {
            tracing::debug! {?e, "email not created"};
            return Err(StatusCode::UNAUTHORIZED);
        }
    };
    // TODO: check, if expireAt is small. Only refresh jwt then
    let jwt;
    let date_stamp = NaiveDateTime::from_timestamp(
        claims
            .get("ExpiresAt")
            .expect("missing expiration")
            .parse::<i64>()
            .expect("expiration date not valid"),
        0,
    );

    if DateTime::<Utc>::from_utc(date_stamp, Utc) < Utc::now() {
        jwt = create_jwt_from_userprofile(
            config.as_ref(),
            user_storage
                .read()
                .unwrap()
                .get_user(&email)
                .unwrap()
                .as_ref(),
        );
        tracing::debug! {"JWT expired. Generated a new one."}
    } else {
        jwt = payload.jwt;
        tracing::debug! {"JWT not expired. Send the old one back."}
    }

    return Ok(Json(JWT { jwt }));
}

pub async fn about_handler(Extension(config): Extension<Arc<Config>>) -> Html<String> {
    let about = About {
        // TODO: values should be taken from the parent project Cargo.toml
        api_version: "0.1.0".to_string(),
        servername: "rmcloud".to_string(),
        hostname: config.api.url.to_string(),
        software: vec!["rust", "cargo", "axum"],
    };

    Html(match serde_json::to_string(&about) {
        Ok(v) => v,
        Err(v) => {
            tracing::debug! {?v, "cannot serialize about"};
            String::from("")
        }
    })
}

pub async fn health_handler() -> Html<String> {
    tracing::debug! {"report health"}
    Html(format!("status: {}", "excellent"))
}

pub fn get_router() -> Router {
    Router::new()
        .route("/about", get(about_handler))
        .route("/login", post(login_handler))
        .route("/jwt", post(jwt_handler))
        .route("/health", get(health_handler))
        .route("/", any(api_handler))
}
