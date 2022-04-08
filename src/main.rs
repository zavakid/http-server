#![allow(dead_code)]

mod server;
mod http;
mod website_handler;

use server::Server;
use website_handler::WebsiteHandler;
use std::env;
use std::sync::Arc;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public path: {}", public_path);

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(Arc::new(WebsiteHandler::new(public_path)));
}