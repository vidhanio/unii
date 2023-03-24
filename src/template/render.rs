use std::convert::Infallible;

use clap::Parser;
use color_eyre::Report;

use super::Template;
use crate::{Course, Error, Settings};

#[derive(Parser)]
pub struct Args {
    /// The name of the template
    #[clap(value_parser = parse_course_template, name = "[COURSE_CODE:]TEMPLATE_NAME")]
    name: (Option<String>, String),

    /// The context to use when rendering the template
    #[clap(value_parser = parse_context, name = "KEY=VALUE")]
    context: Vec<(String, serde_json::Value)>,

    /// The course code to render the template under
    #[clap(short, long, aliases = ["course", "code"])]
    course_code: Option<String>,
}

fn parse_context(s: &str) -> Result<(String, serde_json::Value), String> {
    let (key, value) = s.split_once('=').ok_or("invalid context: missing '='")?;

    let value = serde_json::from_str(value)
        .unwrap_or_else(|_| serde_json::Value::String(value.to_string()));

    Ok((key.to_string(), value))
}

#[allow(clippy::unnecessary_wraps)]
fn parse_course_template(s: &str) -> Result<(Option<String>, String), Infallible> {
    Ok(s.split_once(':').map_or_else(
        || (None, s.to_owned()),
        |(course_code, template_name)| (Some(course_code.to_owned()), template_name.to_owned()),
    ))
}

pub fn run(settings: &Settings, args: Args) -> color_eyre::Result<()> {
    let (source_code, template_name) = args.name;

    let (source_code, course_code) = match (source_code, args.course_code) {
        (s, Some(c)) => (s, c),
        (Some(s), None) => (Some(s.clone()), s),
        (None, None) => Err(Error::TemplateCourseCodeMissing)?,
    };

    let source = source_code
        .as_ref()
        .map(|source| {
            Course::open(settings, source)?
                .ok_or_else(|| Report::from(Error::CourseDoesNotExist(source.clone())))
        })
        .transpose()?;

    let course =
        Course::open(settings, &course_code)?.ok_or(Error::CourseDoesNotExist(course_code))?;

    let template = Template::open(settings, source.as_ref(), &template_name)?
        .ok_or(Error::TemplateDoesNotExist(source_code, template_name))?;

    let context = args
        .context
        .into_iter()
        .map(|(key, value)| {
            if template.context_parameters().contains(&key) {
                Ok((key, value))
            } else {
                Err(Error::TemplateContextParameterDoesNotExist(key))
            }
        })
        .collect::<Result<_, _>>()?;

    template.render(settings, &course, &context)?;

    Ok(())
}
