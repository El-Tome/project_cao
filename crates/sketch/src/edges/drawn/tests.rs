//! What sketch · edges/drawn.rs is held to: the areas of a drawing whose
//! corners are cut.

use std::f64::consts::PI;

use glam::DVec2;

use crate::WorkPlane;
use crate::chamfer::Chamfer;
use crate::corner_cut::{CornerCutId, CutShape};
use crate::naming::CurveId;
use crate::sketch::{SegmentId, Sketch};

/// The area of a region read from its curves holds to the arithmetic to this
/// share of it.
const AREA_NEAR: f64 = 1e-9;

/// A rectangle `width` by `height` away from the origin, its sides in order
/// from the bottom one.
fn a_rectangle(width: f64, height: f64) -> (Sketch, Vec<SegmentId>) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [(0.0, 0.0), (width, 0.0), (width, height), (0.0, height)]
        .map(|(x, y)| sketch.add_point(DVec2::new(x + 10.0, y + 10.0)));
    let sides = (0..4)
        .map(|rank| sketch.add_segment(corners[rank], corners[(rank + 1) % 4]))
        .collect();
    (sketch, sides)
}

fn cut_every_corner(sketch: &mut Sketch, sides: &[SegmentId], shape: CutShape, construction: bool) {
    for rank in 0..4 {
        sketch
            .cut_corner(sides[rank], sides[(rank + 1) % 4], shape, construction, 0)
            .expect("a rectangle's corner takes a cut");
    }
}

fn the_one_area(sketch: &Sketch) -> f64 {
    let regions = sketch.regions();
    assert_eq!(regions.len(), 1, "one closed area");
    regions[0].area()
}

fn holds(measured: f64, expected: f64) {
    assert!(
        (measured - expected).abs() < AREA_NEAR * expected,
        "{measured} against {expected}"
    );
}

#[test]
fn a_plate_rounded_all_round_encloses_its_rounded_area() {
    let (mut sketch, sides) = a_rectangle(100.0, 50.0);
    cut_every_corner(
        &mut sketch,
        &sides,
        CutShape::Fillet { radius: 10.0 },
        false,
    );

    holds(the_one_area(&sketch), 100.0 * 50.0 - (4.0 - PI) * 100.0);
}

#[test]
fn the_area_of_a_rounded_plate_is_bounded_by_its_fillets() {
    let (mut sketch, sides) = a_rectangle(100.0, 50.0);
    cut_every_corner(
        &mut sketch,
        &sides,
        CutShape::Fillet { radius: 10.0 },
        false,
    );

    let regions = sketch.regions();
    let bounds = regions[0].bounds();
    for rank in 0..4 {
        assert!(
            bounds
                .iter()
                .any(|border| border.contains(&CurveId::Corner(CornerCutId(rank)))),
            "{bounds:?}"
        );
    }
}

#[test]
fn a_plate_chamfered_all_round_encloses_its_chamfered_area() {
    let (mut sketch, sides) = a_rectangle(100.0, 50.0);
    cut_every_corner(
        &mut sketch,
        &sides,
        CutShape::Chamfer(Chamfer::Equal(5.0)),
        false,
    );

    holds(the_one_area(&sketch), 100.0 * 50.0 - 4.0 * 12.5);
}

#[test]
fn an_obround_made_by_rounding_a_rectangle_encloses_its_area() {
    let (mut sketch, sides) = a_rectangle(100.0, 20.0);
    cut_every_corner(
        &mut sketch,
        &sides,
        CutShape::Fillet { radius: 10.0 },
        false,
    );

    holds(the_one_area(&sketch), 80.0 * 20.0 + PI * 100.0);
}

#[test]
fn a_cut_that_does_not_fit_leaves_the_area_its_sharp_corner() {
    let (mut sketch, sides) = a_rectangle(100.0, 30.0);
    cut_every_corner(
        &mut sketch,
        &sides,
        CutShape::Fillet { radius: 20.0 },
        false,
    );

    holds(the_one_area(&sketch), 100.0 * 30.0);
}

#[test]
fn a_cut_in_construction_leaves_the_area_its_sharp_corner() {
    let (mut sketch, sides) = a_rectangle(100.0, 50.0);
    cut_every_corner(&mut sketch, &sides, CutShape::Fillet { radius: 10.0 }, true);

    holds(the_one_area(&sketch), 100.0 * 50.0);
}
