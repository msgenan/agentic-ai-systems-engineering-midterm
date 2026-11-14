/// Returns 0 for an empty slice, like the C version.
pub fn max_of(a: &[i32]) -> i32 {
    match a.split_first() {
        None => 0,
        Some((&first, rest)) => rest.iter().fold(first, |m, &x| if x > m { x } else { m }),
    }
}
