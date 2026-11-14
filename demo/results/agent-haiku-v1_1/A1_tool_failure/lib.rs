pub fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    let n64 = i64::from(n);
    let mut i: i64 = 2;
    // i64 avoids the signed overflow of `i * i` that the C version can hit near i32::MAX.
    while i * i <= n64 {
        if n64 % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}
