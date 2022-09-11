use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use url::Url;

use crate::{config::Config, Error, Result};

/// A course.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Course {
    code: String,
    information: Information,
}

impl Course {
    /// Create a new course.
    ///
    /// # Errors
    ///
    /// This will error if creating the directory fails.
    pub fn create(config: &Config, code: &str, information: Option<Information>) -> Result<Self> {
        if config.directory.join(code).exists() {
            Err(Error::CourseAlreadyExists(code.into()))
        } else {
            config
                .course_code_regex
                .as_ref()
                .map(|regex| {
                    regex.is_match(code).then_some(()).ok_or_else(|| {
                        Error::CourseCodeDidNotMatchRegex {
                            code: code.into(),
                            regex: regex.to_string(),
                        }
                    })
                })
                .transpose()?;

            let course = Self {
                code: code.into(),
                information: information.unwrap_or_default(),
            };

            course.write(config).map(|_| course)
        }
    }

    /// Open an existing course.
    ///
    /// # Errors
    ///
    /// This will error if the course does not exist.
    pub fn open(config: &Config, code: &str) -> Result<Option<Self>> {
        let path = config.directory.join(code);

        Ok(if path.exists() {
            Some(Self {
                code: code.into(),
                information: toml::from_str(&fs::read_to_string(path.join("information.toml"))?)?,
            })
        } else {
            None
        })
    }

    /// Opens a course, or creates it if it does not exist.
    ///
    /// # Errors
    ///
    /// This will error if opening or creating the course fails.
    pub fn open_or_create(
        config: &Config,
        code: &str,
        information: Option<Information>,
    ) -> Result<Self> {
        Self::open(config, code)?.map_or_else(|| Self::create(config, code, information), Ok)
    }

    /// Get the path to the course.
    #[must_use]
    pub fn path(&self, config: &Config) -> PathBuf {
        config.directory.join(&self.code)
    }

    /// Write the course to the filesystem.
    ///
    /// Note that this will overwrite the course if it already exists.
    ///
    /// # Errors
    ///
    /// This will error if writing the course fails.
    fn write(&self, config: &Config) -> Result<()> {
        fs::create_dir_all(self.path(config))?;

        self.write_information(config)
    }

    /// Delete the course.
    ///
    /// # Errors
    ///
    /// This will error if deleting the directory fails.
    pub fn delete(self, config: &Config) -> Result<()> {
        fs::remove_dir_all(self.path(config)).map_err(Into::into)
    }

    /// Get the course's code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Get the course's information.
    #[must_use]
    pub const fn information(&self) -> &Information {
        &self.information
    }

    /// Get the path to the information file.
    #[must_use]
    pub fn information_path(&self, config: &Config) -> PathBuf {
        self.path(config).join("information.toml")
    }

    /// Write the course's information to the information file.
    ///
    /// # Errors
    ///
    /// This will error if writing the file or serializing the information fails.
    fn write_information(&self, config: &Config) -> Result<()> {
        fs::write(
            self.information_path(config),
            toml::to_string(&self.information)?,
        )
        .map_err(Into::into)
    }

    /// Set the course's information.
    ///
    /// # Errors
    ///
    /// This will error if writing the file or serializing the information fails.
    pub fn set_information(&mut self, config: &Config, information: Information) -> Result<()> {
        self.information = information;

        self.write_information(config)
    }

    /// List all courses.
    ///
    /// # Errors
    ///
    /// This will error if reading the directory fails.
    pub fn list(config: &Config) -> Result<impl Iterator<Item = Result<Self>> + '_> {
        fs::read_dir(&config.directory)
            .map(|i| {
                i.map(|entry| {
                    let entry = entry?;
                    let file_name = entry.file_name();
                    let code = file_name.to_string_lossy();

                    Self::open_or_create(config, &code, None)
                })
            })
            .map_err(Into::into)
    }
}

/// Information about a course.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Information {
    /// The course's name.
    pub name: Option<String>,

    /// The course's description.
    pub description: Option<String>,

    /// The course's website.
    pub url: Option<Url>,
}
