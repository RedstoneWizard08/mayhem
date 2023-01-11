use std::{
    env,
    fs::{self, OpenOptions},
    io::prelude::*,
};

use super::config::{BackgroundColors, Colors, ForegroundColors};

static mut HAS_WRITTEN: bool = false;

pub fn debug(value: &str) {
    if &env::var("VERBOSE").unwrap_or("false".to_string()) == "true" {
        println!(
            "{}{}{} DEBUG {} {}{}",
            BackgroundColors::Yellow,
            Colors::Bold,
            ForegroundColors::Black,
            Colors::Reset,
            value,
            Colors::Reset
        );
    }

    let log = format!("[DEBUG] {}", value);
    let debug_log = env::current_dir().unwrap().join("debug.log");

    unsafe {
        if !HAS_WRITTEN {
            fs::write(
                debug_log,
                "========== Mayhem WebSocket Server Debug Log ==========\n",
            ).unwrap();

            HAS_WRITTEN = true;
        }
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(env::current_dir().unwrap().join("debug.log"))
        .unwrap();

    writeln!(file, "{}", log).unwrap();
}
