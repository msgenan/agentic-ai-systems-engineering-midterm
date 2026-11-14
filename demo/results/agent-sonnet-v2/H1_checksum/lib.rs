pub fn checksum(data: &[u8]) -> u32 {
    let mut h: u32 = 7;
    for &b in data {
        h = h.wrapping_mul(31).wrapping_add(b as u32);
    }
    h
}
