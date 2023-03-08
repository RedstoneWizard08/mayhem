use super::config::{BackgroundColors, Colors, ForegroundColors};

pub fn info(value: &str) {
    println!(
        "{}{}{} INFO {} {}{}",
        BackgroundColors::Cyan,
        Colors::Bold,
        ForegroundColors::Black,
        Colors::Reset,
        value,
        Colors::Reset
    );
}
