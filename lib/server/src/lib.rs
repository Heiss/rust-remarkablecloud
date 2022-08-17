use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{body::Body, extract::Host, http::Request, routing::any, Extension, Router};
use config::Config;
use std::sync::{Arc, Mutex, RwLock};
use std::{net::SocketAddr, sync::atomic::AtomicUsize};
use storage::{CodeStorage, Storages, UserStorage};
use tower::{ServiceBuilder, ServiceExt};

mod api;
mod ui;
use tower_http::trace::TraceLayer;

pub struct State {
    api_requests: AtomicUsize,
    website_requests: AtomicUsize,
}

/// Helper structure to access storages without the need to wait another storage.
pub struct ThreadStorage<U: UserStorage + ?Sized, C: CodeStorage + ?Sized> {
    user_storage: RwLock<Box<U>>,
    code_storage: RwLock<Box<C>>,
}

type SharedStorage = Arc<ThreadStorage<dyn UserStorage, dyn CodeStorage>>;

#[tokio::main]
pub async fn run<U: UserStorage, C: CodeStorage>(config: Config, storages: Storages<U, C>) {
    let config = Arc::new(config);
    let config_req = config.clone();

    let notfound_router = Router::new().fallback(any(handler_404));

    let state = Arc::new(State {
        website_requests: AtomicUsize::new(0),
        api_requests: AtomicUsize::new(0),
    });

    let storages: SharedStorage = Arc::new(ThreadStorage {
        user_storage: RwLock::new(storages.user_storage),
        code_storage: RwLock::new(storages.code_storage),
    });

    let app = Router::new()
        .route(
            "/*path",
            any(|Host(hostname): Host, request: Request<Body>| async move {
                if hostname.as_str() == config_req.api.url.as_str().to_string() {
                    api::get_router::<U, C>().oneshot(request).await
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
                .layer(Extension(storages))
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
