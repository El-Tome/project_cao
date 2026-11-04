//! What prefs · config.rs is held to.
//!
//! Closes #519.
//! - one formatter for every length, in millimetres to 4 decimals (0.1 µm),
//!   trailing zeros trimmed — `a_length_is_shown_to_a_tenth_of_a_micron_whatever_its_size`
//! - in a unit other than the millimetre, the same 0.1 µm said in that unit —
//!   `a_tenth_of_a_micron_holds_in_whatever_unit_is_shown`
//! - a value shown with its unit, trailing zeros trimmed: « 12.5 », not
//!   « 12.500 » — `lengths_are_formatted_without_useless_decimals`
//! - a non-zero length is never shown as zero —
//!   `a_length_under_a_tenth_of_a_micron_is_never_shown_as_nothing`
//! - a driving dimension's label shows the real value: 50.0003 reads
//!   « 50.0003 mm », not « 50 mm » —
//!   `a_height_retyped_to_fifty_and_three_tenths_of_a_micron_reads_so`
//! - the canvas label reaches it through `UnitDisplay::format` — no test:
//!   `render/dimensions.rs` is a place with no net
//! - the `40 mm` pin changes on purpose, by the same logic —
//!   `a_measure_carries_the_unit_the_rest_of_the_interface_shows`
//!
//! Closes #428.
//! - a surface is said in the square of its unit —
//!   `a_surface_is_said_in_the_square_of_its_unit`, which fails if that
//!   squaring goes and which every other test here was blind to, running in
//!   the one unit where it is the identity
//! - and the unit is chosen from the side of a square of that surface —
//!   `the_unit_a_surface_is_said_in_comes_from_the_side_of_a_square_of_it`

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
        "40.0015 mm",
        "and a dimension reads to the same tenth of a micron rather than \
         rounding the micron and a half away",
    );
}

#[test]
fn a_surface_is_said_in_the_square_of_its_unit() {
    let metres = UnitDisplay::Fixed(LengthUnit::Meter);

    assert_eq!(
        metres.surface_in_figures(2_500_000.0, MOST_FIGURES),
        "2.5 m²",
        "two and a half square metres is two and a half million square \
         millimetres, and the unit is squared with the number",
    );
}

#[test]
fn the_unit_a_surface_is_said_in_comes_from_the_side_of_a_square_of_it() {
    let automatic = UnitDisplay::Auto;

    assert_eq!(
        automatic.surface_in_figures(100.0, MOST_FIGURES),
        "100 mm²",
        "a hundred square millimetres is ten millimetres square, so it is said \
         in millimetres — chosen from the surface itself it would be under the \
         tenth of a millimetre that sends the reading to micrometres",
    );
}

fn number_in(shown: &str) -> f64 {
    let number = shown.split_whitespace().next().unwrap_or_default();
    number
        .parse()
        .unwrap_or_else(|_| panic!("{shown:?} does not start on a number"))
}

#[test]
fn a_length_is_shown_to_a_tenth_of_a_micron_whatever_its_size() {
    let mm = UnitDisplay::Fixed(LengthUnit::Millimeter);

    for (length, reads) in [
        (37.461_829_3, "37.4618 mm"),
        (123.4567, "123.4567 mm"),
        (1_234.567_89, "1234.5679 mm"),
        (12.5, "12.5 mm"),
        (0.0004, "0.0004 mm"),
    ] {
        assert_eq!(mm.format(length), reads, "{length} mm");
    }
}

#[test]
fn a_height_retyped_to_fifty_and_three_tenths_of_a_micron_reads_so() {
    assert_eq!(
        UnitDisplay::Auto.format(50.0003),
        "50.0003 mm",
        "the solver holds the three tenths of a micron, and a label that \
         drops them makes the finer geometry invisible",
    );
}

#[test]
fn a_tenth_of_a_micron_holds_in_whatever_unit_is_shown() {
    let micrometres = UnitDisplay::Fixed(LengthUnit::Micrometer);
    let centimetres = UnitDisplay::Fixed(LengthUnit::Centimeter);
    let metres = UnitDisplay::Fixed(LengthUnit::Meter);

    for (unit, length, reads) in [
        (micrometres, 0.012_345, "12.3 µm"),
        (centimetres, 25.0001, "2.50001 cm"),
        (metres, 1_000.000_31, "1.0000003 m"),
        (UnitDisplay::Auto, 1_000.000_31, "1.0000003 m"),
        (UnitDisplay::Auto, 0.012_345, "12.3 µm"),
    ] {
        assert_eq!(unit.format(length), reads, "{length} mm in {unit:?}");
    }
}

#[test]
fn a_length_under_a_tenth_of_a_micron_is_never_shown_as_nothing() {
    for unit in [
        UnitDisplay::Auto,
        UnitDisplay::Fixed(LengthUnit::Micrometer),
        UnitDisplay::Fixed(LengthUnit::Millimeter),
        UnitDisplay::Fixed(LengthUnit::Meter),
    ] {
        for length in [0.000_03, -0.000_03, 1e-9] {
            let shown = unit.format(length);
            let number = number_in(&shown);
            assert!(
                number != 0.0 && number.signum() == length.signum(),
                "{length} mm is not nothing, and {unit:?} shows it as {shown:?}",
            );
        }
        assert_eq!(
            number_in(&unit.format(0.0)),
            0.0,
            "nothing is still shown as nothing in {unit:?}",
        );
    }
}
