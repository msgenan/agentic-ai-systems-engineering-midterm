pub fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    // Use i64 for the square so `i * i` cannot overflow for any i32 `n`
    // (the C version relies on int arithmetic that can overflow).
    let n64 = i64::from(n);
    let mut i: i64 = 2;
    while i * i <= n64 {
        if n64 % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}
