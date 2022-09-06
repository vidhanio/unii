use std::{
    fmt::{self, Display, Formatter},
    path::PathBuf,
    str::FromStr,
};

use thiserror::Error;

/// A course year.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Year {
    /// First year.
    First,
    /// Second year.
    Second,
    /// Third year.
    Third,
    /// Fourth year.
    Fourth,
    /// Other year.
    Other(u32),
}

impl From<u32> for Year {
    fn from(year: u32) -> Self {
        match year {
            1 => Self::First,
            2 => Self::Second,
            3 => Self::Third,
            4 => Self::Fourth,
            _ => Self::Other(year),
        }
    }
}

impl From<Year> for u32 {
    fn from(year: Year) -> Self {
        match year {
            Year::First => 1,
            Year::Second => 2,
            Year::Third => 3,
            Year::Fourth => 4,
            Year::Other(year) => year,
        }
    }
}

impl Display for Year {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", u32::from(*self))
    }
}

/// A course code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Code {
    /// The year of the course.
    pub year: Year,
    /// The rest of the course code.
    pub rest: String,
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{}", self.year, self.rest)
    }
}

impl FromStr for Code {
    type Err = CodeError;

    fn from_str(s: &str) -> Result<Self, CodeError> {
        let mut chars = s.chars();
        let year = chars
            .next()
            .ok_or(CodeError::InvalidLength)?
            .to_string()
            .parse::<u32>()
            .map_err(|_| CodeError::InvalidYear)?
            .into();
        let rest = chars.collect::<String>();

        Ok(Self { year, rest })
    }
}

/// An error that can occur when parsing a course code.
#[derive(Error, Debug, Copy, Clone, PartialEq, Eq)]
pub enum CodeError {
    /// The code is not long enough.
    InvalidLength,
    /// The course code has an invalid year.
    InvalidYear,
}

impl Display for CodeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "invalid length"),
            Self::InvalidYear => write!(f, "invalid year"),
        }
    }
}

/// A course.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Course {
    code: Code,
    name: Option<String>,
    description: Option<String>,
}

impl Course {
    /// Create a new course.
    #[must_use]
    pub const fn new(code: Code) -> Self {
        Self {
            code,
            name: None,
            description: None,
        }
    }

    /// Add a name to the course.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Add a description to the course.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Get the course code.
    #[must_use]
    pub const fn code(&self) -> &Code {
        &self.code
    }

    /// Get the course name.
    #[must_use]
    pub const fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Get the course description.
    #[must_use]
    pub const fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Set the course name.
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Set the course description.
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    /// Clear the course name.
    pub fn clear_name(&mut self) {
        self.name = None;
    }

    /// Clear the course description.
    pub fn clear_description(&mut self) {
        self.description = None;
    }
}

impl From<Course> for PathBuf {
    fn from(course: Course) -> Self {
        let mut path = Self::new();
        path.push(course.code().to_string());
        path.set_extension("md");

        path
    }
}

#[cfg(test)]
mod tests;
