use tokio;
use tokio::sync::oneshot;

pub fn create_receivers() -> (
    tokio::sync::oneshot::Receiver<()>,
    tokio::sync::oneshot::Receiver<()>,
) {
    let (socket_tx, socket_rx) = oneshot::channel();
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
    (socket_rx, axum_rx)
}
