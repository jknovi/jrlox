#[derive(Debug, Clone)]
pub struct Token {}

pub struct Scanner {
    source: String, // TODO: str
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self { source }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, super::ErrorList> {
        Ok(Vec::new())
    }
}
