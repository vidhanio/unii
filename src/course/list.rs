use clap::Parser;

use super::Course;
use crate::Settings;

#[derive(Parser)]
pub struct Args {}

pub fn run(settings: &Settings, _: Args) -> color_eyre::Result<()> {
    for course in Course::all(settings)? {
        println!("{}", course?.code());
    }

    Ok(())
}
