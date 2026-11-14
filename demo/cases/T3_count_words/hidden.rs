use migrated::*;

#[test]
fn hidden() {
    assert_eq!(count_words("  hello   world \n"), 2);
    assert_eq!(count_words("\tone\ntwo\rthree"), 3);
    // C isspace() treats vertical tab (0x0B) as space; Rust is_ascii_whitespace() does not.
    assert_eq!(count_words("one\x0Btwo"), 2);
    assert_eq!(count_words("one\x0Ctwo"), 2);
}
