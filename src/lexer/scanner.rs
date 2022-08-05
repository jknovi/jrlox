use crate::error::ErrorList;
use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use crate::text::TextCursor;

pub struct Scanner {
    cursor: TextCursor,
    tokens: Vec<Token>,
    pub error_list: ErrorList,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            cursor: TextCursor::new(&source),
            tokens: Default::default(),
            error_list: ErrorList::default(),
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, ErrorList> {
        while !self.cursor.is_done() {
            self.cursor.new_section();

            self.scan();
        }

        Ok(self.tokens.clone())
    }

    fn scan(&mut self) {
        let c = self.cursor.next().expect("Unexpected end of file"); // maybe dump all errors there..

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

            '!' => {
                if self.consume_when_match('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                }
            }
            '=' => {
                if self.consume_when_match('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                }
            }
            '<' => {
                if self.consume_when_match('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }
            '>' => {
                if self.consume_when_match('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }

            '/' => {
                if self.consume_when_match('/') {
                    self.cursor.consume_until_match('\n');

                    TokenKind::Skip
                } else {
                    TokenKind::Slash
                }
            }

            ' ' | '\r' | '\t' | '\n' => TokenKind::Skip,

            '"' => {
                self.cursor.consume_until_match('"');
                if self.cursor.is_done() {
                    let error = crate::error::ErrorBuilder::new()
                        .section(self.cursor.section())
                        .message("Unterminated string.")
                        .build();

                    self.error_list.add(error);

                    return;
                }

                // skip opening '"' char
                let string = self.cursor.section_slice()[1..].iter().collect();
                // consume closing '"' char
                self.cursor.consume();

                TokenKind::String(string)
            }

            c if is_digit(&c) => {
                self.cursor.consume_while(is_digit);

                if self.cursor.match_next('.') {
                    self.cursor.consume();
                    self.cursor.consume_while(is_digit);
                }

                let number = self
                    .cursor
                    .section_slice()
                    .iter()
                    .collect::<String>()
                    .parse()
                    .expect("Impossible error parsing known number");

                TokenKind::Number(number)
            }
            c if is_alpha(&c) => TokenKind::Skip,

            unexpected => {
                let error = crate::error::ErrorBuilder::new()
                    .section(self.cursor.section())
                    .message(format!("Unexpected character '{}'", unexpected))
                    .build();

                self.error_list.add(error);

                TokenKind::Skip
            }
        };

        self.add_token(kind)
    }

    fn consume_when_match(&mut self, c: char) -> bool {
        if self.cursor.match_next(c) {
            self.cursor.consume();

            true
        } else {
            false
        }
    }

    fn add_token(&mut self, kind: TokenKind) {
        if kind == TokenKind::Skip {
            return;
        }

        let token = Token {
            kind,
            lexeme: self.cursor.section_slice().iter().collect(),
            section: self.cursor.section(),
        };

        self.tokens.push(token);
    }
}

fn is_digit(c: &char) -> bool {
    matches!(c, '0'..='9')
}

fn is_alpha(c: &char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
}
