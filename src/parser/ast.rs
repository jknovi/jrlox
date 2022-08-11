use crate::lexer::token::Token;

macro_rules! def_rule {
    ($name:ident => $($id:ident : $type:ty),+) => {
        pub struct $name {
            $(pub $id: std::boxed::Box<$type>,)*
        }
    };
    ($name:ident => $($untyped:ident)|* $([$variant:ident as $type:ty])|* ) => {
        pub enum $name {
            $($variant($type)),*
            $($untyped($untyped)),*
        }
    };
}

macro_rules! def_visitor {
    // Had to adf func, because can't use macros on certain parts of the macros
    // but now it works, had to add more stuff.
    ($($name:ident : $func:ident),+) => {

        pub trait SyntaxVisitor<T> {
            $(fn $func(&mut self, arg: &$name) -> T;)+
        }

        pub trait Visitable<T> {
            fn accept(&self, visitor: &mut impl SyntaxVisitor<T>) -> T;
        }

        $(impl<T> Visitable<T> for $name {
            fn accept(&self, visitor: &mut impl SyntaxVisitor<T>) -> T {
                visitor.$func(&self)
            }
        })+
    }
}

def_rule!(
    Expression => Binary
        | Unary
        | Grouping
        | Literal
);

def_rule!(
    Grouping => expression: Expression
);

def_rule!(
    Binary => left: Expression, operator: Token, right: Expression
);

// This could be just a token.. although we lose the "type" then..
def_rule!(
    Literal => [Number as f64]
        | [String as String]
        | [Boolean as Token]
        | [Nil as Token]
);

def_rule!(
    Unary => operator: Token, expression: Expression
);

def_visitor!(
    Expression: visit_expr,
    Grouping: visit_grouping,
    Binary: visit_binary,
    Literal: visit_literal,
    Unary: visit_unary
);

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
        fn visit_expr(&mut self, arg: &Expression) -> String {
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
            match literal {
                Literal::Number(value) => value.to_string(),
                Literal::String(value) => value.clone(),
                Literal::Boolean(token) | Literal::Nil(token) => token.lexeme.clone(),
            }
        }

        fn visit_unary(&mut self, unary: &Unary) -> String {
            format!(
                "({} {})",
                unary.operator.lexeme,
                unary.expression.accept(self)
            )
        }
    }
}
