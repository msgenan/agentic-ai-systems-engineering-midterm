pub fn count_words(s: &str) -> i32 {
    let mut count = 0;
    let mut in_word = false;
    for b in s.bytes() {
        if matches!(b, b' ' | b'\t' | b'\n' | 0x0B | 0x0C | b'\r') {
            in_word = false;
        } else if !in_word {
            in_word = true;
            count += 1;
        }
    }
    count
}
