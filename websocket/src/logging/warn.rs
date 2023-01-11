use super::config::{BackgroundColors, Colors, ForegroundColors};

pub fn warn(value: &str) {
    println!(
        "{}{}{} WARN {} {}{}",
        BackgroundColors::Yellow,
        Colors::Bold,
        ForegroundColors::Black,
        Colors::Reset,
        value,
        Colors::Reset
    );
}
