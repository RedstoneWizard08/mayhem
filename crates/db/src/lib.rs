use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use error::DbError;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate diesel_derive_enum;

#[macro_use]
pub extern crate diesel;
pub extern crate diesel_async;

pub mod error;
pub mod helpers;
pub mod macros;
pub mod migrate;
pub mod models;
pub mod relations;
pub mod schema;
pub mod token;
pub mod types;

pub type Result<T, E = DbError> = std::result::Result<T, E>;

pub fn connect(url: impl AsRef<str>) -> Result<Pool<AsyncPgConnection>> {
    Ok(
        Pool::builder(AsyncDieselConnectionManager::<AsyncPgConnection>::new(
            url.as_ref(),
        ))
        .build()?,
    )
}
