#![allow(dead_code)]

use std::env;
use std::sync::Arc;

use server::Server;
use website_handler::WebsiteHandler;

mod protocol;
mod server;
mod handler;
mod website_handler;

fn main() {
    env_logger::init();

    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    log::info!("public path: {}", public_path);

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(Arc::new(WebsiteHandler::new(public_path)));
}
