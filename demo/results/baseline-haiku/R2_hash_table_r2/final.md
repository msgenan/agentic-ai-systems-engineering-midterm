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
            slots: (0..8).map(|_| Slot::Empty).collect(),
            len: 0,
            used: 0,
        }
    }

    fn resize(&mut self, new_cap: usize) {
        let old = std::mem::replace(
            &mut self.slots,
            (0..new_cap).map(|_| Slot::Empty).collect(),
        );
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
        if (self.used + 1) * 10 > self.slots.len() * 7 {
            let new_cap = self.slots.len() * 2;
            self.resize(new_cap);
        }
        let cap = self.slots.len();
        let mut i = hash(key) as usize % cap;
        let mut first_deleted: Option<usize> = None;
        loop {
            match &mut self.slots[i] {
                Slot::Empty => break,
                Slot::Used { key: k, value: v } => {
                    if k.as_str() == key {
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
            i = (i + 1) % cap;
        }
        match first_deleted {
            Some(d) => i = d,
            None => self.used += 1,
        }
        self.slots[i] = Slot::Used {
            key: key.to_string(),
            value,
        };
        self.len += 1;
    }

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
        if let Some(i) = self.find(key) {
            if let Slot::Used { value, .. } = &self.slots[i] {
                return Some(*value);
            }
        }
        None
    }

    /// Returns true if the key was removed.
    pub fn delete(&mut self, key: &str) -> bool {
        if let Some(i) = self.find(key) {
            self.slots[i] = Slot::Deleted;
            self.len -= 1;
            true
        } else {
            false
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
- Keys are hashed and compared over their full UTF-8 bytes; C strings would stop at an embedded NUL, so behaviour differs only for keys containing `\0`.
- `put` checks the load factor before probing and rehashes by re-inserting through `put` in old slot order, matching the C slot layout and `keys()` ordering.
- `keys()` returns all live keys since the stub has no `max` argument.