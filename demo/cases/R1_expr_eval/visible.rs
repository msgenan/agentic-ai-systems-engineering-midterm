use migrated::*;

#[test]
fn basic() {
    assert_eq!(eval("1 + 2"), Ok(3));
    assert_eq!(eval("2 * (3 + 4)"), Ok(14));
    assert_eq!(eval("1 +"), Err((EvalError::Syntax, 3)));
}
