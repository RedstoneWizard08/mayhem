use rocket::{
    http::Status,
    request::Outcome,
    request::{self, FromRequest},
    Request,
};

pub struct Authorization<'r>(pub &'r str);

#[derive(Debug)]
pub enum AuthorizationError {
    Invalid,
    Missing,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authorization<'r> {
    type Error = AuthorizationError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let headers = request.headers();
        let token = headers.get_one("Authorization");

        match token {
            Some(token) => Outcome::Success(Authorization(token)),
            None => Outcome::Failure((Status::Forbidden, AuthorizationError::Missing)),
        }
    }
}
