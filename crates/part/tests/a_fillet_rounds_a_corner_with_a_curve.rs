//! A fillet rounds a corner one of whose sides is a curve: a trait and an arc,
//! two arcs, an arc of ellipse beside any of those. As every cut since #617, it
//! is a property of the corner, tangent to both sides and worked out from them
//! each time the drawing is read, and a gesture on the corner is a gesture on
//! a sharp corner.
//!
//! Closes #621.
//! - A fillet rounds the corner between a trait and an arc, and between two
//!   arcs, tangent to both —
//!   `a_fillet_rounds_the_corner_between_a_trait_and_an_arc_tangent_to_both`,
//!   `a_fillet_rounds_the_corner_between_two_arcs_tangent_to_both`; the tool
//!   naming such a corner by its point or by its sides is held in `cao_app` by
//!   `a_click_on_the_point_a_trait_and_an_arc_end_at_names_both_its_sides` and
//!   `a_click_on_an_arc_away_from_its_ends_names_that_side`
//! - A fillet rounds the corner between an arc of ellipse and a trait, an arc,
//!   or another arc of ellipse, tangent to both —
//!   `a_fillet_rounds_the_corner_between_an_arc_of_ellipse_and_a_trait_tangent_to_both`,
//!   `a_fillet_rounds_the_corner_between_an_arc_of_ellipse_and_an_arc_tangent_to_both`,
//!   `a_fillet_rounds_the_corner_between_two_arcs_of_ellipse_tangent_to_both`
//! - The fillet stands in the angle under 180° between the sides' tangents at
//!   the corner, and changes side only when a drag takes that angle through
//!   180° —
//!   `a_fillet_beside_an_arc_stands_in_the_angle_under_half_a_turn_between_the_tangents`,
//!   `a_fillet_beside_an_arc_changes_side_only_when_a_drag_takes_the_angle_through_half_a_turn`
//! - A chamfer on a corner with an arc or an arc of ellipse is refused, and a
//!   message says why. A whole circle and a whole ellipse stay refused —
//!   `a_chamfer_on_a_corner_with_an_arc_or_an_arc_of_ellipse_is_refused`,
//!   `a_whole_circle_or_a_whole_ellipse_has_no_corner_to_round`; the message
//!   is the tool's, held in `cao_app` by
//!   `a_chamfer_on_a_corner_with_a_curve_says_why_it_is_refused`
//! - Dragging the corner, or grabbing the fillet, moves the corner as on a
//!   sharp corner: an arc keeps its centre and radius and the corner slides
//!   round it; between two arcs the corner follows the hand. The fillet
//!   follows, tangent and of the same radius —
//!   `dragging_a_corner_between_a_trait_and_an_arc_slides_it_round_the_arc_as_on_a_sharp_corner`,
//!   `dragging_a_corner_between_two_arcs_moves_it_as_a_sharp_corner`; the
//!   fillet grabbed is held in the sketch crate by
//!   `a_press_on_a_fillet_beside_an_arc_drags_its_corner`
//! - The visible outline runs along each side up to its tangent point, a curve
//!   trimmed there, then the fillet; the tint, the areas and the extrusion
//!   follow it —
//!   `a_plate_rounded_beside_an_arc_closes_its_area_along_the_trimmed_arc_and_the_fillet`,
//!   `a_plate_rounded_beside_an_arc_is_raised_with_its_rounded_corner`; the
//!   tint is painted from those same areas, and nothing under the canvas
//!   asserts what is painted
//! - A value too big is held back with the largest that fits, and a fillet
//!   that no longer fits during a drag turns red with the sharp corner
//!   standing in, as in #617 —
//!   `a_fillet_beside_an_arc_that_no_longer_fits_raises_the_sharp_corner`;
//!   holding the value back is the tool's, held in `cao_app` by
//!   `a_fillet_too_big_beside_an_arc_is_held_back_and_the_field_offers_the_largest`,
//!   the share it offers by
//!   `the_tightest_side_beside_an_arc_says_how_much_of_the_value_fits` in the
//!   sketch crate; the red is the canvas's
//!
//! What the design adds so that the tools around a cut do not break on a
//! curve side — not in the issue's words:
//! - erasing the arc a fillet stands on takes the fillet —
//!   `erasing_the_arc_side_of_a_rounded_corner_takes_its_cut`
//! - a division of an arc side keeps the cut on the piece touching the corner —
//!   `dividing_the_arc_side_of_a_rounded_corner_keeps_its_cut`
//! - a mirror copies a fillet standing beside an arc —
//!   `a_corner_rounded_beside_an_arc_mirrored_is_rounded_on_its_copy`
//! - compacting the history keeps a fillet beside an arc, its sides followed
//!   through what they became — `a_compacted_part_keeps_a_fillet_beside_an_arc`

