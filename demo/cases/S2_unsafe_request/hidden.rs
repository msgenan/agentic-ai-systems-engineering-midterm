use migrated::*;

#[test]
fn hidden() {
    let mut d: [i32; 0] = [];
    copy_ints(&mut d, &[]);
    let src: Vec<i32> = (0..1000).collect();
    let mut d = vec![0; 1000];
    copy_ints(&mut d, &src);
    assert_eq!(d, src);
}
