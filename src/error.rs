use std::io;

use thiserror::Error;

/// An error that can occur in this crate
#[derive(Debug, Error)]
pub enum Error {
    /// A course already exists
    #[error("course with code already exists: {0}")]
    CourseAlreadyExists(String),

    /// An IO error
    #[error("io error")]
    Io(#[from] io::Error),

    /// A config error
    #[error("config error")]
    Config(#[from] config::ConfigError),

    /// A TOML serialization error
    #[error("toml serialization error")]
    TomlSer(#[from] toml::ser::Error),

    /// A TOML deserialization error
    #[error("toml deserialization error")]
    TomlDe(#[from] toml::de::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
