use migrated::*;

#[test]
fn basic() {
    assert!(is_valid_code("ABC-1234"));
    assert!(!is_valid_code("abc-1234"));
}
