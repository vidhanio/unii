use clap::Parser;

use super::Template;
use crate::{Error, Settings, Course};

#[derive(Parser)]
pub struct Args {
    /// The name of the template
    name: String,

    /// The context to use when creating the item
    #[clap(value_parser = parse_context)]
    context: Vec<(String, serde_json::Value)>,

    /// The course code to create the item under
    #[clap(short, long, alias = "course", alias = "code")]
    course_code: String,

}

fn parse_context(s: &str) -> Result<(String, serde_json::Value), String> {
    let (key, value) = s.split_once('=').ok_or("invalid context: missing '='")?;

    let value = serde_json::from_str(value).
        unwrap_or_else(|_| serde_json::Value::String(value.to_string()));

    Ok((key.to_string(), value))
}




pub fn run(settings: &Settings, args: Args) -> crate::Result<()> {
    let Some(template) = Template::open(settings, &args.name)? else { 
        return Err(Error::TemplateDoesNotExist(args.name))
    };

    let Some(course) = Course::open(settings, &args.course_code)? else { 
        return Err(Error::CourseDoesNotExist(args.course_code))
    };

    let context = args.context.into_iter()
        .map(|(key, value)| if template.context_parameters().contains(&key) {
            Ok((key, value))
        } else {
            Err(Error::TemplateContextParameterDoesNotExist(key))
        })
        .collect::<crate::Result<_>>()?;

    template.run(settings, &course, &context)?;

    Ok(())
}
