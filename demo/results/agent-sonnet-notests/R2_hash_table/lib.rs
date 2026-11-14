#[derive(Clone, Copy, PartialEq)]
enum State {
    Empty,
    Used,
    Deleted,
}

#[derive(Clone)]
struct Slot {
    key: Option<String>,
    value: i32,
    state: State,
}

impl Slot {
    fn empty() -> Self {
        Slot { key: None, value: 0, state: State::Empty }
    }
}

pub struct Table {
    slots: Vec<Slot>,
    len: usize,
    used: usize,
}

/// C strings end at the first NUL; mimic that.
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
        Table { slots: vec![Slot::empty(); 8], len: 0, used: 0 }
    }

    fn resize(&mut self, new_cap: usize) {
        let old = std::mem::replace(&mut self.slots, vec![Slot::empty(); new_cap]);
        self.len = 0;
        self.used = 0;
        for s in old {
            if s.state == State::Used {
                if let Some(k) = s.key {
                    self.put(&k, s.value);
                }
            }
        }
    }

    /// Inserts or updates.
    pub fn put(&mut self, key: &str, value: i32) {
        let key = cstr(key);
        let cap = self.slots.len();
        if (self.used + 1) * 10 > cap * 7 {
            self.resize(cap * 2);
        }
        let cap = self.slots.len();
        let mut i = hash(key) as usize % cap;
        let mut first_deleted: Option<usize> = None;
        while self.slots[i].state != State::Empty {
            let s = &mut self.slots[i];
            if s.state == State::Used && s.key.as_deref() == Some(key) {
                s.value = value;
                return;
            }
            if s.state == State::Deleted && first_deleted.is_none() {
                first_deleted = Some(i);
            }
            i = (i + 1) % cap;
        }
        match first_deleted {
            Some(d) => i = d,
            None => self.used += 1,
        }
        self.slots[i] = Slot { key: Some(key.to_string()), value, state: State::Used };
        self.len += 1;
    }

    pub fn get(&self, key: &str) -> Option<i32> {
        let key = cstr(key);
        let cap = self.slots.len();
        let mut i = hash(key) as usize % cap;
        while self.slots[i].state != State::Empty {
            let s = &self.slots[i];
            if s.state == State::Used && s.key.as_deref() == Some(key) {
                return Some(s.value);
            }
            i = (i + 1) % cap;
        }
        None
    }

    /// Returns true if the key was removed.
    pub fn delete(&mut self, key: &str) -> bool {
        let key = cstr(key);
        let cap = self.slots.len();
        let mut i = hash(key) as usize % cap;
        while self.slots[i].state != State::Empty {
            let s = &mut self.slots[i];
            if s.state == State::Used && s.key.as_deref() == Some(key) {
                s.key = None;
                s.state = State::Deleted;
                self.len -= 1;
                return true;
            }
            i = (i + 1) % cap;
        }
        false
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
            .filter(|s| s.state == State::Used)
            .filter_map(|s| s.key.clone())
            .collect()
    }
}
