use crate::lexer::token::TokenKind;

crate::parser::macros::def_rule!(
    Expr => Binary
        | Unary
        | Grouping
        | Literal
);

crate::parser::macros::def_rule!(Grouping => expression: Expr);

crate::parser::macros::def_rule!(Binary => left: Expr, operator: TokenKind, right: Expr);

crate::parser::macros::def_rule!(
    Literal => [Number as f64]
        | [String as String]
        | [False as ()]
        | [Nil as ()]
        | [True as ()]
);

crate::parser::macros::def_rule!(Unary => operator: TokenKind, expression: Expr);

crate::parser::macros::def_visitor!(
    Expr: visit_expr,
    Grouping: visit_grouping,
    Binary: visit_binary,
    Literal: visit_literal,
    Unary: visit_unary
);
