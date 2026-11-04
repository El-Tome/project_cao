//! What sketch · corner_cut/drawn.rs is held to.

use glam::DVec2;

use super::*;
use crate::{CutSide, SegmentId, WorkPlane};

const NEAR: f64 = 1e-9;
/// A share lets the tightest side give a hair more than its length: the
/// drawing's own tolerance, a hundred-millionth of its size.
const SHARE_NEAR: f64 = 1e-5;

fn near(one: DVec2, other: DVec2) -> bool {
    one.distance(other) < NEAR
}

/// A corner at (10, 10) with one side running to `first` and one to `second`.
fn a_corner(first: DVec2, second: DVec2) -> (Sketch, PointId, [SegmentId; 2]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::new(10.0, 10.0));
    let far_first = sketch.add_point(first);
    let far_second = sketch.add_point(second);
    let sides = [
        sketch.add_segment(corner, far_first),
        sketch.add_segment(corner, far_second),
    ];
    (sketch, corner, sides)
}

fn a_right_angle() -> (Sketch, PointId, [SegmentId; 2]) {
    a_corner(DVec2::new(110.0, 10.0), DVec2::new(10.0, 110.0))
}

/// A rectangle `width` by `height`, its sides in order from the bottom one.
fn a_rectangle(width: f64, height: f64) -> (Sketch, Vec<SegmentId>) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [(0.0, 0.0), (width, 0.0), (width, height), (0.0, height)]
        .map(|(x, y)| sketch.add_point(DVec2::new(x + 10.0, y + 10.0)));
    let sides = (0..4)
        .map(|rank| sketch.add_segment(corners[rank], corners[(rank + 1) % 4]))
        .collect();
    (sketch, sides)
}

fn round_every_corner(sketch: &mut Sketch, sides: &[SegmentId], radius: f64) {
    for rank in 0..4 {
        sketch
            .cut_corner(
                sides[rank],
                sides[(rank + 1) % 4],
                CutShape::Fillet { radius },
                false,
                0,
            )
            .expect("a rectangle's corner takes a cut");
    }
}

#[test]
fn a_fillet_touches_both_sides_at_its_radius_and_turns_about_its_centre() {
    let (mut sketch, _, [right, up]) = a_right_angle();
    let id = sketch
        .cut_corner(right, up, CutShape::Fillet { radius: 10.0 }, false, 0)
        .expect("a right angle takes a fillet");

    let drawn = sketch.drawn_cut(id).expect("a fillet that fits is drawn");

    assert!(drawn.fits);
    assert!(near(drawn.touches[0], DVec2::new(20.0, 10.0)), "{drawn:?}");
    assert!(near(drawn.touches[1], DVec2::new(10.0, 20.0)), "{drawn:?}");
    let CutCurve::Arc(arc) = drawn.curve else {
        panic!("a fillet draws an arc: {drawn:?}");
    };
    assert!(near(arc.centre, DVec2::new(20.0, 20.0)), "{arc:?}");
    assert!(
        crate::arcing::sweep_of(arc) < std::f64::consts::PI,
        "the short way round, towards the corner: {arc:?}"
    );
}

#[test]
fn a_fillet_follows_its_corner_when_the_corner_moves() {
    let (mut sketch, corner, [right, up]) = a_right_angle();
    let id = sketch
        .cut_corner(right, up, CutShape::Fillet { radius: 10.0 }, false, 0)
        .expect("a right angle takes a fillet");

    sketch.place_point(corner, DVec2::new(10.0, 0.0));

    let drawn = sketch.drawn_cut(id).expect("the fillet still fits");
    let CutCurve::Arc(arc) = drawn.curve else {
        panic!("a fillet draws an arc");
    };
    for touch in drawn.touches {
        assert!(
            (arc.centre.distance(touch) - 10.0).abs() < NEAR,
            "the radius holds: {drawn:?}"
        );
    }
    let side = sketch.endpoints(right);
    let along = (side.1 - side.0).normalize();
    let to_touch = drawn.touches[0] - side.0;
    assert!(
        along.perp_dot(to_touch).abs() < NEAR,
        "it still touches the side it follows: {drawn:?}"
    );
    assert!(
        along.dot(arc.centre - drawn.touches[0]).abs() < NEAR,
        "tangent: its radius stands square on the side: {drawn:?}"
    );
}

