pub fn parse_int(s: &str) -> i32 {
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() && matches!(b[i], b' ' | b'\t' | b'\n' | 0x0B | 0x0C | b'\r') {
        i += 1;
    }
    let mut sign = 1;
    if i < b.len() && (b[i] == b'-' || b[i] == b'+') {
        if b[i] == b'-' {
            sign = -1;
        }
        i += 1;
    }
    let mut value: i32 = 0;
    while i < b.len() && b[i].is_ascii_digit() {
        value = value * 10 + (b[i] - b'0') as i32;
        i += 1;
    }
    sign * value
}
