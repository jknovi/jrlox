mod evaluator;

pub fn eval(expr: &crate::parser::ast::Expression) -> Result<evaluator::ExpressionValue, String> {
    evaluator::Evaluator::new().eval(expr)
}
