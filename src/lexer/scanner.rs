use crate::error::ErrorBuilder;
use crate::error::ErrorList;
use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use crate::text::TextCursor;

pub struct Scanner {
    cursor: TextCursor,
}

#[derive(Default, Clone)]
pub struct ScanResult {
    pub tokens: Vec<Token>,
    pub errors: ErrorList,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            cursor: TextCursor::new(&source),
        }
    }

    pub fn scan_tokens(&mut self) -> ScanResult {
        let mut tokens = Vec::new();
        let mut errors = ErrorList::default();

        while !self.cursor.is_done() {
            self.cursor.new_section();

            match self.scan() {
                Ok(TokenKind::Skip) => (),
                Ok(kind) => tokens.push(self.add_context(kind)),
                Err(error_builder) => {
                    errors.add(error_builder.section(self.cursor.section()).build())
                }
            }
        }

        ScanResult { tokens, errors }
    }

    fn scan(&mut self) -> Result<TokenKind, ErrorBuilder> {
        let c = self.cursor.next().expect("Unexpected end of file");

        let kind = match c {
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            '-' => TokenKind::Minus,
            '+' => TokenKind::Plus,
            ';' => TokenKind::Semicolon,
            '*' => TokenKind::Star,

            '!' => self.check_for_equal(TokenKind::Bang),
            '=' => self.check_for_equal(TokenKind::Equal),
            '<' => self.check_for_equal(TokenKind::Less),
            '>' => self.check_for_equal(TokenKind::Greater),

            '/' => self.scan_slash_or_comment()?,

            ' ' | '\r' | '\t' | '\n' => TokenKind::Skip,

            '"' => self.scan_string()?,

            c if is_digit(&c) => self.scan_number(),

            c if is_alpha(&c) => self.scan_identifier_or_keyword(),

            unexpected => {
                let error = crate::error::ErrorBuilder::new()
                    .message(format!("Unexpected character '{}'", unexpected));

                return Err(error);
            }
        };

        Ok(kind)
    }

    fn consume_when_match(&mut self, c: char) -> bool {
        if self.cursor.match_next(c) {
            self.cursor.consume();

            true
        } else {
            false
        }
    }

    fn add_context(&mut self, kind: TokenKind) -> Token {
        Token {
            kind,
            lexeme: self.cursor.section_slice().iter().collect(),
            section: self.cursor.section(),
        }
    }

    fn scan_string(&mut self) -> Result<TokenKind, crate::error::ErrorBuilder> {
        self.cursor.consume_until_match('"');
        if self.cursor.is_done() {
            let error = crate::error::ErrorBuilder::new().message("Unterminated string.");

            return Err(error);
        }

        // skip opening '"' char
        let string = self.cursor.section_slice()[1..].iter().collect();
        // consume closing '"' char
        self.cursor.consume();

        Ok(TokenKind::String(string))
    }

    fn scan_slash_or_comment(&mut self) -> Result<TokenKind, ErrorBuilder> {
        match self.cursor.current() {
            Some('/') => {
                self.cursor.consume_until_match('\n');

                Ok(TokenKind::Skip)
            }
            Some('*') => {
                // C-Style
                self.cursor.consume();
                loop {
                    self.cursor.consume_until_match('*');
                    self.cursor.consume();

                    match self.cursor.next() {
                        Some('/') => break,
                        Some(_) => continue,
                        None => {
                            return Err(
                                crate::error::ErrorBuilder::new().message("Unterminated comment.")
                            )
                        }
                    }
                }

                Ok(TokenKind::Skip)
            }
            _ => Ok(TokenKind::Slash),
        }
    }

    fn check_for_equal(&mut self, kind: TokenKind) -> TokenKind {
        if self.consume_when_match('=') {
            kind.with_equal()
        } else {
            kind
        }
    }

    fn scan_number(&mut self) -> TokenKind {
        self.cursor.consume_while(is_digit);

        if self.cursor.match_next('.') {
            self.cursor.consume();
            self.cursor.consume_while(is_digit);
        }

        let number = self
            .cursor
            .section_string()
            .parse()
            .expect("Impossible error parsing known number");

        TokenKind::Number(number)
    }

    fn scan_identifier_or_keyword(&mut self) -> TokenKind {
        self.cursor.consume_while(is_alpha_numeric);

        let text = self.cursor.section_string();

        match TokenKind::keyword_token(&text) {
            Some(token) => token,
            None => TokenKind::Identifier(text),
        }
    }
}

fn is_digit(c: &char) -> bool {
    matches!(c, '0'..='9')
}

fn is_alpha(c: &char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
}

fn is_alpha_numeric(c: &char) -> bool {
    is_alpha(c) || is_digit(c)
}
