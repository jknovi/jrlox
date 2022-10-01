use crate::error::Error;
use crate::error::ErrorBuilder;
use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use crate::parser::ast::Binary;
use crate::parser::ast::Expression;
use crate::parser::ast::Grouping;
use crate::parser::ast::Literal;
use crate::parser::ast::Unary;

type Result<T> = std::result::Result<T, Error>;

/// Parses the following unambigous grammar:
/// ```
/// expression -> equality ;
/// equality   -> comparison ( ( "!=" | "==" ) comparison )* ;
/// comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
/// term       -> factor ( ( "-" | "+" ) factor )* ;
/// factor     -> unary ( ( "/" | "*" ) unary )* ;
/// unary      -> ( "!" | "-" ) unary | primary ;
/// primary    -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
/// ```
pub struct Parser {
    tokens: Vec<Token>,
    scan_position: usize,
}

fn error_at(msg: impl Into<std::borrow::Cow<'static, str>>, token: &Token) -> Error {
    ErrorBuilder::new()
        .message(msg)
        .section(token.section)
        .build()
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            scan_position: 0,
        }
    }

    // maybe consume self instead? yes
    pub fn parse(mut self) -> Result<Expression> {
        // Do error handling/recovery here
        // TODO: After this, the token stream should have been completely consumed...
        self.expression()
    }

    /// expression -> equality ;
    fn expression(&mut self) -> Result<Expression> {
        self.equality()
    }

    /// equality   -> comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Result<Expression> {
        let mut left = self.comparison()?;

        while self.matches(TokenKind::BangEqual) || self.matches(TokenKind::EqualEqual) {
            let binary = Binary {
                left: std::boxed::Box::new(left),
                operator: std::boxed::Box::new(self.previous().clone()),
                right: std::boxed::Box::new(self.comparison()?),
            };

            left = Expression::Binary(binary);
        }

        Ok(left)
    }

    /// comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<Expression> {
        let mut left = self.term()?;

        while self.matches(TokenKind::Greater)
            || self.matches(TokenKind::GreaterEqual)
            || self.matches(TokenKind::Less)
            || self.matches(TokenKind::LessEqual)
        {
            let binary = Binary {
                left: std::boxed::Box::new(left),
                operator: std::boxed::Box::new(self.previous().clone()),
                right: std::boxed::Box::new(self.term()?),
            };

            left = Expression::Binary(binary);
        }

        Ok(left)
    }

    /// term       -> factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<Expression> {
        let mut left = self.factor()?;

        while self.matches(TokenKind::Minus) || self.matches(TokenKind::Plus) {
            let binary = Binary {
                left: std::boxed::Box::new(left),
                operator: std::boxed::Box::new(self.previous().clone()),
                right: std::boxed::Box::new(self.factor()?),
            };

            left = Expression::Binary(binary);
        }

        Ok(left)
    }

    /// factor     -> unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Result<Expression> {
        let mut left = self.unary()?;

        while self.matches(TokenKind::Slash) || self.matches(TokenKind::Star) {
            let binary = Binary {
                left: std::boxed::Box::new(left),
                operator: std::boxed::Box::new(self.previous().clone()),
                right: std::boxed::Box::new(self.unary()?),
            };

            left = Expression::Binary(binary);
        }

        Ok(left)
    }

    /// unary      -> ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> Result<Expression> {
        if self.matches(TokenKind::Bang) || self.matches(TokenKind::Minus) {
            let unary = Unary {
                operator: std::boxed::Box::new(self.previous().clone()),
                expression: std::boxed::Box::new(self.unary()?),
            };

            Ok(Expression::Unary(unary))
        } else {
            self.primary()
        }
    }

    /// primary    -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expression> {
        // TODO: rewrite this into a proper match, same for all the other matches..
        if self.matches(TokenKind::False) {
            return Ok(Expression::Literal(Literal::False));
        }

        if self.matches(TokenKind::True) {
            return Ok(Expression::Literal(Literal::True));
        }

        if self.matches(TokenKind::Nil) {
            return Ok(Expression::Literal(Literal::Nil));
        }

        if let Some(number) = self.number() {
            let literal = Literal::Number(number);

            // TODO: fix this should advance on number
            self.advance();

            return Ok(Expression::Literal(literal));
        }

        if let Some(string) = self.string() {
            let literal = Literal::String(string);

            // TODO: fix this should advance on string
            self.advance();

            return Ok(Expression::Literal(literal));
        }

        if self.matches(TokenKind::LeftParen) {
            let grouped = Grouping {
                expression: std::boxed::Box::new(self.expression()?),
            };

            self.consume(TokenKind::RightParen)?;

            return Ok(Expression::Grouping(grouped));
        }

        Err(self.unexpected())
    }

    //
    //
    // Utility functions, may abstract into token walker/cursor or something
    //
    //
    fn number(&mut self) -> Option<f64> {
        // Had a single "literal" exctractor
        match self.peek().kind {
            TokenKind::Number(number) => Some(number),
            _ => None,
        }
    }

    fn string(&mut self) -> Option<String> {
        match &self.peek().kind {
            TokenKind::String(string) => Some(string.clone()),
            _ => None,
        }
    }

    fn matches(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();

            true
        } else {
            false
        }
    }

    fn check(&self, kind: TokenKind) -> bool {
        if self.is_done() {
            false
        } else {
            self.peek().kind == kind
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_done() {
            self.scan_position += 1;
        }

        self.previous()
    }

    fn is_done(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    fn peek(&self) -> &Token {
        self.tokens
            .get(self.scan_position)
            .expect("Fatal, AST didn't end at EOF")
    }

    fn checked_previous(&self) -> Option<&Token> {
        self.tokens.get(self.scan_position - 1) // check subtract?
    }

    fn previous(&self) -> &Token {
        self.checked_previous().expect("Out of bounds")
    }

    fn unexpected(&self) -> Error {
        let current = self.peek();

        let msg = format!("Unexpected token '{}'", current.lexeme);

        error_at(msg, current)
    }

    fn consume(&mut self, expected: TokenKind) -> Result<()> {
        let current = self.peek();

        if current.kind == expected {
            self.advance();

            Ok(())
        } else {
            let msg = format!(
                "Expecting to find '{:?}' fount '{}' instead",
                expected, current.lexeme
            );

            Err(error_at(msg, current))
        }
    }

    fn synchronize(&mut self) {
        while !self.is_done() && !self.at_synchronization_point() {
            self.advance();
        }
    }

    fn at_synchronization_point(&self) -> bool {
        matches! { self.peek().kind,
            TokenKind::Class
            | TokenKind::For
            | TokenKind::Fun
            | TokenKind::If
            | TokenKind::Print
            | TokenKind::Return
            | TokenKind::Var
            | TokenKind::While
        }
    }
}