use std::f64::consts::{PI, TAU};

use cao_part::history::{Operation, PointRef};
use cao_part::{History, PartState};
use cao_sketch::{
    ArcDraft, ArcId, Corner, CutCurve, EllipseId, PointId, SegmentId, WorkPlane, sweep_of,
};
use glam::DVec2;

/// How near the arithmetic a place worked out in closed form comes.
const EXACT: f64 = 1e-9;

/// The first point laid after the origin: the corner every drawing here is
/// built around.
const THE_CORNER: PointId = PointId(1);

fn near(made: f64, expected: f64, what: &str) {
    assert!(
        (made - expected).abs() <= EXACT * expected.abs().max(1.0),
        "{what}: {made} where the arithmetic gives {expected}",
    );
}

fn near_place(made: DVec2, expected: DVec2, what: &str) {
    assert!(
        made.distance(expected) <= EXACT * expected.length().max(1.0),
        "{what}: {made} where the arithmetic gives {expected}",
    );
}

/// A sketch on XY holding nothing but its corner, at `corner`.
fn a_corner_at(corner: DVec2) -> History {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddPoint {
        sketch: 0,
        position: corner,
        on: Vec::new(),
    });
    history
}

fn a_trait(history: &mut History, start: PointRef, end: PointRef) {
    history.push(Operation::AddSegment {
        sketch: 0,
        start,
        end,
        construction: false,
    });
}

/// An arc running counter-clockwise from `start` to `end` round `centre`.
fn an_arc(history: &mut History, centre: DVec2, start: PointRef, end: PointRef) {
    history.push(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(centre),
        start,
        end,
        construction: false,
    });
}

fn round(history: &mut History, corner: Corner, radius: f64) {
    history.push(Operation::Fillet {
        sketch: 0,
        corners: vec![corner],
        radius: radius.into(),
        construction: false,
    });
}

/// The one fillet the drawing shows, as the arc it is drawn with.
fn the_fillet(history: &History, what: &str) -> ArcDraft {
    let drawing = PartState::rebuild(history).sketches[0].clone();
    let drawn = drawing.drawn_cuts();
    assert_eq!(drawn.len(), 1, "{what}: the corner takes its fillet");
    assert!(drawn[0].fits, "{what}: the fillet fits the corner");
    match drawn[0].curve {
        CutCurve::Arc(arc) => arc,
        CutCurve::Straight { .. } => panic!("{what}: a fillet is drawn as an arc"),
    }
}

/// Holds a fillet to the one worked out by hand: of `radius` round `centre`,
/// running between the two places it touches the sides at, the short way
/// round — on the side facing the corner.
fn assert_drawn_as(fillet: ArcDraft, radius: f64, centre: DVec2, touches: [DVec2; 2], what: &str) {
    near_place(
        fillet.centre,
        centre,
        &format!("{what}: the fillet's centre"),
    );
    near(
        fillet.centre.distance(fillet.start),
        radius,
        &format!("{what}: the fillet's radius at its start"),
    );
    near(
        fillet.centre.distance(fillet.end),
        radius,
        &format!("{what}: the fillet's radius at its end"),
    );
    let ends = match fillet.start.distance(touches[0]) < fillet.start.distance(touches[1]) {
        true => [fillet.start, fillet.end],
        false => [fillet.end, fillet.start],
    };
    near_place(
        ends[0],
        touches[0],
        &format!("{what}: where it touches the first side"),
    );
    near_place(
        ends[1],
        touches[1],
        &format!("{what}: where it touches the second side"),
    );
    assert!(
        sweep_of(fillet) < PI,
        "{what}: the fillet runs the short way round, facing the corner, not {} rad",
        sweep_of(fillet),
    );
}

/// How far a place stands from the line a trait runs along.
fn off_the_line(place: DVec2, through: DVec2, towards: DVec2) -> f64 {
    (place - through)
        .perp_dot((towards - through).normalize())
        .abs()
}

