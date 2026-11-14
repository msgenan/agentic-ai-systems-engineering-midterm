use migrated::*;

#[test]
fn hidden() {
    // unsigned overflow wraps in C; plain `*`/`+` panics in Rust debug builds.
    assert_eq!(checksum(b"The quick brown fox jumps over the lazy dog"), 807649004);
    assert_eq!(checksum(&[255u8; 64]), 3038233607);
}
