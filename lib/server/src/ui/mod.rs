use crate::axum_server::State;
use axum::Extension;
use axum::{
    body::{boxed, Full},
    handler::Handler,
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::{get, Router},
};
use mime_guess;
use rust_embed::RustEmbed;
use std::sync::atomic::Ordering;
use std::sync::Arc;

// REMOVEME: This is an example for state exchange
#[allow(dead_code)]
async fn website_handler(Extension(state): Extension<Arc<State>>) -> Html<String> {
    state.website_requests.fetch_add(1, Ordering::SeqCst);
    Html(format!(
        "website: {}",
        state.website_requests.load(Ordering::SeqCst)
    ))
}

pub fn get_router() -> Router {
    Router::new()
        .route("/assets/*file", static_handler.into_service())
        .route("/api/*path", crate::api::api_handler.into_service())
        .fallback(get(index_handler))
}

// We use static route matchers ("/" and "/index.html") to serve our home
// page.
async fn index_handler() -> impl IntoResponse {
    static_handler("/index.html".parse::<Uri>().unwrap()).await
}

// We use a wildcard matcher ("/dist/*file") to match against everything
// within our defined assets directory. This is the directory on our Asset
// struct below, where folder = "examples/public/".
async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/').to_string();

    StaticFile(path)
}

#[derive(RustEmbed)]
#[folder = "../../ui/dist"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();
        tracing::debug! {%path, "requested asset file"};

        match Asset::get(path.as_str()) {
            Some(content) => {
                let body = boxed(Full::from(content.data));
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                Response::builder()
                    .header(header::CONTENT_TYPE, mime.as_ref())
                    .body(body)
                    .unwrap()
            }
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(boxed(Full::from("404")))
                .unwrap(),
        }
    }
}
