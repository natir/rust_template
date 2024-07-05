//! Error definition of rust_template

/* std use */

/* crate use */

/* project use */

/// Enum to define error
#[derive(std::fmt::Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Log(#[from] log::SetLoggerError),
}

/// Alias of result
// lib project result alias
// pub type Result<T> = core::result::Result<T, Error>;
// bin project result alias
pub type Result<T> = anyhow::Result<T>;
