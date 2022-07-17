#![allow(dead_code)]
#![allow(unused_variables)]

use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    let default_path = format!("{}\\public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("RUST_SERVER_PUBLIC_PATH").unwrap_or(default_path.to_string());
    let server = Server::new("127.0.0.1:8000".to_string());
    server.run(WebsiteHandler::new(public_path));
}
