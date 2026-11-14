use migrated::*;

#[test]
fn hidden() {
    assert!(!is_valid_code("ABC-123"));
    assert!(!is_valid_code("ABC-12345"));
    assert!(!is_valid_code("ABCD-1234"));
    assert!(!is_valid_code(""));
    assert!(!is_valid_code("ABC-1234\n"));
    assert!(is_valid_code("XYZ-0000"));
}
