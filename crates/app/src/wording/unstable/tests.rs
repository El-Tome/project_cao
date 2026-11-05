//! What app · wording/unstable.rs is held to.

use super::*;

fn said(lean: Lean, degrees: f64) -> String {
    notice(&Catalogue::french(), Unstable { lean, degrees })
}

#[test]
fn a_plane_nearly_square_to_an_axis_is_said_with_its_angle() {
    let said = said(Lean::Square, 0.01);

    assert!(said.contains("perpendiculaire"), "{said}");
    assert!(said.contains("0.01°"), "{said}");
}

#[test]
fn a_plane_nearly_along_an_axis_is_said_nearly_parallel_to_it() {
    let said = said(Lean::Along, 0.01);

    assert!(said.contains("parallèle"), "{said}");
    assert!(!said.contains("perpendiculaire"), "{said}");
    assert!(said.contains("0.01°"), "{said}");
}

#[test]
fn a_ten_millionth_of_a_degree_is_written_out_without_an_exponent() {
    let said = said(Lean::Square, 2e-7);

    assert!(said.contains("0.0000002°"), "{said}");
    assert!(!said.contains("e-7"), "{said}");
}

#[test]
fn an_angle_is_said_to_two_figures() {
    assert_eq!(angle(0.0573), "0.057");
    assert_eq!(angle(0.01), "0.01");
    assert_eq!(angle(3.2e-5), "0.000032");
}

#[test]
fn two_walls_whose_axes_are_nearly_square_are_said_nearly_perpendicular() {
    let said = said(Lean::Across, 0.001);

    assert!(said.contains("axes"), "{said}");
    assert!(said.contains("presque perpendiculaires"), "{said}");
    assert!(!said.contains("plan"), "{said}");
    assert!(said.contains("0.001°"), "{said}");
}

#[test]
fn two_walls_whose_axes_are_nearly_parallel_are_said_nearly_parallel() {
    let said = said(Lean::Abreast, 0.00003);

    assert!(said.contains("axes"), "{said}");
    assert!(said.contains("presque parallèles"), "{said}");
    assert!(!said.contains("perpendiculaire"), "{said}");
    assert!(!said.contains("plan"), "{said}");
    assert!(said.contains("0.00003°"), "{said}");
}
