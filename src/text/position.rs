#[derive(Clone, Copy, Debug, Default)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl Position {
    pub fn new() -> Self {
        Self {
            line: 1,
            column: 1,
            offset: 0,
        }
    }

    pub fn new_line(&mut self) {
        self.line += 1;
        self.column = 1;
        self.offset += 1;
    }

    pub fn advance(&mut self) {
        self.column += 1;
        self.offset += 1;
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Position { line, column, .. } if *line == 0 || *column == 0 => {
                write!(f, "Invalid Position")
            }
            Position { line, column, .. } => write!(f, "line: {}, col: {}", line, column),
        }
    }
}
