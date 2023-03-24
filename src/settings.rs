use std::{
    fs,
    path::{Path, PathBuf},
};

use config::Config;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

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
    pub fn open_or_create_at<P: AsRef<Path>>(self, path: P) -> crate::Result<Self> {
        let path = path.as_ref();
        if let Some(settings) = Self::try_open_from(path)? {
            Ok(settings)
        } else {
            self.create_at(path)?;
            println!("Created settings file at: {}", path.display());
            Ok(self)
        }
    }

    pub fn try_open_from<P: AsRef<Path>>(path: P) -> crate::Result<Option<Self>> {
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

    pub fn create_at<P: AsRef<Path>>(&self, path: P) -> crate::Result<()> {
        fs::create_dir_all(&*CONFIG_DIR)?;

        fs::write(path, serde_yaml::to_string(&self)?).map_err(Into::into)
    }

    pub fn template_dir(&self) -> PathBuf {
        self.path.join("templates")
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
