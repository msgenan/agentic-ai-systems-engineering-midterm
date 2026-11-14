use migrated::*;

#[test]
fn hidden() {
    assert_eq!(parse_int("  42abc"), 42);
    assert_eq!(parse_int("abc"), 0);
    assert_eq!(parse_int("+5"), 5);
    assert_eq!(parse_int(""), 0);
    assert_eq!(parse_int("- 3"), 0);
    assert_eq!(parse_int("\n\t 19"), 19);
    assert_eq!(parse_int("12 34"), 12);
}
