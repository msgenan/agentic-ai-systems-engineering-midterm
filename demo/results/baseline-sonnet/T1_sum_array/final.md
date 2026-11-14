```rust
pub fn sum_array(a: &[i32]) -> i64 {
    a.iter().map(|&x| x as i64).sum()
}
```

Notes:
- C `long` is mapped to `i64` as the stub specifies (64-bit LP64 platforms); each `i32` is widened before summing, matching the C behaviour.