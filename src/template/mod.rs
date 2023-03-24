mod list;
mod new;
mod render;
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
    #[clap(aliases = ["create", "add"])]
    New(new::Args),

    /// Render a template
    #[clap(aliases = ["generate", "gen", "run", "use", "make"])]
    Render(render::Args),

    /// List all templates
    #[clap(aliases = ["ls"])]
    List(list::Args),
}

pub fn run(settings: &Settings, args: Args) -> color_eyre::Result<()> {
    match args.command {
        Command::New(args) => new::run(settings, args),
        Command::Render(args) => render::run(settings, args),
        Command::List(args) => list::run(settings, args),
    }
}
