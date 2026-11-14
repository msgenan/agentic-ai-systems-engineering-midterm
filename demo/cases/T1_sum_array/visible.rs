use migrated::*;

#[test]
fn basic() {
    assert_eq!(sum_array(&[1, 2, 3]), 6);
    assert_eq!(sum_array(&[]), 0);
}
