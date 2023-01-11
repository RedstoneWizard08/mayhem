use chrono::{DateTime, Utc};
use rocket::{
    async_trait,
    fairing::{Fairing, Info, Kind},
    http::{Header, Method},
    Data, Request, Response,
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
            kind: Kind::Response | Kind::Request,
        };
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        req.add_header(Header::new("Request-Send-Time", Utc::now().to_rfc3339()));
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let time_start =
            DateTime::parse_from_rfc3339(req.headers().get_one("Request-Send-Time").unwrap())
                .unwrap()
                .time();
        let now = Utc::now().time();

        let elapsed = now - time_start;

        let method_type: CustomType;

        match req.method() {
            Method::Get => method_type = CustomType::GET,
            Method::Put => method_type = CustomType::PUT,
            Method::Post => method_type = CustomType::POST,
            Method::Patch => method_type = CustomType::PATCH,
            Method::Delete => method_type = CustomType::DELETE,

            _ => method_type = CustomType::WARN,
        }

        let temp_output = format!(
            "{}{}{}{} {} ({} MS)",
            ForegroundColors::Magenta,
            Colors::Bold,
            req.uri().path(),
            Colors::Reset,
            res.status(),
            elapsed.num_milliseconds()
        );

        let output = temp_output.as_str();

        custom(method_type, req.method().as_str(), output);
    }
}
