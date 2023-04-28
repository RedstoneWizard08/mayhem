use axum::{http::Request, middleware::Next, response::Response};

use chrono::Utc;
use http::Method;

use crate::logging::{
    config::{Colors, ForegroundColors},
    custom::{custom, CustomType},
};

pub async fn logging_middleware<B>(req: Request<B>, next: Next<B>) -> Response {
    let time_start = Utc::now().time();

    let method = &req.method().clone();
    let uri = &req.uri().clone();

    let res = next.run(req).await;

    let now = Utc::now().time();

    let elapsed = now - time_start;

    let method_type: CustomType = match method.clone() {
        Method::GET => CustomType::GET,
        Method::PUT => CustomType::PUT,
        Method::POST => CustomType::POST,
        Method::PATCH => CustomType::PATCH,
        Method::DELETE => CustomType::DELETE,

        _ => CustomType::WARN,
    };

    let temp_output = format!(
        "{}{}{}{} {} ({} MS)",
        ForegroundColors::Magenta,
        Colors::Bold,
        uri.path(),
        Colors::Reset,
        res.status(),
        elapsed.num_milliseconds()
    );

    let output = temp_output.as_str();

    custom(method_type, method.as_str(), output);

    return res;
}