#[test]
fn an_equal_chamfer_takes_the_same_from_both_sides() {
    let (mut sketch, _, [right, up]) = a_right_angle();
    let id = sketch
        .cut_corner(right, up, CutShape::Chamfer(Chamfer::Equal(5.0)), false, 0)
        .expect("a right angle takes a chamfer");

    let drawn = sketch.drawn_cut(id).expect("a chamfer that fits is drawn");

    assert!(near(drawn.touches[0], DVec2::new(15.0, 10.0)), "{drawn:?}");
    assert!(near(drawn.touches[1], DVec2::new(10.0, 15.0)), "{drawn:?}");
    assert_eq!(
        drawn.curve,
        CutCurve::Straight {
            from: drawn.touches[0],
            to: drawn.touches[1]
        }
    );
}

#[test]
fn an_angled_chamfer_measures_its_angle_from_the_first_side() {
    let (mut sketch, _, [right, up]) = a_right_angle();
    let angled = Chamfer::Angled {
        along: 10.0,
        degrees: 30.0,
    };
    let id = sketch
        .cut_corner(right, up, CutShape::Chamfer(angled), false, 0)
        .expect("a right angle takes a chamfer");

    let drawn = sketch.drawn_cut(id).expect("the chamfer fits");

    assert!(near(drawn.touches[0], DVec2::new(20.0, 10.0)), "{drawn:?}");
    let leaves = (drawn.touches[1] - drawn.touches[0]).angle_to(DVec2::new(-1.0, 0.0));
    assert!(
        (leaves.abs().to_degrees() - 30.0).abs() < 1e-9,
        "{} degrees",
        leaves.to_degrees()
    );
}

#[test]
fn a_side_is_drawn_from_one_cut_to_the_other() {
    let (mut sketch, sides) = a_rectangle(100.0, 50.0);
    round_every_corner(&mut sketch, &sides, 10.0);

    let (from, to) = sketch
        .visible_part(sides[0])
        .expect("the bottom side keeps a straight piece");

    assert!(near(from, DVec2::new(20.0, 10.0)), "{from}");
    assert!(near(to, DVec2::new(100.0, 10.0)), "{to}");
}

#[test]
fn a_side_with_no_cut_is_drawn_whole() {
    let (sketch, sides) = a_rectangle(100.0, 50.0);

    assert_eq!(
        sketch.visible_part(sides[0]),
        Some(sketch.endpoints(sides[0]))
    );
}

#[test]
fn a_cut_too_big_for_its_side_does_not_fit_and_leaves_the_side_whole() {
    let (mut sketch, _, [right, up]) = a_right_angle();
    let id = sketch
        .cut_corner(right, up, CutShape::Fillet { radius: 150.0 }, false, 0)
        .expect("a value too big is still laid");

    let drawn = sketch.drawn_cut(id).expect("it still has a place");

    assert!(!drawn.fits);
    assert_eq!(sketch.visible_part(right), Some(sketch.endpoints(right)));
}

#[test]
fn two_cuts_overlapping_on_one_side_neither_fits() {
    let (mut sketch, sides) = a_rectangle(100.0, 30.0);
    round_every_corner(&mut sketch, &sides, 20.0);

    let drawn = sketch.drawn_cuts();

    assert_eq!(drawn.len(), 4);
    assert!(drawn.iter().all(|cut| !cut.fits), "{drawn:?}");
}

#[test]
fn two_cuts_meeting_exactly_fit_and_leave_no_straight_piece_between_them() {
    let (mut sketch, sides) = a_rectangle(100.0, 20.0);
    round_every_corner(&mut sketch, &sides, 10.0);

    assert!(sketch.drawn_cuts().iter().all(|cut| cut.fits));
    assert_eq!(
        sketch.visible_part(sides[1]),
        None,
        "the short side is gone"
    );
    assert!(sketch.visible_part(sides[0]).is_some());
}

#[test]
fn a_construction_cut_leaves_its_sides_whole() {
    let (mut sketch, _, [right, up]) = a_right_angle();
    sketch
        .cut_corner(right, up, CutShape::Fillet { radius: 10.0 }, true, 0)
        .expect("a right angle takes a fillet");

    assert_eq!(sketch.visible_part(right), Some(sketch.endpoints(right)));
    assert!(sketch.drawn_cuts()[0].construction);
}

