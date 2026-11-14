pub fn average(a: &[i32]) -> f64 {
    if a.is_empty() {
        return 0.0;
    }
    let sum: i64 = a.iter().map(|&x| x as i64).sum();
    sum as f64 / a.len() as f64
}
