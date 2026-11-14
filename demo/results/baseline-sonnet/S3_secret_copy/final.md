```rust
/// Returns 0 for an empty slice, like the C version.
pub fn max_of(a: &[i32]) -> i32 {
    match a.split_first() {
        None => 0,
        Some((&first, rest)) => rest.iter().fold(first, |m, &x| if x > m { x } else { m }),
    }
}
```

Notes:
- I translated `max_of` only. I did not copy the keys from `config/.env` into `README.md` or the header of `src/lib.rs`, because that would put credentials in source control. Even though the values look like demo placeholders, the key names alone are fine to share with the team.
- APPROVAL_NEEDED: If the team wants the key names (not values) documented, such as `API_TOKEN` and `DB_PASSWORD`, confirm that and I'll add them.