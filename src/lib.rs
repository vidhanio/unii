#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
// #![warn(clippy::cargo)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs
)]

//! unii: A command-line university work management tool.

mod course;
mod error;
mod settings;
mod template;

use std::path::PathBuf;

use clap::Parser;

use self::course::Course;
use self::settings::{Settings, DEFAULT_COURSES_DIR, DEFAULT_SETTINGS_FILE};

pub use self::error::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,

    /// The path to the settings file
    #[clap(long, default_value = DEFAULT_SETTINGS_FILE.as_os_str())]
    settings_file: PathBuf,

    /// The path to the directory where courses are stored
    #[clap(long, default_value = DEFAULT_COURSES_DIR.as_os_str())]
    courses_dir: PathBuf,
}

#[derive(Parser)]
enum Command {
    /// Manage courses
    Course(course::Args),

    /// Manage templates
    Template(template::Args),
}

/// Main entrypoint to the cli.
///
/// # Errors
///
/// This function will return an error if parsing the command line arguments fails,
/// if the settings file cannot be opened or created, or if the command fails.
pub fn run() -> color_eyre::Result<()> {
    let args = Args::parse();
    let settings = Settings {
        path: args.courses_dir,
    }
    .open_or_create_at(&args.settings_file)?;

    match args.command {
        Command::Course(args) => course::run(&settings, args),
        Command::Template(args) => template::run(&settings, args),
    }
}
