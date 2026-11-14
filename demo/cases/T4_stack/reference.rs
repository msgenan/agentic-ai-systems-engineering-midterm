pub struct Stack {
    data: Vec<i32>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { data: Vec::with_capacity(4) }
    }
    pub fn push(&mut self, v: i32) {
        self.data.push(v)
    }
    pub fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }
    pub fn peek(&self) -> Option<i32> {
        self.data.last().copied()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
}
