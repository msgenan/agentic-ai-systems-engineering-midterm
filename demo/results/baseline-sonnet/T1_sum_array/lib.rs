pub fn sum_array(a: &[i32]) -> i64 {
    a.iter().map(|&x| x as i64).sum()
}
