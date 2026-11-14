pub fn checksum(data: &[u8]) -> u32 {
    data.iter()
        .fold(7u32, |h, &b| h.wrapping_mul(31).wrapping_add(b as u32))
}