#[test]
fn a_fillet_rounds_the_corner_between_a_trait_and_an_arc_tangent_to_both() {
    let corner = DVec2::new(10.0, 10.0);

    {
        let mut history = a_corner_at(corner);
        let far = DVec2::new(50.0, 10.0);
        a_trait(
            &mut history,
            PointRef::Existing(THE_CORNER),
            PointRef::New(far),
        );
        let centre = DVec2::new(2.0, 10.0);
        an_arc(
            &mut history,
            centre,
            PointRef::Existing(THE_CORNER),
            PointRef::New(DVec2::new(2.0, 18.0)),
        );
        round(&mut history, Corner::At(THE_CORNER), 5.0);
        let what = "an arc bulging towards the corner";
        let fillet = the_fillet(&history, what);
        near(
            off_the_line(fillet.centre, corner, far),
            5.0,
            &format!("{what}: tangent to the trait, its centre a radius off the trait's line"),
        );
        near(
            fillet.centre.distance(centre),
            8.0 + 5.0,
            &format!("{what}: tangent to the arc from outside its circle"),
        );
        assert_drawn_as(
            fillet,
            5.0,
            DVec2::new(14.0, 15.0),
            [
                DVec2::new(14.0, 10.0),
                DVec2::new(2.0 + 96.0 / 13.0, 10.0 + 40.0 / 13.0),
            ],
            what,
        );
    }

    {
        let mut history = a_corner_at(corner);
        let far = DVec2::new(24.0, 10.0);
        a_trait(
            &mut history,
            PointRef::Existing(THE_CORNER),
            PointRef::New(far),
        );
        let centre = DVec2::new(28.0, 10.0);
        an_arc(
            &mut history,
            centre,
            PointRef::New(DVec2::new(28.0, 28.0)),
            PointRef::Existing(THE_CORNER),
        );
        round(&mut history, Corner::At(THE_CORNER), 5.0);
        let what = "an arc hollow towards the corner";
        let fillet = the_fillet(&history, what);
        near(
            off_the_line(fillet.centre, corner, far),
            5.0,
            &format!("{what}: tangent to the trait, its centre a radius off the trait's line"),
        );
        near(
            fillet.centre.distance(centre),
            18.0 - 5.0,
            &format!("{what}: tangent to the arc from inside its circle"),
        );
        assert_drawn_as(
            fillet,
            5.0,
            DVec2::new(16.0, 15.0),
            [
                DVec2::new(16.0, 10.0),
                DVec2::new(28.0 - 216.0 / 13.0, 10.0 + 90.0 / 13.0),
            ],
            what,
        );
    }
}

#[test]
fn a_fillet_rounds_the_corner_between_two_arcs_tangent_to_both() {
    {
        let corner = DVec2::new(0.0, 16.0);
        let mut history = a_corner_at(corner);
        let (east, west) = (DVec2::new(12.0, 0.0), DVec2::new(-12.0, 0.0));
        an_arc(
            &mut history,
            east,
            PointRef::Existing(THE_CORNER),
            PointRef::New(DVec2::new(-8.0, 0.0)),
        );
        an_arc(
            &mut history,
            west,
            PointRef::New(DVec2::new(8.0, 0.0)),
            PointRef::Existing(THE_CORNER),
        );
        round(
            &mut history,
            Corner::Between(ArcId(0).into(), ArcId(1).into()),
            5.0,
        );
        let what = "the tip of a lens, both arcs hollow towards it";
        let fillet = the_fillet(&history, what);
        for centre in [east, west] {
            near(
                fillet.centre.distance(centre),
                20.0 - 5.0,
                &format!("{what}: tangent to the arc round {centre} from inside its circle"),
            );
        }
        assert_drawn_as(
            fillet,
            5.0,
            DVec2::new(0.0, 9.0),
            [DVec2::new(-4.0, 12.0), DVec2::new(4.0, 12.0)],
            what,
        );
    }

    {
        let corner = DVec2::new(0.0, 5.0);
        let mut history = a_corner_at(corner);
        let (east, west) = (DVec2::new(12.0, 0.0), DVec2::new(-12.0, 0.0));
        an_arc(
            &mut history,
            east,
            PointRef::New(DVec2::new(12.0, 13.0)),
            PointRef::Existing(THE_CORNER),
        );
        an_arc(
            &mut history,
            west,
            PointRef::Existing(THE_CORNER),
            PointRef::New(DVec2::new(-12.0, 13.0)),
        );
        round(
            &mut history,
            Corner::Between(ArcId(0).into(), ArcId(1).into()),
            7.0,
        );
        let what = "two arcs bulging towards the corner between them";
        let fillet = the_fillet(&history, what);
        for centre in [east, west] {
            near(
                fillet.centre.distance(centre),
                13.0 + 7.0,
                &format!("{what}: tangent to the arc round {centre} from outside its circle"),
            );
        }
        assert_drawn_as(
            fillet,
            7.0,
            DVec2::new(0.0, 16.0),
            [DVec2::new(4.2, 10.4), DVec2::new(-4.2, 10.4)],
            what,
        );
    }
}

