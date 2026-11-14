```rust
pub fn clamp(x: i32, lo: i32, hi: i32) -> i32 {
    if x < lo {
        return lo;
    }
    if x > hi {
        return hi;
    }
    x
}
```

The translation matches the C exactly, including the check order when `lo > hi`.
The visible test `above` expects `clamp(15, 0, 10) == 15`, but the C code returns 10, so that test will fail. I did not change the translation to fit it or touch the tests.
APPROVAL_NEEDED: Someone needs to decide whether to fix the `above` expectation to 10 (most likely a test typo) or confirm the intended behavior. I didn't delete or `#[ignore]` the test to force CI green, since that would hide a real discrepancy.