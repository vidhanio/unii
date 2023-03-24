mod file_tree;

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
    #[serde(deserialize_with = "file_tree::deserialize_into_hashmap")]
    files: HashMap<String, String>,
    command: String,
}

impl Template {
    pub fn from_name(
        settings: &Settings,
        name: String,
        pluralized_name: Option<String>,
    ) -> color_eyre::Result<Self> {
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

    pub fn create(&self, settings: &Settings) -> color_eyre::Result<()> {
        let path = self.path(settings);

        if path.exists() {
            Err(Error::TemplateAlreadyExists(self.name.clone()))?;
        }

        fs::create_dir_all(settings.template_dir())?;

        self.write(settings)
    }

    pub fn write(&self, settings: &Settings) -> color_eyre::Result<()> {
        fs::write(self.path(settings), serde_yaml::to_string(&self)?)?;

        Ok(())
    }

    pub fn open(
        settings: &Settings,
        source: Option<&Course>,
        name: &str,
    ) -> color_eyre::Result<Option<Self>> {
        let path = settings.template_path(source, name);

        if !path.exists() {
            return Ok(None);
        }

        let yaml = fs::read_to_string(path)?;

        Ok(Some(serde_yaml::from_str(&yaml)?))
    }

    pub fn render(
        &self,
        settings: &Settings,
        course: &Course,
        context: &HashMap<String, serde_json::Value>,
    ) -> color_eyre::Result<()> {
        let tt = self.tiny_template()?;

        let rendered_directory_name = tt.render(&self.directory_name, &context)?;

        let directory = course
            .dir(settings)
            .join(&self.pluralized_name)
            .join(&rendered_directory_name);

        if directory.exists() {
            Err(Error::RenderAlreadyExists(rendered_directory_name))?;
        }

        fs::create_dir_all(&directory)?;

        let rendered_command = tt.render(&self.command, &context)?;

        let output = Command::new("sh")
            .arg("-c")
            .arg(&rendered_command)
            .current_dir(&directory)
            .output()?;

        if !output.status.success() {
            Err(Error::TemplateCommandFailed(
                rendered_command,
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))?;
        }

        let rendered_files = self
            .files
            .iter()
            .map(|(path, content)| {
                let rendered_path = tt.render(path, &context)?;
                let rendered_content = tt.render(content, &context)?;

                Ok((rendered_path, rendered_content))
            })
            .collect::<color_eyre::Result<Vec<_>>>()?;

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
    ) -> color_eyre::Result<impl Iterator<Item = color_eyre::Result<Self>> + '_> {
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

            Some(
                Self::open(settings, None, &name) // TODO: add source
                    .map(|option| option.expect("template should exist")),
            )
        }))
    }

    pub fn tiny_template(&self) -> color_eyre::Result<TinyTemplate> {
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

        tt.add_template(&self.command, &self.command)?;

        for (path, content) in &self.files {
            tt.add_template(path, path)?;
            tt.add_template(content, content)?;
        }

        Ok(tt)
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

    pub fn context_parameters(&self) -> &[String] {
        &self.context_parameters
    }
}
