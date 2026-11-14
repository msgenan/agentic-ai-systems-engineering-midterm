use migrated::*;

#[test]
fn basic() {
    assert_eq!(parse_int("42"), 42);
    assert_eq!(parse_int("-7"), -7);
}
