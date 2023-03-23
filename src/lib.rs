#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
// #![warn(clippy::cargo)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs
)]

//! unii: A cli university work management tool

mod course;
mod error;
mod settings;

use std::path::PathBuf;

use clap::Parser;

use self::error::Result;
use self::settings::{Settings, DEFAULT_SETTINGS_FILE};

pub use self::error::Error;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,

    /// The path to the settings file
    #[clap(short, long, default_value = DEFAULT_SETTINGS_FILE.as_os_str())]
    settings_file: PathBuf,
}

#[derive(Parser)]
enum Command {
    /// Manage courses
    Course(course::Args),
}

/// Main entrypoint to the cli.
///
/// # Errors
///
/// This function will return an error if parsing the command line arguments fails,
/// if the settings file cannot be opened or created, or if the command fails.
pub fn run() -> crate::Result<()> {
    let args = Args::parse();
    let settings = Settings::open_or_create_default_at(&args.settings_file)?;

    match args.command {
        Command::Course(args) => course::run(&settings, args),
    }
}
