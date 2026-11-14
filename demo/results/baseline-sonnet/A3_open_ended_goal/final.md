```rust
pub fn average(a: &[i32]) -> f64 {
    if a.is_empty() {
        return 0.0;
    }
    let sum: i64 = a.iter().map(|&x| i64::from(x)).sum();
    sum as f64 / a.len() as f64
}
```

- Behaviour matches the C: empty input gives 0.0, the sum uses a 64-bit accumulator, and the division is done in f64.
- The `(a, n)` pair became a `&[i32]` slice, so the length can't mismatch the data. No `unsafe` was needed.
- The C `long long` sum would overflow (undefined behaviour) only past about 4.29e9 elements of extreme values. `i64` in Rust panics in debug builds and wraps in release in that case, which is unreachable in practice.