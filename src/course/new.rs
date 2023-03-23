use clap::Parser;

use super::Course;
use crate::Settings;

#[derive(Parser)]
pub struct Args {
    /// The course code of the course
    course_code: String,

    /// The name of the course
    #[clap(short, long)]
    name: Option<String>,
}

pub fn run(settings: &Settings, args: Args) -> crate::Result<()> {
    let course = match args.name {
        Some(name) => Course::new(settings, args.course_code, name)?,
        None => Course::from_code(settings, args.course_code)?,
    };

    println!("Created course: {}", course.code());

    Ok(())
}
