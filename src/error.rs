use thiserror::Error;

/// An error that can occur in this crate
#[derive(Debug, Error)]
pub enum Error {
    /// A course already exists
    #[error("course with code already exists: {0}")]
    CourseAlreadyExists(String),

    /// A course does not exist
    #[error("course with code does not exist: {0}")]
    CourseDoesNotExist(String),

    /// A template already exists
    #[error("template already exists: {0}")]
    TemplateAlreadyExists(String),

    /// A template does not exist
    #[error("template does not exist: {}{1}", .0.clone().map(|s| format!("{s}:")).unwrap_or_default())]
    TemplateDoesNotExist(Option<String>, String),

    /// A template context parameter does not exist
    #[error("template context parameter does not exist: {0}")]
    TemplateContextParameterDoesNotExist(String),

    /// A template command is empty
    #[error("template command is empty")]
    TemplateCommandIsEmpty,

    /// A template command failed
    #[error("template command failed: {0}\n{1}")]
    TemplateCommandFailed(String, String),

    /// A render already exists
    #[error("render already exists: {0}")]
    RenderAlreadyExists(String),

    /// A course code was not provided
    #[error("course code was not provided to render template into (use `CODE:TEMPLATE` or `--course CODE`)")]
    TemplateCourseCodeMissing,
}