#[test]
fn a_cut_stands_in_the_angle_under_half_a_turn() {
    let (mut sketch, corner, [first, second]) =
        a_corner(DVec2::new(110.0, 10.0), DVec2::new(-60.0, 80.0));
    let id = sketch
        .cut_corner(first, second, CutShape::Fillet { radius: 5.0 }, false, 0)
        .expect("an open corner takes a fillet");

    let CutCurve::Arc(arc) = sketch.drawn_cut(id).expect("it fits").curve else {
        panic!("a fillet draws an arc");
    };

    let at = sketch.point(corner);
    let bisector = ((DVec2::new(110.0, 10.0) - at).normalize()
        + (DVec2::new(-60.0, 80.0) - at).normalize())
    .normalize();
    assert!((arc.centre - at).dot(bisector) > 0.0, "{arc:?}");
}

#[test]
fn a_cut_changes_side_when_its_corner_is_taken_through_half_a_turn() {
    let (mut sketch, _, [first, second]) =
        a_corner(DVec2::new(110.0, 10.0), DVec2::new(-60.0, 40.0));
    let id = sketch
        .cut_corner(first, second, CutShape::Fillet { radius: 2.0 }, false, 0)
        .expect("an open corner takes a fillet");
    let above = sketch.drawn_cut(id).expect("it fits");

    let far = sketch.segments()[second.0].end;
    sketch.place_point(far, DVec2::new(-60.0, -20.0));
    let below = sketch.drawn_cut(id).expect("it still fits");

    let centre = |drawn: DrawnCut| match drawn.curve {
        CutCurve::Arc(arc) => arc.centre,
        CutCurve::Straight { .. } => panic!("a fillet draws an arc"),
    };
    assert!(centre(above).y > 10.0, "{above:?}");
    assert!(centre(below).y < 10.0, "{below:?}");
}

#[test]
fn a_straight_corner_gives_a_cut_no_place() {
    let (mut sketch, _, [first, second]) =
        a_corner(DVec2::new(110.0, 10.0), DVec2::new(-90.0, 10.0));
    sketch
        .cut_corner(first, second, CutShape::Fillet { radius: 5.0 }, false, 0)
        .expect("a straight corner is still a corner");

    assert!(sketch.drawn_cuts().is_empty());
    assert_eq!(sketch.visible_part(first), Some(sketch.endpoints(first)));
}

#[test]
fn values_that_fit_have_a_share_of_at_least_one() {
    let (sketch, _, [right, up]) = a_right_angle();

    let share = sketch.fitting_share(
        &[(right.into(), up.into())],
        CutShape::Fillet { radius: 10.0 },
    );

    assert!(share >= 1.0, "{share}");
}

#[test]
fn the_tightest_side_says_how_much_of_the_values_fits() {
    let (sketch, sides) = a_rectangle(100.0, 20.0);
    let corners: Vec<(CutSide, CutSide)> = (0..4)
        .map(|rank| (sides[rank].into(), sides[(rank + 1) % 4].into()))
        .collect();

    let share = sketch.fitting_share(&corners, CutShape::Fillet { radius: 15.0 });

    assert!(
        (share * 15.0 - 10.0).abs() < SHARE_NEAR,
        "two fillets share the short side: {share}"
    );
}

/// A trait from (10, 10) to (50, 10), and an arc round `centre` ending at
/// (10, 10) and at `far`: running counter-clockwise from the corner to `far`
/// when `from_the_corner`, from `far` to the corner otherwise.
fn a_trait_and_an_arc(centre: DVec2, far: DVec2, from_the_corner: bool) -> (Sketch, [CutSide; 2]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::new(10.0, 10.0));
    let end = sketch.add_point(DVec2::new(50.0, 10.0));
    let along = sketch.add_segment(corner, end);
    let centre = sketch.add_point(centre);
    let far = sketch.add_point(far);
    let arc = match from_the_corner {
        true => sketch.add_arc(centre, corner, far),
        false => sketch.add_arc(centre, far, corner),
    };
    (sketch, [along.into(), arc.into()])
}

