use glam::DVec3;

use super::*;
use crate::brep::relation::{Relation, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, EllipticCylinder};

/// An ellipse of half-axes 7 and 3 about the origin along X, raised along Z.
fn oval() -> Surface {
    Surface::Elliptic(EllipticCylinder::about(
        DVec3::ZERO,
        DVec3::Z,
        DVec3::X * 7.0,
        3.0,
    ))
}

fn round(x: f64, y: f64, radius: f64) -> Surface {
    Surface::Cylinder(Cylinder::about(DVec3::new(x, y, 0.0), DVec3::Z, radius))
}

/// The point of the ellipse at eccentric angle `theta`, at height 2.
fn on_the_oval(theta: f64) -> DVec3 {
    DVec3::new(7.0 * theta.cos(), 3.0 * theta.sin(), 2.0)
}

/// Whether the second stands above the first at `place`, read off their
/// pair.
fn above(first: &Surface, second: &Surface, place: DVec3) -> Option<bool> {
    let lines: Vec<Line> = match relation(first, second, Scale::of(40.0)) {
        Relation::Lines(lines) => lines.to_vec(),
        Relation::Tangent(line) => vec![line],
        Relation::Parallel(rulings) => rulings.iter().map(|ruled| ruled.line).collect(),
        other => panic!("rulings expected, got {other:?}"),
    };
    standing_above(first, second, &lines, place, 1e-15)
}

#[test]
fn a_circle_inscribed_across_the_minor_axis_stands_inside_the_ellipse_by_both_touches() {
    let inscribed = round(0.0, 0.0, 3.0);
    for theta in [1.56, 1.58, -1.56, -1.58] {
        let place = on_the_oval(theta);
        assert_eq!(above(&oval(), &inscribed, place), Some(false), "{theta}");
        assert_eq!(above(&inscribed, &oval(), place), Some(true), "{theta}");
    }
}

#[test]
fn two_walls_touching_side_by_side_each_stand_beyond_the_other() {
    let beside = round(9.0, 0.0, 2.0);
    let place = on_the_oval(0.01);
    assert_eq!(above(&oval(), &beside, place), Some(true));
    assert_eq!(above(&beside, &oval(), place), Some(true));
}

#[test]
fn a_circle_touching_inside_at_a_vertex_and_crossing_twice_is_read_stretch_by_stretch() {
    let looser = round(4.0, 0.0, 3.0);
    let crossing = 0.4f64.acos();
    assert_eq!(
        above(&oval(), &looser, on_the_oval(0.02)),
        Some(true),
        "the circle bulges past the ellipse about the touch"
    );
    assert_eq!(
        above(&oval(), &looser, on_the_oval(crossing - 0.01)),
        Some(true)
    );
    assert_eq!(
        above(&oval(), &looser, on_the_oval(crossing + 0.01)),
        Some(false)
    );
}

#[test]
fn a_stretch_of_an_ellipse_inside_a_circle_it_crosses_has_the_circle_beyond_it() {
    let at_its_end = round(7.0, 0.0, 2.0);
    let crossing = 0.5793;
    assert_eq!(
        above(&oval(), &at_its_end, on_the_oval(crossing - 0.01)),
        Some(true)
    );
    assert_eq!(
        above(&oval(), &at_its_end, on_the_oval(crossing + 0.01)),
        Some(false)
    );
}

#[test]
fn a_stretch_is_read_between_the_two_rulings_about_it_round_the_turn() {
    assert_eq!(stretch(&[-1.0, 2.0], 0.5), Some([-1.0, 2.0]));
    assert_eq!(stretch(&[-1.0, 2.0], 3.0), Some([2.0, -1.0 + TAU]));
    assert_eq!(stretch(&[-1.0, 2.0], -3.0), Some([2.0, -1.0 + TAU]));
    assert_eq!(stretch(&[1.0], 0.0), Some([1.0, 1.0 + TAU]));
    assert_eq!(stretch(&[], 0.0), None);
}
