mod list;
mod new;
mod types;

use clap::Parser;
use types::Course;

use crate::Settings;

#[derive(Parser)]
pub struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    /// Create a new course
    New(new::Args),

    /// List all courses
    #[clap(alias = "ls")]
    List(list::Args),
}

pub fn run(settings: &Settings, args: Args) -> crate::Result<()> {
    match args.command {
        Command::New(args) => new::run(settings, args),
        Command::List(args) => list::run(settings, args),
    }
}
