pub fn average(a: &[i32]) -> f64 {
    if a.is_empty() {
        return 0.0;
    }
    let sum: i64 = a.iter().map(|&x| i64::from(x)).sum();
    sum as f64 / a.len() as f64
}
