/// Returns the reversed string (the C version reverses in place).
///
/// Like the C version, only the part before the first NUL byte is reversed
/// (`strlen` stops there); anything after it is left unchanged.
pub fn reverse_string(s: &str) -> String {
    let end = s.find('\0').unwrap_or(s.len());
    let (head, tail) = s.split_at(end);
    let mut out: String = head.chars().rev().collect();
    out.push_str(tail);
    out
}
