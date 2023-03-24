use std::{
    fs,
    path::{Path, PathBuf},
};

use config::Config;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::course::{Course, COURSE_YAML};

static CONFIG_DIR: Lazy<PathBuf> = Lazy::new(|| {
    dirs::config_dir()
        .expect("should have config directory")
        .join("unii")
});

pub static DEFAULT_SETTINGS_FILE: Lazy<PathBuf> = Lazy::new(|| CONFIG_DIR.join("settings.yml"));

pub static DEFAULT_COURSES_DIR: Lazy<PathBuf> = Lazy::new(|| {
    dirs::home_dir()
        .expect("should have home directory")
        .join("unii")
});

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Settings {
    pub path: PathBuf,
}

impl Settings {
    pub fn open_or_create_at<P: AsRef<Path>>(self, path: P) -> color_eyre::Result<Self> {
        let path = path.as_ref();
        if let Some(settings) = Self::try_open_from(path)? {
            Ok(settings)
        } else {
            self.create_at(path)?;
            println!("Created settings file at: {}", path.display());
            Ok(self)
        }
    }

    pub fn try_open_from<P: AsRef<Path>>(path: P) -> color_eyre::Result<Option<Self>> {
        let path = path.as_ref();

        if !path.exists() {
            return Ok(None);
        }

        let settings = Config::builder()
            .add_source(config::File::from(path))
            .build()?
            .try_deserialize()?;

        Ok(Some(settings))
    }

    pub fn create_at<P: AsRef<Path>>(&self, path: P) -> color_eyre::Result<()> {
        fs::create_dir_all(&*CONFIG_DIR)?;

        fs::write(path, serde_yaml::to_string(&self)?).map_err(Into::into)
    }

    pub fn course_dir(&self, code: &str) -> PathBuf {
        self.path.join(code)
    }

    pub fn course_unii_dir(&self, code: &str) -> PathBuf {
        self.course_dir(code).join(".unii")
    }

    pub fn course_template_dir(&self, code: &str) -> PathBuf {
        self.course_unii_dir(code).join("templates")
    }

    pub fn course_yaml_path(&self, code: &str) -> PathBuf {
        self.course_unii_dir(code).join(&*COURSE_YAML)
    }

    pub fn template_dir(&self) -> PathBuf {
        self.path.join(".unii").join("templates")
    }

    pub fn template_path(&self, source: Option<&Course>, name: &str) -> PathBuf {
        source
            .map_or_else(|| self.template_dir(), |source| source.template_dir(self))
            .join(name)
            .with_extension("yml")
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            path: dirs::home_dir()
                .expect("could not find home directory")
                .join("unii"),
        }
    }
}
