use crate::text::Position;

#[derive(Clone, Copy, Default, Debug)]
pub struct TextSection {
    pub start: Position,
    pub end: Position,
}

pub struct TextCursor {
    text: Vec<char>,

    section_start: Position,
    current_position: Position,
}

impl TextCursor {
    pub fn new<T: AsRef<str>>(text: T) -> Self {
        Self {
            text: text.as_ref().chars().collect(),
            section_start: Position::new(),
            current_position: Position::new(),
        }
    }

    pub fn new_section(&mut self) {
        self.section_start = self.current_position;
    }

    pub fn current(&self) -> Option<char> {
        self.text.get(self.current_position.offset).copied()
    }

    pub fn next(&mut self) -> Option<char> {
        let current = self.current();

        self.consume();

        current
    }

    pub fn consume(&mut self) {
        match self.current() {
            Some('\n') => self.current_position.new_line(),
            Some(_) => self.current_position.advance(),
            None => (),
        }
    }

    pub fn is_done(&self) -> bool {
        self.current().is_none()
    }

    pub fn section_slice(&self) -> &[char] {
        &self.text[self.section_start.offset..self.current_position.offset]
    }

    pub fn section(&self) -> TextSection {
        TextSection {
            start: self.section_start,
            end: self.current_position,
        }
    }

    pub fn match_next(&mut self, c: char) -> bool {
        self.current() == Some(c)
    }

    pub fn consume_until_match(&mut self, c: char) {
        loop {
            match self.current() {
                Some(curr) if curr != c => self.consume(),
                _ => return,
            }
        }
    }

    pub fn consume_while<F: Fn(&char) -> bool>(&mut self, pred: F) {
        while self.current().filter(&pred).is_some() {
            self.consume();
        }
    }
}
