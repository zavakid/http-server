use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread::spawn;

use crate::handler::Handler;
use crate::protocol::Request;
use crate::server::Server;

pub struct MultiThreadServer {
    addr: String,
}

impl Server for MultiThreadServer {
    fn new(addr: String) -> Self {
        Self { addr }
    }

    fn run(self, handler: Arc<dyn Handler>) {
        log::info!("listening to {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((stream, _addr)) => {
                    handle_connection(stream, Arc::clone(&handler));
                }
                Err(e) => log::error!("Failed to establish a connection: {}", e),
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, handler: Arc<dyn Handler>) {
    let handler_for_request = handler.clone();
    spawn(move || {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(_) => {
                log::info!("Received a request: {}", String::from_utf8_lossy(&buffer));

                let response = match Request::try_from(&buffer[..]) {
                    Ok(request) => handler_for_request.handle_request(&request),
                    Err(e) => handler_for_request.handle_bad_request(&e),
                };

                if let Err(e) = response.send(&mut stream) {
                    log::error!("Failed to send response: {}", e);
                }
            }
            Err(e) => log::error!("Failed to read from connection: {}", e),
        }
    });
}
