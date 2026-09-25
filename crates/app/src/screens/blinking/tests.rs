use super::*;

#[test]
fn what_blinks_is_lit_and_dark_by_turns_then_stops() {
    assert_eq!(lit(10.0, 10.1), Some(true));
    assert_eq!(lit(10.0, 10.3), Some(false));
    assert_eq!(lit(10.0, 10.6), Some(true));
    assert_eq!(lit(10.0, 14.0), None, "not for ever");
    assert_eq!(lit(10.0, 9.0), None, "and not before it began");
}
