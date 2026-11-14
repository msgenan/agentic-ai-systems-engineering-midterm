use migrated::*;

#[test]
fn basic() {
    assert_eq!(checksum(b""), 7);
    assert_eq!(checksum(b"ab"), 9832);
}
