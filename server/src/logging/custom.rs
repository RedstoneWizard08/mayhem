use super::config::{BackgroundColors, Colors, ForegroundColors};

pub enum CustomType {
    INFO,
    WARN,
    ERROR,

    GET,
    PUT,
    POST,
    PATCH,
    DELETE,
}

pub fn custom(r#type: CustomType, prefix: &str, value: &str) {
    match r#type {
        CustomType::INFO => {
            println!(
                "{}{}{} {} {} {}{}",
                BackgroundColors::Cyan,
                Colors::Bold,
                ForegroundColors::Black,
                prefix,
                Colors::Reset,
                value,
                Colors::Reset
            );
        }

        CustomType::WARN => {
            println!(
                "{}{}{} {} {} {}{}",
                BackgroundColors::Yellow,
                Colors::Bold,
                ForegroundColors::Black,
                prefix,
                Colors::Reset,
                value,
                Colors::Reset
            );
        }

        CustomType::ERROR => {
            println!(
                "{}{}{} {} {} {}{}",
                BackgroundColors::Red,
                Colors::Bold,
                ForegroundColors::White,
                prefix,
                Colors::Reset,
                value,
                Colors::Reset
            );
        }

        // ===================================================================================
        CustomType::GET => {
            println!(
                "{}{}{} {} {} {}{}",
                BackgroundColors::Green,
                Colors::Bold,
                ForegroundColors::Black,
                prefix,
                Colors::Reset,
                value,
                Colors::Reset
            );
        }

        CustomType::PUT => {
            println!(
                "{}{}{} {} {} {}{}",
                BackgroundColors::Light_Blue,
                Colors::Bold,
                ForegroundColors::Black,
                prefix,
                Colors::Reset,
                value,
                Colors::Reset
            );
        }

        CustomType::POST => {
            println!(
                "{}{}{} {} {} {}{}",
                BackgroundColors::Magenta,
                Colors::Bold,
                ForegroundColors::Black,
                prefix,
                Colors::Reset,
                value,
                Colors::Reset
            );
        }

        CustomType::PATCH => {
            println!(
                "{}{}{} {} {} {}{}",
                BackgroundColors::Light_Yellow,
                Colors::Bold,
                ForegroundColors::Black,
                prefix,
                Colors::Reset,
                value,
                Colors::Reset
            );
        }

        CustomType::DELETE => {
            println!(
                "{}{}{} {} {} {}{}",
                BackgroundColors::Light_Red,
                Colors::Bold,
                ForegroundColors::White,
                prefix,
                Colors::Reset,
                value,
                Colors::Reset
            );
        }
    }
}
