//! What prefs · config.rs is held to.

use super::*;

#[test]
fn auto_units_keep_numbers_short() {
    assert_eq!(UnitDisplay::Auto.format(10.0), "10 mm");
    assert_eq!(UnitDisplay::Auto.format(500.0), "500 mm");
    assert_eq!(UnitDisplay::Auto.format(1000.0), "1 m");
    assert_eq!(UnitDisplay::Auto.format(50_000.0), "50 m");
    assert_eq!(UnitDisplay::Auto.format(2_000_000.0), "2 km");
    assert_eq!(UnitDisplay::Auto.format(0.05), "50 µm");
    assert_eq!(
        UnitDisplay::Fixed(LengthUnit::Millimeter).format(50_000.0),
        "50000 mm"
    );
}

#[test]
fn lengths_are_formatted_without_useless_decimals() {
    assert_eq!(LengthUnit::Millimeter.format(10.0), "10 mm");
    assert_eq!(LengthUnit::Millimeter.format(0.5), "0.5 mm");
    assert_eq!(LengthUnit::Millimeter.format(1000.0), "1000 mm");
    assert_eq!(LengthUnit::Meter.format(1000.0), "1 m");
    assert_eq!(LengthUnit::Centimeter.format(25.0), "2.5 cm");
}

#[test]
fn a_length_shown_in_full_keeps_every_digit_it_has() {
    let mm = UnitDisplay::Fixed(LengthUnit::Millimeter);

    assert_eq!(
        mm.in_full(40.001531),
        "40.001531 mm",
        "a measure reports what is there; rounding a report is how a drawing \
         that drifted by a hair goes on looking exact",
    );
    assert_eq!(
        mm.format(40.001531),
        "40 mm",
        "while a dimension, which is typed and read back, still rounds",
    );
}

#[test]
fn a_round_length_shown_in_full_grows_no_decimals_of_its_own() {
    let mm = UnitDisplay::Fixed(LengthUnit::Millimeter);

    assert_eq!(
        mm.in_full(35.0),
        "35 mm",
        "nothing is invented either: a length that is exactly thirty-five says so",
    );
}
