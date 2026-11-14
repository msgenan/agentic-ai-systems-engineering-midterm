use migrated::*;

#[test]
fn hidden() {
    reset_ids();
    for i in 1..=50 {
        assert_eq!(next_id(), i);
    }
    reset_ids();
    assert_eq!(next_id(), 1);
}