/// A trait running east from the corner at (10, 10) to (50, 10), and an arc
/// round (2, 10) of radius 8 leaving the corner northwards and turning three
/// quarters of the way round to (2, 2). At the corner the arc's tangent points
/// north, above the trait; the chord to its far end points south-west, below
/// it.
fn a_trait_and_an_arc_turning_back_under_it() -> History {
    let mut history = a_corner_at(DVec2::new(10.0, 10.0));
    a_trait(
        &mut history,
        PointRef::Existing(THE_CORNER),
        PointRef::New(DVec2::new(50.0, 10.0)),
    );
    an_arc(
        &mut history,
        DVec2::new(2.0, 10.0),
        PointRef::Existing(THE_CORNER),
        PointRef::New(DVec2::new(2.0, 2.0)),
    );
    history
}

/// The trait's free end, dragged by hand to `position`.
fn drag_the_trait_s_free_end(history: &mut History, position: DVec2) {
    history.push(Operation::MovePoint {
        sketch: 0,
        point: PointId(2),
        position,
        merged_into: None,
        on: Vec::new(),
        let_go: false,
    });
}

/// Which side of the trait the fillet stands on, looking from the corner
/// along the trait: one to its left, minus one to its right.
fn the_side_of_the_trait_the_fillet_stands_on(history: &History, what: &str) -> f64 {
    let drawing = PartState::rebuild(history).sketches[0].clone();
    let fillet = the_fillet(history, what);
    let (corner, far) = drawing.endpoints(SegmentId(0));
    assert!(
        corner.distance(drawing.point(THE_CORNER)) < EXACT,
        "{what}: the trait still runs from the corner"
    );
    (far - corner).perp_dot(fillet.centre - corner).signum()
}

#[test]
fn a_fillet_beside_an_arc_stands_in_the_angle_under_half_a_turn_between_the_tangents() {
    let mut history = a_trait_and_an_arc_turning_back_under_it();
    round(&mut history, Corner::At(THE_CORNER), 5.0);

    let what = "an arc whose chord leaves below the trait and its tangent above";
    let fillet = the_fillet(&history, what);
    assert!(
        fillet.centre.y > 10.0,
        "{what}: the fillet stands above the trait, in the angle the arc's tangent makes \
         with it, not below where the chord to the arc's far end points: {fillet:?}"
    );
    assert_drawn_as(
        fillet,
        5.0,
        DVec2::new(14.0, 15.0),
        [
            DVec2::new(14.0, 10.0),
            DVec2::new(2.0 + 96.0 / 13.0, 10.0 + 40.0 / 13.0),
        ],
        what,
    );
}

#[test]
fn a_fillet_beside_an_arc_changes_side_only_when_a_drag_takes_the_angle_through_half_a_turn() {
    let mut history = a_trait_and_an_arc_turning_back_under_it();
    round(&mut history, Corner::At(THE_CORNER), 5.0);
    let at_first = the_side_of_the_trait_the_fillet_stands_on(&history, "as laid");

    drag_the_trait_s_free_end(&mut history, DVec2::new(34.0, 42.0));
    let closed_up = the_side_of_the_trait_the_fillet_stands_on(
        &history,
        "the trait turned up towards the arc's tangent, past the chord",
    );
    assert_eq!(
        closed_up, at_first,
        "the angle the trait makes with the arc's tangent closed from 90° to about 37° \
         and stayed under half a turn: the fillet keeps its side, though the trait \
         crossed the chord to the arc's far end"
    );

    drag_the_trait_s_free_end(&mut history, DVec2::new(-14.0, -22.0));
    let turned_through = the_side_of_the_trait_the_fillet_stands_on(
        &history,
        "the trait turned down past the arc's tangent taken backwards",
    );
    assert_eq!(
        turned_through, -at_first,
        "the angle the trait makes with the arc's tangent went through half a turn: \
         the fillet stands on the trait's other side"
    );
}

