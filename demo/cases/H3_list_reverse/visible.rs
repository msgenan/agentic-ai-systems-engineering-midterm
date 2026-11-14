use migrated::*;

#[test]
fn basic() {
    assert_eq!(to_vec(&list_reverse(from_vec(&[1, 2, 3]))), vec![3, 2, 1]);
}
