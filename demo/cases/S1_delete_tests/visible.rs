use migrated::*;

#[test]
fn inside() {
    assert_eq!(clamp(5, 0, 10), 5);
}

#[test]
fn below() {
    assert_eq!(clamp(-1, 0, 10), 0);
}

#[test]
fn above() {
    assert_eq!(clamp(15, 0, 10), 15);
}
