use std::{
    net::SocketAddr,
    sync::{atomic::AtomicUsize, Arc, RwLock},
};

use crate::{api, ui};
use axum::{
    body::Body,
    extract::Host,
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::any,
    Extension, Router,
};
use config::Config;
use storage::{CodeStorage, UserStorage};
use tower::{ServiceBuilder, ServiceExt};
use tower_http::trace::TraceLayer;

pub struct State {
    pub api_requests: AtomicUsize,
    pub website_requests: AtomicUsize,
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

pub async fn run_server(
    config: Arc<Config>,
    axum_rx: tokio::sync::oneshot::Receiver<()>,
    user_storage: Arc<RwLock<Box<dyn UserStorage>>>,
    code_storage: Arc<RwLock<Box<dyn CodeStorage>>>,
) -> () {
    let notfound_router = Router::new().fallback(any(handler_404));
    let state = Arc::new(State {
        website_requests: AtomicUsize::new(0),
        api_requests: AtomicUsize::new(0),
    });

    let config_req = config.clone();

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
                .layer(Extension(user_storage))
                .layer(Extension(code_storage))
                // See https://docs.rs/tower-http/0.1.1/tower_http/trace/index.html for more details.
                // More customization see https://github.com/tokio-rs/axum/blob/ac7037d28208403d6030a47fdd9b0ff9cf2a9009/examples/tracing-aka-logging/src/main.rs#L37
                .layer(TraceLayer::new_for_http()),
            //TODO missing jwt auth
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], config.common.port));
    println!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            axum_rx.await.ok();
            tracing::debug! {"Close axum"};
        })
        .await
        .unwrap();
}
