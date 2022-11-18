use crate::middleware::auth::Authorization;
use rocket::{get, http::Status, response::status};

#[get("/")]
pub fn index(auth: Authorization) -> status::Custom<&str> {
    let tkn = auth.0;

    return status::Custom(Status::Ok, tkn);
}
