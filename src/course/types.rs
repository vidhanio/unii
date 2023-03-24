use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{Error, Settings};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Course {
    #[serde(skip)]
    code: String,
    name: Option<String>,
}

impl Course {
    pub fn new(settings: &Settings, code: String, name: String) -> crate::Result<Self> {
        let course = Self {
            code,
            name: Some(name),
        };

        course.create(settings)?;

        Ok(course)
    }

    pub fn from_code(settings: &Settings, code: String) -> crate::Result<Self> {
        let course = Self { code, name: None };

        course.create(settings)?;

        Ok(course)
    }

    pub fn create(&self, settings: &Settings) -> crate::Result<()> {
        let path = self.path(settings);

        if path.exists() {
            return Err(Error::CourseAlreadyExists(self.code.clone()));
        }

        fs::create_dir_all(&path)?;

        self.write(settings)
    }

    pub fn write(&self, settings: &Settings) -> crate::Result<()> {
        fs::write(self.yaml_path(settings), serde_yaml::to_string(&self)?)?;

        Ok(())
    }

    pub fn open(settings: &Settings, code: &str) -> crate::Result<Option<Self>> {
        let path = settings.path.join(code);

        if !path.exists() {
            return Ok(None);
        }

        let yaml = fs::read_to_string(path.join("course.yml"))?;

        Ok(Some(Self {
            code: code.to_string(),
            ..serde_yaml::from_str(&yaml)?
        }))
    }

    pub fn all(
        settings: &Settings,
    ) -> crate::Result<impl Iterator<Item = crate::Result<Self>> + '_> {
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

            if !path.join("course.yml").exists() {
                return None;
            }

            Some(Self::open(settings, &code).map(|option| option.expect("course should exist")))
        }))
    }

    pub fn path(&self, settings: &Settings) -> PathBuf {
        settings.path.join(&self.code)
    }

    pub fn yaml_path(&self, settings: &Settings) -> PathBuf {
        self.path(settings).join("course.yml")
    }

    pub fn code(&self) -> &str {
        &self.code
    }
}