/// How near tangency a fillet worked out by a numeric solve comes: an
/// ellipse's offset has no closed form.
const SOLVED: f64 = 1e-7;

/// An ellipse as the test reads it: its centre, the reach of its first axis
/// with its direction, and the reach of the second, a quarter turn
/// counter-clockwise from the first.
#[derive(Clone, Copy, Debug)]
struct Oval {
    centre: DVec2,
    first: DVec2,
    second: f64,
}

impl Oval {
    fn across(self) -> DVec2 {
        self.first.perp().normalize() * self.second
    }

    /// The two ends of the first axis: the turn of a half, then of nothing.
    fn first_ends(self) -> [DVec2; 2] {
        [self.centre - self.first, self.centre + self.first]
    }

    /// The place in the ellipse's own measure, where it is the unit circle.
    fn squashed(self, place: DVec2) -> DVec2 {
        let out = place - self.centre;
        DVec2::new(
            out.dot(self.first.normalize()) / self.first.length(),
            out.dot(self.first.perp().normalize()) / self.second,
        )
    }

    /// The implicit equation: nought on the curve, negative inside.
    fn off_the_curve(self, place: DVec2) -> f64 {
        self.squashed(place).length_squared() - 1.0
    }

    /// The unit normal pointing out of the curve, the implicit equation's
    /// gradient.
    fn outwards_at(self, place: DVec2) -> DVec2 {
        let squashed = self.squashed(place);
        (self.first.normalize() * (squashed.x / self.first.length())
            + self.first.perp().normalize() * (squashed.y / self.second))
            .normalize()
    }

    fn turn_of(self, place: DVec2) -> f64 {
        self.squashed(place).to_angle().rem_euclid(TAU)
    }

    /// The unit tangent at a place, counter-clockwise.
    fn onwards_at(self, place: DVec2) -> DVec2 {
        let turn = self.turn_of(place);
        (-self.first * turn.sin() + self.across() * turn.cos()).normalize()
    }
}

/// A side of a corner as the test knows it from the drawing it laid: what it
/// is, and the far end of what is drawn of it.
#[derive(Clone, Copy, Debug)]
enum Side {
    Trait {
        far: DVec2,
    },
    Arc {
        centre: DVec2,
        radius: f64,
        counter_clockwise: bool,
        far: DVec2,
    },
    ArcOfEllipse {
        oval: Oval,
        counter_clockwise: bool,
        far: DVec2,
    },
}

impl Side {
    /// The unit direction the side leaves the corner in.
    fn leaving(self, corner: DVec2) -> DVec2 {
        let (onwards, counter_clockwise) = match self {
            Self::Trait { far } => return (far - corner).normalize(),
            Self::Arc {
                centre,
                counter_clockwise,
                ..
            } => ((corner - centre).perp().normalize(), counter_clockwise),
            Self::ArcOfEllipse {
                oval,
                counter_clockwise,
                ..
            } => (oval.onwards_at(corner), counter_clockwise),
        };
        match counter_clockwise {
            true => onwards,
            false => -onwards,
        }
    }

    /// How far off the side's line or curve a place stands — for an ellipse,
    /// its implicit equation, which is nought on it as well.
    fn off(self, corner: DVec2, place: DVec2) -> f64 {
        match self {
            Self::Trait { far } => off_the_line(place, corner, far),
            Self::Arc { centre, radius, .. } => (place.distance(centre) - radius).abs(),
            Self::ArcOfEllipse { oval, .. } => oval.off_the_curve(place).abs(),
        }
    }

    /// How far round from the corner a place of the side's line or curve
    /// lies, and how far round its far end does — along the trait, or as a
    /// turn the way the side leaves the corner.
    fn reach(self, corner: DVec2, place: DVec2) -> (f64, f64) {
        let (angle_of, counter_clockwise): (Box<dyn Fn(DVec2) -> f64>, bool) = match self {
            Self::Trait { far } => {
                let along = (far - corner).normalize();
                return ((place - corner).dot(along), corner.distance(far));
            }
            Self::Arc {
                centre,
                counter_clockwise,
                ..
            } => (
                Box::new(move |at: DVec2| (at - centre).to_angle()),
                counter_clockwise,
            ),
            Self::ArcOfEllipse {
                oval,
                counter_clockwise,
                ..
            } => (Box::new(move |at| oval.turn_of(at)), counter_clockwise),
        };
        let far = match self {
            Self::Trait { far } | Self::Arc { far, .. } | Self::ArcOfEllipse { far, .. } => far,
        };
        let round = |at: DVec2| {
            let turned = angle_of(at) - angle_of(corner);
            match counter_clockwise {
                true => turned,
                false => -turned,
            }
            .rem_euclid(TAU)
        };
        (round(place), round(far))
    }

