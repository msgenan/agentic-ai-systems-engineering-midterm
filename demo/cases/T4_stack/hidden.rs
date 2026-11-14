use migrated::*;

#[test]
fn hidden() {
    let mut s = Stack::new();
    assert_eq!(s.pop(), None);
    assert_eq!(s.peek(), None);
    for i in 0..100 {
        s.push(i);
    }
    assert_eq!(s.len(), 100);
    assert_eq!(s.peek(), Some(99));
    assert_eq!(s.len(), 100);
    for i in (0..100).rev() {
        assert_eq!(s.pop(), Some(i));
    }
    assert_eq!(s.pop(), None);
}
