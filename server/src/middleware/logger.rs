use rocket::{
    async_trait,
    fairing::{Fairing, Info, Kind},
    http::Method,
    Request, Response,
};

use crate::logging::{
    config::{Colors, ForegroundColors},
    custom::{custom, CustomType},
};

pub struct LoggingMiddleware;

#[async_trait]
impl Fairing for LoggingMiddleware {
    fn info(&self) -> Info {
        return Info {
            name: "Logger Middleware",
            kind: Kind::Response,
        };
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let mut method_type = CustomType::WARN;

        match req.method() {
            Method::Get => method_type = CustomType::GET,
            Method::Put => method_type = CustomType::PUT,
            Method::Post => method_type = CustomType::POST,
            Method::Patch => method_type = CustomType::PATCH,
            Method::Delete => method_type = CustomType::DELETE,

            _ => method_type = CustomType::WARN,
        }

        let temp_output = format!(
            "{}{}{}{} {}",
            ForegroundColors::Magenta,
            Colors::Bold,
            req.uri().path(),
            Colors::Reset,
            res.status(),
        );

        let output = temp_output.as_str();

        custom(method_type, req.method().as_str(), output);
    }
}
