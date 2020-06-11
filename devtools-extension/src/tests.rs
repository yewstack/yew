use crate::*;
#[test]
fn test_get_depth() {
    assert_eq!(compute_depth(String::from("DIV/H1")), 2);
    assert_eq!(compute_depth(String::from("DIV/H1/P/H4")), 4);
    assert_eq!(compute_depth(String::from("DIV/P/B/I/UL/LI")), 6);
}
