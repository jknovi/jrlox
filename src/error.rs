use crate::text::TextSection;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Error {
    pub section: TextSection,
    pub location: Cow<'static, str>,
    pub message: Cow<'static, str>,
}

impl ToString for Error {
    fn to_string(&self) -> String {
        format!(
            "{} at line: {}, column: {}",
            self.message, self.section.start.line, self.section.start.column
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct ErrorBuilder {
    section: Option<TextSection>,
    location: Option<Cow<'static, str>>,
    message: Option<Cow<'static, str>>,
}

impl ErrorBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    // actually use loc instead
    pub fn section(mut self, section: TextSection) -> Self {
        self.section.replace(section);

        self
    }

    pub fn location(mut self, location: impl Into<Cow<'static, str>>) -> Self {
        self.location.replace(location.into());

        self
    }

    pub fn message(mut self, message: impl Into<Cow<'static, str>>) -> Self {
        self.message.replace(message.into());

        self
    }

    pub fn build(self) -> Error {
        Error {
            section: self.section.unwrap_or_default(),
            location: self.location.unwrap_or_default(),
            message: self.message.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ErrorList {
    inner: Vec<Error>,
}

impl ErrorList {
    pub fn add(&mut self, error: Error) {
        self.inner.push(error)
    }

    pub fn print(&self) {
        for error in self.inner.iter() {
            eprintln!(
                "[{}] Error {}: {}",
                error.section.start, error.location, error.message
            );
        }
    }

    pub fn size(&self) -> usize {
        self.inner.len()
    }
}
