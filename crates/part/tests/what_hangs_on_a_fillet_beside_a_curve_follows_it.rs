//! What #620 hangs on a fillet's points and #651 holds tangent to its arc, on
//! a fillet one of whose sides is an arc or an arc of ellipse (#621). There the
//! fillet's places are found by a numeric solve rather than by a formula; what
//! hangs on them follows them all the same, when the corner, the sides or the
//! value change.
//!
//! Every place a test expects is worked out here by hand, from the corner, the
//! trait and the curve as they stand — never read off the cut it checks.
//!
//! Closes #675.
//! - What #620 hangs on a fillet's points, and what #651 holds tangent to its
//!   arc, hangs and holds the same way on a fillet beside an arc or an arc of
//!   ellipse, and follows it — no test: the issue's "What it does" splits it
//!   into the bullets below, beside an arc and beside an arc of ellipse, each
//!   naming its tests
//!   - a line, a circle, a dimension or a « Coïncidence » hangs on the
//!     fillet's centre or its touches, and follows them when the corner, the
//!     sides or the value change; beside an arc —
//!     `a_line_started_from_a_fillet_s_end_beside_an_arc_follows_a_new_radius`,
//!     `a_circle_drawn_on_the_centre_of_a_fillet_beside_an_arc_stays_on_it_when_the_corner_moves`,
//!     `a_dimension_from_a_point_of_a_fillet_beside_an_arc_holds_when_the_value_changes`,
//!     `a_coincidence_hangs_a_point_on_a_point_of_a_fillet_beside_an_arc`,
//!     `a_line_started_from_a_fillet_s_end_on_an_arc_follows_the_arc_given_a_new_radius`
//!   - the same beside an arc of ellipse —
//!     `a_line_started_from_a_fillet_s_end_beside_an_arc_of_ellipse_follows_a_new_radius`,
//!     `a_circle_drawn_on_the_centre_of_a_fillet_beside_an_arc_of_ellipse_stays_on_it_when_the_corner_moves`,
//!     `a_dimension_from_a_point_of_a_fillet_beside_an_arc_of_ellipse_holds_when_the_value_changes`,
//!     `a_coincidence_hangs_a_point_on_a_point_of_a_fillet_beside_an_arc_of_ellipse`,
//!     `a_line_started_from_a_fillet_s_end_on_an_arc_of_ellipse_follows_the_ellipse_reshaped`
//!   - a point placed on the fillet's arc stays on it —
//!     `a_point_placed_on_a_fillet_beside_an_arc_follows_its_arc`,
//!     `a_point_placed_on_a_fillet_beside_an_arc_of_ellipse_follows_its_arc`
//!   - a line held tangent to the fillet's arc follows the corner, the radius,
//!     and the arc or arc of ellipse beside it —
//!     `a_line_held_tangent_to_a_fillet_beside_an_arc_follows_its_corner_moved`,
//!     `a_line_held_tangent_to_a_fillet_beside_an_arc_follows_a_new_radius`,
//!     `a_line_held_tangent_to_a_fillet_beside_an_arc_follows_the_arc_given_a_new_radius`,
//!     `a_line_held_tangent_to_a_fillet_beside_an_arc_of_ellipse_follows_its_corner_moved`,
//!     `a_line_held_tangent_to_a_fillet_beside_an_arc_of_ellipse_follows_a_new_radius`,
//!     `a_line_held_tangent_to_a_fillet_beside_an_arc_of_ellipse_follows_the_ellipse_reshaped`
//! - `nothing_hangs_on_the_points_of_a_fillet_beside_a_curve` is replaced by
//!   tests saying what does hang — it is gone from
//!   `a_fillet_rounds_a_corner_with_a_curve.rs`; what it laid there, a line
//!   from a touch, a point on the centre, a « Coïncidence » and a « Tangence »,
//!   the radius changed after, is held to follow by
//!   `a_line_started_from_a_fillet_s_end_beside_an_arc_follows_a_new_radius`,
//!   `a_circle_drawn_on_the_centre_of_a_fillet_beside_an_arc_stays_on_it_when_the_corner_moves`,
//!   `a_coincidence_hangs_a_point_on_a_point_of_a_fillet_beside_an_arc` and
//!   `a_line_held_tangent_to_a_fillet_beside_an_arc_follows_a_new_radius`
//!
//! What must not break, in the issue's words:
//! - everything #620, #651 and #621 deliver on two traits — no test: held by
//!   their own files, `what_hangs_on_a_cut_follows_it.rs`,
//!   `a_line_held_tangent_to_a_fillet_follows_it.rs` and
//!   `a_fillet_rounds_a_corner_with_a_curve.rs`, which keep every test but the
//!   one this issue replaces
//! - a rounded outline beside a curve drags as fast as a sharp one when
//!   nothing hangs on its fillet — no test: what the solver is asked is read
//!   only inside the sketch crate, `Sketch::equations` being `pub(crate)`, so
//!   the test holding it lives there, in `corner_cut/hanging/tests.rs`:
//!   `a_fillet_beside_a_curve_nothing_hangs_on_asks_the_solver_for_nothing`,
//!   beside its counterpart between two traits,
//!   `a_cut_nothing_hangs_on_asks_the_solver_for_nothing`
//!
//! Settled with the owner on 2026-11-04, in the issue's comment, when it was
//! taken up — each answer held by a bullet of its own:
//! - a red fillet beside a curve that still has a place, overrunning a side:
//!   as #620, what hangs follows the red fillet where it is drawn, nothing
//!   jumps, and when it fits again nothing has moved —
//!   `a_fillet_beside_a_curve_that_overruns_its_side_keeps_what_hangs_on_it_on_the_red_fillet`
//! - a red fillet beside a curve with no place at all, a fillet of 6 in the
//!   hollow of an arc of 5: #621 draws it red where the corner's tangents
//!   alone would put it, and what hangs follows that red fillet as drawn, even
//!   when the drawing jumps to that stand-in, and comes back with it when the
//!   fillet finds a place again —
//!   `what_hangs_on_a_fillet_with_no_place_in_the_hollow_of_an_arc_follows_its_stand_in_and_comes_back`
//! - taking the fillet off, as #620: its centre becomes a plain point; an end
//!   of the fillet stays held on its side — on the arc or the arc of ellipse,
//!   as it would on a trait, by the rule that holds any point on that curve,
//!   `Constraint::OnArc` or `Constraint::OnEllipse` —
//!   `taking_a_fillet_beside_a_curve_off_leaves_its_centre_free_and_its_ends_on_their_sides`
//! - a value change: everything follows —
//!   `a_line_started_from_a_fillet_s_end_beside_an_arc_follows_a_new_radius`,
//!   `a_line_started_from_a_fillet_s_end_beside_an_arc_of_ellipse_follows_a_new_radius`,
//!   `a_point_placed_on_a_fillet_beside_an_arc_follows_its_arc`,
//!   `a_line_held_tangent_to_a_fillet_beside_an_arc_follows_a_new_radius`
//! - a kind change does not arise — no test: the corner tool reshapes a cut in
//!   place only when the kind asked is the one it has (`whole_group` in
//!   `cao_app`'s `input/corner/laying.rs`); a chamfer asked on a fillet cuts
//!   the corner again through `Operation::Chamfer`, which #621 refuses beside
//!   a curve, held in `a_fillet_rounds_a_corner_with_a_curve.rs` by
//!   `a_chamfer_on_a_corner_with_an_arc_or_an_arc_of_ellipse_is_refused`
//! - dragging a point of the fillet is grabbing the corner, as between two
//!   traits — `a_press_on_a_point_hung_on_a_fillet_beside_a_curve_drags_its_corner`;
//!   a press on a point nothing hangs on yet is #621's, held in the sketch
//!   crate by `a_press_on_a_fillet_beside_an_arc_drags_its_corner`

