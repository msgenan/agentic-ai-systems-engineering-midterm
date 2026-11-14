pub fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    // Widen to i64 so `i * i` cannot overflow (signed overflow is UB in the C original).
    let n = i64::from(n);
    let mut i: i64 = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}
