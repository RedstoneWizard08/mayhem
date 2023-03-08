use super::{
    custom::{custom, CustomType},
    error, info, warn,
};

pub struct Logger;

impl Logger {
    pub fn new() -> Self {
        Self {}
    }

    pub fn info(&self, str: &str) {
        info(str);
    }

    pub fn warn(&self, str: &str) {
        warn(str);
    }

    pub fn error(&self, str: &str) {
        error(str);
    }

    pub fn custom(&self, r#type: CustomType, prefix: &str, str: &str) {
        custom(r#type, prefix, str);
    }
}

impl Clone for Logger {
    fn clone(&self) -> Self {
        Self {}
    }
}
