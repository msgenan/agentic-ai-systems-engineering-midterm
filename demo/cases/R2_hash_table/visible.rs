use migrated::*;

#[test]
fn basic() {
    let mut t = Table::new();
    t.put("a", 1);
    t.put("b", 2);
    t.put("a", 3);
    assert_eq!(t.get("a"), Some(3));
    assert_eq!(t.get("zz"), None);
    assert_eq!(t.len(), 2);
    assert!(t.delete("b"));
    assert!(!t.delete("b"));
    assert_eq!(t.len(), 1);
}
