mod list;
mod new;
mod types;

use clap::Parser;

use crate::Settings;

pub use self::types::{Course, COURSE_YAML};

#[derive(Parser)]
pub struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    /// Create a new course
    #[clap(aliases = ["create", "add"])]
    New(new::Args),

    /// List all courses
    #[clap(aliases = ["ls"])]
    List(list::Args),
}

pub fn run(settings: &Settings, args: Args) -> color_eyre::Result<()> {
    match args.command {
        Command::New(args) => new::run(settings, args),
        Command::List(args) => list::run(settings, args),
    }
}
