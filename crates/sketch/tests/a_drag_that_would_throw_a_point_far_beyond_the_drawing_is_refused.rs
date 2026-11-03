//! A drag that would throw a point far beyond the drawing is refused, as a cut
//! corner turned inside out is: the landing may hold every rule, a point other
//! than the one dragged sent that far is still not what the hand asked for.
//!
//! The bound. A drag throws a point when a point other than the dragged one
//! ends farther than twice the hand's way plus the drawing's size from where
//! the press found it:
//!
//! ```text
//! 2 × (|landing − press place of the dragged point| + drawing size at the press)
//! ```
//!
//! the landing being the place a frame of the drag asks for, the one
//! `Sketch::settle_around` is given, and the drawing's size
//! `cao_sketch::precision::drawing_size` read on the drawing as the press
//! found it. Why that much: a shape stretched after a corner taken
//! far carries its other points as far as the hand went, never farther; a
//! shape turned about a point near the hand swings its far side by up to twice
//! the drawing's size. The factor two covers both at once.
//!
//! Closes #597.
//! - The flights of the campaign are refused, each drag `BestEffort` with the
//!   drawing as the press found it, or stopping where the shape still holds
//!   together — one shrunk seed of each kind,
//!   `a_point_laid_on_a_half_ellipse_s_end_is_dragged_without_throwing_its_other_end_away`,
//!   `an_arc_s_end_held_on_a_rectangle_s_side_is_dragged_without_throwing_the_far_corner_away`,
//!   `a_circle_s_rim_point_held_on_a_rectangle_s_side_is_dragged_without_throwing_the_far_corner_away`,
//!   `a_point_laid_on_a_corner_of_a_two_point_closed_chain_is_dragged_without_throwing_the_other_corner_away`;
//!   the other flights of the campaign are read back in the pull request
//! - No drag of the campaign that kept every point within the bound changes —
//!   no test: held by the campaign, read back in the pull request
//! - A seed of each kind found is shrunk and named in a test — a point laid on
//!   a half-ellipse's end, seed 239,
//!   `a_point_laid_on_a_half_ellipse_s_end_is_dragged_without_throwing_its_other_end_away`;
//!   a curve's point held on a rectangle's side, an arc's end in seed 354,
//!   `an_arc_s_end_held_on_a_rectangle_s_side_is_dragged_without_throwing_the_far_corner_away`,
//!   and a circle's rim point in seed 584,
//!   `a_circle_s_rim_point_held_on_a_rectangle_s_side_is_dragged_without_throwing_the_far_corner_away`;
//!   a point laid on a corner of a closed chain of two points, seed 468,
//!   `a_point_laid_on_a_corner_of_a_two_point_closed_chain_is_dragged_without_throwing_the_other_corner_away`
//!
//! What must not break. A point dragged far by the hand itself goes where the
//! hand puts it, however far, and a shape stretched after it carries its other
//! points no farther than the hand went —
//! `a_lone_trait_s_end_dragged_ten_sizes_away_goes_where_the_hand_puts_it`,
//! `a_square_cornered_rectangle_s_corner_dragged_ten_sizes_away_stretches_it_after_the_hand`.
//! A shape turned about a point near the hand swings its far side by nearly
//! twice the drawing's size for a small hand way, and is left alone —
//! `a_bar_turned_about_a_corner_near_the_hand_swings_its_far_side_whole`. The
//! tests of the drag of #422, #445, #473, #565, #591 and #592 are their own
//! files', run in the same gate.

// The drawings and their laying are the campaign's; this file only drags them.
#[allow(dead_code, unused_imports)]
mod random_sketches;

use cao_sketch::precision::{self, drawing_size};
use cao_sketch::{
    Constraint, DimensionTarget, LengthOutcome, PointId, SegmentId, Sketch, WorkPlane,
};
use glam::DVec2;
use random_sketches::Gesture;

/// The way the campaign found every flight along, as a share of the drawing's
/// size.
const CAMPAIGN_WAY: DVec2 = DVec2::new(-0.13, 0.21);

/// Ten drawing sizes away.
const TEN_SIZES: DVec2 = DVec2::new(6.0, 8.0);

