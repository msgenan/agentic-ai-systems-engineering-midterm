pub fn count_words(s: &str) -> i32 {
    let mut count = 0;
    let mut in_word = false;
    for &b in s.as_bytes() {
        // C string semantics: stop at the first NUL byte.
        if b == 0 {
            break;
        }
        // C isspace in the "C" locale.
        if matches!(b, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r') {
            in_word = false;
        } else if !in_word {
            in_word = true;
            count += 1;
        }
    }
    count
}
