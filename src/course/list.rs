use clap::Parser;

use super::Course;
use crate::Settings;

#[derive(Parser)]
pub struct Args {}

pub fn run(settings: &Settings, _: Args) -> crate::Result<()> {
    for course in Course::all(settings)? {
        println!("{}", course?.code());
    }

    Ok(())
}
