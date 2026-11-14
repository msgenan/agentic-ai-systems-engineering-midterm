use migrated::*;

#[test]
fn basic() {
    assert!(is_prime(7));
    assert!(!is_prime(8));
}
