```rust
#[derive(Clone)]
enum Slot {
    Empty,
    Used(String, i32),
    Deleted,
}

pub struct Table {
    slots: Vec<Slot>,
    cap: usize,
    len: usize,
    used: usize,
}

/// C strings end at the first NUL byte, so mimic that.
fn cstr(s: &str) -> &str {
    match s.find('\0') {
        Some(p) => &s[..p],
        None => s,
    }
}

fn hash(s: &str) -> u32 {
    let mut h: u32 = 5381;
    for &b in s.as_bytes() {
        h = h.wrapping_mul(33).wrapping_add(b as u32);
    }
    h
}

impl Table {
    pub fn new() -> Self {
        Table {
            slots: vec![Slot::Empty; 8],
            cap: 8,
            len: 0,
            used: 0,
        }
    }

    fn resize(&mut self, new_cap: usize) {
        let old = std::mem::replace(&mut self.slots, vec![Slot::Empty; new_cap]);
        self.cap = new_cap;
        self.len = 0;
        self.used = 0;
        for slot in old {
            if let Slot::Used(k, v) = slot {
                self.put(&k, v);
            }
        }
    }

    /// Inserts or updates.
    pub fn put(&mut self, key: &str, value: i32) {
        let key = cstr(key);
        if (self.used + 1) * 10 > self.cap * 7 {
            self.resize(self.cap * 2);
        }
        let mut i = hash(key) as usize % self.cap;
        let mut first_deleted: Option<usize> = None;
        loop {
            match &mut self.slots[i] {
                Slot::Empty => break,
                Slot::Used(k, v) => {
                    if k == key {
                        *v = value;
                        return;
                    }
                }
                Slot::Deleted => {
                    if first_deleted.is_none() {
                        first_deleted = Some(i);
                    }
                }
            }
            i = (i + 1) % self.cap;
        }
        match first_deleted {
            Some(d) => i = d,
            None => self.used += 1,
        }
        self.slots[i] = Slot::Used(key.to_string(), value);
        self.len += 1;
    }

    pub fn get(&self, key: &str) -> Option<i32> {
        let key = cstr(key);
        let mut i = hash(key) as usize % self.cap;
        loop {
            match &self.slots[i] {
                Slot::Empty => return None,
                Slot::Used(k, v) if k == key => return Some(*v),
                _ => {}
            }
            i = (i + 1) % self.cap;
        }
    }

    /// Returns true if the key was removed.
    pub fn delete(&mut self, key: &str) -> bool {
        let key = cstr(key);
        let mut i = hash(key) as usize % self.cap;
        loop {
            match &self.slots[i] {
                Slot::Empty => return false,
                Slot::Used(k, _) if k == key => {
                    self.slots[i] = Slot::Deleted;
                    self.len -= 1;
                    return true;
                }
                _ => {}
            }
            i = (i + 1) % self.cap;
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Number of slots, like ht_capacity.
    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// Live keys in slot order, like ht_keys.
    pub fn keys(&self) -> Vec<String> {
        self.slots
            .iter()
            .filter_map(|s| match s {
                Slot::Used(k, _) => Some(k.clone()),
                _ => None,
            })
            .collect()
    }
}
```

- Keys are truncated at the first NUL byte, matching C string semantics for hashing and comparison.
- `resize` re-inserts through `put`, as in C, and tombstones are dropped on resize.
- I haven't compiled this or run the tests.