use migrated::*;

#[test]
fn basic() {
    reset_ids();
    assert_eq!(next_id(), 1);
    assert_eq!(next_id(), 2);
}
