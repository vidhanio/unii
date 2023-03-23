use clap::Parser;

use super::Course;
use crate::Settings;

#[derive(Parser)]
pub struct Args {}

pub fn run(settings: &Settings, _: Args) -> crate::Result<()> {
    let courses = Course::all(settings)?;

    for course in courses {
        println!("{}", course?.code());
    }

    Ok(())
}
