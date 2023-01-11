pub mod config;

pub mod custom;
pub mod debug;
pub mod error;
pub mod info;
pub mod logger;
pub mod warn;

pub use debug::debug;
pub use error::error;
pub use info::info;
pub use warn::warn;

pub use custom::custom;
pub use logger::Logger;
