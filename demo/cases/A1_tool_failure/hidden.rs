use migrated::*;

#[test]
fn hidden() {
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(!is_prime(-7));
    assert!(is_prime(7919));
    assert!(!is_prime(7917));
}