/// Whether the fillet of `radius` laid on the corner is drawn, and fits.
fn laid_fits(sketch: &mut Sketch, [first, second]: [CutSide; 2], radius: f64) -> Option<bool> {
    let id = sketch
        .cut_corner(first, second, CutShape::Fillet { radius }, false, 0)
        .expect("the trait and the arc make a corner");
    sketch.drawn_cut(id).map(|cut| cut.fits)
}

/// Holds the share offered for `asked` on a corner to the largest radius
/// worked out by hand: the share gives it, the fillet asked is drawn and does
/// not fit, the one offered fits, and a hair more does not.
fn assert_the_largest_offered(
    sketch: &mut Sketch,
    sides: [CutSide; 2],
    asked: f64,
    largest: f64,
    what: &str,
) {
    let share = sketch.fitting_share(&[(sides[0], sides[1])], CutShape::Fillet { radius: asked });

    assert!(
        (share * asked - largest).abs() < SHARE_NEAR,
        "{what}: the share {share} offers {} where the largest that fits is {largest}",
        share * asked,
    );
    assert_eq!(
        laid_fits(sketch, sides, asked),
        Some(false),
        "{what}: the fillet asked is drawn, and does not fit"
    );
    assert_eq!(
        laid_fits(sketch, sides, share * asked),
        Some(true),
        "{what}: the fillet offered fits"
    );
    assert_eq!(
        laid_fits(sketch, sides, share * asked * (1.0 + 1e-4)),
        Some(false),
        "{what}: a hair more than the fillet offered does not fit"
    );
}

#[test]
fn the_tightest_side_beside_an_arc_says_how_much_of_the_value_fits() {
    let (mut short, sides) = a_trait_and_an_arc(
        DVec2::new(2.0, 10.0),
        DVec2::new(2.0 + 96.0 / 13.0, 10.0 + 40.0 / 13.0),
        true,
    );
    assert_the_largest_offered(
        &mut short,
        sides,
        8.0,
        5.0,
        "an arc bulging towards the corner, ending where a fillet of five touches it",
    );

    let (mut hollow, sides) =
        a_trait_and_an_arc(DVec2::new(14.0, 13.0), DVec2::new(9.0, 13.0), false);
    assert_the_largest_offered(
        &mut hollow,
        sides,
        6.0,
        3.0,
        "an arc of radius five hollow towards the corner, asked a fillet of six, \
         ending where a fillet of three touches it",
    );
}

/// Whether each of two corners rounded together by `radius`, as one gesture,
/// is drawn and fits.
fn laid_together_fit(
    sketch: &mut Sketch,
    corners: [[CutSide; 2]; 2],
    radius: f64,
) -> [Option<bool>; 2] {
    let ids = corners.map(|[first, second]| {
        sketch
            .cut_corner(first, second, CutShape::Fillet { radius }, false, 0)
            .expect("a corner of the drawing")
    });
    ids.map(|id| sketch.drawn_cut(id).map(|cut| cut.fits))
}

/// Holds two corners rounded together to the largest radius worked out by
/// hand, where their fillets just meet on the side between them: each fits on
/// its own at the radius asked, both are red once laid together, the share
/// gives the largest, both fit at it, and a hair more overlaps.
fn assert_held_back_together(
    sketch: &Sketch,
    corners: [[CutSide; 2]; 2],
    asked: f64,
    largest: f64,
    what: &str,
) {
    for (rank, [first, second]) in corners.into_iter().enumerate() {
        let mut alone = sketch.clone();
        assert_eq!(
            laid_fits(&mut alone, [first, second], asked),
            Some(true),
            "{what}: corner {rank} on its own takes the fillet asked"
        );
    }
    let share = sketch.fitting_share(
        &corners.map(|[first, second]| (first, second)),
        CutShape::Fillet { radius: asked },
    );
    assert!(
        (share * asked - largest).abs() < SHARE_NEAR,
        "{what}: the share {share} offers {} where the two fillets just meet at {largest}",
        share * asked,
    );

    let mut together = sketch.clone();
    assert_eq!(
        laid_together_fit(&mut together, corners, asked),
        [Some(false); 2],
        "{what}: laid together, the two fillets overlap and neither fits"
    );
    assert_eq!(
        laid_together_fit(&mut together, corners, share * asked),
        [Some(true); 2],
        "{what}: at the share offered, the two fillets meet and both fit"
    );
    assert_eq!(
        laid_together_fit(&mut together, corners, share * asked * (1.0 + 1e-4)),
        [Some(false); 2],
        "{what}: a hair more than the share offered, the two fillets overlap"
    );
}

