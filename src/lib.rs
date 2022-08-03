use std::borrow::Cow;
pub mod scanner;

#[derive(Debug, Clone)]
pub struct Error {
    pub line: usize,
    pub location: Cow<'static, str>,
    pub message: Cow<'static, str>,
}

#[derive(Debug, Clone, Default)]
pub struct ErrorBuilder {
    line: Option<usize>,
    location: Option<Cow<'static, str>>,
    message: Option<Cow<'static, str>>,
}

impl ErrorBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn line(mut self, line: usize) -> Self {
        self.line.replace(line);

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
            line: self.line.unwrap_or_default(),
            location: self.location.unwrap_or_default(),
            message: self.message.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone)]
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
                error.line, error.location, error.message
            );
        }
    }

    pub fn size(&self) -> usize {
        self.inner.len()
    }
}
