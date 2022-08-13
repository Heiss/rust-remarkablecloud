use crate::State;
use axum::{response::Html, routing::any, Extension, Router};
use std::sync::atomic::Ordering;
use std::sync::Arc;

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

pub fn get_router() -> Router {
    Router::new().route("/*path", any(api_handler))
}
