//! What sketch · corner_cut/hanging.rs is held to.

use glam::DVec2;

use super::*;
use crate::WorkPlane;
use crate::chamfer::Chamfer;
use crate::constraints::DimensionTarget;
use crate::element::Element;
use crate::sketch::SegmentId;

/// How near two places must stand to be the same after a solve.
const SAME: f64 = 1e-6;

/// Two traits meeting square at (10, 10): one running right for 100, one
/// running up for 100, rounded to 10.
fn a_rounded_corner() -> (Sketch, PointId, [SegmentId; 2], CornerCutId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::new(10.0, 10.0));
    let right = sketch.add_point(DVec2::new(110.0, 10.0));
    let up = sketch.add_point(DVec2::new(10.0, 110.0));
    let sides = [
        sketch.add_segment(corner, right),
        sketch.add_segment(corner, up),
    ];
    let cut = sketch
        .cut_corner(
            sides[0],
            sides[1],
            CutShape::Fillet { radius: 10.0 },
            false,
            0,
        )
        .unwrap();
    (sketch, corner, sides, cut)
}

/// Lays a point at a place of the cut and hangs it there, as a tool does.
fn hang(sketch: &mut Sketch, cut: CornerCutId, at: CutPoint) -> PointId {
    let place = sketch.cut_point(cut, at).unwrap();
    let on = sketch.supports_at(place);
    assert_eq!(
        on,
        vec![Support::CutPoint(cut, at)],
        "what a point laid at {at:?} lands on"
    );
    let point = sketch.add_point(place);
    for support in on {
        sketch.add_constraint(support.holding(point));
    }
    point
}

fn assert_at(sketch: &Sketch, point: PointId, expected: DVec2, what: &str) {
    let stands = sketch.point(point);
    assert!(
        stands.distance(expected) < SAME,
        "{what}: the point stands at {stands}, where {expected} was expected",
    );
}

#[test]
fn a_cut_nothing_hangs_on_asks_the_solver_for_nothing() {
    let (sketch, ..) = a_rounded_corner();

    assert!(sketch.equations(1.0).is_empty());
}

#[test]
fn a_point_hung_on_each_place_follows_a_new_radius() {
    let (mut sketch, _, _, cut) = a_rounded_corner();
    let hung = [CutPoint::Centre, CutPoint::First, CutPoint::Second]
        .map(|at| (at, hang(&mut sketch, cut, at)));

    sketch.reshape_cuts(cut, CutShape::Fillet { radius: 25.0 });
    sketch.solve(1.0);

    let expected = [
        DVec2::new(35.0, 35.0),
        DVec2::new(35.0, 10.0),
        DVec2::new(10.0, 35.0),
    ];
    for ((at, point), expected) in hung.into_iter().zip(expected) {
        assert_at(&sketch, point, expected, &format!("{at:?}"));
    }
    assert_eq!(
        sketch.point(PointId(1)),
        DVec2::new(10.0, 10.0),
        "the corner stays"
    );
}

#[test]
fn a_point_hung_on_a_cut_follows_its_corner_moved() {
    let (mut sketch, corner, _, cut) = a_rounded_corner();
    let centre = hang(&mut sketch, cut, CutPoint::Centre);

    sketch.settle_around(corner, DVec2::new(0.0, 0.0), 1.0);

    assert_at(
        &sketch,
        centre,
        sketch.cut_point(cut, CutPoint::Centre).unwrap(),
        "centre",
    );
    assert!(
        sketch.point(centre).distance(DVec2::new(20.0, 20.0)) > 1.0,
        "the centre moved"
    );
}

#[test]
fn a_length_typed_on_a_trait_hung_on_a_cut_moves_the_corner() {
    let (mut sketch, corner, _, cut) = a_rounded_corner();
    let touch = hang(&mut sketch, cut, CutPoint::First);
    let below = sketch.add_point(DVec2::new(20.0, -30.0));
    let leg = sketch.add_segment(touch, below);
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Point(below),
    });

    sketch.set_dimension(DimensionTarget::Length(leg), 60.0, false);
    sketch.solve(1.0);

    assert!((sketch.segment_length(leg) - 60.0).abs() < SAME);
    assert_at(
        &sketch,
        touch,
        sketch.cut_point(cut, CutPoint::First).unwrap(),
        "the touch",
    );
    assert!(
        sketch.point(corner).distance(DVec2::new(10.0, 10.0)) > 1.0,
        "the corner gave, since the end below could not",
    );
}

