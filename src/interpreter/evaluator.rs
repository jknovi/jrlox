use crate::lexer::token::TokenKind;
use crate::parser::ast::Binary;
use crate::parser::ast::Expression;
use crate::parser::ast::Grouping;
use crate::parser::ast::Literal;
use crate::parser::ast::SyntaxVisitor;
use crate::parser::ast::Unary;
use crate::parser::ast::Visitable;

// TODO: use actual error type
type Error = String;
type Result<T> = std::result::Result<T, Error>;

#[derive(Default)]
pub struct Evaluator {}

#[derive(Debug)]
pub struct ExpressionValue {
    literal: Literal,
}

impl ToString for ExpressionValue {
    fn to_string(&self) -> String {
        self.literal.to_string()
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn eval(&mut self, expression: &Expression) -> Result<ExpressionValue> {
        expression.accept(self)
    }
}

impl SyntaxVisitor<Result<ExpressionValue>> for Evaluator {
    fn visit_expression(&mut self, arg: &Expression) -> Result<ExpressionValue> {
        match arg {
            Expression::Binary(binary) => self.visit_binary(binary),
            Expression::Unary(unary) => self.visit_unary(unary),
            Expression::Grouping(grouping) => self.visit_grouping(grouping),
            Expression::Literal(literal) => self.visit_literal(literal),
        }
    }

    fn visit_grouping(&mut self, grouping: &Grouping) -> Result<ExpressionValue> {
        self.eval(&grouping.expression)
    }

    fn visit_binary(&mut self, binary: &Binary) -> Result<ExpressionValue> {
        let lval = binary.left.accept(self)?;
        let rval = binary.right.accept(self)?;

        let literal = match binary.operator.kind {
                TokenKind::Minus => match (lval.literal, rval.literal) {
                    (Literal::Number(left), Literal::Number(right)) => {
                        Literal::Number(left - right)
                    }
                    (left, right) => return Err(
                        format!(
                            "Binary operator '{}' expects two numbers, instead got: left='{:?}' right='{:?}'",
                            binary.operator.lexeme,
                            left,
                            right,
                        )
                    ),
                },

                TokenKind::Slash => match (lval.literal, rval.literal) {
                    (Literal::Number(left), Literal::Number(right)) => {
                        Literal::Number(left / right)
                    }
                    (left, right) => return Err(
                        format!(
                            "Binary operator '{}' expects two numbers, instead got: left='{:?}' right='{:?}'",
                            binary.operator.lexeme,
                            left,
                            right,
                        )
                    ),
                },

                TokenKind::Star => match (lval.literal, rval.literal) {
                    (Literal::Number(left), Literal::Number(right)) => {
                        Literal::Number(left * right)
                    }
                    (left, right) => return Err(
                        format!(
                            "Binary operator '{}' expects two numbers, instead got: left='{:?}' right='{:?}'",
                            binary.operator.lexeme,
                            left,
                            right,
                        )
                    ),
                },

                TokenKind::Plus => match (lval.literal, rval.literal) {
                    (Literal::Number(left), Literal::Number(right)) => {
                        Literal::Number(left + right)
                    }

                    (Literal::String(left), Literal::String(right)) => {
                        Literal::String(format!("{}{}", left, right))
                    }

                    (left, right) => return Err(
                        format!(
                            "Binary operator '{}' expects two numbers or two strings, instead got: left='{:?}' right='{:?}'",
                            binary.operator.lexeme,
                            left,
                            right,
                        )
                    ),
                },

                // Comparisons... and other boolean binary ops

                _ => return Err(format!("Binary operator {} not supported", binary.operator.lexeme)),
            };

        Ok(ExpressionValue { literal })
    }

    fn visit_literal(&mut self, literal: &Literal) -> Result<ExpressionValue> {
        Ok(ExpressionValue {
            literal: literal.clone(),
        })
    }

    fn visit_unary(&mut self, unary: &Unary) -> Result<ExpressionValue> {
        let rval = self.eval(&unary.expression)?.literal;

        let literal = match unary.operator.kind {
            TokenKind::Minus => match rval {
                Literal::Number(num) => Literal::Number(-num),
                _ => panic!(),
            },
            TokenKind::Bang => not(&rval),

            _ => panic!(),
        };

        Ok(ExpressionValue { literal })
    }
}

fn not(literal: &Literal) -> Literal {
    match literal {
        Literal::False | Literal::Nil => Literal::True,
        _ => Literal::False, // everything is truthy but false
    }
}
