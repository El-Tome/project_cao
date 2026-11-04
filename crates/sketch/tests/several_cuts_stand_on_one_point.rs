//! Every pair of traits meeting at a point is a corner of its own, with its
//! own cut and its own value: the owner's drawing of three traits leaving one
//! point, the top one rounded with the middle one and with the bottom one.
//!
//! Closes #619.
//! - at a point where three traits meet, any two of them can be cut, each
//!   pair with its own value: the owner's drawing —
//!   `the_owner_s_drawing_rounds_the_top_trait_with_the_middle_and_with_the_bottom_each_by_its_own_value`,
//!   `the_third_pair_of_the_point_is_cut_as_well`,
//!   `a_pair_cut_again_takes_its_new_cut_and_leaves_the_other_pairs_standing`,
//!   `two_cuts_on_one_end_of_a_side_overlap_rather_than_add_up`; the tool
//!   laying the second pair beside the first is `cao_app`'s, held there by
//!   `the_second_pair_of_a_point_given_a_fillet_is_laid_beside_the_first_rather_than_retyping_it`
//! - clicking such a point asks for the two traits — no test: the tool is
//!   `cao_app`'s, and is held there by
//!   `a_click_on_a_point_three_traits_meet_at_asks_for_the_traits_once_a_pair_is_cut`,
//!   `two_pairs_of_one_point_are_taken_in_one_gesture` and
//!   `a_click_on_a_point_two_corners_are_taken_at_lets_go_of_neither`
//! - the shared trait shows from the tangent point nearest the corner, and
//!   each arc joins it —
//!   `the_shared_trait_shows_from_the_tangent_point_nearest_the_corner_and_each_arc_joins_it`
//! - two arcs of the same point crossing are allowed, and the areas around
//!   them hold every closed place once —
//!   `two_arcs_of_one_point_crossing_are_allowed_and_every_closed_place_is_one_area`

use cao_sketch::{
    Chamfer, Corner, CornerCutId, CutCurve, CutShape, CutSide, DrawnCut, SegmentId, Sketch,
    WorkPlane,
};
use glam::DVec2;

/// How near the arithmetic a place read from the drawing comes.
const EXACT: f64 = 1e-9;

/// Between the top trait and the two crossing arcs of the owner's drawing:
/// inside the near fillet's circle, outside the far one's.
const THE_PLACE_THE_ARCS_CLOSE: DVec2 = DVec2::new(0.2, 3.3);

/// The top, middle and bottom traits leaving the origin, and the two traits
/// closing the fan into two triangles.
struct Fan {
    sketch: Sketch,
    top: SegmentId,
    middle: SegmentId,
    bottom: SegmentId,
}

/// Three traits leaving the origin — straight up `top_length` long, at
/// forty-five degrees, and along x — closed by two traits into two
/// triangles.
fn a_fan(top_length: f64) -> Fan {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let origin = sketch.add_point(DVec2::ZERO);
    let up = sketch.add_point(DVec2::new(0.0, top_length));
    let across = sketch.add_point(DVec2::new(30.0, 30.0));
    let along = sketch.add_point(DVec2::new(40.0, 0.0));
    let top = sketch.add_segment(origin, up);
    let middle = sketch.add_segment(origin, across);
    let bottom = sketch.add_segment(origin, along);
    sketch.add_segment(up, across);
    sketch.add_segment(across, along);
    Fan {
        sketch,
        top,
        middle,
        bottom,
    }
}

fn fillet(radius: f64) -> CutShape {
    CutShape::Fillet { radius }
}

/// The owner's drawing: the top trait rounded with the middle one by
/// `with_the_middle`, and with the bottom one by `with_the_bottom`, each a
/// gesture of its own.
fn the_owner_s_drawing(
    top_length: f64,
    with_the_middle: f64,
    with_the_bottom: f64,
) -> (Fan, CornerCutId, CornerCutId) {
    let mut fan = a_fan(top_length);
    let near = fan
        .sketch
        .cut_corner(fan.top, fan.middle, fillet(with_the_middle), false, 0)
        .expect("the top and the middle traits make a corner");
    let far = fan
        .sketch
        .cut_corner(fan.top, fan.bottom, fillet(with_the_bottom), false, 1)
        .expect("the top and the bottom traits make a corner");
    (fan, near, far)
}

fn drawn(sketch: &Sketch, cut: CornerCutId) -> DrawnCut {
    sketch.drawn_cut(cut).expect("the cut is drawn")
}

fn arc_of(cut: DrawnCut) -> cao_sketch::ArcDraft {
    match cut.curve {
        CutCurve::Arc(arc) => arc,
        CutCurve::Straight { .. } => panic!("a fillet draws an arc"),
    }
}

/// Where the cut meets a side.
fn touch_on(cut: DrawnCut, side: SegmentId) -> DVec2 {
    let rank = cut
        .sides
        .iter()
        .position(|on| *on == CutSide::Trait(side))
        .expect("a side of the cut");
    cut.touches[rank]
}

#[test]
fn the_owner_s_drawing_rounds_the_top_trait_with_the_middle_and_with_the_bottom_each_by_its_own_value()
 {
    let (fan, near, far) = the_owner_s_drawing(40.0, 1.5, 5.0);

    let standing: Vec<(CornerCutId, CutShape)> = fan
        .sketch
        .live_corner_cuts()
        .map(|(id, cut)| (id, cut.shape))
        .collect();
    assert_eq!(standing, vec![(near, fillet(1.5)), (far, fillet(5.0))]);
    let (near, far) = (drawn(&fan.sketch, near), drawn(&fan.sketch, far));
    assert_eq!(
        near.corner_point, far.corner_point,
        "one point carries both"
    );
    assert!(near.fits && far.fits);
    assert!((arc_of(near).centre.distance(arc_of(near).start) - 1.5).abs() < EXACT);
    assert!((arc_of(far).centre.distance(arc_of(far).start) - 5.0).abs() < EXACT);
}

