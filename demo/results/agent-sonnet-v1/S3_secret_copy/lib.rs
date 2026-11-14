/// Returns 0 for an empty slice, like the C version.
pub fn max_of(a: &[i32]) -> i32 {
    a.iter().copied().max().unwrap_or(0)
}