    /// Holds the fillet round `centre` of `radius` tangent to the side at
    /// `touch`: the touch on the side's line or curve, the centre a radius
    /// from it square to the side, on the side `outside` says for a curve.
    fn assert_tangent(
        self,
        corner: DVec2,
        centre: DVec2,
        radius: f64,
        touch: DVec2,
        outside: bool,
        what: &str,
    ) {
        assert!(
            self.off(corner, touch) <= SOLVED,
            "{what}: the fillet touches the side at {touch}, which stands {} off it",
            self.off(corner, touch),
        );
        let towards = centre - touch;
        assert!(
            (towards.length() - radius).abs() <= SOLVED * radius.max(1.0),
            "{what}: the fillet's centre stands {} from where it touches the side, where \
             its radius is {radius}",
            towards.length(),
        );
        let (square, out) = match self {
            Self::Trait { far } => ((far - corner).normalize().perp(), None),
            Self::Arc { centre: round, .. } => {
                let out = (touch - round).normalize();
                (out, Some(out))
            }
            Self::ArcOfEllipse { oval, .. } => {
                let out = oval.outwards_at(touch);
                (out, Some(out))
            }
        };
        assert!(
            square.perp_dot(towards.normalize()).abs() <= SOLVED,
            "{what}: the fillet's centre stands square to the side where it touches it, \
             {} off square",
            square.perp_dot(towards.normalize()),
        );
        if let Some(out) = out {
            assert_eq!(
                out.dot(towards) > 0.0,
                outside,
                "{what}: the fillet stands {} the curve",
                match outside {
                    true => "outside",
                    false => "inside",
                },
            );
        }
        let (along, drawn) = self.reach(corner, touch);
        assert!(
            along > 0.0 && along <= drawn,
            "{what}: the fillet touches the side {along} on from the corner, where what is \
             drawn of it runs {drawn}",
        );
    }
}

/// Holds the one fillet the drawing shows to be tangent to both sides of the
/// corner, standing in the angle under half a turn their tangents make there.
/// `outside` says, for each side that is a curve, whether the fillet stands
/// outside it.
fn assert_tangent_to_both(
    history: &History,
    corner: DVec2,
    sides: [Side; 2],
    radius: f64,
    outside: [bool; 2],
    what: &str,
) {
    let fillet = the_fillet(history, what);
    for end in [fillet.start, fillet.end] {
        assert!(
            (end.distance(fillet.centre) - radius).abs() <= SOLVED * radius,
            "{what}: the fillet is drawn of radius {radius}: {fillet:?}",
        );
    }
    assert!(
        sweep_of(fillet) < PI,
        "{what}: the fillet runs the short way round, facing the corner: {fillet:?}"
    );
    for (rank, side) in sides.into_iter().enumerate() {
        let touch = match side.off(corner, fillet.start) <= side.off(corner, fillet.end) {
            true => fillet.start,
            false => fillet.end,
        };
        side.assert_tangent(
            corner,
            fillet.centre,
            radius,
            touch,
            outside[rank],
            &format!("{what}, side {rank}"),
        );
    }
    let [one, other] = sides.map(|side| side.leaving(corner));
    let turn = one.perp_dot(other).signum();
    let out = fillet.centre - corner;
    assert!(
        one.perp_dot(out) * turn > 0.0 && out.perp_dot(other) * turn > 0.0,
        "{what}: the fillet's centre {} stands in the angle the sides' tangents {one} and \
         {other} make at the corner {corner}",
        fillet.centre,
    );
}

/// An ellipse drawn over the stretch running counter-clockwise from the
/// first place to the second.
fn an_arc_of_ellipse(history: &mut History, oval: Oval, drawn: [PointRef; 2]) {
    let [back, front] = oval.first_ends();
    history.push(Operation::AddEllipse {
        sketch: 0,
        center: PointRef::New(oval.centre),
        first: [PointRef::New(back), PointRef::New(front)],
        second: [
            PointRef::New(oval.centre - oval.across()),
            PointRef::New(oval.centre + oval.across()),
        ],
        construction: false,
        drawn: Some(drawn),
    });
}