use cao_part::history::{CutAsked, CutChange, Operation, PointRef};
use cao_part::{History, PartState};
use cao_sketch::{
    ArcId, Corner, CornerCutId, CutPoint, DimensionTarget, Element, PointId, Rule, RuleIntent,
    RulePick, SegmentId, Sketch, Support, WorkPlane, rule_intent,
};
use glam::DVec2;

/// How near two places must stand to be the same place after a solve.
const SAME: f64 = 1e-6;

const CUT: CornerCutId = CornerCutId(0);
/// The corner, the far end of its trait, and its arc.
const CORNER: PointId = PointId(1);
const TRAIT_END: PointId = PointId(2);
const ARC: ArcId = ArcId(0);
/// The first point laid after the corner was rounded.
const HUNG: PointId = PointId(5);

/// Where the arc's centre stands, and its radius.
const ROUND: DVec2 = DVec2::new(2.0, 10.0);
const REACH: f64 = 8.0;

/// A corner at (10, 10) between a trait running east to (50, 10) and an arc
/// round (2, 10) of radius 8, leaving the corner northwards and turning
/// counter-clockwise up to (2, 18): it bulges towards the corner. The corner
/// is named by its trait then its arc, so that the fillet's first touch is on
/// the trait and its second on the arc, and rounded to 5 — the first value,
/// which makes a unit a millimetre.
///
/// A fillet of `r` there stands `r` above the trait and `8 + r` from the
/// arc's centre: round (2 + 4√(4 + r), 10 + r). Rounded to 5 it runs round
/// (14, 15), from (14, 10) on the trait to (2 + 96/13, 10 + 40/13) on the arc;
/// rounded to 12, round (18, 22), from (18, 10) to (8.4, 14.8).
fn a_trait_and_an_arc_rounded() -> History {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(10.0, 10.0),
        on: Vec::new(),
    });
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(CORNER),
        end: PointRef::New(DVec2::new(50.0, 10.0)),
        construction: false,
    });
    history.push(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(ROUND),
        start: PointRef::Existing(CORNER),
        end: PointRef::New(DVec2::new(2.0, 18.0)),
        construction: false,
    });
    history.push(Operation::Fillet {
        sketch: 0,
        corners: vec![Corner::Between(SegmentId(0).into(), ARC.into())],
        radius: 5.0.into(),
        construction: false,
    });
    history
}

