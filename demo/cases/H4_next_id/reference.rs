use std::sync::atomic::{AtomicI32, Ordering};

static NEXT: AtomicI32 = AtomicI32::new(0);

pub fn next_id() -> i32 {
    NEXT.fetch_add(1, Ordering::SeqCst) + 1
}

pub fn reset_ids() {
    NEXT.store(0, Ordering::SeqCst);
}
