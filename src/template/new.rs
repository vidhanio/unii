use clap::Parser;

use super::Template;
use crate::Settings;

#[derive(Parser)]
pub struct Args {
    /// The name of the template
    name: String,

    /// The pluralized name of the template
    #[clap(long)]
    pluralized_name: Option<String>,
}

pub fn run(settings: &Settings, args: Args) -> crate::Result<()> {
    let template = Template::from_name(settings, args.name, args.pluralized_name)?;

    println!("Created template: {}", template.name());

    Ok(())
}
