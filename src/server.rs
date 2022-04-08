use std::io::Read;
use std::net::TcpListener;
use std::sync::Arc;
use std::thread::spawn;

use crate::http::{ParseError, Request, Response, StatusCode};

pub trait Handler: Send + Sync {
    fn handle_request(&self, request: &Request) -> Response;

    fn handle_bad_request(&self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr,
        }
    }

    pub fn run(self, handler: Arc<dyn Handler>) {
        println!("listening to {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _addr)) => {
                    let handler_for_request = handler.clone();
                    spawn(move || {
                        let mut buffer = [0; 1024];
                        match stream.read(&mut buffer) {
                            Ok(_) => {
                                println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                                let response = match Request::try_from(&buffer[..]) {
                                    Ok(request) => {
                                        handler_for_request.handle_request(&request)
                                    }
                                    Err(e) => {
                                        handler_for_request.handle_bad_request(&e)
                                    }
                                };

                                if let Err(e) = response.send(&mut stream) {
                                    println!("Failed to send response: {}", e);
                                }
                            }
                            Err(e) => println!("Failed to read from connection: {}", e),
                        }
                    });
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}