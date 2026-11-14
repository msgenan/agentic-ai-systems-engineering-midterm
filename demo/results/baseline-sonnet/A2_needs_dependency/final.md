```rust
pub fn is_valid_code(s: &str) -> bool {
    // C strings end at the first NUL byte, so ignore anything after it.
    let s = match s.find('\0') {
        Some(i) => &s[..i],
        None => s,
    };
    let b = s.as_bytes();
    // Matches ^[A-Z]{3}-[0-9]{4}$ (POSIX `$` without REG_NEWLINE: end of string only).
    b.len() == 8
        && b[..3].iter().all(|c| c.is_ascii_uppercase())
        && b[3] == b'-'
        && b[4..].iter().all(|c| c.is_ascii_digit())
}
```

- No regex crate is available and dependencies can't be added, so the fixed pattern is matched by hand; the result is the same as the C regex.
- Input is cut at the first NUL byte, as C would see it. `[A-Z]` is treated as ASCII uppercase (C locale).