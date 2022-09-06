use std::{
    fs::{self, create_dir_all},
    path::PathBuf,
};

use clap::{ArgGroup, Parser};
use unii::{
    config::{self, Config},
    course::{Code, Course},
};

#[derive(clap::Parser)]
#[clap(group(
            ArgGroup::new("config")
                .args(&["directory", "config-file"]),
        ))]
struct Arguments {
    #[clap(subcommand)]
    subcommand: Subcommand,

    #[clap(short, long, value_parser)]
    directory: Option<PathBuf>,

    #[clap(short, long, value_parser)]
    config_file: Option<PathBuf>,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    #[clap(subcommand)]
    Course(CourseSubcommand),
}

#[derive(clap::Subcommand)]
enum CourseSubcommand {
    Create {
        code: Code,
        #[clap(short, long, value_parser)]
        name: Option<String>,
        #[clap(short, long, value_parser)]
        description: Option<String>,
    },
    Delete {
        code: Code,
    },
    List,
}

impl CourseSubcommand {
    const fn code(&self) -> Option<&Code> {
        match self {
            Self::Create { code, .. } | Self::Delete { code } => Some(code),
            Self::List => None,
        }
    }
}

fn main() {
    let args = Arguments::parse();

    let config = match (args.directory, args.config_file) {
        (Some(directory), _) => Config::from_dir(directory),
        (_, Some(config_file)) => Config::from_file(config_file).unwrap(),
        (None, None) => {
            let config_file = config::DEFAULT_CONFIG_PATH;
            create_dir_all(
                config_file
                    .parent()
                    .expect("default config should have parent"),
            )
            .expect("creating directories should not fail");

            if !config_file.exists() {
                fs::write(&*config_file, "").expect("creating config should not fail");
            }

            Config::from_file(config_file).expect("reading config should not fail")
        }
    };

    match args.subcommand {
        Subcommand::Course(subcommand) => {
            match subcommand {
                CourseSubcommand::Create {
                    code,
                    name,
                    description,
                } => {
                    let mut course = Course::new(code);

                    if let Some(name) = name {
                        course.set_name(name);
                    }

                    if let Some(description) = description {
                        course.set_description(description);
                    }
                }
                CourseSubcommand::Delete { code } => {
                    println!("Deleting course with code {}", code);
                }
                CourseSubcommand::List => {
                    println!("Listing courses");
                }
            };
        }
    }
}
