```rust
pub fn parse_int(s: &str) -> i32 {
    let b = s.as_bytes();
    let mut i = 0;
    // C isspace: ' ', \t, \n, \v (0x0B), \f (0x0C), \r
    while i < b.len() && matches!(b[i], b' ' | b'\t' | b'\n' | 0x0B | 0x0C | b'\r') {
        i += 1;
    }
    let mut sign: i32 = 1;
    if i < b.len() && (b[i] == b'-' || b[i] == b'+') {
        if b[i] == b'-' {
            sign = -1;
        }
        i += 1;
    }
    let mut value: i32 = 0;
    while i < b.len() && b[i].is_ascii_digit() {
        value = value.wrapping_mul(10).wrapping_add((b[i] - b'0') as i32);
        i += 1;
    }
    sign.wrapping_mul(value)
}
```

- Whitespace is matched by hand because Rust's `is_ascii_whitespace` omits `\v`, which C's `isspace` includes.
- Arithmetic wraps, so there are no panics on overflow; inputs are stated to stay within `int` range anyway.