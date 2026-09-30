use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use super::*;
use crate::brep::combine::laid;
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;
use crate::brep::topology::Body;
use crate::profile::{Contour, Frame, Run};

fn ground(height: f64) -> Frame {
    Frame {
        origin: DVec3::Z * height,
        u: DVec3::X,
        v: DVec3::Y,
    }
}

fn block(low: [f64; 3], high: [f64; 3]) -> Body {
    let outline = Contour::rectangle(DVec2::new(low[0], low[1]), DVec2::new(high[0], high[1]));
    Body::raised(&outline, &[], ground(low[2]), DVec3::Z * (high[2] - low[2]))
        .expect("a block raises")
}

fn standing(center: [f64; 2], radius: f64, from: f64, to: f64) -> Body {
    let center = DVec2::from(center);
    let outline = Contour {
        corners: vec![center + DVec2::X * radius],
        runs: vec![Run::Round { center, turn: TAU }],
    };
    Body::raised(&outline, &[], ground(from), DVec3::Z * (to - from)).expect("a cylinder raises")
}

/// The faces an operation keeps on the plane square to `axis` at `offset`,
/// each by whether it is flipped.
fn kept_on(
    first: &Body,
    second: &Body,
    operation: Operation,
    axis: DVec3,
    offset: f64,
) -> Vec<bool> {
    let operands = Operands::of(first, second, first.scale().joined(second.scale()));
    let arena = laid(&operands).expect("the arena is laid");
    let surface = arena
        .body
        .surfaces
        .iter()
        .position(|surface| {
            matches!(surface, Surface::Plane(plane)
                if plane.normal.abs_diff_eq(axis, 1e-12) && (plane.offset() - offset).abs() < 1e-12)
        })
        .expect("the plane is there");
    selected(&operands, &arena, operation)
        .expect("the faces are selected")
        .iter()
        .filter(|face| face.surface.0 as usize == surface)
        .map(|face| face.flipped)
        .collect()
}

#[test]
fn a_region_covered_by_both_operands_facing_one_way_is_kept_once_by_a_join_and_dropped_by_a_cut() {
    let stock = standing([0.0, 0.0], 20.0, 0.0, 10.0);
    let hole = standing([8.0, 0.0], 5.0, 0.0, 10.0);
    assert_eq!(
        kept_on(&stock, &hole, Operation::Or, DVec3::Z, 10.0),
        vec![false, false]
    );
    assert_eq!(
        kept_on(&stock, &hole, Operation::AndNot, DVec3::Z, 10.0),
        vec![false]
    );
}

#[test]
fn a_region_covered_from_each_side_is_dropped_by_a_join_and_kept_by_a_cut() {
    let one = block([-20.0, -20.0, 0.0], [20.0, 20.0, 10.0]);
    let other = block([20.0, -20.0, 0.0], [60.0, 20.0, 10.0]);
    assert_eq!(kept_on(&one, &other, Operation::Or, DVec3::X, 20.0), vec![]);
    assert_eq!(
        kept_on(&one, &other, Operation::AndNot, DVec3::X, 20.0),
        vec![false]
    );
    assert_eq!(
        kept_on(&other, &one, Operation::AndNot, DVec3::X, 20.0),
        vec![true]
    );
}

#[test]
fn a_region_covered_by_one_operand_is_kept_where_the_other_does_not_wrap_it() {
    let one = block([-20.0, -20.0, 0.0], [20.0, 20.0, 10.0]);
    let inside = block([-5.0, -5.0, 2.0], [5.0, 5.0, 8.0]);
    let outside = block([30.0, -5.0, 2.0], [40.0, 5.0, 8.0]);
    for operation in [Operation::Or, Operation::AndNot] {
        assert_eq!(
            kept_on(&one, &inside, operation, DVec3::Z, 10.0),
            vec![false]
        );
        assert_eq!(
            kept_on(&one, &outside, operation, DVec3::Z, 8.0),
            if operation == Operation::Or {
                vec![false]
            } else {
                vec![]
            }
        );
    }
    assert_eq!(kept_on(&one, &inside, Operation::Or, DVec3::Z, 8.0), vec![]);
    assert_eq!(
        kept_on(&one, &inside, Operation::AndNot, DVec3::Z, 8.0),
        vec![true]
    );
}

#[test]
fn a_sliver_of_a_face_thinner_than_twice_the_tolerance_is_told_covered_all_the_same() {
    let one = block([0.0, 0.0, 0.0], [10.0, 10.0, 10.0]);
    let eps = Scale::of(15.0).eps();
    let almost = block([-5.0, -5.0, 5.0], [10.0 - 1.5 * eps, 15.0, 15.0]);
    assert_eq!(almost.scale(), Scale::of(15.0));
    assert_eq!(
        kept_on(&one, &almost, Operation::AndNot, DVec3::Z, 10.0),
        vec![false]
    );
}
