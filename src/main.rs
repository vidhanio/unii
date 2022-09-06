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

fn main() {
    let args = Arguments::parse();

    let config = match (args.directory, args.config_file) {
        (Some(directory), _) => Config::from_dir(directory),
        (_, Some(config_file)) => Config::from_file(config_file).unwrap(),
        (None, None) => {
            create_dir_all(config::DEFAULT_CONFIG_PATH.parent().unwrap()).unwrap();

            if !config::DEFAULT_CONFIG_PATH.exists() {
                fs::write(&*config::DEFAULT_CONFIG_PATH, "").unwrap();
            }

            Config::from_file(&*config::DEFAULT_CONFIG_PATH).unwrap()
        }
    };

    // debug
    match args.subcommand {
        Subcommand::Course(CourseSubcommand::Create {
            code,
            name,
            description,
        }) => {
            let course = Course::new(code)
                .with_name(name.unwrap_or_else(|| "Untitled".to_string()))
                .with_description(description.unwrap_or_else(|| "No description".to_string()));

            println!("{:#?}", course);
        }
        Subcommand::Course(CourseSubcommand::Delete { code }) => {
            println!("{:#?}", code);
        }
        Subcommand::Course(CourseSubcommand::List) => {
            println!("List");
        }
    }
}