/// Close enough for points the solver placed, as a share of the drawing's
/// size: it stops at a hundred-thousandth of it.
const SETTLED: f64 = 1e-5;

/// One frame of a drag, with the drawing as the press found it.
struct Drag {
    sketch: Sketch,
    point: PointId,
    hand: DVec2,
    press: Vec<(PointId, DVec2)>,
    size: f64,
    outcome: LengthOutcome,
}

impl Drag {
    /// `point` taken to `hand`, as one frame of a drag lays it.
    fn to(mut sketch: Sketch, point: PointId, hand: DVec2) -> Self {
        let press = sketch.live_points().collect();
        let size = drawing_size(&sketch);
        let outcome = sketch.settle_around(point, hand, 1.0);
        Drag {
            sketch,
            point,
            hand,
            press,
            size,
            outcome,
        }
    }

    /// `point` taken `way` times the drawing's size from where it stands.
    fn by(sketch: Sketch, point: PointId, way: DVec2) -> Self {
        let hand = sketch.point(point) + way * drawing_size(&sketch);
        Self::to(sketch, point, hand)
    }

    fn pressed(&self, point: PointId) -> DVec2 {
        self.press
            .iter()
            .find(|(each, _)| *each == point)
            .map(|(_, at)| *at)
            .expect("a point the press found")
    }

    fn gone(&self, point: PointId) -> f64 {
        self.sketch.point(point).distance(self.pressed(point))
    }

    fn hand_way(&self) -> f64 {
        self.hand.distance(self.pressed(self.point))
    }

    /// How far from where the press found it a point other than the dragged
    /// one may end.
    fn bound(&self) -> f64 {
        2.0 * (self.hand_way() + self.size)
    }

    /// The point other than the one dragged that ended farthest from where
    /// the press found it, and how far.
    fn farthest_other(&self) -> (PointId, f64) {
        self.press
            .iter()
            .filter(|(point, _)| *point != self.point)
            .map(|(point, _)| (*point, self.gone(*point)))
            .fold((self.point, 0.0), |far, each| match each.1 > far.1 {
                true => each,
                false => far,
            })
    }

    /// How near a point the solver placed stands to where it should, on the
    /// drawing as the drag left it.
    fn settled(&self) -> f64 {
        SETTLED * drawing_size(&self.sketch)
    }
}

/// The point laid last at a place: a point laid on another stands where it
/// does, and comes after it.
fn the_point_laid_last_at(sketch: &Sketch, place: DVec2) -> PointId {
    sketch
        .live_points()
        .filter(|(_, at)| at.distance(place) < 1e-9)
        .last()
        .map(|(point, _)| point)
        .unwrap_or_else(|| panic!("a point laid at {place}"))
}

/// The drag threw no point past the bound, and when it was refused, the
/// drawing stands as the press found it.
fn assert_nothing_thrown(drag: &Drag) {
    let (farthest, gone) = drag.farthest_other();
    assert!(
        gone <= drag.bound(),
        "{farthest:?} was thrown from {} to {}, {gone} away for a bound of {}, \
         the drag reported {:?}",
        drag.pressed(farthest),
        drag.sketch.point(farthest),
        drag.bound(),
        drag.outcome,
    );
    if drag.outcome == LengthOutcome::BestEffort {
        for (point, was) in &drag.press {
            let now = drag.sketch.point(*point);
            assert!(
                now.distance(*was) <= precision::SAME * drag.size,
                "the drag was refused, yet {point:?} left {was} for {now}",
            );
        }
    }
}

/// Seed 239 of `random_sketches::drawn`, shrunk. Before #597 the point laid
/// on the half-ellipse's right end slid along the axis and stretched the
/// ellipse until its left end stood a hundred and fifty million units away.
#[test]
fn a_point_laid_on_a_half_ellipse_s_end_is_dragged_without_throwing_its_other_end_away() {
    let sketch = random_sketches::laid(&[
        Gesture::Rectangle {
            corner: [180.0, 0.0],
            opposite: [0.0, -90.0],
            construction: false,
        },
        Gesture::HalfEllipse {
            from: [300.0, 150.0],
            to: [465.0, 150.0],
            rise: 75.0,
            construction: false,
        },
        Gesture::Point { at: [465.0, 150.0] },
    ]);
    let point = the_point_laid_last_at(&sketch, DVec2::new(465.0, 150.0));

    assert_nothing_thrown(&Drag::by(sketch, point, CAMPAIGN_WAY));
}

