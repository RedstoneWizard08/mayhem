use std::convert::Infallible;

use rocket::{request::{FromRequest, self}, request::Outcome, Request, http::HeaderMap};

pub struct RequestHeaders<'h>(pub &'h HeaderMap<'h>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestHeaders<'r> {
    type Error = Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let headers = request.headers();
        Outcome::Success(RequestHeaders(headers))
    }
}
