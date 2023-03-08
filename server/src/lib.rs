#![allow(unused_must_use, unused_assignments)]
#![feature(proc_macro_hygiene, decl_macro, arc_unwrap_or_clone, async_closure)]

pub mod config;
pub mod database;
pub mod errors;
pub mod logging;
pub mod middleware;
pub mod routes;
pub mod server;
pub mod util;
pub mod ws;
pub mod state;
