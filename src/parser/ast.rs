use crate::lexer::token::Token;
use ast_macros::grammar;

grammar! {
    Expression => Binary
        | Unary
        | Grouping
        | Literal;

    Literal => Number as f64
        | String
        | @True
        | @False
        | @Nil;

    Binary => left: Expression, operator: Token, right: Expression;

    Unary => operator: Token, expression: Expression;

    Grouping => expression: Expression;
}

impl ToString for Literal {
    fn to_string(&self) -> String {
        match self {
            Literal::Number(value) => value.to_string(),
            Literal::String(value) => format!("\"{}\"", value),
            Literal::True => String::from("true"),
            Literal::False => String::from("false"),
            Literal::Nil => String::from("nil"),
        }
    }
}

pub mod prefix_printer {
    use super::*;

    #[derive(Default)]
    pub struct PrefixPrinter {}

    impl PrefixPrinter {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn print(&mut self, expression: &Expression) -> String {
            expression.accept(self)
        }
    }

    impl SyntaxVisitor<String> for PrefixPrinter {
        fn visit_expression(&mut self, arg: &Expression) -> String {
            match arg {
                Expression::Binary(binary) => self.visit_binary(binary),
                Expression::Unary(unary) => self.visit_unary(unary),
                Expression::Grouping(grouping) => self.visit_grouping(grouping),
                Expression::Literal(literal) => self.visit_literal(literal),
            }
        }

        fn visit_grouping(&mut self, grouping: &Grouping) -> String {
            format!("(grouping {})", grouping.expression.accept(self))
        }

        fn visit_binary(&mut self, binary: &Binary) -> String {
            format!(
                "({} {} {})",
                binary.operator.lexeme,
                binary.left.accept(self),
                binary.right.accept(self),
            )
        }
        fn visit_literal(&mut self, literal: &Literal) -> String {
            literal.to_string()
        }

        fn visit_unary(&mut self, unary: &Unary) -> String {
            format!(
                "({} {})",
                unary.operator.lexeme,
                unary.expression.accept(self)
            )
        }
    }

    #[test]
    fn pretty_print_renders_correct_tree() {
        use crate::lexer::token::TokenKind;

        let expression = Expression::Binary(Binary {
            left: Box::new(Expression::Unary(Unary {
                operator: Box::new(Token {
                    kind: TokenKind::Minus,
                    lexeme: "-".into(),
                    section: Default::default(),
                }),
                expression: Box::new(Expression::Binary(Binary {
                    left: Box::new(Expression::Literal(Literal::True)),
                    operator: Box::new(Token {
                        kind: TokenKind::Slash,
                        lexeme: "/".into(),
                        section: Default::default(),
                    }),
                    right: Box::new(Expression::Literal(Literal::Number(123.0))),
                })),
            })),
            operator: Box::new(Token {
                kind: TokenKind::Star,
                lexeme: "*".into(),
                section: Default::default(),
            }),
            right: Box::new(Expression::Grouping(Grouping {
                expression: Box::new(Expression::Binary(Binary {
                    left: Box::new(Expression::Literal(Literal::Nil)),
                    operator: Box::new(Token {
                        kind: TokenKind::Plus,
                        lexeme: "+".into(),
                        section: Default::default(),
                    }),
                    right: Box::new(Expression::Literal(Literal::Number(45.67))),
                })),
            })),
        });

        assert_eq!(
            "(* (- (/ true 123)) (grouping (+ nil 45.67)))",
            PrefixPrinter::new().print(&expression),
        );
    }
}
