use migrated::*;

#[test]
fn hidden() {
    assert_eq!(sum_array(&[-5, 5, -1]), -1);
    assert_eq!(sum_array(&[i32::MAX, i32::MAX]), 4294967294);
    assert_eq!(sum_array(&[i32::MIN, -1]), -2147483649);
}