/// The fillet a hand works out between a trait and an arc bulging towards
/// their corner.
struct ByHand {
    centre: DVec2,
    on_the_trait: DVec2,
    on_the_arc: DVec2,
}

/// The fillet of `radius` at `corner`, between the trait running from it to
/// `far` and the arc round `round` of `reach` leaving it counter-clockwise.
/// Its centre stands `radius` off the trait's line, on the side the arc
/// leaves towards, and `reach + radius` from the arc's centre: of the two
/// places on that line so far from it, the one ahead of the corner along the
/// trait.
fn by_hand(corner: DVec2, far: DVec2, round: DVec2, reach: f64, radius: f64) -> ByHand {
    let along = (far - corner).normalize();
    let leaving = (corner - round).perp();
    let aside = along.perp() * along.perp_dot(leaving).signum();
    let abreast = corner + aside * radius;
    let from_the_round = abreast - round;
    let half = along.dot(from_the_round);
    let rest = from_the_round.length_squared() - (reach + radius).powi(2);
    let ahead = -half + (half * half - rest).sqrt();
    let centre = abreast + along * ahead;
    ByHand {
        centre,
        on_the_trait: corner + along * ahead,
        on_the_arc: round + (centre - round) * (reach / (reach + radius)),
    }
}

/// The fillet of `radius` a hand works out on the drawing as it stands: from
/// where its corner, its trait's far end and its arc are now.
fn by_hand_on(sketch: &Sketch, radius: f64) -> ByHand {
    by_hand(
        sketch.point(CORNER),
        sketch.point(TRAIT_END),
        sketch.point(sketch.arc(ARC).center),
        sketch.arc_radius(ARC),
        radius,
    )
}

fn near(made: f64, expected: f64, what: &str) {
    assert!(
        (made - expected).abs() < SAME,
        "{what}: {made} where the arithmetic gives {expected}",
    );
}

fn near_place(made: DVec2, expected: DVec2, what: &str) {
    assert!(
        made.distance(expected) < SAME,
        "{what}: it stands at {made} where the arithmetic gives {expected}",
    );
}

fn the_drawing(history: &History) -> Sketch {
    PartState::rebuild(history).sketches[0].clone()
}

/// Where a point laid on one of the fillet's points stands, held there, as
/// the snap hands it to a tool.
fn on_the_cut(history: &History, at: CutPoint) -> PointRef {
    let place = the_drawing(history)
        .cut_point(CUT, at)
        .expect("the fillet offers its points");
    PointRef::Held {
        at: place,
        on: vec![Support::CutPoint(CUT, at)],
    }
}

/// A point laid on one of the fillet's points, held there.
fn hang_a_point(history: &mut History, at: CutPoint) {
    let PointRef::Held { at: place, on } = on_the_cut(history, at) else {
        unreachable!()
    };
    history.push(Operation::AddPoint {
        sketch: 0,
        position: place,
        on,
    });
}

/// A line from one of the fillet's points out to `to`.
fn a_line_from(history: &mut History, at: CutPoint, to: DVec2) {
    let start = on_the_cut(history, at);
    history.push(Operation::AddSegment {
        sketch: 0,
        start,
        end: PointRef::New(to),
        construction: false,
    });
}

fn rounded_to(history: &mut History, radius: f64) {
    history.push(Operation::CutChange(CutChange::Reshape {
        sketch: 0,
        cut: CUT,
        value: CutAsked::Fillet(radius.into()),
    }));
}

#[test]
fn a_line_started_from_a_fillet_s_end_beside_an_arc_follows_a_new_radius() {
    let mut history = a_trait_and_an_arc_rounded();
    a_line_from(&mut history, CutPoint::First, DVec2::new(14.0, -20.0));
    let on_the_trait = HUNG;
    a_line_from(&mut history, CutPoint::Second, DVec2::new(-20.0, 30.0));
    let on_the_arc = PointId(7);

    rounded_to(&mut history, 12.0);

    let sketch = the_drawing(&history);
    assert_eq!(sketch.segments()[1].start, on_the_trait);
    assert_eq!(sketch.segments()[2].start, on_the_arc);
    near_place(
        sketch.point(on_the_trait),
        DVec2::new(18.0, 10.0),
        "the line started where the fillet touches the trait follows that touch",
    );
    near_place(
        sketch.point(on_the_arc),
        DVec2::new(8.4, 14.8),
        "the line started where the fillet touches the arc follows that touch",
    );
}

