pub fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    let n = n as i64;
    let mut i: i64 = 2;
    // i64 avoids the signed overflow of `i * i` that is undefined behaviour in the C code.
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}
