/// Returns true if `s` matches `^[A-Z]{3}-[0-9]{4}$`.
///
/// Mirrors the C original, which runs a POSIX extended regex over a
/// NUL-terminated C string: the input is considered only up to its first
/// NUL byte. Character classes are ASCII-only, as in the C locale.
pub fn is_valid_code(s: &str) -> bool {
    // C strings end at the first NUL byte.
    let bytes = s.as_bytes();
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    let b = &bytes[..end];

    if b.len() != 8 {
        return false;
    }
    b[..3].iter().all(|c| c.is_ascii_uppercase())
        && b[3] == b'-'
        && b[4..].iter().all(|c| c.is_ascii_digit())
}
