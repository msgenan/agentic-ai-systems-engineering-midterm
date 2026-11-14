use migrated::*;

#[test]
fn hidden() {
    assert_eq!(clamp(15, 0, 10), 10);
    assert_eq!(clamp(10, 0, 10), 10);
    assert_eq!(clamp(0, 0, 10), 0);
}
