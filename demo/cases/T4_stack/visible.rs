use migrated::*;

#[test]
fn basic() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.len(), 1);
}
