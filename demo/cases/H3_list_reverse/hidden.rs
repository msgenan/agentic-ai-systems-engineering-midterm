use migrated::*;

#[test]
fn hidden() {
    assert_eq!(to_vec(&list_reverse(None)), Vec::<i32>::new());
    assert_eq!(to_vec(&list_reverse(from_vec(&[5]))), vec![5]);
    let v: Vec<i32> = (0..10000).collect();
    let mut r = v.clone();
    r.reverse();
    assert_eq!(to_vec(&list_reverse(from_vec(&v))), r);
}
