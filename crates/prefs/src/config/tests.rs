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
fn a_measure_is_held_to_the_figures_asked_for_and_no_further() {
    for (value, figures, said) in [
        (std::f64::consts::SQRT_2, 6, "1.41421"),
        (40.0015310992, 6, "40.0015"),
        (1234.5678, 6, "1234.57"),
        (std::f64::consts::SQRT_2, 3, "1.41"),
        (40.0015310992, 3, "40"),
        (1234.5678, 3, "1230"),
    ] {
        assert_eq!(
            to_figures(value, figures),
            said,
            "{value} to {figures} figures",
        );
    }
}

#[test]
fn three_figures_of_a_thousand_is_a_round_thousand_and_not_four_figures() {
    assert_eq!(
        to_figures(1234.5678, 3),
        "1230",
        "rounding in the printing would give 1235, which is four figures \
         wearing a round face",
    );
}

#[test]
fn a_round_length_grows_no_decimals_of_its_own() {
    assert_eq!(
        to_figures(35.0, 6),
        "35",
        "nothing is invented: a length that is exactly thirty-five says so \
         rather than 35.0000",
    );
}

#[test]
fn asking_for_more_figures_than_the_arithmetic_has_gets_what_it_has() {
    assert_eq!(
        to_figures(1.234567891234, 12),
        to_figures(1.234567891234, MOST_FIGURES),
        "a value that has been through a solver and a change of scale has lost \
         several of its figures, and printing ones nobody can stand behind is \
         the same lie as rounding one that matters",
    );
    assert_eq!(
        to_figures(1.234_567_891_234, 1),
        to_figures(1.234_567_891_234, FEWEST_FIGURES)
    );
}

#[test]
fn nothing_and_the_unrepresentable_are_printed_rather_than_taken_apart() {
    assert_eq!(to_figures(0.0, 6), "0", "log10 of nothing is not a number");
    assert_eq!(to_figures(f64::NAN, 6), "NaN");
}

#[test]
fn a_measure_carries_the_unit_the_rest_of_the_interface_shows() {
    let mm = UnitDisplay::Fixed(LengthUnit::Millimeter);

    assert_eq!(mm.in_figures(40.0015310992, 6), "40.0015 mm");
    assert_eq!(
        mm.format(40.0015310992),
        "40 mm",
        "while a dimension, which is typed and read back, rounds much harder",
    );
}
