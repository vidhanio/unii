use std::{fs, path::PathBuf};

use clap::{ArgGroup, Parser};
use color_eyre::eyre::{self, bail};
use unii::{
    config::{self, Config},
    course::{Course, Information},
};
use url::Url;

#[derive(clap::Parser)]
#[clap(group(
            ArgGroup::new("config")
                .args(&["directory", "config-file"]),
        ))]
struct Arguments {
    #[clap(short, long, value_parser)]
    directory: Option<PathBuf>,

    #[clap(short, long, value_parser)]
    config_file: Option<PathBuf>,

    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    #[clap(subcommand)]
    Course(CourseSubcommand),
}

#[derive(clap::Subcommand)]
enum CourseSubcommand {
    Create {
        code: String,
        #[clap(short, long, value_parser)]
        name: Option<String>,
        #[clap(short, long, value_parser)]
        description: Option<String>,
        #[clap(short, long, value_parser)]
        url: Option<Url>,
    },
    Delete {
        code: String,
    },
    List,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let args = Arguments::parse();

    let config = match (args.directory, args.config_file) {
        (Some(directory), _) => Config::from_dir(directory),
        (_, config_file) => {
            let config_file = config_file
                .as_ref()
                .unwrap_or(&*config::DEFAULT_CONFIG_FILE);

            fs::create_dir_all(
                config_file
                    .parent()
                    .expect("default config path should have a parent"),
            )?;

            if !config_file.exists() {
                fs::write(config_file, "")?;

                bail!(
                    "config file not found. created empty config file at `{}`, please edit it",
                    config_file.display()
                );
            }

            Config::from_file(&config_file)?
        }
    };

    match args.subcommand {
        Subcommand::Course(subcommand) => match subcommand {
            CourseSubcommand::Create {
                code,
                name,
                description,
                url,
            } => {
                Course::create(
                    &config,
                    &code,
                    Some(Information {
                        name,
                        description,
                        url,
                    }),
                )?;

                println!("created course `{code}`");

                Ok(())
            }
            CourseSubcommand::Delete { code } => {
                Course::open(&config, &code)?
                    .ok_or_else(|| eyre::eyre!("course `{code}` does not exist"))?
                    .delete(&config)?;

                println!("deleted course `{code}`");

                Ok(())
            }
            CourseSubcommand::List => Course::list(&config)?.try_for_each(|course| {
                let course = course?;

                print!("{}", course.code());

                course
                    .information()
                    .name
                    .as_ref()
                    .map_or_else(|| println!(), |name| println!(" ({})", name));

                Ok(())
            }),
        },
    }
}