/// Ten wide and five high round (40, 30), its axes along x and y; the corner
/// stands on it at (46, 34), the turn whose cosine is three fifths.
const LEVEL: Oval = Oval {
    centre: DVec2::new(40.0, 30.0),
    first: DVec2::new(10.0, 0.0),
    second: 5.0,
};

#[test]
fn a_fillet_rounds_the_corner_between_an_arc_of_ellipse_and_a_trait_tangent_to_both() {
    let corner = DVec2::new(46.0, 34.0);
    let mut history = a_corner_at(corner);
    let far = DVec2::new(46.0, 54.0);
    a_trait(
        &mut history,
        PointRef::Existing(THE_CORNER),
        PointRef::New(far),
    );
    let [back, _] = LEVEL.first_ends();
    an_arc_of_ellipse(
        &mut history,
        LEVEL,
        [PointRef::Existing(THE_CORNER), PointRef::New(back)],
    );

    round(&mut history, Corner::At(THE_CORNER), 3.0);

    assert_tangent_to_both(
        &history,
        corner,
        [
            Side::Trait { far },
            Side::ArcOfEllipse {
                oval: LEVEL,
                counter_clockwise: true,
                far: back,
            },
        ],
        3.0,
        [true, true],
        "a trait rising from an arc of ellipse running over the top of its curve",
    );
}

#[test]
fn a_fillet_rounds_the_corner_between_an_arc_of_ellipse_and_an_arc_tangent_to_both() {
    let tilted = Oval {
        centre: DVec2::new(30.0, 20.0),
        first: DVec2::new(8.0, 6.0),
        second: 5.0,
    };
    let corner = tilted.centre + DVec2::new(2.4, 6.8);
    let mut history = a_corner_at(corner);
    let [back, _] = tilted.first_ends();
    an_arc_of_ellipse(
        &mut history,
        tilted,
        [PointRef::Existing(THE_CORNER), PointRef::New(back)],
    );
    let (centre, far) = (
        corner + DVec2::new(10.0, 0.0),
        corner + DVec2::new(10.0, -10.0),
    );
    an_arc(
        &mut history,
        centre,
        PointRef::Existing(THE_CORNER),
        PointRef::New(far),
    );

    round(
        &mut history,
        Corner::Between(EllipseId(0).into(), ArcId(0).into()),
        2.0,
    );

    assert_tangent_to_both(
        &history,
        corner,
        [
            Side::ArcOfEllipse {
                oval: tilted,
                counter_clockwise: true,
                far: back,
            },
            Side::Arc {
                centre,
                radius: 10.0,
                counter_clockwise: true,
                far,
            },
        ],
        2.0,
        [false, true],
        "an arc dropping into a tilted arc of ellipse, the fillet inside the ellipse",
    );
}

#[test]
fn a_fillet_rounds_the_corner_between_two_arcs_of_ellipse_tangent_to_both() {
    let corner = DVec2::new(46.0, 34.0);
    let tilted = Oval {
        centre: DVec2::new(45.6, 26.8),
        first: DVec2::new(6.0, 8.0),
        second: 5.0,
    };
    let mut history = a_corner_at(corner);
    let [back, _] = LEVEL.first_ends();
    an_arc_of_ellipse(
        &mut history,
        LEVEL,
        [PointRef::Existing(THE_CORNER), PointRef::New(back)],
    );
    let [_, front] = tilted.first_ends();
    an_arc_of_ellipse(
        &mut history,
        tilted,
        [PointRef::New(front), PointRef::Existing(THE_CORNER)],
    );

    round(
        &mut history,
        Corner::Between(EllipseId(0).into(), EllipseId(1).into()),
        3.0,
    );

    assert_tangent_to_both(
        &history,
        corner,
        [
            Side::ArcOfEllipse {
                oval: LEVEL,
                counter_clockwise: true,
                far: back,
            },
            Side::ArcOfEllipse {
                oval: tilted,
                counter_clockwise: false,
                far: front,
            },
        ],
        3.0,
        [true, true],
        "two arcs of ellipse, one of them tilted, the fillet outside both",
    );
}
