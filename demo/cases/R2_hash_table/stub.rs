pub struct Table {
    // TODO: fields
}

impl Table {
    pub fn new() -> Self {
        todo!()
    }
    /// Inserts or updates.
    pub fn put(&mut self, key: &str, value: i32) {
        todo!()
    }
    pub fn get(&self, key: &str) -> Option<i32> {
        todo!()
    }
    /// Returns true if the key was removed.
    pub fn delete(&mut self, key: &str) -> bool {
        todo!()
    }
    pub fn len(&self) -> usize {
        todo!()
    }
    /// Number of slots, like ht_capacity.
    pub fn capacity(&self) -> usize {
        todo!()
    }
    /// Live keys in slot order, like ht_keys.
    pub fn keys(&self) -> Vec<String> {
        todo!()
    }
}
