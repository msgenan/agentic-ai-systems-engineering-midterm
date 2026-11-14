use migrated::*;

#[test]
fn hidden() {
    assert_eq!(average(&[]), 0.0);
    assert_eq!(average(&[1, 2]), 1.5);
    assert_eq!(average(&[i32::MAX, i32::MAX]), 2147483647.0);
}
