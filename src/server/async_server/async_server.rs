use std::sync::Arc;
use tokio::io::AsyncReadExt;

use tokio::net::{TcpListener, TcpStream};

use crate::handler::Handler;
use crate::protocol::Request;
use crate::Server;

pub struct AsyncServer {
    addr: String,
}

impl Server for AsyncServer {
    fn new(addr: String) -> Self {
        Self { addr }
    }

    fn run(self, handler: Arc<dyn Handler>) {
        log::info!("listening to {}", self.addr);

        let runtime = tokio::runtime::Runtime::new().unwrap();

        runtime.block_on(async move {
            let listener = TcpListener::bind(&self.addr).await.unwrap();

            loop {
                match listener.accept().await {
                    Ok((stream, _addr)) => {
                        let request_handler = Arc::clone(&handler);
                        tokio::spawn(async move {
                            handle_connection(stream, request_handler);
                        });
                    }
                    Err(e) => log::error!("Failed to establish a connection: {}", e),
                }
            }
        });
    }
}

async fn handle_connection(mut stream: TcpStream, handler: Arc<dyn Handler>) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer).await {
        Ok(_) => {
            log::info!("Received a request: {}", String::from_utf8_lossy(&buffer));

            let response = match Request::try_from(&buffer[..]) {
                Ok(request) => handler.handle_request(&request),
                Err(e) => handler.handle_bad_request(&e),
            };

            if let Err(e) = response.send(&mut stream) {
                log::error!("Failed to send response: {}", e);
            }
        }
        Err(_) => {}
    }
    todo!()
}
