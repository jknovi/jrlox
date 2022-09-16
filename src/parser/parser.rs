use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use crate::parser::ast::Binary;
use crate::parser::ast::Expression;
use crate::parser::ast::Grouping;
use crate::parser::ast::Literal;
use crate::parser::ast::Unary;

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

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            scan_position: 0,
        }
    }

    // maybe consume self instead? yes
    pub fn parse(mut self) -> Expression {
        // Do error handling/recovery here
        self.expression()
    }

    /// expression -> equality ;
    fn expression(&mut self) -> Expression {
        self.equality()
    }

    /// equality   -> comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Expression {
        let mut left = self.comparison();

        while self.matches(TokenKind::BangEqual) || self.matches(TokenKind::EqualEqual) {
            let binary = Binary {
                left: std::boxed::Box::new(left),
                operator: std::boxed::Box::new(self.previous().clone()),
                right: std::boxed::Box::new(self.comparison()),
            };

            left = Expression::Binary(binary);
        }

        left
    }

    /// comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Expression {
        let mut left = self.term();

        while self.matches(TokenKind::Greater)
            || self.matches(TokenKind::GreaterEqual)
            || self.matches(TokenKind::Less)
            || self.matches(TokenKind::LessEqual)
        {
            let binary = Binary {
                left: std::boxed::Box::new(left),
                operator: std::boxed::Box::new(self.previous().clone()),
                right: std::boxed::Box::new(self.term()),
            };

            left = Expression::Binary(binary);
        }

        left
    }

    /// term       -> factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Expression {
        let mut left = self.factor();

        while self.matches(TokenKind::Minus) || self.matches(TokenKind::Plus) {
            let binary = Binary {
                left: std::boxed::Box::new(left),
                operator: std::boxed::Box::new(self.previous().clone()),
                right: std::boxed::Box::new(self.factor()),
            };

            left = Expression::Binary(binary);
        }

        left
    }

    /// factor     -> unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Expression {
        let mut left = self.unary();

        while self.matches(TokenKind::Slash) || self.matches(TokenKind::Star) {
            let binary = Binary {
                left: std::boxed::Box::new(left),
                operator: std::boxed::Box::new(self.previous().clone()),
                right: std::boxed::Box::new(self.unary()),
            };

            left = Expression::Binary(binary);
        }

        left
    }

    /// unary      -> ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> Expression {
        if self.matches(TokenKind::Bang) || self.matches(TokenKind::Minus) {
            let unary = Unary {
                operator: std::boxed::Box::new(self.previous().clone()),
                expression: std::boxed::Box::new(self.unary()),
            };

            Expression::Unary(unary)
        } else {
            self.primary()
        }
    }

    /// primary    -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Expression {
        // TODO: rewrite this into a proper match, same for all the other matches..
        if self.matches(TokenKind::False) {
            return Expression::Literal(Literal::False);
        }

        if self.matches(TokenKind::True) {
            return Expression::Literal(Literal::True);
        }

        if self.matches(TokenKind::Nil) {
            return Expression::Literal(Literal::Nil);
        }

        if let Some(number) = self.number() {
            let literal = Literal::Number(number);

            // TODO: fix this should advance on number
            self.advance();

            return Expression::Literal(literal);
        }

        if let Some(string) = self.string() {
            let literal = Literal::String(string);

            // TODO: fix this should advance on string
            self.advance();

            return Expression::Literal(literal);
        }

        if self.matches(TokenKind::LeftParen) {
            let grouped = Grouping {
                expression: std::boxed::Box::new(self.expression()),
            };

            // TODO: consume/check Right paren

            return Expression::Grouping(grouped);
        }

        todo!()
    }

    //
    //
    // Utility functions, may abstract into token walker/cusor or something
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
}
