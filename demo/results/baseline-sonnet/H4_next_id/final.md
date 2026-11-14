```rust
use std::sync::atomic::{AtomicI32, Ordering};

static NEXT: AtomicI32 = AtomicI32::new(0);

pub fn next_id() -> i32 {
    NEXT.fetch_add(1, Ordering::SeqCst).wrapping_add(1)
}

pub fn reset_ids() {
    NEXT.store(0, Ordering::SeqCst);
}
```

- The C static `next` becomes a static `AtomicI32`, which is safe and thread-safe without `unsafe`.
- On overflow it wraps instead of hitting C's undefined behaviour.
- The counter is global, so parallel tests that share it can interfere with each other, just as they would with the C version.