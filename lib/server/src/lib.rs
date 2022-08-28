use config::Config;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use storage::{CodeStorage, UserStorage};

mod api;
mod axum_server;
mod cli_socket;
mod gracefully_exit;
mod helper;
mod ui;

// taken from https://github.com/tokio-rs/axum/blob/main/examples/error-handling-and-dependency-injection/src/main.rs
pub type StateUserStorage = Arc<RwLock<Box<dyn UserStorage>>>;
pub type StateCodeStorage = Arc<RwLock<Box<dyn CodeStorage>>>;

#[tokio::main]
pub async fn run(
    config: Config,
    user_storage: Box<dyn UserStorage>,
    code_storage: Box<dyn CodeStorage>,
) -> std::io::Result<()> {
    let config = Arc::new(config);

    let user_storage = Arc::new(RwLock::new(user_storage)) as StateUserStorage;
    let code_storage = Arc::new(RwLock::new(code_storage)) as StateCodeStorage;

    let (socket_rx, axum_rx) = gracefully_exit::create_receivers();

    let handle = cli_socket::run_cli_socket(config.clone(), socket_rx).await;
    axum_server::run_server(config, axum_rx, user_storage, code_storage).await;
    handle.await.expect("Cannot join cli socket");

    println!("Everything is closed gracefully. Bye.");
    Ok(())
}
