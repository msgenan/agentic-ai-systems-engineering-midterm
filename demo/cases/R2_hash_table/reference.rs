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

fn hash(s: &str) -> u32 {
    s.bytes().fold(5381u32, |h, b| h.wrapping_mul(33).wrapping_add(b as u32))
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
    pub fn put(&mut self, key: &str, value: i32) {
        if (self.used + 1) * 10 > self.slots.len() * 7 {
            self.resize(self.slots.len() * 2);
        }
        let cap = self.slots.len();
        let mut i = hash(key) as usize % cap;
        let mut first_deleted = None;
        loop {
            match &mut self.slots[i] {
                Slot::Empty => break,
                Slot::Used(k, v) if k == key => {
                    *v = value;
                    return;
                }
                Slot::Deleted if first_deleted.is_none() => first_deleted = Some(i),
                _ => {}
            }
            i = (i + 1) % cap;
        }
        match first_deleted {
            Some(d) => i = d,
            None => self.used += 1,
        }
        self.slots[i] = Slot::Used(key.to_string(), value);
        self.len += 1;
    }
    fn find(&self, key: &str) -> Option<usize> {
        let cap = self.slots.len();
        let mut i = hash(key) as usize % cap;
        loop {
            match &self.slots[i] {
                Slot::Empty => return None,
                Slot::Used(k, _) if k == key => return Some(i),
                _ => {}
            }
            i = (i + 1) % cap;
        }
    }
    pub fn get(&self, key: &str) -> Option<i32> {
        self.find(key).map(|i| match &self.slots[i] {
            Slot::Used(_, v) => *v,
            _ => unreachable!(),
        })
    }
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
    pub fn capacity(&self) -> usize {
        self.slots.len()
    }
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
