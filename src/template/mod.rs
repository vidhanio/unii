mod list;
mod new;
mod run;
mod types;

use clap::Parser;

pub use self::types::Template;

use crate::Settings;

#[derive(Parser)]
pub struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    /// Create a new course
    #[clap(alias = "create", alias = "add")]
    New(new::Args),

    /// Run an item from a template
    #[clap(alias = "use")]
    Run(run::Args),

    /// List all templates
    #[clap(alias = "ls")]
    List(list::Args),
}

pub fn run(settings: &Settings, args: Args) -> crate::Result<()> {
    match args.command {
        Command::New(args) => new::run(settings, args),
        Command::Run(args) => run::run(settings, args),
        Command::List(args) => list::run(settings, args),
    }
}