/// Seed 354 of `random_sketches::drawn`, shrunk. The arc's end at (3, 8) is
/// held on the rectangle's lower side; before #597 its drag turned that side
/// about the arc's centre and threw the side's far corner, at (5, 8), some
/// seventy units down the rectangle's right side, for a drawing twelve across.
#[test]
fn an_arc_s_end_held_on_a_rectangle_s_side_is_dragged_without_throwing_the_far_corner_away() {
    let sketch = random_sketches::laid(&[
        Gesture::Chain {
            through: vec![[11.5, 5.0], [4.5, 5.0]],
            closed: false,
            construction: false,
        },
        Gesture::Rectangle {
            corner: [1.5, 8.0],
            opposite: [5.0, 10.0],
            construction: false,
        },
        Gesture::Arc {
            centre: [1.5, 8.0],
            start: [1.5, 9.5],
            degrees: 270.0,
            construction: false,
        },
    ]);
    let point = the_point_laid_last_at(&sketch, DVec2::new(3.0, 8.0));

    assert_nothing_thrown(&Drag::by(sketch, point, CAMPAIGN_WAY));
}

/// Seed 584 of `random_sketches::drawn`, shrunk. The point the circle is
/// drawn with, at (120, −15), is held on the rectangle's lower side; before
/// #597 its drag turned that side about the circle's centre and threw the
/// side's far corner, at (210, −15), some four thousand units up the
/// rectangle's right side, for a drawing four hundred across.
#[test]
fn a_circle_s_rim_point_held_on_a_rectangle_s_side_is_dragged_without_throwing_the_far_corner_away()
{
    let sketch = random_sketches::laid(&[
        Gesture::Rectangle {
            corner: [60.0, 30.0],
            opposite: [210.0, -15.0],
            construction: false,
        },
        Gesture::Chain {
            through: vec![[300.0, 30.0], [390.0, 120.0]],
            closed: false,
            construction: false,
        },
        Gesture::Circle {
            centre: [60.0, -15.0],
            radius: 60.0,
            construction: false,
        },
    ]);
    let point = the_point_laid_last_at(&sketch, DVec2::new(120.0, -15.0));

    assert_nothing_thrown(&Drag::by(sketch, point, CAMPAIGN_WAY));
}

/// Seed 468 of `random_sketches::drawn`, shrunk. The chain closes on two
/// traits between the origin and (3, 3), and the point laid on the origin is
/// held on both; before #597 a hand way of about one threw the corner at
/// (3, 3) eighteen units away, for a drawing four across.
#[test]
fn a_point_laid_on_a_corner_of_a_two_point_closed_chain_is_dragged_without_throwing_the_other_corner_away()
 {
    let sketch = random_sketches::laid(&[
        Gesture::Chain {
            through: vec![[3.0, 3.0], [0.0, 0.0]],
            closed: true,
            construction: false,
        },
        Gesture::Point { at: [0.0, 0.0] },
    ]);
    let point = the_point_laid_last_at(&sketch, DVec2::ZERO);

    assert_nothing_thrown(&Drag::by(sketch, point, CAMPAIGN_WAY));
}

#[test]
fn a_lone_trait_s_end_dragged_ten_sizes_away_goes_where_the_hand_puts_it() {
    let sketch = random_sketches::laid(&[Gesture::Chain {
        through: vec![[20.0, 10.0], [60.0, 40.0]],
        closed: false,
        construction: false,
    }]);
    let end = the_point_laid_last_at(&sketch, DVec2::new(60.0, 40.0));

    let drag = Drag::by(sketch, end, TEN_SIZES);

    assert_eq!(drag.outcome, LengthOutcome::Exact);
    let landed = drag.sketch.point(end);
    assert!(
        landed.distance(drag.hand) <= drag.settled(),
        "the end was taken to {}, and stands at {landed}",
        drag.hand,
    );
    let (farthest, gone) = drag.farthest_other();
    assert!(
        gone <= drag.hand_way() + drag.settled(),
        "{farthest:?} was carried {gone}, farther than the hand went, {}",
        drag.hand_way(),
    );
}

