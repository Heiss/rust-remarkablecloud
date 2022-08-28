use std::{
    io::{self, Read},
    net::TcpListener,
    sync::Arc,
    time::Duration,
};

use config::Config;
use tokio::time::sleep;

pub async fn run_cli_socket(
    config: Arc<Config>,
    mut socket_rx: tokio::sync::oneshot::Receiver<()>,
) -> tokio::task::JoinHandle<()> {
    let socket_port = config.common.socket;

    tokio::spawn(async move {
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
    })
}
