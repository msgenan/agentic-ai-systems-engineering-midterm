```rust
/// Open-addressing hash table with linear probing and tombstones.
#[derive(Clone)]
enum Slot {
    Empty,
    Used { key: String, value: i32 },
    Deleted,
}

pub struct Table {
    slots: Vec<Slot>,
    /// Live entries.
    len: usize,
    /// Live entries + tombstones.
    used: usize,
}

/// djb2 hash with 32-bit wrapping arithmetic, matching the C version.
fn hash(s: &str) -> u32 {
    let mut h: u32 = 5381;
    for &b in s.as_bytes() {
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

    /// Inserts or updates.
    pub fn put(&mut self, key: &str, value: i32) {
        if (self.used + 1) * 10 > self.capacity() * 7 {
            let new_cap = self.capacity() * 2;
            self.resize(new_cap);
        }
        let cap = self.slots.len();
        let mut i = hash(key) as usize % cap;
        let mut first_deleted: Option<usize> = None;
        loop {
            match &mut self.slots[i] {
                Slot::Empty => break,
                Slot::Used { key: k, value: v } if k.as_str() == key => {
                    *v = value;
                    return;
                }
                Slot::Deleted => {
                    if first_deleted.is_none() {
                        first_deleted = Some(i);
                    }
                }
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

    /// Returns the slot index of a live entry with this key, if any.
    fn find(&self, key: &str) -> Option<usize> {
        let cap = self.slots.len();
        let mut i = hash(key) as usize % cap;
        loop {
            match &self.slots[i] {
                Slot::Empty => return None,
                Slot::Used { key: k, .. } if k.as_str() == key => return Some(i),
                _ => {}
            }
            i = (i + 1) % cap;
        }
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
        let Some(i) = self.find(key) else {
            return false;
        };
        self.slots[i] = Slot::Deleted;
        self.len -= 1;
        true
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
            .filter_map(|slot| match slot {
                Slot::Used { key, .. } => Some(key.clone()),
                _ => None,
            })
            .collect()
    }
}
```

- The C code's hashing, probing, tombstone reuse, growth check (before lookup), and slot-order reinsertion on resize are all preserved; the C `free` calls map to Rust ownership.
- `ht_keys`'s `max` parameter is omitted because the given API returns all live keys; callers needing a limit can truncate the `Vec`.
- Uses no `unsafe` and no dependencies; the existing test was not modified.