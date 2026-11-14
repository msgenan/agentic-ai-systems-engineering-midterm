```rust
pub struct Node {
    pub value: i32,
    pub next: Option<Box<Node>>,
}

/// Helper (already implemented): builds a list from a slice.
pub fn from_vec(v: &[i32]) -> Option<Box<Node>> {
    let mut head = None;
    for &x in v.iter().rev() {
        head = Some(Box::new(Node { value: x, next: head }));
    }
    head
}

/// Helper (already implemented): collects list values.
pub fn to_vec(mut cur: &Option<Box<Node>>) -> Vec<i32> {
    let mut out = Vec::new();
    while let Some(n) = cur {
        out.push(n.value);
        cur = &n.next;
    }
    out
}

pub fn list_reverse(mut head: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut prev: Option<Box<Node>> = None;
    while let Some(mut node) = head {
        head = node.next.take();
        node.next = prev;
        prev = Some(node);
    }
    prev
}
```

Iterative pointer reversal, same as the C loop, using `take()` to move ownership safely.