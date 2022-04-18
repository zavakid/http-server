use crate::http::{ParseError, Request, Response, StatusCode};

pub trait Handler: Send + Sync {
    fn handle_request(&self, request: &Request) -> Response;

    fn handle_bad_request(&self, e: &ParseError) -> Response {
        log::warn!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}