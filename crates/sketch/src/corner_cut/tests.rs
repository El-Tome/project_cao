//! What sketch · corner_cut.rs is held to.

use glam::DVec2;

use super::*;
use crate::element::Element;
use crate::{CutSide, SegmentId, WorkPlane};

/// Two traits meeting at a right angle away from the origin: one running
/// right from the corner, one running up.
fn a_right_angle() -> (Sketch, PointId, [SegmentId; 2]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::new(10.0, 10.0));
    let right = sketch.add_point(DVec2::new(110.0, 10.0));
    let up = sketch.add_point(DVec2::new(10.0, 110.0));
    let sides = [
        sketch.add_segment(corner, right),
        sketch.add_segment(corner, up),
    ];
    (sketch, corner, sides)
}

const ROUND: CutShape = CutShape::Fillet { radius: 10.0 };

/// The trait a cut's side is, for a cut laid between two traits.
fn trait_of(side: CutSide) -> SegmentId {
    match side {
        CutSide::Trait(segment) => segment,
        curve => panic!("a side laid as a trait is a trait, not {curve:?}"),
    }
}

#[test]
fn a_cut_leaves_both_sides_whole_and_lays_nothing_else() {
    let (mut sketch, corner, [right, up]) = a_right_angle();
    let before = sketch.clone();

    let cut = sketch.cut_corner(right, up, ROUND, false, 0);

    assert_eq!(cut, Ok(CornerCutId(0)));
    assert_eq!(sketch.points(), before.points());
    assert_eq!(sketch.segments().len(), before.segments().len());
    assert_eq!(sketch.endpoints(right), before.endpoints(right));
    assert_eq!(sketch.endpoints(up), before.endpoints(up));
    assert!(sketch.arcs().is_empty());
    assert!(sketch.constraints().is_empty());
    assert!(sketch.dimensions().is_empty());
    assert_eq!(sketch.cuts_standing_at(corner), vec![CornerCutId(0)]);
}

#[test]
fn two_traits_that_do_not_meet_make_no_corner() {
    let (mut sketch, _, [right, _]) = a_right_angle();
    let away = sketch.add_point(DVec2::new(200.0, 200.0));
    let further = sketch.add_point(DVec2::new(300.0, 200.0));
    let apart = sketch.add_segment(away, further);

    assert_eq!(
        sketch.cut_corner(right, apart, ROUND, false, 0),
        Err(CutRefused::NoCorner)
    );
}

#[test]
fn a_value_too_big_is_still_laid() {
    let (mut sketch, _, [right, up]) = a_right_angle();

    let cut = sketch.cut_corner(right, up, CutShape::Fillet { radius: 500.0 }, false, 0);

    assert_eq!(cut, Ok(CornerCutId(0)));
}

#[test]
fn erasing_a_side_takes_its_cut_away() {
    let (mut sketch, corner, [right, up]) = a_right_angle();
    sketch
        .cut_corner(right, up, ROUND, false, 0)
        .expect("a right angle takes a fillet");

    sketch.erase(Element::Segment(up));

    assert_eq!(sketch.cuts_standing_at(corner), Vec::<CornerCutId>::new());
    assert_eq!(sketch.live_corner_cuts().count(), 0);
}

#[test]
fn erasing_the_corner_takes_its_cut_away() {
    let (mut sketch, corner, [right, up]) = a_right_angle();
    sketch
        .cut_corner(right, up, ROUND, false, 0)
        .expect("a right angle takes a fillet");

    sketch.erase(Element::Point(corner));

    assert_eq!(sketch.live_corner_cuts().count(), 0);
}

#[test]
fn cuts_laid_together_are_grouped_and_a_fresh_group_is_none_of_theirs() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [(0.0, 0.0), (100.0, 0.0), (100.0, 50.0), (0.0, 50.0)]
        .map(|(x, y)| sketch.add_point(DVec2::new(x, y)));
    let sides: Vec<SegmentId> = (0..4)
        .map(|rank| sketch.add_segment(corners[rank], corners[(rank + 1) % 4]))
        .collect();
    let group = sketch.fresh_cut_group();
    for rank in 0..4 {
        sketch
            .cut_corner(sides[rank], sides[(rank + 1) % 4], ROUND, false, group)
            .expect("a rectangle's corner takes a fillet");
    }

    assert_eq!(sketch.cuts_grouped_with(CornerCutId(2)).len(), 4);
    assert_ne!(sketch.fresh_cut_group(), group);
}

#[test]
fn a_rounded_corner_mirrored_is_rounded_the_same_way_on_its_copy() {
    let (mut sketch, _, [right, up]) = a_right_angle();
    sketch
        .cut_corner(right, up, ROUND, false, 0)
        .expect("a right angle takes a fillet");

    let copied = sketch.duplicate(&[Element::Segment(right), Element::Segment(up)], |place| {
        DVec2::new(-place.x, place.y)
    });

    assert_eq!(copied.cuts.len(), 1);
    let copy = sketch.corner_cuts()[copied.cuts[0].0];
    assert_eq!(copy.shape, ROUND);
    assert_eq!(
        copy.sides,
        [
            CutSide::Trait(copied.segments[0]),
            CutSide::Trait(copied.segments[1])
        ]
    );
    assert_ne!(copy.group, 0, "the copy changes on its own");
}

