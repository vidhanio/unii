use std::{
    collections::HashMap, fmt::Write, fs, path::PathBuf, process::Command, string::ToString,
};

use heck::ToPascalCase;
use serde::{Deserialize, Serialize};
use tinytemplate::TinyTemplate;

use crate::{Course, Error, Settings};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Template {
    #[serde(skip)]
    name: String,
    pluralized_name: String,

    context_parameters: Vec<String>,
    directory_name: String,
    files: HashMap<String, String>,
    command: String,
}

impl Template {
    pub fn from_name(
        settings: &Settings,
        name: String,
        pluralized_name: Option<String>,
    ) -> crate::Result<Self> {
        let pluralized_name = pluralized_name.unwrap_or_else(|| format!("{name}s"));

        let template = Self {
            name,
            pluralized_name,
            context_parameters: Vec::new(),
            directory_name: String::new(),
            files: HashMap::new(),
            command: String::new(),
        };

        template.create(settings)?;

        Ok(template)
    }

    pub fn create(&self, settings: &Settings) -> crate::Result<()> {
        let path = self.path(settings);

        if path.exists() {
            return Err(Error::TemplateAlreadyExists(self.name.clone()));
        }

        fs::create_dir_all(settings.template_dir())?;

        self.write(settings)
    }

    pub fn write(&self, settings: &Settings) -> crate::Result<()> {
        fs::write(self.path(settings), serde_yaml::to_string(&self)?)?;

        Ok(())
    }

    pub fn open(settings: &Settings, name: &str) -> crate::Result<Option<Self>> {
        let path = settings
            .path
            .join("templates")
            .join(name)
            .with_extension("yml");

        if !path.exists() {
            return Ok(None);
        }

        let yaml = fs::read_to_string(path)?;

        Ok(Some(serde_yaml::from_str(&yaml)?))
    }

    pub fn run(
        &self,
        settings: &Settings,
        course: &Course,
        context: &HashMap<String, serde_json::Value>,
    ) -> crate::Result<()> {
        let mut tt = TinyTemplate::new();

        tt.set_default_formatter(&tinytemplate::format_unescaped);
        tt.add_formatter("PascalCase", |v, s| {
            let pascal = match v {
                serde_json::Value::String(s) => s.to_pascal_case(),
                _ => {
                    return tinytemplate::format_unescaped(v, s);
                }
            };

            write!(s, "{pascal}").map_err(Into::into)
        });

        tt.add_template(&self.directory_name, &self.directory_name)?;
        let rendered_directory_name = tt.render(&self.directory_name, &context)?;

        let directory = course
            .path(settings)
            .join(self.pluralized_name())
            .join(&rendered_directory_name);

        if directory.exists() {
            return Err(Error::TemplateDirectoryAlreadyExists(
                rendered_directory_name,
            ));
        }

        tt.add_template(&self.command, &self.command)?;
        let rendered_command = tt.render(&self.command, &context)?;

        let rendered_files = &self
            .files
            .iter()
            .map(|(path, content)| {
                tt.add_template(path, path)?;
                tt.add_template(content, content)?;

                let rendered_path = tt.render(path, &context)?;
                let rendered_content = tt.render(content, &context)?;

                Ok((rendered_path, rendered_content))
            })
            .collect::<crate::Result<Vec<_>>>()?;

        fs::create_dir_all(&directory)?;

        let output = Command::new("sh")
            .arg("-c")
            .arg(&rendered_command)
            .current_dir(&directory)
            .output()?;

        if !output.status.success() {
            return Err(Error::TemplateCommandFailed(
                rendered_command,
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        for (path, content) in rendered_files {
            let full_path = directory.join(path);

            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::write(full_path, content)?;
        }

        Ok(())
    }

    pub fn all(
        settings: &Settings,
    ) -> crate::Result<impl Iterator<Item = crate::Result<Self>> + '_> {
        Ok(fs::read_dir(settings.template_dir())?.filter_map(|entry| {
            let path = match entry {
                Ok(entry) => entry.path(),
                Err(err) => return Some(Err(err.into())),
            };

            if !path.is_file() {
                return None;
            }

            let name = match path.file_stem() {
                Some(name) => name.to_string_lossy(),
                None => return None,
            };

            if !path.extension().map(|ext| ext == "yml").unwrap_or_default() {
                return None;
            }

            Some(Self::open(settings, &name).map(|option| option.expect("template should exist")))
        }))
    }

    pub fn path(&self, settings: &Settings) -> PathBuf {
        settings
            .path
            .join("templates")
            .join(&self.name)
            .with_extension("yml")
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn pluralized_name(&self) -> &str {
        &self.pluralized_name
    }

    pub fn context_parameters(&self) -> &[String] {
        &self.context_parameters
    }
}
