#![allow(dead_code)]


use std::sync::Arc;

use server::Server;
use handler::website_handler::WebsiteHandler;
use crate::server::MultiThreadServer;

mod protocol;
mod server;
mod handler;

fn main() {
    env_logger::init();
    let website_handler= WebsiteHandler::new();
    let server = MultiThreadServer::new("127.0.0.1:8080".to_string());
    server.run(Arc::new(website_handler));
}