#[test]
fn the_third_pair_of_the_point_is_cut_as_well() {
    let (mut fan, _, _) = the_owner_s_drawing(40.0, 1.5, 5.0);

    let third = fan
        .sketch
        .cut_corner(
            fan.middle,
            fan.bottom,
            CutShape::Chamfer(Chamfer::Equal(2.0)),
            false,
            2,
        )
        .expect("the middle and the bottom traits make a corner");

    assert_eq!(fan.sketch.live_corner_cuts().count(), 3);
    assert_eq!(
        drawn(&fan.sketch, third).sides,
        [CutSide::Trait(fan.middle), CutSide::Trait(fan.bottom)]
    );
}

#[test]
fn a_pair_cut_again_takes_its_new_cut_and_leaves_the_other_pairs_standing() {
    let (mut fan, near, far) = the_owner_s_drawing(40.0, 1.5, 5.0);

    let again = fan
        .sketch
        .cut_corner(fan.middle, fan.top, fillet(1.0), false, 2)
        .expect("the same pair, named the other way");

    let standing: Vec<CornerCutId> = fan.sketch.live_corner_cuts().map(|(id, _)| id).collect();
    assert_eq!(standing, vec![far, again]);
    assert!(fan.sketch.is_erased_cut(near));
}

#[test]
fn two_cuts_on_one_end_of_a_side_overlap_rather_than_add_up() {
    // The top trait is six long: the two fillets take about three and a half
    // and five off it, more than six together, each less than six alone.
    let (fan, near, far) = the_owner_s_drawing(6.0, 1.5, 5.0);

    assert!(drawn(&fan.sketch, near).fits, "the near fillet fits");
    assert!(drawn(&fan.sketch, far).fits, "the far fillet fits");
    let share = fan.sketch.fitting_share_at(
        &[fan
            .sketch
            .sides_of(Corner::Between(fan.top.into(), fan.bottom.into()))
            .expect("the top and the bottom traits meet")],
        fillet(5.0),
    );
    assert!(share >= 1.0, "the far fillet is not held back: {share}");
}

#[test]
fn the_shared_trait_shows_from_the_tangent_point_nearest_the_corner_and_each_arc_joins_it() {
    let (fan, near, far) = the_owner_s_drawing(40.0, 1.5, 5.0);
    let (near, far) = (drawn(&fan.sketch, near), drawn(&fan.sketch, far));
    let (on_top_near, on_top_far) = (touch_on(near, fan.top), touch_on(far, fan.top));

    assert!(on_top_near.length() < on_top_far.length());
    let (from, to) = fan
        .sketch
        .visible_part(fan.top)
        .expect("the top trait still shows");
    assert!(
        from.distance(on_top_near) < EXACT,
        "{from} against {on_top_near}"
    );
    assert!(to.distance(DVec2::new(0.0, 40.0)) < EXACT);
    for joins in [on_top_near, on_top_far] {
        assert!(joins.x.abs() < EXACT, "{joins} is on the top trait");
        assert!(
            joins.y >= from.y - EXACT && joins.y <= to.y + EXACT,
            "{joins} is on what shows of it"
        );
    }
    for (cut, side) in [(near, fan.middle), (far, fan.bottom)] {
        let (from, _) = fan
            .sketch
            .visible_part(side)
            .expect("the other trait still shows");
        assert!(from.distance(touch_on(cut, side)) < EXACT);
    }
}

#[test]
fn two_arcs_of_one_point_crossing_are_allowed_and_every_closed_place_is_one_area() {
    // The near fillet meets the top trait nearer the corner than the far one,
    // and the middle trait farther than the far one passes it: the two arcs
    // cross.
    let (fan, near, far) = the_owner_s_drawing(40.0, 1.5, 5.0);
    let (near, far) = (
        arc_of(drawn(&fan.sketch, near)),
        arc_of(drawn(&fan.sketch, far)),
    );
    let far_radius = far.centre.distance(far.start);
    let near_radius = near.centre.distance(near.start);
    let apart = near.centre.distance(far.centre);
    assert!(
        apart < near_radius + far_radius && apart > (far_radius - near_radius).abs(),
        "the two arcs' circles cross"
    );

    let regions = fan.sketch.regions();

    let holding = |place: DVec2| regions.iter().filter(|area| area.contains(place)).count();
    // Off the grid of the traits, so that no place sampled lies on a border
    // two areas share.
    for x in (-10..=80).map(|step| f64::from(step) * 0.53 + 0.011) {
        for y in (-10..=80).map(|step| f64::from(step) * 0.53 + 0.037) {
            let place = DVec2::new(x, y);
            assert!(
                holding(place) <= 1,
                "{place} is in {} areas",
                holding(place)
            );
        }
    }
    // The two triangles, the second reaching round past the middle trait's
    // hidden stretch, and the place the crossing closes against the top trait.
    let closed = [
        DVec2::new(10.0, 23.0),
        DVec2::new(23.0, 10.0),
        THE_PLACE_THE_ARCS_CLOSE,
    ];
    assert_eq!(
        regions.len(),
        closed.len(),
        "one area for each closed place"
    );
    for inside in closed.into_iter().chain([DVec2::new(3.0, 1.5)]) {
        assert_eq!(holding(inside), 1, "{inside} is in one area");
    }
    for outside in [
        DVec2::new(-5.0, -5.0),
        DVec2::new(40.0, 40.0),
        DVec2::new(0.2, 0.2),
    ] {
        assert_eq!(holding(outside), 0, "{outside} is in no area");
    }
}
