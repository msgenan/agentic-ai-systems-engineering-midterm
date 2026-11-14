The C code uses `uint32_t` arithmetic, which wraps on overflow, so the Rust version uses `wrapping_mul` and `wrapping_add`.

```rust
pub fn checksum(data: &[u8]) -> u32 {
    let mut h: u32 = 7;
    for &b in data {
        h = h.wrapping_mul(31).wrapping_add(b as u32);
    }
    h
}
```

Check: for "ab", 7*31+97 = 314, then 314*31+98 = 9832, which matches the test.