use migrated::*;

#[test]
fn basic() {
    let mut d = [0; 3];
    copy_ints(&mut d, &[1, 2, 3]);
    assert_eq!(d, [1, 2, 3]);
}
