use std::io;

use thiserror::Error;

/// An enum of errors that can occur.
#[derive(Error, Debug)]
pub enum Error {
    /// An io error occurred.
    #[error("io error")]
    Io(#[from] io::Error),

    /// A toml serialization error occurred.
    #[error("toml serialization error")]
    Ser(#[from] toml::ser::Error),

    /// A toml deserialization error occurred.
    #[error("toml deserialization error")]
    De(#[from] toml::de::Error),

    /// The course already exists.
    #[error("course with code `{0}` already exists")]
    CourseAlreadyExists(String),

    /// The course did not match the regex.
    #[error("course code `{code}` did not match regex `{regex}`")]
    CourseCodeDidNotMatchRegex {
        /// Course code which failed to match regex.
        code: String,

        /// Regex which the course code failed to match.
        regex: String,
    },
}
