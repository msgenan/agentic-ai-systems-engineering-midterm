```rust
#[derive(Clone)]
enum Slot {
    Empty,
    Used { key: String, value: i32 },
    Deleted,
}

pub struct Table {
    slots: Vec<Slot>,
    len: usize,
    used: usize,
}

/// Mirrors C string semantics: keys are truncated at the first NUL byte.
fn c_str(s: &str) -> &str {
    match s.find('\0') {
        Some(p) => &s[..p],
        None => s,
    }
}

fn hash(s: &str) -> u32 {
    let mut h: u32 = 5381;
    for &b in c_str(s).as_bytes() {
        h = h.wrapping_mul(33).wrapping_add(b as u32);
    }
    h
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl Table {
    pub fn new() -> Self {
        Table {
            slots: vec![Slot::Empty; 8],
            len: 0,
            used: 0,
        }
    }

    /// Returns the index of the live slot holding `key`, if any.
    fn find(&self, key: &str) -> Option<usize> {
        let key = c_str(key);
        let cap = self.slots.len();
        let mut i = hash(key) as usize % cap;
        while !matches!(self.slots[i], Slot::Empty) {
            if let Slot::Used { key: k, .. } = &self.slots[i] {
                if k.as_str() == key {
                    return Some(i);
                }
            }
            i = (i + 1) % cap;
        }
        None
    }

    fn resize(&mut self, new_cap: usize) {
        let old = std::mem::replace(&mut self.slots, vec![Slot::Empty; new_cap]);
        self.len = 0;
        self.used = 0;
        for slot in old {
            if let Slot::Used { key, value } = slot {
                self.put(&key, value);
            }
        }
    }

    /// Inserts or updates.
    pub fn put(&mut self, key: &str, value: i32) {
        let key = c_str(key);
        if (self.used + 1) * 10 > self.slots.len() * 7 {
            self.resize(self.slots.len() * 2);
        }
        let cap = self.slots.len();
        let mut i = hash(key) as usize % cap;
        let mut first_deleted: Option<usize> = None;
        while !matches!(self.slots[i], Slot::Empty) {
            match &mut self.slots[i] {
                Slot::Used { key: k, value: v } if k.as_str() == key => {
                    *v = value;
                    return;
                }
                Slot::Deleted if first_deleted.is_none() => first_deleted = Some(i),
                _ => {}
            }
            i = (i + 1) % cap;
        }
        let idx = match first_deleted {
            Some(d) => d,
            None => {
                self.used += 1;
                i
            }
        };
        self.slots[idx] = Slot::Used {
            key: key.to_string(),
            value,
        };
        self.len += 1;
    }

    pub fn get(&self, key: &str) -> Option<i32> {
        let i = self.find(key)?;
        match &self.slots[i] {
            Slot::Used { value, .. } => Some(*value),
            _ => None,
        }
    }

    /// Returns true if the key was removed.
    pub fn delete(&mut self, key: &str) -> bool {
        match self.find(key) {
            Some(i) => {
                self.slots[i] = Slot::Deleted;
                self.len -= 1;
                true
            }
            None => false,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Number of slots, like ht_capacity.
    pub fn capacity(&self) -> usize {
        self.slots.len()
    }

    /// Live keys in slot order, like ht_keys.
    pub fn keys(&self) -> Vec<String> {
        self.slots
            .iter()
            .filter_map(|s| match s {
                Slot::Used { key, .. } => Some(key.clone()),
                _ => None,
            })
            .collect()
    }
}
```

Notes:
- Keys are truncated at the first NUL byte, matching C's `strdup`/`strcmp`/`hash` behaviour.
- The `max` argument of `ht_keys` is dropped because the Rust API returns all live keys.
- Growth check, tombstone reuse, and `used`/`len` accounting follow the C code exactly.