#[test]
fn a_point_laid_on_a_fillet_s_curve_is_held_on_it_and_follows_the_radius() {
    let (mut sketch, _, _, cut) = a_rounded_corner();
    let on_the_curve = DVec2::new(20.0, 20.0) - DVec2::splat(10.0 / 2f64.sqrt());
    assert_eq!(sketch.supports_at(on_the_curve), vec![Support::Cut(cut)]);
    let point = sketch.add_point(on_the_curve);
    sketch.add_constraint(Support::Cut(cut).holding(point));

    sketch.reshape_cuts(cut, CutShape::Fillet { radius: 30.0 });
    sketch.solve(1.0);

    let centre = sketch.cut_point(cut, CutPoint::Centre).unwrap();
    assert!((sketch.point(point).distance(centre) - 30.0).abs() < SAME);
}

#[test]
fn a_cut_that_does_not_fit_keeps_what_hangs_on_it_where_it_is_drawn() {
    let (mut sketch, _, _, cut) = a_rounded_corner();
    let touch = hang(&mut sketch, cut, CutPoint::First);

    sketch.reshape_cuts(cut, CutShape::Fillet { radius: 150.0 });
    sketch.solve(1.0);

    assert!(!sketch.drawn_cut(cut).unwrap().fits);
    assert_at(
        &sketch,
        touch,
        DVec2::new(160.0, 10.0),
        "past the far end, where the red cut is drawn",
    );
}

#[test]
fn taking_the_cut_off_leaves_an_end_on_its_side_and_the_centre_where_it_was() {
    let (mut sketch, _, [right, _], cut) = a_rounded_corner();
    let centre = hang(&mut sketch, cut, CutPoint::Centre);
    let touch = hang(&mut sketch, cut, CutPoint::First);

    assert!(sketch.take_cut_off(cut));

    assert_eq!(sketch.holds_on(centre), Vec::new());
    assert_eq!(sketch.holds_on(touch), vec![Support::Segment(right)]);
    assert_eq!(sketch.point(centre), DVec2::new(20.0, 20.0));
    assert_eq!(sketch.point(touch), DVec2::new(20.0, 10.0));
}

#[test]
fn a_fillet_become_a_chamfer_lets_its_centre_go_and_keeps_its_ends_and_its_curve() {
    let (mut sketch, _, _, cut) = a_rounded_corner();
    let centre = hang(&mut sketch, cut, CutPoint::Centre);
    let touch = hang(&mut sketch, cut, CutPoint::Second);
    let on_the_curve = sketch.add_point(DVec2::new(20.0, 20.0) - DVec2::splat(10.0 / 2f64.sqrt()));
    sketch.add_constraint(Support::Cut(cut).holding(on_the_curve));

    sketch.reshape_cuts(cut, CutShape::Chamfer(Chamfer::Equal(15.0)));
    sketch.solve(1.0);

    assert_eq!(sketch.holds_on(centre), Vec::new());
    assert_eq!(
        sketch.point(centre),
        DVec2::new(20.0, 20.0),
        "let go where it was"
    );
    assert_at(
        &sketch,
        touch,
        DVec2::new(10.0, 25.0),
        "the end on the second side",
    );
    let off_the_cut = (sketch.point(on_the_curve) - DVec2::new(25.0, 10.0))
        .perp_dot(DVec2::new(-1.0, 1.0).normalize());
    assert!(
        off_the_cut.abs() < SAME,
        "onto the chamfer's cut, {off_the_cut} off it"
    );
}

#[test]
fn swapping_a_chamfer_s_sides_keeps_each_end_on_its_side() {
    let (mut sketch, _, _, cut) = a_rounded_corner();
    sketch.reshape_cuts(
        cut,
        CutShape::Chamfer(Chamfer::Sided {
            first: 5.0,
            second: 20.0,
        }),
    );
    let on_the_first = hang(&mut sketch, cut, CutPoint::First);

    assert!(sketch.swap_cut_sides(cut));
    sketch.solve(1.0);

    assert_eq!(
        sketch.holds_on(on_the_first),
        vec![Support::CutPoint(cut, CutPoint::Second)]
    );
    assert_at(
        &sketch,
        on_the_first,
        DVec2::new(30.0, 10.0),
        "on the same side, now 20 along",
    );
}

