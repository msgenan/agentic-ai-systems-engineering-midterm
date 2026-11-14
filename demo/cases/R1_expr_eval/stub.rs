#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EvalError {
    Syntax,
    DivByZero,
}

/// Evaluates an integer expression with + - * / % ( ) and unary minus.
/// Returns Ok(value), or Err((error, offset where parsing stopped)) with the same
/// error code and offset as the C version.
pub fn eval(s: &str) -> Result<i64, (EvalError, usize)> {
    todo!()
}
