#[derive(Clone, Default)]
enum State {
    #[default]
    Empty,
    Used,
    Deleted,
}

#[derive(Clone, Default)]
struct Slot {
    key: Option<String>,
    value: i32,
    state: State,
}

pub struct Table {
    slots: Vec<Slot>,
    cap: usize,
    len: usize,
    used: usize,
}

/// C strings end at the first NUL byte; mimic that.
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
        let cap = 8;
        Table {
            slots: vec![Slot::default(); cap],
            cap,
            len: 0,
            used: 0,
        }
    }

    fn resize(&mut self, new_cap: usize) {
        let old = std::mem::replace(&mut self.slots, vec![Slot::default(); new_cap]);
        self.cap = new_cap;
        self.len = 0;
        self.used = 0;
        for slot in old {
            if let (State::Used, Some(k)) = (&slot.state, &slot.key) {
                self.put(k, slot.value);
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
            match self.slots[i].state {
                State::Empty => break,
                State::Used => {
                    if self.slots[i].key.as_deref() == Some(key) {
                        self.slots[i].value = value;
                        return;
                    }
                }
                State::Deleted => {
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
        self.slots[i] = Slot {
            key: Some(key.to_string()),
            value,
            state: State::Used,
        };
        self.len += 1;
    }

    pub fn get(&self, key: &str) -> Option<i32> {
        let key = cstr(key);
        let mut i = hash(key) as usize % self.cap;
        loop {
            let s = &self.slots[i];
            match s.state {
                State::Empty => return None,
                State::Used if s.key.as_deref() == Some(key) => return Some(s.value),
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
            match self.slots[i].state {
                State::Empty => return false,
                State::Used if self.slots[i].key.as_deref() == Some(key) => {
                    self.slots[i].key = None;
                    self.slots[i].state = State::Deleted;
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
            .filter(|s| matches!(s.state, State::Used))
            .filter_map(|s| s.key.clone())
            .collect()
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}
