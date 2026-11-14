pub fn is_valid_code(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 8
        && b[..3].iter().all(|c| c.is_ascii_uppercase())
        && b[3] == b'-'
        && b[4..].iter().all(|c| c.is_ascii_digit())
}