#[test]
fn a_corner_cut_again_hands_what_hung_on_it_to_the_new_cut() {
    let (mut sketch, _, [right, up], cut) = a_rounded_corner();
    let touch = hang(&mut sketch, cut, CutPoint::First);
    let centre = hang(&mut sketch, cut, CutPoint::Centre);

    let again = sketch
        .cut_corner(up, right, CutShape::Chamfer(Chamfer::Equal(30.0)), false, 1)
        .unwrap();
    sketch.solve(1.0);

    assert_eq!(
        sketch.holds_on(touch),
        vec![Support::CutPoint(again, CutPoint::Second)]
    );
    assert_at(
        &sketch,
        touch,
        DVec2::new(40.0, 10.0),
        "the end on the right side",
    );
    assert_eq!(
        sketch.holds_on(centre),
        Vec::new(),
        "a chamfer has no centre"
    );
}

#[test]
fn a_side_divided_keeps_what_hung_on_its_cut() {
    let (mut sketch, _, [right, _], cut) = a_rounded_corner();
    let touch = hang(&mut sketch, cut, CutPoint::First);

    sketch.split(&[right], &[], DVec2::new(70.0, 10.0)).unwrap();
    let standing = sketch
        .live_corner_cuts()
        .map(|(id, _)| id)
        .collect::<Vec<_>>();
    assert_eq!(
        standing,
        vec![cut],
        "the cut is handed to the piece at the corner"
    );

    sketch.reshape_cuts(cut, CutShape::Fillet { radius: 20.0 });
    sketch.solve(1.0);

    assert_eq!(
        sketch.holds_on(touch),
        vec![Support::CutPoint(cut, CutPoint::First)]
    );
    assert_at(
        &sketch,
        touch,
        DVec2::new(30.0, 10.0),
        "the touch, followed",
    );
}

#[test]
fn erasing_a_side_lets_go_of_what_hung_on_its_cut() {
    let (mut sketch, _, [right, _], cut) = a_rounded_corner();
    let touch = hang(&mut sketch, cut, CutPoint::First);

    sketch.erase(Element::Segment(right));

    assert!(!sketch.is_erased_point(touch));
    assert_eq!(sketch.holds_on(touch), Vec::new());
}

#[test]
fn a_corner_merged_into_another_point_keeps_its_cut_and_what_hangs_on_it() {
    let (mut sketch, corner, _, cut) = a_rounded_corner();
    let touch = hang(&mut sketch, cut, CutPoint::First);
    let beside = sketch.add_point(DVec2::new(10.0, 10.0));

    sketch.merge_points(beside, corner);

    assert_eq!(sketch.corner_cuts()[cut.0].corner, beside);
    assert_eq!(sketch.cuts_standing_at(beside), vec![cut]);
    assert_eq!(
        sketch.holds_on(touch),
        vec![Support::CutPoint(cut, CutPoint::First)]
    );
}

#[test]
fn a_point_the_cut_stands_on_cannot_hang_on_it() {
    let (mut sketch, corner, [right, _], cut) = a_rounded_corner();
    let far = sketch.segments()[right.0].end;

    for point in [corner, far] {
        sketch.add_constraint(Constraint::AtCutPoint {
            point,
            cut,
            at: CutPoint::First,
        });
        sketch.add_constraint(Constraint::OnCut { point, cut });
    }

    assert!(sketch.constraints().is_empty());
}

#[test]
fn a_point_on_a_chamfer_goes_onto_the_fillet_it_becomes() {
    let (mut sketch, _, _, cut) = a_rounded_corner();
    sketch.reshape_cuts(cut, CutShape::Chamfer(Chamfer::Equal(10.0)));
    let on_the_cut = sketch.add_point(DVec2::new(15.0, 15.0));
    sketch.add_constraint(Support::Cut(cut).holding(on_the_cut));

    sketch.reshape_cuts(cut, CutShape::Fillet { radius: 10.0 });
    sketch.solve(1.0);

    let reach = sketch.point(on_the_cut).distance(DVec2::new(20.0, 20.0));
    assert!(
        (reach - 10.0).abs() < SAME,
        "{reach} from the fillet's centre"
    );
}
