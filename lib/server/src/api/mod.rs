use crate::{SharedStorage, State, ThreadStorage};
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{any, get, post},
    Extension, Json, Router,
};
use config::Config;
use std::sync::Arc;
use std::{sync::atomic::Ordering, vec};
use storage::{CodeLocalStorage, CodeStorage, UserLocalStorage, UserStorage};

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
    _code: String,
}

async fn login_handler(Json(payload): Json<Login>) -> impl IntoResponse {
    tracing::debug! {?payload, "Got code for login exchange"};
    (StatusCode::UNAUTHORIZED, "")
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
pub fn health_handler(Extension(storage): Extension<SharedStorage>) -> Html<String> {
    tracing::debug! {"report health"}
    Html(format!("status: {}", "excellent"))
}

pub fn get_router<U: UserStorage, C: CodeStorage>() -> Router {
    Router::new()
        .route("/about", get(about_handler))
        .route("/login", post(login_handler))
        .route("/health", get(health_handler))
        .route("/", any(api_handler))
}
