use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{body::Body, extract::Host, http::Request, routing::any, Extension, Router};
use axum::{body::Bytes, http::HeaderMap, response::Response};
use config::Config;
use std::sync::Arc;
use std::time::Duration;
use std::{net::SocketAddr, sync::atomic::AtomicUsize};
use tower::{ServiceBuilder, ServiceExt};

mod api;
mod ui;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::Span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct State {
    api_requests: AtomicUsize,
    website_requests: AtomicUsize,
}

#[tokio::main]
pub async fn run(config: Config) {
    let config = Arc::new(config);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            config.loglevel.to_string(),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config_req = config.clone();

    let notfound_router = Router::new().fallback(any(handler_404));

    let state = Arc::new(State {
        website_requests: AtomicUsize::new(0),
        api_requests: AtomicUsize::new(0),
    });

    let app = Router::new()
        .route(
            "/*path",
            any(|Host(hostname): Host, request: Request<Body>| async move {
                if hostname.as_str() == config_req.api.url.as_str().to_string() {
                    api::get_router().oneshot(request).await
                } else if hostname.as_str() == config_req.ui.url.as_str().to_string() {
                    ui::get_router().oneshot(request).await
                } else {
                    notfound_router.oneshot(request).await
                }
            }),
        )
        .layer(
            ServiceBuilder::new()
                .layer(Extension(config.clone()))
                .layer(Extension(state))
                // `TraceLayer` is provided by tower-http so you have to add that as a dependency.
                // It provides good defaults but is also very customizable.
                //
                // See https://docs.rs/tower-http/0.1.1/tower_http/trace/index.html for more details.
                // More customization see https://github.com/tokio-rs/axum/blob/ac7037d28208403d6030a47fdd9b0ff9cf2a9009/examples/tracing-aka-logging/src/main.rs#L37
                .layer(TraceLayer::new_for_http()),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    println!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
