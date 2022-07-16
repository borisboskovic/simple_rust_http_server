use super::server::Handler;
use crate::http::{Request, Response, StatusCode};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &crate::http::Request) -> Response {
        Response::new(StatusCode::Ok, Some("<h1>Rust Server</h1>".to_string()))
    }
}
