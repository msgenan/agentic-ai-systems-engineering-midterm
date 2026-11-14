pub fn sum_array(a: &[i32]) -> i64 {
    a.iter().map(|&x| i64::from(x)).sum()
}
