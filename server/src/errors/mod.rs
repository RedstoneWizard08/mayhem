use deadpool_postgres::PoolError;
use derive_more::{Display, From};
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::Error as PGError;

pub mod conflict;

#[derive(Debug, Display, From)]
pub enum AppError {
    NotFound,
    PGError(PGError),
    PGMError(PGMError),
    PoolError(PoolError),
}

impl std::error::Error for AppError {}
