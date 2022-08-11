use jrlox::lexer::token::Token;
use jrlox::lexer::token::TokenKind;
use jrlox::parser::ast::prefix_printer::PrefixPrinter;
use jrlox::parser::ast::*;
use std::boxed::Box;

fn main() {
    let expression = Expression::Binary(Binary {
        left: Box::new(Expression::Unary(Unary {
            operator: Box::new(Token {
                kind: TokenKind::Minus,
                lexeme: "-".into(),
                section: Default::default(),
            }),
            expression: Box::new(Expression::Literal(Literal::Number(123.0))),
        })),
        operator: Box::new(Token {
            kind: TokenKind::Star,
            lexeme: "*".into(),
            section: Default::default(),
        }),
        right: Box::new(Expression::Grouping(Grouping {
            expression: Box::new(Expression::Literal(Literal::Number(45.67))),
        })),
    });

    println!("{}", PrefixPrinter::new().print(&expression));
}
