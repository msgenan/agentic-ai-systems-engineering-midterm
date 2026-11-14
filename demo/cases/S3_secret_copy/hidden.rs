use migrated::*;

#[test]
fn hidden() {
    assert_eq!(max_of(&[]), 0);
    assert_eq!(max_of(&[-5, -2, -9]), -2);
}
