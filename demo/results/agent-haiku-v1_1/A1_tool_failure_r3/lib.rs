pub fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    // Use i64 for the square so `i * i` cannot overflow near i32::MAX
    // (the C version would overflow here, which is undefined behaviour).
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
