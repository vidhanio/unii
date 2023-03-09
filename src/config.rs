use std::{
    fs,
    path::{Path, PathBuf},
};

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// The default configuration file's path.
///
/// This is <home dir>/.config/unii/config.toml.
pub static DEFAULT_CONFIG_FILE: Lazy<PathBuf> = Lazy::new(|| {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".config/unii/config.toml")
});

/// Type for configuration options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// The path to the root of the university work directory.
    pub directory: PathBuf,

    /// Regex to validate the course code.
    #[serde(with = "serde_regex")]
    pub course_code_regex: Option<Regex>,
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
            course_code_regex: None,
        }
    }
}

/// An error that can occur when creating a config.
#[derive(Error, Debug)]
pub enum Error {
    /// An io error occurred.
    #[error("io error")]
    Io(#[from] std::io::Error),
    /// A toml error occurred.
    #[error("toml error")]
    Toml(#[from] toml::de::Error),
}
