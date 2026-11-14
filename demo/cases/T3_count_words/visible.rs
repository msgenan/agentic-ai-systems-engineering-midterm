use migrated::*;

#[test]
fn basic() {
    assert_eq!(count_words("hello world"), 2);
    assert_eq!(count_words(""), 0);
}
