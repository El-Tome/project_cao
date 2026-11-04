//! What sketch · corner_cut/changing.rs is held to.

use glam::DVec2;

use crate::chamfer::Chamfer;
use crate::corner_cut::{CornerCutId, CutShape};
use crate::sketch::{SegmentId, Sketch};
use crate::{CutSide, WorkPlane};

/// A rectangle with its two bottom corners rounded together by a radius of
/// five, and its two top corners chamfered together.
fn a_plate() -> (Sketch, Vec<SegmentId>) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [(0.0, 0.0), (100.0, 0.0), (100.0, 50.0), (0.0, 50.0)]
        .map(|(x, y)| sketch.add_point(DVec2::new(x + 10.0, y + 10.0)));
    let sides: Vec<SegmentId> = (0..4)
        .map(|rank| sketch.add_segment(corners[rank], corners[(rank + 1) % 4]))
        .collect();
    for (first, second, group) in [(3, 0, 0), (0, 1, 0)] {
        sketch
            .cut_corner(
                sides[first],
                sides[second],
                CutShape::Fillet { radius: 5.0 },
                false,
                group,
            )
            .expect("a corner of the plate");
    }
    let sided = CutShape::Chamfer(Chamfer::Sided {
        first: 2.0,
        second: 6.0,
    });
    for (first, second) in [(1, 2), (2, 3)] {
        sketch
            .cut_corner(sides[first], sides[second], sided, false, 1)
            .expect("a corner of the plate");
    }
    (sketch, sides)
}

#[test]
fn a_new_shape_given_to_one_cut_changes_every_cut_laid_with_it() {
    let (mut sketch, _) = a_plate();

    assert!(sketch.reshape_cuts(CornerCutId(1), CutShape::Fillet { radius: 3.0 }));

    let shapes: Vec<CutShape> = sketch
        .live_corner_cuts()
        .map(|(_, cut)| cut.shape)
        .collect();
    assert_eq!(shapes[0], CutShape::Fillet { radius: 3.0 });
    assert_eq!(shapes[1], CutShape::Fillet { radius: 3.0 });
    assert!(
        matches!(shapes[2], CutShape::Chamfer(_)),
        "the other group is left alone"
    );
}

#[test]
fn a_corner_cut_again_takes_its_new_cut_and_leaves_its_group() {
    let (mut sketch, sides) = a_plate();

    let again = sketch
        .cut_corner(
            sides[0],
            sides[1],
            CutShape::Chamfer(Chamfer::Equal(4.0)),
            false,
            7,
        )
        .expect("a corner already cut is cut again");

    let standing: Vec<_> = sketch.live_corner_cuts().collect();
    assert_eq!(standing.len(), 4, "one cut per corner still");
    let (_, cut) = standing
        .iter()
        .find(|(id, _)| *id == again)
        .expect("the new cut stands");
    assert_eq!(cut.shape, CutShape::Chamfer(Chamfer::Equal(4.0)));
    assert_eq!(
        sketch.cuts_grouped_with(CornerCutId(0)),
        vec![CornerCutId(0)]
    );
}

#[test]
fn a_cut_taken_off_leaves_its_corner_sharp_and_the_others_standing() {
    let (mut sketch, sides) = a_plate();

    assert!(sketch.take_cut_off(CornerCutId(0)));

    assert_eq!(sketch.live_corner_cuts().count(), 3);
    assert_eq!(
        sketch.visible_part(sides[3]).map(|(_, to)| to),
        Some(DVec2::new(10.0, 10.0))
    );
    assert!(
        !sketch.take_cut_off(CornerCutId(0)),
        "nothing is left to take off"
    );
}

#[test]
fn a_chamfer_swapped_measures_from_its_other_side() {
    let (mut sketch, _) = a_plate();
    let before = sketch
        .drawn_cut(CornerCutId(2))
        .expect("the chamfer is drawn");

    assert!(sketch.swap_cut_sides(CornerCutId(2)));

    let after = sketch
        .drawn_cut(CornerCutId(2))
        .expect("the chamfer is drawn");
    let taken_from = |cut: crate::corner_cut::DrawnCut, side: CutSide| {
        let rank = cut
            .sides
            .iter()
            .position(|on| *on == side)
            .expect("a side of the cut");
        cut.touches[rank].distance(cut.corner)
    };
    let first = before.sides[0];
    assert!((taken_from(before, first) - 2.0).abs() < 1e-9);
    assert!(
        (taken_from(after, first) - 6.0).abs() < 1e-9,
        "the side that gave two gives six"
    );
    assert_eq!(
        sketch.drawn_cut(CornerCutId(3)).map(|cut| cut.sides),
        Some([CutSide::Trait(SegmentId(2)), CutSide::Trait(SegmentId(3))]),
        "the chamfer laid with it is left as it was"
    );
}

#[test]
fn a_fillet_has_no_sides_to_swap() {
    let (mut sketch, _) = a_plate();

    assert!(!sketch.swap_cut_sides(CornerCutId(0)));
}

#[test]
fn one_cut_is_switched_to_construction_alone() {
    let (mut sketch, _) = a_plate();

    assert!(sketch.set_cut_construction(CornerCutId(0), true));

    let construction: Vec<bool> = sketch
        .live_corner_cuts()
        .map(|(_, cut)| cut.construction)
        .collect();
    assert_eq!(construction, vec![true, false, false, false]);
}

#[test]
fn a_cut_taken_off_stays_off_when_its_side_is_divided() {
    let (mut sketch, sides) = a_plate();
    sketch.take_cut_off(CornerCutId(1));

    sketch
        .split(&[sides[0]], &[], DVec2::new(60.0, 10.0))
        .expect("the bottom side is divided");

    assert_eq!(sketch.live_corner_cuts().count(), 3);
    assert!(
        sketch
            .live_corner_cuts()
            .all(|(id, _)| id != CornerCutId(1))
    );
}

#[test]
fn a_cut_replaced_stays_replaced_when_its_side_is_divided() {
    let (mut sketch, sides) = a_plate();
    let again = sketch
        .cut_corner(
            sides[0],
            sides[1],
            CutShape::Chamfer(Chamfer::Equal(4.0)),
            false,
            7,
        )
        .expect("a corner already cut is cut again");

    sketch
        .split(&[sides[0]], &[], DVec2::new(60.0, 10.0))
        .expect("the bottom side is divided");

    let on_that_corner: Vec<CornerCutId> = sketch
        .live_corner_cuts()
        .filter(|(_, cut)| cut.corner == sketch.corner_cuts()[again.0].corner)
        .map(|(id, _)| id)
        .collect();
    assert_eq!(on_that_corner, vec![again], "one cut a corner, the new one");
}
