use std::io;

use thiserror::Error;

/// An error that can occur in this crate
#[derive(Debug, Error)]
pub enum Error {
    /// A course already exists
    #[error("course with code already exists: {0}")]
    CourseAlreadyExists(String),

    /// A course does not exist
    #[error("course with code does not exist: {0}")]
    CourseDoesNotExist(String),

    /// A template already exists
    #[error("template already exists: {0}")]
    TemplateAlreadyExists(String),

    /// A template does not exist
    #[error("template does not exist: {0}")]
    TemplateDoesNotExist(String),

    /// A template directory already exists
    #[error("template directory already exists: {0}")]
    TemplateDirectoryAlreadyExists(String),

    /// A template context parameter does not exist
    #[error("template context parameter does not exist: {0}")]
    TemplateContextParameterDoesNotExist(String),

    /// A template command is empty
    #[error("template command is empty")]
    TemplateCommandIsEmpty,

    /// A template command failed
    #[error("template command failed: {0}\n{1}")]
    TemplateCommandFailed(String, String),

    /// An IO error
    #[error("io error")]
    Io(#[from] io::Error),

    /// A config error
    #[error("config error")]
    Config(#[from] config::ConfigError),

    /// A YAML error
    #[error("yaml error")]
    TomlSer(#[from] serde_yaml::Error),

    /// A tinytemplate error
    #[error("tinytemplate error")]
    TinyTemplate(#[from] tinytemplate::error::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
