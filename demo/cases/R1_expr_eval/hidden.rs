use migrated::*;

#[test]
fn hidden() {
    assert_eq!(eval("1 + 2 * 3"), Ok(7));
    assert_eq!(eval("(1 + 2) * 3"), Ok(9));
    assert_eq!(eval("7 / -2"), Ok(-3));
    assert_eq!(eval("-7 % 3"), Ok(-1));
    assert_eq!(eval("--3"), Ok(3));
    assert_eq!(eval("-(2+3)*2"), Ok(-10));
    assert_eq!(eval("2*-3"), Ok(-6));
    assert_eq!(eval("8 / 2 / 2"), Ok(2));
    assert_eq!(eval("2 - 3 - 4"), Ok(-5));
    assert_eq!(eval("100 % 7 * 3"), Ok(6));
    assert_eq!(eval("((((((5))))))"), Ok(5));
    assert_eq!(eval("   42   "), Ok(42));
    assert_eq!(eval("10/(5-5)"), Err((EvalError::DivByZero, 8)));
    assert_eq!(eval("1+*2"), Err((EvalError::Syntax, 4)));
    assert_eq!(eval("1+2)"), Err((EvalError::Syntax, 3)));
    assert_eq!(eval("(1+2"), Err((EvalError::Syntax, 4)));
    assert_eq!(eval(""), Err((EvalError::Syntax, 0)));
    assert_eq!(eval("1 + a"), Err((EvalError::Syntax, 4)));
    assert_eq!(eval("3 4"), Err((EvalError::Syntax, 2)));
    assert_eq!(eval("4 % 0 + 1"), Err((EvalError::DivByZero, 9)));
    assert_eq!(eval("2 * (3 + 4) - -1"), Ok(15));
}
