mod file_tree;

use std::{collections::HashMap, fs, path::PathBuf, process::Command, string::ToString};

use heck::{
    ToKebabCase, ToLowerCamelCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase, ToTrainCase,
    ToUpperCamelCase,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tera::{Context, Tera};

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
        context: &HashMap<String, Value>,
    ) -> color_eyre::Result<()> {
        let mut tera = Self::tera();
        let context = Context::from_serialize(context)?;
        let mut render = |s: &str| tera.render_str(s, &context);

        let directory_name = render(&self.directory_name)?;

        let directory = course
            .dir(settings)
            .join(&self.pluralized_name)
            .join(&directory_name);

        if directory.exists() {
            Err(Error::RenderAlreadyExists(directory_name))?;
        }

        fs::create_dir_all(&directory)?;

        let command = render(&self.command)?;

        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .current_dir(&directory)
            .output()?;

        if !output.status.success() {
            Err(Error::TemplateCommandFailed(
                command,
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))?;
        }

        let rendered_files = self
            .files
            .iter()
            .map(|(path, content)| {
                let rendered_path = render(path)?;
                let rendered_content = render(content)?;

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

    fn tera() -> Tera {
        type Case = fn(&str) -> String;

        let mut tt = Tera::default();

        let cases: &[(&[_], Case)] = &[
            (
                &["UpperCamelCase", "PascalCase"],
                ToUpperCamelCase::to_upper_camel_case,
            ),
            (
                &["lowerCamelCase", "camelCase"],
                ToLowerCamelCase::to_lower_camel_case,
            ),
            (
                &["snake_case", "lower_snake_case"],
                ToSnakeCase::to_snake_case,
            ),
            (
                &["kebab-case", "lower-kebab-case"],
                ToKebabCase::to_kebab_case,
            ),
            (
                &[
                    "SHOUTY_SNAKE_CASE",
                    "UPPER_SNAKE_CASE",
                    "SCREAMING_SNAKE_CASE",
                ],
                ToShoutySnakeCase::to_shouty_snake_case,
            ),
            (
                &[
                    "shouty-kebab-case",
                    "upper-kebab-case",
                    "screaming-kebab-case",
                ],
                ToShoutyKebabCase::to_shouty_kebab_case,
            ),
            (
                &["Train-Case", "Title-Kebab-Case"],
                ToTrainCase::to_train_case,
            ),
        ];

        for (names, case) in cases {
            for name in *names {
                tt.register_filter(
                    name,
                    Box::new(move |value: &Value, args: &HashMap<String, Value>| {
                        let value = tera::try_get_value!(name, "value", String, value);
                        if !args.is_empty() {
                            return Err(
                                format!("Filter `{name}` does not accept any arguments").into()
                            );
                        }

                        Ok(Value::String(case(&value)))
                    }),
                );
            }
        }

        tt
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
