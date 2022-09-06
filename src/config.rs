use std::{
    fs,
    path::{Path, PathBuf},
};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// An error that can occur when creating a config.
#[derive(Error, Debug)]
pub enum Error {
    /// An io error occurred.
    #[error("{0}")]
    Io(#[from] std::io::Error),
    /// A toml error occurred.
    #[error("{0}")]
    Toml(#[from] toml::de::Error),
}

/// The default configuration path.
///
/// This is <home dir>/.config/unii/config.toml.
pub static DEFAULT_CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".config/unii/config.toml")
});

/// Type for configuration options.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    /// The path to the root of the university work directory.
    pub directory: PathBuf,
}

impl Config {
    /// Create a new configuration with the given directory.
    ///
    /// # Errors
    ///
    /// This will error if the file cannot be read, or if the file is not valid TOML.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let s = fs::read_to_string(path)?;
        toml::from_str(&s).map_err(Into::into)
    }

    /// Create a new configuration with the given directory.
    #[must_use]
    pub fn from_dir<D: AsRef<Path>>(directory: D) -> Self {
        Self {
            directory: directory.as_ref().into(),
        }
    }
}
