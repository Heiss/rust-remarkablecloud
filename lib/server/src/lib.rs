use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{body::Body, extract::Host, http::Request, routing::any, Extension, Router};
use config::Config;
use std::io::{self, BufRead, BufReader, Read};
use std::net::TcpListener;
use std::sync::{Arc, RwLock};
use std::{net::SocketAddr, sync::atomic::AtomicUsize};
use storage::{CodeStorage, UserStorage};
use tokio::sync::oneshot;
use tokio::time::{sleep, Duration};
use tower::{ServiceBuilder, ServiceExt};

mod api;
mod helper;
mod ui;

use tower_http::trace::TraceLayer;

pub struct State {
    api_requests: AtomicUsize,
    website_requests: AtomicUsize,
}

// taken from https://github.com/tokio-rs/axum/blob/main/examples/error-handling-and-dependency-injection/src/main.rs
pub type StateUserStorage = Arc<RwLock<Box<dyn UserStorage>>>;
pub type StateCodeStorage = Arc<RwLock<Box<dyn CodeStorage>>>;

#[tokio::main]
pub async fn run(
    config: Config,
    user_storage: Box<dyn UserStorage>,
    code_storage: Box<dyn CodeStorage>,
) -> io::Result<()> {
    let config = Arc::new(config);
    let config_req = config.clone();

    let notfound_router = Router::new().fallback(any(handler_404));

    let state = Arc::new(State {
        website_requests: AtomicUsize::new(0),
        api_requests: AtomicUsize::new(0),
    });

    let user_storage = Arc::new(RwLock::new(user_storage)) as StateUserStorage;
    let code_storage = Arc::new(RwLock::new(code_storage)) as StateCodeStorage;

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
                .layer(Extension(user_storage.clone()))
                .layer(Extension(code_storage.clone()))
                // See https://docs.rs/tower-http/0.1.1/tower_http/trace/index.html for more details.
                // More customization see https://github.com/tokio-rs/axum/blob/ac7037d28208403d6030a47fdd9b0ff9cf2a9009/examples/tracing-aka-logging/src/main.rs#L37
                .layer(TraceLayer::new_for_http()),
            //TODO missing jwt auth
        );

    let (socket_tx, mut socket_rx) = oneshot::channel();
    let (axum_tx, axum_rx) = oneshot::channel();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("expect tokio signal ctrl-c");
        print!("Signal exit...");
        socket_tx
            .send(())
            .expect("Cannot send close command to cli socket.");

        axum_tx
            .send(())
            .expect("Cannot send close command to axum server.");
    });

    let socket_port = config.clone().common.socket;

    let handle = tokio::spawn(async move {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", socket_port))
            .expect("Cannot create socket listener for cli.");
        listener
            .set_nonblocking(true)
            .expect("Cannot set non-blocking");

        loop {
            // TODO: process tcp socket incoming messages from cli
            for stream in listener.incoming() {
                match stream {
                    Ok(mut s) => {
                        // do something with the TcpStream
                        tracing::debug! {?s, "got cli message"};
                        /*
                        let buf_reader = BufReader::new(&mut s);
                        let http_request: Vec<_> = buf_reader
                        .lines()
                        .map(|result| result.unwrap())
                        .take_while(|line| !line.is_empty())
                        .collect();
                        */
                        let mut http_request = [0; 512];
                        let bytes_read = s.read(&mut http_request).expect("Cannot read tcp stream");
                        if bytes_read == 0 {
                            return;
                        }

                        let http_request = String::from_utf8(http_request.to_vec());
                        tracing::debug! {?http_request, "got cli request"};
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => panic!("encountered IO error: {}", e),
                }
                if socket_rx.try_recv().is_ok() {
                    tracing::debug! {"Close cli socket"};
                    return;
                }
                sleep(Duration::from_millis(100)).await;
            }
        }
    });

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

    handle.await.unwrap();

    println!("Everything is closed gracefully. Bye.\n");
    Ok(())
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
