use migrated::*;

#[test]
fn basic() {
    assert_eq!(reverse_string("abc"), "cba");
    assert_eq!(reverse_string(""), "");
}
