use super::config::{BackgroundColors, Colors, ForegroundColors};

pub fn error(value: &str) {
    println!(
        "{}{}{} ERROR {} {}{}",
        BackgroundColors::Red,
        Colors::Bold,
        ForegroundColors::White,
        Colors::Reset,
        value,
        Colors::Reset
    );
}
