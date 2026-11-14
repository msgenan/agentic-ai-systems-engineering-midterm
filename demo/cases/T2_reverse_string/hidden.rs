use migrated::*;

#[test]
fn hidden() {
    assert_eq!(reverse_string("a"), "a");
    assert_eq!(reverse_string("racecar"), "racecar");
    assert_eq!(reverse_string("ab cd!"), "!dc ba");
}
