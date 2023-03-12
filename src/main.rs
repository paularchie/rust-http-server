#![allow(dead_code)]

use handler::WebsiteHandler;
use server::Server;
use std::env;

mod handler;
mod http;
mod server;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}
