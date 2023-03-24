use clap::Parser;

use super::Template;
use crate::Settings;

#[derive(Parser)]
pub struct Args {}

pub fn run(settings: &Settings, _: Args) -> color_eyre::Result<()> {
    for template in Template::all(settings)? {
        println!("{}", template?.name());
    }

    Ok(())
}