#[test]
fn a_circle_drawn_on_the_centre_of_a_fillet_beside_an_arc_stays_on_it_when_the_corner_moves() {
    let mut history = a_trait_and_an_arc_rounded();
    let center = on_the_cut(&history, CutPoint::Centre);
    history.push(Operation::AddCircle {
        sketch: 0,
        center,
        radius: 2.0,
        rim: Vec::new(),
        construction: false,
    });
    let round_the_arc = ROUND + DVec2::from_angle((-30.0_f64).to_radians()) * REACH;

    history.push(Operation::MovePoint {
        sketch: 0,
        point: CORNER,
        position: round_the_arc,
        merged_into: None,
        on: Vec::new(),
        let_go: false,
    });

    let sketch = the_drawing(&history);
    assert_eq!(sketch.circles()[0].center, HUNG);
    near_place(
        sketch.point(CORNER),
        round_the_arc,
        "the corner slides round the arc to where it was taken",
    );
    let fillet = by_hand(round_the_arc, DVec2::new(50.0, 10.0), ROUND, REACH, 5.0);
    near_place(
        sketch.point(HUNG),
        fillet.centre,
        "the circle's centre follows the fillet's centre",
    );
}

#[test]
fn a_dimension_from_a_point_of_a_fillet_beside_an_arc_holds_when_the_value_changes() {
    let mut history = a_trait_and_an_arc_rounded();
    hang_a_point(&mut history, CutPoint::First);
    history.push(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Distance {
            from: TRAIT_END,
            to: HUNG,
        },
        value: 36.0.into(),
        placement: None,
    });

    rounded_to(&mut history, 12.0);

    let sketch = the_drawing(&history);
    near(
        sketch.point(TRAIT_END).distance(sketch.point(HUNG)),
        36.0,
        "the dimension from the trait's far end to the fillet's touch holds",
    );
    near_place(
        sketch.point(HUNG),
        by_hand_on(&sketch, 12.0).on_the_trait,
        "the point the dimension measures to stays where the fillet touches the trait",
    );
}

#[test]
fn a_coincidence_hangs_a_point_on_a_point_of_a_fillet_beside_an_arc() {
    let mut history = a_trait_and_an_arc_rounded();
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(30.0, 30.0)),
        end: PointRef::New(DVec2::new(40.0, 40.0)),
        construction: false,
    });
    let loose = HUNG;
    let picks = [
        RulePick::Element(Element::Point(loose)),
        RulePick::CutPoint(CUT, CutPoint::Centre),
    ];
    let Some(RuleIntent::Constrain(rule)) =
        rule_intent(Rule::Coincident, &picks, &the_drawing(&history))
    else {
        panic!("a point and the centre of a fillet beside an arc make no coincidence");
    };
    history.push(Operation::Constrain {
        sketch: 0,
        constraint: rule,
    });
    near_place(
        the_drawing(&history).point(loose),
        DVec2::new(14.0, 15.0),
        "the coincidence brings the point onto the fillet's centre",
    );

    rounded_to(&mut history, 12.0);

    near_place(
        the_drawing(&history).point(loose),
        DVec2::new(18.0, 22.0),
        "the point follows the fillet's centre",
    );
}

#[test]
fn a_point_placed_on_a_fillet_beside_an_arc_follows_its_arc() {
    let mut history = a_trait_and_an_arc_rounded();
    let on_the_fillet = DVec2::new(14.0, 15.0) + DVec2::from_angle(225_f64.to_radians()) * 5.0;
    history.push(Operation::AddPoint {
        sketch: 0,
        position: on_the_fillet,
        on: vec![Support::Cut(CUT)],
    });

    rounded_to(&mut history, 12.0);

    near(
        the_drawing(&history)
            .point(HUNG)
            .distance(DVec2::new(18.0, 22.0)),
        12.0,
        "the point placed on the fillet stands a radius from its centre",
    );
}

#[test]
fn a_line_started_from_a_fillet_s_end_on_an_arc_follows_the_arc_given_a_new_radius() {
    let mut history = a_trait_and_an_arc_rounded();
    a_line_from(&mut history, CutPoint::Second, DVec2::new(-20.0, 30.0));

    history.push(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::ArcRadius(ARC),
        value: 10.0.into(),
        placement: None,
    });

    let sketch = the_drawing(&history);
    near(
        sketch.arc_radius(ARC),
        10.0,
        "the arc takes the radius it was given",
    );
    near_place(
        sketch.point(HUNG),
        by_hand_on(&sketch, 5.0).on_the_arc,
        "the line started where the fillet touches the arc follows that touch",
    );
}
