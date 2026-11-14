#[derive(Clone)]
enum Slot {
    Empty,
    Used(String, i32),
    Deleted,
}

pub struct Table {
    slots: Vec<Slot>,
    len: usize,
    used: usize,
}

/// C strings end at the first NUL byte; mimic that.
fn cstr(s: &str) -> &[u8] {
    let b = s.as_bytes();
    match b.iter().position(|&c| c == 0) {
        Some(p) => &b[..p],
        None => b,
    }
}

fn hash(s: &[u8]) -> u32 {
    let mut h: u32 = 5381;
    for &c in s {
        h = h.wrapping_mul(33).wrapping_add(c as u32);
    }
    h
}

impl Table {
    pub fn new() -> Self {
        Table { slots: vec![Slot::Empty; 8], len: 0, used: 0 }
    }

    fn resize(&mut self, new_cap: usize) {
        let old = std::mem::replace(&mut self.slots, vec![Slot::Empty; new_cap]);
        self.len = 0;
        self.used = 0;
        for s in old {
            if let Slot::Used(k, v) = s {
                self.put(&k, v);
            }
        }
    }

    /// Inserts or updates.
    pub fn put(&mut self, key: &str, value: i32) {
        let key = String::from_utf8_lossy(cstr(key)).into_owned();
        let kb = key.as_bytes();
        if (self.used + 1) * 10 > self.slots.len() * 7 {
            self.resize(self.slots.len() * 2);
        }
        let cap = self.slots.len();
        let mut i = hash(kb) as usize % cap;
        let mut first_deleted: Option<usize> = None;
        loop {
            match &mut self.slots[i] {
                Slot::Empty => break,
                Slot::Used(k, v) => {
                    if k.as_bytes() == kb {
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
        self.slots[i] = Slot::Used(key, value);
        self.len += 1;
    }

    fn find(&self, key: &str) -> Option<usize> {
        let kb = cstr(key);
        let cap = self.slots.len();
        let mut i = hash(kb) as usize % cap;
        loop {
            match &self.slots[i] {
                Slot::Empty => return None,
                Slot::Used(k, _) if k.as_bytes() == kb => return Some(i),
                _ => {}
            }
            i = (i + 1) % cap;
        }
    }

    pub fn get(&self, key: &str) -> Option<i32> {
        match self.find(key) {
            Some(i) => match &self.slots[i] {
                Slot::Used(_, v) => Some(*v),
                _ => None,
            },
            None => None,
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
                Slot::Used(k, _) => Some(k.clone()),
                _ => None,
            })
            .collect()
    }
}
