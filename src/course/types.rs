use std::{fs, path::PathBuf};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::{Error, Settings};

pub static COURSE_YAML: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("course.yml"));

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Course {
    #[serde(skip)]
    code: String,
    name: Option<String>,
}

impl Course {
    pub fn new(settings: &Settings, code: String, name: String) -> color_eyre::Result<Self> {
        let course = Self {
            code,
            name: Some(name),
        };

        course.create(settings)?;

        Ok(course)
    }

    pub fn from_code(settings: &Settings, code: String) -> color_eyre::Result<Self> {
        let course = Self { code, name: None };

        course.create(settings)?;

        Ok(course)
    }

    pub fn create(&self, settings: &Settings) -> color_eyre::Result<()> {
        let path = self.dir(settings);

        if path.exists() {
            Err(Error::CourseAlreadyExists(self.code.clone()))?;
        }

        fs::create_dir_all(self.unii_dir(settings))?;

        self.write(settings)
    }

    pub fn write(&self, settings: &Settings) -> color_eyre::Result<()> {
        fs::write(self.yaml_path(settings), serde_yaml::to_string(&self)?)?;

        Ok(())
    }

    pub fn open(settings: &Settings, code: &str) -> color_eyre::Result<Option<Self>> {
        let dir = settings.course_dir(code);

        if !dir.exists() {
            return Ok(None);
        }

        let yaml = fs::read_to_string(settings.course_yaml_path(code))?;

        Ok(Some(Self {
            code: code.to_string(),
            ..serde_yaml::from_str(&yaml)?
        }))
    }

    pub fn all(
        settings: &Settings,
    ) -> color_eyre::Result<impl Iterator<Item = color_eyre::Result<Self>> + '_> {
        Ok(fs::read_dir(&settings.path)?.filter_map(|entry| {
            let path = match entry {
                Ok(entry) => entry.path(),
                Err(err) => return Some(Err(err.into())),
            };

            if !path.is_dir() {
                return None;
            }

            let code = match path.file_name() {
                Some(code) => code.to_string_lossy(),
                None => return None,
            };

            if !settings.course_yaml_path(&code).exists() {
                return None;
            }

            Some(Self::open(settings, &code).map(|option| option.expect("course should exist")))
        }))
    }

    pub fn dir(&self, settings: &Settings) -> PathBuf {
        settings.course_dir(&self.code)
    }

    pub fn unii_dir(&self, settings: &Settings) -> PathBuf {
        settings.course_unii_dir(&self.code)
    }

    pub fn template_dir(&self, settings: &Settings) -> PathBuf {
        settings.course_template_dir(&self.code)
    }

    pub fn yaml_path(&self, settings: &Settings) -> PathBuf {
        settings.course_yaml_path(&self.code)
    }

    pub fn code(&self) -> &str {
        &self.code
    }
}
