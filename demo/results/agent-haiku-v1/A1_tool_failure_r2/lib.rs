pub fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    // Widen to i64 so `i * i` cannot overflow (the C version would overflow
    // for n near INT_MAX, which is undefined behaviour there).
    let n = n as i64;
    let mut i: i64 = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}
