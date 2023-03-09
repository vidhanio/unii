use chrono::{Local, NaiveDate};

/// A note.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Note {
    /// The date the note was created.
    pub date: NaiveDate,
    /// The content of the note.
    pub content: String,
}

impl Note {
    /// Create a new note.
    #[must_use]
    pub fn new(content: &str) -> Self {
        Self {
            date: Local::today().naive_local(),
            content: content.into(),
        }
    }

    /// Create a new note with a custom date.
    #[must_use]
    pub fn new_with_date(date: NaiveDate, content: &str) -> Self {
        Self {
            date,
            content: content.into(),
        }
    }
}
