#![feature(arc_unwrap_or_clone, associated_type_defaults)]

pub use migration::{MigrationTrait, Migrator, MigratorTrait};
pub use sea_orm;

pub mod client;
pub mod models;
pub mod util;

pub use client::Client;
