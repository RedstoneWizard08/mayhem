use rocket::{response::status, get, http::Status};
use crate::middleware::auth::Authorization;

#[get("/")]
pub fn index(auth: Authorization) -> status::Custom<&str> {
    let tkn = auth.0;

    return status::Custom(Status::Ok, tkn);
}
