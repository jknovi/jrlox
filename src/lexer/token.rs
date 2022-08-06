use crate::text::TextSection;

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String, // Cow?
    pub section: TextSection,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    /// Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftCStyleComment,
    RightCStyleComment,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    /// One or two characters
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    /// Literals
    Identifier(String),
    String(String),
    Number(f64),

    /// Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Skip,

    Eof,
}

impl TokenKind {
    pub fn with_equal(self) -> Self {
        match self {
            Self::Bang => Self::BangEqual,
            Self::Equal => Self::EqualEqual,
            Self::Greater => Self::GreaterEqual,
            Self::Less => Self::LessEqual,
            _ => self,
        }
    }

    pub fn keyword_token<T: AsRef<str>>(word: T) -> Option<Self> {
        match word.as_ref() {
            "and" => Some(Self::And),
            "class" => Some(Self::Class),
            "else" => Some(Self::Else),
            "false" => Some(Self::False),
            "for" => Some(Self::For),
            "fun" => Some(Self::Fun),
            "if" => Some(Self::If),
            "nil" => Some(Self::Nil),
            "or" => Some(Self::Or),
            "print" => Some(Self::Print),
            "return" => Some(Self::Return),
            "super" => Some(Self::Super),
            "this" => Some(Self::This),
            "true" => Some(Self::True),
            "var" => Some(Self::Var),
            "while" => Some(Self::While),
            _ => None,
        }
    }
}