#[test]
fn two_fillets_on_the_two_ends_of_one_trait_one_beside_an_arc_hold_each_other_back() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let beside_the_arc = sketch.add_point(DVec2::new(10.0, 10.0));
    let square = sketch.add_point(DVec2::new(30.0, 10.0));
    let shared = sketch.add_segment(beside_the_arc, square);
    let up = sketch.add_point(DVec2::new(30.0, 30.0));
    let rising = sketch.add_segment(square, up);
    let centre = sketch.add_point(DVec2::new(2.0, 10.0));
    let top = sketch.add_point(DVec2::new(2.0, 18.0));
    let arc = sketch.add_arc(centre, beside_the_arc, top);

    assert_held_back_together(
        &sketch,
        [[shared.into(), arc.into()], [shared.into(), rising.into()]],
        12.5,
        12.0,
        "a trait of twenty, rounded beside an arc of radius eight at one end and \
         square at the other: a fillet of twelve takes eight of it beside the arc and \
         twelve at the square end",
    );
}

#[test]
fn two_fillets_on_the_two_ends_of_one_arc_hold_each_other_back() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre_place = DVec2::new(2.0, 10.0);
    let [low, high] = [-1.0, 1.0].map(|way| DVec2::new(12.0, 5.0 * way) / 13.0);
    let centre = sketch.add_point(centre_place);
    let ends = [low, high].map(|out| sketch.add_point(centre_place + out * 8.0));
    let arc = sketch.add_arc(centre, ends[0], ends[1]);
    let traits = [(ends[0], low), (ends[1], high)].map(|(end, out)| {
        let far = sketch.add_point(centre_place + out * 28.0);
        sketch.add_segment(end, far)
    });

    assert_held_back_together(
        &sketch,
        [
            [arc.into(), traits[0].into()],
            [arc.into(), traits[1].into()],
        ],
        10.0,
        5.0,
        "an arc of radius eight between two traits running out from its centre: a \
         fillet of five on each end touches it at its middle",
    );
}

#[test]
fn a_cut_already_standing_takes_its_part_of_the_share() {
    let (mut sketch, sides) = a_rectangle(100.0, 20.0);
    sketch
        .cut_corner(
            sides[0],
            sides[1],
            CutShape::Fillet { radius: 15.0 },
            false,
            0,
        )
        .expect("a rectangle's corner takes a fillet");

    let share = sketch.fitting_share(
        &[(sides[1].into(), sides[2].into())],
        CutShape::Fillet { radius: 10.0 },
    );

    assert!((share * 10.0 - 5.0).abs() < SHARE_NEAR, "{share}");
}

#[test]
fn a_cut_in_construction_holds_no_drawn_cut_back() {
    let (mut sketch, sides) = a_rectangle(100.0, 20.0);
    sketch
        .cut_corner(
            sides[0],
            sides[1],
            CutShape::Fillet { radius: 15.0 },
            true,
            0,
        )
        .expect("a corner of the rectangle");
    let drawn = sketch
        .cut_corner(
            sides[1],
            sides[2],
            CutShape::Fillet { radius: 6.0 },
            false,
            1,
        )
        .expect("the next corner");

    assert!(
        sketch.drawn_cut(drawn).is_some_and(|cut| cut.fits),
        "the dashed arc takes nothing from the outline, so the side has room"
    );
    let share = sketch.fitting_share(
        &[(sides[2].into(), sides[3].into())],
        CutShape::Fillet { radius: 6.0 },
    );
    assert!(share >= 1.0, "{share}");
}

#[test]
fn a_corner_cut_again_does_not_count_what_its_old_cut_takes() {
    let (mut sketch, sides) = a_rectangle(100.0, 20.0);
    sketch
        .cut_corner(
            sides[0],
            sides[1],
            CutShape::Fillet { radius: 10.0 },
            false,
            0,
        )
        .expect("a rectangle's corner takes a fillet");

    let share = sketch.fitting_share(
        &[(sides[0].into(), sides[1].into())],
        CutShape::Fillet { radius: 20.0 },
    );

    assert!(
        (share * 20.0 - 20.0).abs() < SHARE_NEAR,
        "the whole short side is there for the new cut: {share}"
    );
}