#[test]
fn a_side_divided_keeps_its_cut_on_the_piece_that_touches_the_corner() {
    let (mut sketch, corner, [right, up]) = a_right_angle();
    sketch
        .cut_corner(right, up, ROUND, false, 0)
        .expect("a right angle takes a fillet");

    sketch
        .split(&[right], &[], DVec2::new(60.0, 10.0))
        .expect("the side is divided");

    let (_, cut) = sketch
        .live_corner_cuts()
        .next()
        .expect("the cut still stands");
    let near = sketch.segments()[trait_of(cut.sides[0]).0];
    assert!(near.start == corner || near.end == corner, "{near:?}");
    assert_ne!(
        cut.sides[0],
        CutSide::Trait(right),
        "the side was replaced by its pieces"
    );
    assert!(sketch.drawn_cuts()[0].fits);
}

#[test]
fn the_piece_holding_the_corner_erased_after_a_division_takes_the_cut_away() {
    let (mut sketch, corner, [right, up]) = a_right_angle();
    sketch
        .cut_corner(right, up, ROUND, false, 0)
        .expect("a right angle takes a fillet");
    sketch
        .split(&[right], &[], DVec2::new(60.0, 10.0))
        .expect("the side is divided");
    let (_, cut) = sketch
        .live_corner_cuts()
        .next()
        .expect("the cut still stands");

    sketch.erase(Element::Segment(trait_of(cut.sides[0])));

    assert_eq!(sketch.cuts_standing_at(corner), Vec::<CornerCutId>::new());
}

#[test]
fn a_divided_side_keeps_its_cut_once_the_division_is_dragged_off_the_line() {
    let (mut sketch, corner, [right, up]) = a_right_angle();
    sketch
        .cut_corner(right, up, ROUND, false, 0)
        .expect("a right angle takes a fillet");
    let divided = sketch
        .split(&[right], &[], DVec2::new(60.0, 10.0))
        .expect("the side is divided");

    sketch.place_point(divided.point, DVec2::new(60.0, 0.0));
    sketch.place_point(corner, DVec2::new(11.0, 11.0));

    assert_eq!(sketch.cuts_standing_at(corner), vec![CornerCutId(0)]);
    assert!(sketch.drawn_cuts()[0].fits);
}

#[test]
fn a_cut_taken_away_stays_away_when_a_trait_is_drawn_again_from_its_corner() {
    let (mut sketch, corner, [right, up]) = a_right_angle();
    sketch
        .cut_corner(right, up, ROUND, false, 0)
        .expect("a right angle takes a fillet");
    sketch.erase(Element::Segment(up));

    let again = sketch.add_point(DVec2::new(10.0, 90.0));
    sketch.add_segment(corner, again);

    assert_eq!(sketch.live_corner_cuts().count(), 0);
    assert_eq!(sketch.cuts_standing_at(corner), Vec::<CornerCutId>::new());
}

#[test]
fn two_sides_sharing_both_ends_name_a_corner_only_with_its_point() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let left = sketch.add_point(DVec2::new(10.0, 10.0));
    let right = sketch.add_point(DVec2::new(30.0, 10.0));
    let across = sketch.add_segment(left, right);
    let centre = sketch.add_point(DVec2::new(20.0, 10.0));
    let over = sketch.add_arc(centre, right, left);
    let small = CutShape::Fillet { radius: 2.0 };

    assert_eq!(
        sketch.cut_corner(over, across, small, false, 0),
        Err(CutRefused::NoCorner),
        "a trait and the arc closing it over share two ends: their sides alone name no corner"
    );
    let at_right = sketch
        .cut_corner_at(right, over, across, small, false, 0)
        .expect("the corner at the right end");
    let at_left = sketch
        .cut_corner_at(left, over, across, small, false, 1)
        .expect("the corner at the left end");

    assert_eq!(
        [at_right, at_left].map(|cut| sketch.drawn_cut(cut).map(|drawn| drawn.corner_point)),
        [Some(right), Some(left)],
        "each cut stands at the end it was laid at"
    );
    assert_eq!(
        sketch.live_corner_cuts().count(),
        2,
        "two corners, two cuts"
    );
}

#[test]
fn a_cut_between_two_sides_sharing_both_ends_is_named_by_its_point_and_its_sides() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let left = sketch.add_point(DVec2::new(10.0, 10.0));
    let right = sketch.add_point(DVec2::new(30.0, 10.0));
    let across = sketch.add_segment(left, right);
    let centre = sketch.add_point(DVec2::new(20.0, 10.0));
    let over = sketch.add_arc(centre, right, left);
    let cut = sketch
        .cut_corner_at(
            left,
            across,
            over,
            CutShape::Fillet { radius: 2.0 },
            false,
            0,
        )
        .expect("the corner at the left end");
    let down = sketch.add_point(DVec2::new(0.0, 0.0));
    sketch.add_segment(left, down);

    let named = crate::Corner::BetweenAt(left, CutSide::Trait(across), CutSide::Arc(over));
    assert_eq!(
        sketch.corner_named(cut),
        Some(named),
        "a third side at that end leaves the point alone no saying which corner, and \
         the two sides alone name two"
    );
    assert_eq!(
        sketch.sides_of(named),
        Some((left, CutSide::Trait(across), CutSide::Arc(over))),
        "named so, the corner is found again however crowded its point"
    );
}

#[test]
fn one_side_given_twice_names_no_corner() {
    let (mut sketch, corner, [right, _]) = a_right_angle();

    assert_eq!(
        sketch.cut_corner_at(corner, right, right, ROUND, false, 0),
        Err(CutRefused::NoCorner)
    );
    assert_eq!(sketch.live_corner_cuts().count(), 0);
}
