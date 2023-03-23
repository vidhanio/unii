use std::{
    fs,
    path::{Path, PathBuf},
};

use config::Config;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static CONFIG_DIR: Lazy<PathBuf> = Lazy::new(|| {
    dirs::config_dir()
        .expect("could not find config directory")
        .join("unii")
});

pub static DEFAULT_SETTINGS_FILE: Lazy<PathBuf> = Lazy::new(|| CONFIG_DIR.join("settings.toml"));

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub path: PathBuf,
}

impl Settings {
    pub fn open_or_create_default_at<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        Self::default().open_or_create_at(path)
    }

    pub fn open_or_create_at<P: AsRef<Path>>(self, path: P) -> crate::Result<Self> {
        if let Some(settings) = Self::try_open_from(path.as_ref())? {
            Ok(settings)
        } else {
            self.create_at(path)?;
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

        fs::write(path, toml::to_string(&self)?).map_err(Into::into)
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
