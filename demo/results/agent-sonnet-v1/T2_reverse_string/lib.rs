/// Returns the reversed string (the C version reverses in place).
pub fn reverse_string(s: &str) -> String {
    // C stops at the first NUL byte.
    let s = s.split('\0').next().unwrap_or("");
    s.chars().rev().collect()
}