/// A rectangle with three right angles, as the rectangle tests of the drag
/// lay it, nothing saying which way up.
fn square_cornered(sketch: &mut Sketch, corners: [PointId; 4]) -> [SegmentId; 4] {
    let sides: [SegmentId; 4] =
        std::array::from_fn(|rank| sketch.add_segment(corners[rank], corners[(rank + 1) % 4]));
    for corner in 0..3 {
        sketch.add_constraint(Constraint::Perpendicular {
            first: sides[corner],
            second: sides[corner + 1],
        });
    }
    sides
}

#[test]
fn a_square_cornered_rectangle_s_corner_dragged_ten_sizes_away_stretches_it_after_the_hand() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [
        DVec2::new(20.0, 20.0),
        DVec2::new(120.0, 20.0),
        DVec2::new(120.0, 70.0),
        DVec2::new(20.0, 70.0),
    ]
    .map(|place| sketch.add_point(place));
    square_cornered(&mut sketch, corners);
    let [a, b, c, d] = corners;

    let drag = Drag::by(sketch, c, TEN_SIZES);

    assert_eq!(drag.outcome, LengthOutcome::Exact);
    let hand = drag.hand;
    for (corner, wanted, what) in [
        (c, hand, "the corner dragged"),
        (a, DVec2::new(20.0, 20.0), "the opposite corner"),
        (b, DVec2::new(hand.x, 20.0), "the corner below the hand"),
        (d, DVec2::new(20.0, hand.y), "the corner beside the hand"),
    ] {
        let found = drag.sketch.point(corner);
        assert!(
            found.distance(wanted) <= drag.settled(),
            "{what}: wanted {wanted}, found {found}",
        );
    }
    let (farthest, gone) = drag.farthest_other();
    assert!(
        gone <= drag.hand_way() + drag.settled(),
        "{farthest:?} was carried {gone}, farther than the hand went, {}",
        drag.hand_way(),
    );
}

/// A bar a hundred long and five wide, its sizes typed and its corner on the
/// origin, which holds it: the corner beside the origin can only turn it.
/// Taken a hundred and fifty degrees round, a hand way of under ten swings the
/// far side by nearly twice the drawing's size.
#[test]
fn a_bar_turned_about_a_corner_near_the_hand_swings_its_far_side_whole() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let b = sketch.add_point(DVec2::new(100.0, 0.0));
    let c = sketch.add_point(DVec2::new(100.0, 5.0));
    let d = sketch.add_point(DVec2::new(0.0, 5.0));
    let sides = square_cornered(&mut sketch, [Sketch::ORIGIN, b, c, d]);
    for side in [sides[0], sides[1]] {
        let length = sketch.segment_length(side);
        sketch.set_dimension(DimensionTarget::Length(side), length, false);
    }
    let hand = DVec2::from_angle(240.0_f64.to_radians()) * 5.0;
    let turn = DVec2::from_angle(DVec2::new(0.0, 5.0).angle_to(hand));
    let size = drawing_size(&sketch);
    let swing = turn
        .rotate(DVec2::new(100.0, 5.0))
        .distance(DVec2::new(100.0, 5.0));
    assert!(
        swing >= 1.9 * size,
        "the far side swings {swing}, which is no wide swing for a drawing of {size}",
    );

    let drag = Drag::to(sketch, d, hand);

    assert_eq!(drag.outcome, LengthOutcome::Exact);
    for (point, was) in &drag.press {
        let (wanted, found) = (turn.rotate(*was), drag.sketch.point(*point));
        assert!(
            found.distance(wanted) <= drag.settled(),
            "{point:?} should have turned from {was} to {wanted}, and stands at {found}",
        );
    }
    assert_nothing_thrown(&drag);
}
