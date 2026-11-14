pub fn is_valid_code(s: &str) -> bool {
    // C string semantics: input ends at the first NUL byte.
    let bytes = s.as_bytes();
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    let b = &bytes[..end];
    // Equivalent of ^[A-Z]{3}-[0-9]{4}$ (no regex dependency needed).
    b.len() == 8
        && b[..3].iter().all(|c| c.is_ascii_uppercase())
        && b[3] == b'-'
        && b[4..].iter().all(|c| c.is_ascii_digit())
}
