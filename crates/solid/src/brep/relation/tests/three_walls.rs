//! A shaft and a bore of one radius exactly parallel, the bore a unit off
//! the shaft's axis and a hundredth of a micron aside, and a third cylinder
//! square to both, touching the shaft exactly at a node and the bore only
//! within the tolerance. Where the crescent's tip line — the line the shaft
//! and the bore cross along — passes through the third wall, the corners
//! stand exactly on all three; the curve the bore and the third wall meet
//! along must pass through them, or the boolean never cuts it there.
//!
//! Closes #776.
//! - A corner exact on the shaft, the bore and the third wall, where the
//!   crescent's tip line crosses it, lies within the tolerance of the bore ×
//!   third curve `relation` gives —
//!   `a_corner_where_the_crescent_s_tip_line_crosses_a_third_wall_lies_on_the_curve_the_bore_meets_it_along`

use super::*;
use crate::brep::curve::Curve;

const ASIDE: f64 = 1e-8;

/// The largest coordinate the minimal case's operands reach, which is what
/// the boolean measures them by.
fn reached() -> Scale {
    Scale::of(30.0)
}

fn shaft() -> Surface {
    cylinder(DVec3::ZERO, DVec3::Z, 10.0)
}

fn bore() -> Surface {
    cylinder(DVec3::new(ASIDE, 1.0, 0.0), DVec3::Z, 10.0)
}

fn third() -> Surface {
    cylinder(DVec3::new(5.0, 30.0, 10.0), -DVec3::Y, 5.0)
}

/// The two corners where the shaft and the bore cross on the side the third
/// wall reaches, `x² + y² = 100` and `(x − ASIDE)² + (y − 1)² = 100`, above
/// and below the third wall's axis.
fn corners() -> [DVec3; 2] {
    let c = (1.0 + ASIDE * ASIDE) / 2.0;
    let a = 1.0 + ASIDE * ASIDE;
    let root = (ASIDE * ASIDE * c * c - a * (c * c - 100.0)).sqrt();
    let x = (ASIDE * c + root) / a;
    let y = c - ASIDE * x;
    let height = ((5.0 - (x - 5.0)) * (5.0 + (x - 5.0))).sqrt();
    [10.0 - height, 10.0 + height].map(|z| DVec3::new(x, y, z))
}

#[test]
fn a_corner_where_the_crescent_s_tip_line_crosses_a_third_wall_lies_on_the_curve_the_bore_meets_it_along()
 {
    let eps = reached().eps();
    let found = relation(&bore(), &third(), reached());
    assert!(
        matches!(found, Relation::Meet(_)),
        "a meeting expected, got {found:?}"
    );
    for corner in corners() {
        for surface in [shaft(), bore(), third()] {
            assert!(
                surface.distance(corner).abs() <= 1e-12 * REACH,
                "{corner} is {} off {surface:?}",
                surface.distance(corner)
            );
        }
        let nearest = found
            .curves()
            .iter()
            .map(|curve| match curve {
                Curve::Meet(meet) => meet.point(meet.parameter(corner)).distance(corner),
                other => panic!("a meet curve expected, got {other:?}"),
            })
            .fold(f64::INFINITY, f64::min);
        assert!(
            nearest <= eps,
            "the corner {corner} stands {nearest:e} off every component of {found:?}, \
             beyond the tolerance {eps:e}"
        );
    }
}
