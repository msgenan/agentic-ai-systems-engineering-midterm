/// One slot of the open-addressing table.
/// Mirrors the C `state` field: Empty = 0, Used = 1, Deleted = 2 (tombstone).
#[derive(Clone, Debug)]
enum Slot {
    Empty,
    Used { key: String, value: i32 },
    Deleted,
}

/// Open-addressing hash table with linear probing and tombstones.
pub struct Table {
    slots: Vec<Slot>,
    /// Live entries.
    len: usize,
    /// Live entries + tombstones.
    used: usize,
}

/// DJB2 hash with 32-bit wrapping arithmetic, as in the C `hash`.
fn hash(s: &str) -> u32 {
    let mut h: u32 = 5381;
    for b in s.bytes() {
        h = h.wrapping_mul(33).wrapping_add(b as u32);
    }
    h
}

impl Table {
    pub fn new() -> Self {
        Table {
            slots: vec![Slot::Empty; 8],
            len: 0,
            used: 0,
        }
    }

    /// Inserts or updates. Grows when (used + 1) / cap would exceed 0.7.
    pub fn put(&mut self, key: &str, value: i32) {
        let cap = self.slots.len();
        if (self.used + 1) * 10 > cap * 7 {
            self.resize(cap * 2);
        }
        let cap = self.slots.len();
        let mut i = hash(key) as usize % cap;
        let mut first_deleted: Option<usize> = None;
        while !matches!(self.slots[i], Slot::Empty) {
            match &mut self.slots[i] {
                Slot::Used { key: k, value: v } if k == key => {
                    *v = value;
                    return;
                }
                Slot::Deleted if first_deleted.is_none() => {
                    first_deleted = Some(i);
                }
                _ => {}
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

    pub fn get(&self, key: &str) -> Option<i32> {
        let cap = self.slots.len();
        let mut i = hash(key) as usize % cap;
        while !matches!(self.slots[i], Slot::Empty) {
            if let Slot::Used { key: k, value } = &self.slots[i] {
                if k == key {
                    return Some(*value);
                }
            }
            i = (i + 1) % cap;
        }
        None
    }

    /// Returns true if the key was removed.
    pub fn delete(&mut self, key: &str) -> bool {
        let cap = self.slots.len();
        let mut i = hash(key) as usize % cap;
        while !matches!(self.slots[i], Slot::Empty) {
            if let Slot::Used { key: k, .. } = &self.slots[i] {
                if k == key {
                    self.slots[i] = Slot::Deleted;
                    self.len -= 1;
                    return true;
                }
            }
            i = (i + 1) % cap;
        }
        false
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
            .filter_map(|s| match s {
                Slot::Used { key, .. } => Some(key.clone()),
                _ => None,
            })
            .collect()
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}
