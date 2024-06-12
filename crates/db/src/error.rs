use std::string::FromUtf8Error;

use base64::DecodeError;
use thiserror::Error;

use crate::force_error_from;

#[derive(Debug, Error)]
pub enum DbError {
    #[error(transparent)]
    Connection(#[from] diesel::ConnectionError),

    #[error(transparent)]
    Diesel(#[from] diesel::result::Error),

    #[error(transparent)]
    Deadpool(#[from] diesel_async::pooled_connection::deadpool::PoolError),

    #[error(transparent)]
    PoolBuild(#[from] deadpool::managed::BuildError),

    #[error(transparent)]
    Validation(#[from] ValidationError),

    #[error(transparent)]
    StringConvert(#[from] FromUtf8Error),

    #[error(transparent)]
    Base64(#[from] DecodeError),

    #[error("argon2 error")]
    Argon2(argon2::Error),

    #[error("password hashing error")]
    Password(argon2::password_hash::Error),

    #[error("birthday conversion failed")]
    BirthdayConversionFailed,
}

force_error_from!(
    DbError:
        Argon2 => argon2::Error;
        Password => argon2::password_hash::Error;
);

#[derive(Debug, Clone, Copy, Error)]
pub enum ValidationError {
    #[error("insufficient privileges")]
    Permissions,

    #[error("incorrect password")]
    IncorrectPassword,
}
