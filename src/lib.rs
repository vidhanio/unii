//! A university work management tool.

#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(missing_copy_implementations, missing_debug_implementations)]
#![warn(missing_docs)]

/// Configuration for the application.
pub mod config;
/// Types and implementations related to courses.
pub mod course;
mod error;

pub use error::Error;

/// A convinient result type for this crate.
pub type Result<T, E = Error> = std::result::Result<T, E>;
