use migrated::*;

#[test]
fn growth_and_order() {
    let mut t = Table::new();
    for i in 0..20 {
        t.put(&format!("k{i}"), i);
    }
    assert_eq!(t.capacity(), 32);
    assert_eq!(t.len(), 20);
    assert_eq!(t.get("k13"), Some(13));
}

#[test]
fn slot_order() {
    let mut t = Table::new();
    for (i, k) in ["zeta", "alpha", "mid", "q", "longer_key", "b"].iter().enumerate() {
        t.put(k, i as i32);
    }
    assert_eq!(t.capacity(), 16);
    assert_eq!(t.keys(), vec!["longer_key", "q", "b", "zeta", "alpha", "mid"]);
    t.delete("mid");
    t.put("new", 7);
    assert_eq!(t.keys(), vec!["longer_key", "q", "b", "zeta", "alpha", "new"]);
}

#[test]
fn tombstones_count_towards_growth() {
    let mut t = Table::new();
    for (i, k) in ["a", "b", "c", "d", "e"].iter().enumerate() {
        t.put(k, i as i32);
    }
    for k in ["a", "b", "c", "d", "e"] {
        t.delete(k);
    }
    assert_eq!(t.capacity(), 8);
    t.put("x", 9);
    assert_eq!(t.capacity(), 16);
    assert_eq!(t.len(), 1);
    assert_eq!(t.keys(), vec!["x"]);
    assert_eq!(t.get("a"), None);
}

#[test]
fn tombstone_reuse() {
    let mut t = Table::new();
    t.put("apple", 1);
    t.put("banana", 2);
    t.put("cherry", 3);
    t.delete("banana");
    t.put("date", 4);
    assert_eq!(t.keys(), vec!["cherry", "date", "apple"]);
    t.put("apple", 10);
    assert_eq!(t.get("apple"), Some(10));
    assert_eq!(t.len(), 3);
    assert_eq!(t.capacity(), 8);
    assert!(!t.delete("banana"));
}

#[test]
fn churn() {
    let mut t = Table::new();
    for r in 0..3 {
        for i in 0..4 {
            t.put(&format!("r{r}_{i}"), i);
        }
        for i in 0..4 {
            t.delete(&format!("r{r}_{i}"));
        }
    }
    t.put("z", 1);
    assert_eq!(t.capacity(), 16);
    assert_eq!(t.len(), 1);
    assert_eq!(t.keys(), vec!["z"]);
}
