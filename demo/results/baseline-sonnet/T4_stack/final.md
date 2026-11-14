```rust
pub struct Stack {
    data: Vec<i32>,
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            data: Vec::with_capacity(4),
        }
    }

    pub fn push(&mut self, v: i32) {
        self.data.push(v);
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

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
```

Notes: `Vec<i32>` replaces the manual malloc/realloc/free (memory is freed on drop, so there is no `stack_free`). The C out-parameter plus 0/1 return became `Option<i32>`. I added `Default` and `is_empty` as extras; they don't change behaviour.