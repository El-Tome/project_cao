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
//!
//! Settled with the owner on 2026-11-04, in the issue's second comment, after
//! the review found a case the answers above did not cover:
//! - taking off a red fillet beside a curve that has no place: its stand-in
//!   touches the corner's tangent, not the curve, so an end hung there is not
//!   on its side; it is let go where it stands, a plain point like the
//!   centre, and nothing moves —
//!   `taking_off_a_fillet_with_no_place_lets_the_end_on_its_stand_in_go_where_it_stands`;
//!   nor does the arc, its centre or the corner when the line hung there is
//!   fixed —
//!   `taking_off_a_fillet_with_no_place_moves_nothing_though_the_line_on_its_stand_in_is_fixed`
//!
//! Surfaced by the review and the reading of the criteria:
//! - dividing or trimming the arc beside a fillet keeps what hangs on the
//!   fillet, handed to the piece left at the corner, and it still follows a
//!   new radius after —
//!   `what_hangs_on_a_fillet_beside_an_arc_stays_hung_once_the_arc_is_divided`,
//!   `what_hangs_on_a_fillet_beside_an_arc_stays_hung_once_the_arc_is_trimmed`;
//!   trimming an arc of ellipse keeps it too —
//!   `what_hangs_on_a_fillet_beside_an_arc_of_ellipse_stays_hung_once_it_is_trimmed`
//! - a line held tangent to a red fillet beside a curve, with a place and
//!   with none, stays tangent to it as it is drawn —
//!   `a_line_held_tangent_to_a_red_fillet_beside_an_arc_stays_tangent_to_it_as_drawn`
//! - beside an arc of ellipse too, what hangs on a fillet with no place
//!   follows its stand-in and comes back —
//!   `what_hangs_on_a_fillet_with_no_place_inside_an_arc_of_ellipse_follows_its_stand_in_and_comes_back`
//! - a trim past a fillet, of its trait or of its arc, counts as lost the
//!   rules it truly drops and not what hangs on the fillet, hung again on the
//!   piece left — the count the interface words as rules lost to the cut —
//!   `a_trim_past_a_fillet_counts_what_it_drops_and_not_what_hangs_on_the_fillet`
//! - trimming the curve beside a fillet from its corner takes the fillet, and
//!   lets go of what hung on it where it stands, as erasing a side does since
//!   #620: no rule names the fillet any more, and the trim counts the rules
//!   lost — an arc of ellipse as an arc —
//!   `trimming_the_curve_beside_a_fillet_from_its_corner_takes_the_fillet_and_lets_go_of_what_hung_on_it`
//! - taking off a red fillet that still has a place and overruns its side
//!   keeps the end past the side held on it — on the trait's whole line, the
//!   arc's whole circle, the whole ellipse — and nothing moves —
//!   `taking_off_a_red_fillet_that_overruns_its_side_keeps_the_end_past_it_held_on_its_side`
//! - nothing hangs on a cut taken off — no test: a rule refused as it is laid
//!   is read inside the sketch crate; held in `corner_cut/hanging/tests.rs` by
//!   `nothing_hangs_on_a_cut_taken_off`

use cao_part::history::{CutAsked, CutChange, Operation, PointRef};
use cao_part::{History, Outcome, PartState};
use cao_sketch::{
    ArcId, Constraint, Corner, CornerCutId, CutPoint, CutSide, DimensionTarget, Element, EllipseId,
    Grip, PointId, Rule, RuleIntent, RulePick, SegmentId, Sketch, Support, WorkPlane, rule_intent,
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

/// The fillet a hand works out between a trait and a curve bulging towards
/// their corner: its centre, and where it touches each side.
struct ByHand {
    centre: DVec2,
    on_the_trait: DVec2,
    on_the_curve: DVec2,
}

/// The fillet of `radius` at `corner`, between the trait running from it to
/// `far` and the arc round `round` of `reach` leaving it counter-clockwise,
/// bulging towards the corner.
fn by_hand(corner: DVec2, far: DVec2, round: DVec2, reach: f64, radius: f64) -> ByHand {
    by_hand_by_an_arc(corner, far, round, reach, radius, 1.0, 1.0)
        .expect("a fillet beside an arc bulging towards its corner has a place")
}

/// The same, the arc leaving the corner `way` round — one for
/// counter-clockwise, minus one for clockwise — and the fillet standing `out`
/// of its circle: one outside, minus one inside. Its centre stands `radius`
/// off the trait's line, on the side the arc leaves towards, and
/// `reach + out · radius` from the arc's centre: of the places on that line so
/// far from it, the nearest ahead of the corner along the trait. Nothing when
/// there is none.
fn by_hand_by_an_arc(
    corner: DVec2,
    far: DVec2,
    round: DVec2,
    reach: f64,
    radius: f64,
    way: f64,
    out: f64,
) -> Option<ByHand> {
    let along = (far - corner).normalize();
    let leaving = (corner - round).perp() * way;
    let aside = along.perp() * along.perp_dot(leaving).signum();
    let abreast = corner + aside * radius;
    let from_the_round = abreast - round;
    let off = reach + out * radius;
    let half = along.dot(from_the_round);
    let rest = from_the_round.length_squared() - off * off;
    let room = half * half - rest;
    if off <= 0.0 || room < 0.0 {
        return None;
    }
    let ahead = [-half - room.sqrt(), -half + room.sqrt()]
        .into_iter()
        .filter(|ahead| *ahead > 0.0)
        .fold(f64::INFINITY, f64::min);
    let centre = abreast + along * ahead;
    Some(ByHand {
        centre,
        on_the_trait: corner + along * ahead,
        on_the_curve: round + (centre - round) * (reach / off),
    })
}

/// The fillet of `radius` a hand works out on the drawing as it stands: from
/// where its corner, its trait's far end and the arc it stands beside are now
/// — the piece at the corner, once the arc is divided or trimmed.
fn by_hand_on(sketch: &Sketch, radius: f64) -> ByHand {
    let CutSide::Arc(arc) = sketch.corner_cuts()[CUT.0].sides[1] else {
        panic!("the fillet stands beside an arc");
    };
    by_hand(
        sketch.point(CORNER),
        sketch.point(TRAIT_END),
        sketch.point(sketch.arc(arc).center),
        sketch.arc_radius(arc),
        radius,
    )
}

/// An ellipse as the test reads it: its centre, the reach of its first axis
/// with its direction, and the reach of the second, a quarter turn
/// counter-clockwise from the first.
#[derive(Clone, Copy)]
struct Oval {
    centre: DVec2,
    first: DVec2,
    second: f64,
}

impl Oval {
    fn across(self) -> DVec2 {
        self.first.perp().normalize() * self.second
    }

    fn at(self, turn: f64) -> DVec2 {
        self.centre + self.first * turn.cos() + self.across() * turn.sin()
    }

    /// The unit normal pointing out of the curve where a turn lands: the
    /// gradient of its implicit equation.
    fn outwards(self, turn: f64) -> DVec2 {
        (self.first.normalize() * (turn.cos() / self.first.length())
            + self.first.perp().normalize() * (turn.sin() / self.second))
            .normalize()
    }

    /// The unit tangent where a turn lands, counter-clockwise.
    fn onwards(self, turn: f64) -> DVec2 {
        (-self.first * turn.sin() + self.across() * turn.cos()).normalize()
    }

    /// The turn a place of the curve stands at.
    fn turn_of(self, place: DVec2) -> f64 {
        let out = place - self.centre;
        DVec2::new(
            out.dot(self.first.normalize()) / self.first.length(),
            out.dot(self.first.perp().normalize()) / self.second,
        )
        .to_angle()
    }
}

/// Ten across and five high round (40, 30), its axes along x and y.
const LEVEL: Oval = Oval {
    centre: DVec2::new(40.0, 30.0),
    first: DVec2::new(10.0, 0.0),
    second: 5.0,
};

/// A corner at (46, 34) between a trait rising to (46, 54) and the arc of
/// ellipse `LEVEL` draws counter-clockwise from that corner, the turn whose
/// cosine is three fifths, over the top to (30, 30): it bulges towards the
/// corner. Named by its trait then its curve, and rounded to 3 — the first
/// value, which makes a unit a millimetre.
fn a_trait_and_an_arc_of_ellipse_rounded() -> History {
    let corner = LEVEL.at(0.8_f64.atan2(0.6));
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
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(CORNER),
        end: PointRef::New(DVec2::new(46.0, 54.0)),
        construction: false,
    });
    let [back, front] = [LEVEL.centre - LEVEL.first, LEVEL.centre + LEVEL.first];
    history.push(Operation::AddEllipse {
        sketch: 0,
        center: PointRef::New(LEVEL.centre),
        first: [PointRef::New(back), PointRef::New(front)],
        second: [
            PointRef::New(LEVEL.centre - LEVEL.across()),
            PointRef::New(LEVEL.centre + LEVEL.across()),
        ],
        construction: false,
        drawn: Some([PointRef::Existing(CORNER), PointRef::New(back)]),
    });
    history.push(Operation::Fillet {
        sketch: 0,
        corners: vec![Corner::Between(SegmentId(0).into(), EllipseId(0).into())],
        radius: 3.0.into(),
        construction: false,
    });
    history
}

/// The fillet of `radius` at `corner`, between the trait running from it to
/// `far` and the arc of ellipse `oval` leaving it counter-clockwise and
/// bulging towards it.
fn by_hand_beside_an_ellipse(corner: DVec2, far: DVec2, oval: Oval, radius: f64) -> ByHand {
    by_hand_by_an_ellipse(corner, far, oval, radius, 1.0, 1.0)
}

/// The same, the arc of ellipse leaving the corner `way` round — one for
/// counter-clockwise, minus one for clockwise — and the fillet standing `out`
/// of it: one outside the curve, minus one inside. An ellipse's offset has no
/// closed form: the hand walks the curve on from the corner, its centre
/// standing a radius off the curve, to where that centre first stands a
/// radius off the trait's line, on the side the curve leaves towards — then
/// halves the last step until the arithmetic gives no more.
fn by_hand_by_an_ellipse(
    corner: DVec2,
    far: DVec2,
    oval: Oval,
    radius: f64,
    way: f64,
    out: f64,
) -> ByHand {
    let along = (far - corner).normalize();
    let from = oval.turn_of(corner);
    let aside = along.perp() * along.perp_dot(oval.onwards(from) * way).signum();
    let centre_at = |turn: f64| oval.at(turn) + oval.outwards(turn) * (radius * out);
    let short = |turn: f64| (centre_at(turn) - corner).dot(aside) - radius;
    let (mut before, mut past) = (from, from);
    while short(past) < 0.0 {
        before = past;
        past += 1e-3 * way;
        assert!(
            (past - from).abs() < std::f64::consts::PI,
            "no fillet of {radius} stands beside the ellipse"
        );
    }
    for _ in 0..100 {
        let middle = (before + past) * 0.5;
        match short(middle) < 0.0 {
            true => before = middle,
            false => past = middle,
        }
    }
    let turn = (before + past) * 0.5;
    let centre = centre_at(turn);
    ByHand {
        centre,
        on_the_trait: corner + along * (centre - corner).dot(along),
        on_the_curve: oval.at(turn),
    }
}

/// The ellipse the fillet stands beside as it stands — the piece at the
/// corner, once it is trimmed — read off its centre and its axes.
fn the_oval(sketch: &Sketch) -> Oval {
    let CutSide::ArcOfEllipse(ellipse) = sketch.corner_cuts()[CUT.0].sides[1] else {
        panic!("the fillet stands beside an arc of ellipse");
    };
    let ellipse = sketch.ellipses()[ellipse.0];
    let half = |axis: SegmentId| {
        let (start, end) = sketch.endpoints(axis);
        (end - start) * 0.5
    };
    Oval {
        centre: sketch.point(ellipse.center),
        first: half(ellipse.first),
        second: half(ellipse.second).length(),
    }
}

/// The fillet of `radius` a hand works out beside the drawing's arc of
/// ellipse as it stands.
fn by_hand_beside_the_ellipse_of(sketch: &Sketch, radius: f64) -> ByHand {
    by_hand_beside_an_ellipse(
        sketch.point(CORNER),
        sketch.point(TRAIT_END),
        the_oval(sketch),
        radius,
    )
}

/// The point the next one laid will be.
fn next_point(history: &History) -> PointId {
    PointId(the_drawing(history).points().len())
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
        by_hand_on(&sketch, 5.0).on_the_curve,
        "the line started where the fillet touches the arc follows that touch",
    );
}

/// Where the corner of `a_trait_and_an_arc_of_ellipse_rounded` and the far
/// end of its trait stand while nothing moves them.
fn the_level_corner() -> (DVec2, DVec2) {
    (LEVEL.at(0.8_f64.atan2(0.6)), DVec2::new(46.0, 54.0))
}

#[test]
fn a_line_started_from_a_fillet_s_end_beside_an_arc_of_ellipse_follows_a_new_radius() {
    let mut history = a_trait_and_an_arc_of_ellipse_rounded();
    let on_the_trait = next_point(&history);
    a_line_from(&mut history, CutPoint::First, DVec2::new(60.0, 40.0));
    let on_the_curve = next_point(&history);
    a_line_from(&mut history, CutPoint::Second, DVec2::new(30.0, 50.0));

    rounded_to(&mut history, 5.0);

    let sketch = the_drawing(&history);
    let lines = &sketch.segments()[3..];
    assert_eq!(
        [lines[0].start, lines[1].start],
        [on_the_trait, on_the_curve]
    );
    let (corner, far) = the_level_corner();
    let fillet = by_hand_beside_an_ellipse(corner, far, LEVEL, 5.0);
    near_place(
        sketch.point(on_the_trait),
        fillet.on_the_trait,
        "the line started where the fillet touches the trait follows that touch",
    );
    near_place(
        sketch.point(on_the_curve),
        fillet.on_the_curve,
        "the line started where the fillet touches the arc of ellipse follows that touch",
    );
}

#[test]
fn a_circle_drawn_on_the_centre_of_a_fillet_beside_an_arc_of_ellipse_stays_on_it_when_the_corner_moves()
 {
    let mut history = a_trait_and_an_arc_of_ellipse_rounded();
    let hung = next_point(&history);
    let center = on_the_cut(&history, CutPoint::Centre);
    history.push(Operation::AddCircle {
        sketch: 0,
        center,
        radius: 1.0,
        rim: Vec::new(),
        construction: false,
    });
    let round_the_curve = LEVEL.at(40_f64.to_radians());

    history.push(Operation::MovePoint {
        sketch: 0,
        point: CORNER,
        position: round_the_curve,
        merged_into: None,
        on: Vec::new(),
        let_go: false,
    });

    let sketch = the_drawing(&history);
    assert_eq!(sketch.circles()[0].center, hung);
    near_place(
        sketch.point(CORNER),
        round_the_curve,
        "the corner slides round the ellipse to where it was taken",
    );
    let (_, far) = the_level_corner();
    let fillet = by_hand_beside_an_ellipse(round_the_curve, far, LEVEL, 3.0);
    near_place(
        sketch.point(hung),
        fillet.centre,
        "the circle's centre follows the fillet's centre",
    );
}

#[test]
fn a_dimension_from_a_point_of_a_fillet_beside_an_arc_of_ellipse_holds_when_the_value_changes() {
    let mut history = a_trait_and_an_arc_of_ellipse_rounded();
    let hung = next_point(&history);
    hang_a_point(&mut history, CutPoint::First);
    let (corner, far) = the_level_corner();
    let span = far.distance(by_hand_beside_an_ellipse(corner, far, LEVEL, 3.0).on_the_trait);
    history.push(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Distance {
            from: TRAIT_END,
            to: hung,
        },
        value: span.into(),
        placement: None,
    });

    rounded_to(&mut history, 5.0);

    let sketch = the_drawing(&history);
    near(
        sketch.point(TRAIT_END).distance(sketch.point(hung)),
        span,
        "the dimension from the trait's far end to the fillet's touch holds",
    );
    near_place(
        sketch.point(hung),
        by_hand_beside_the_ellipse_of(&sketch, 5.0).on_the_trait,
        "the point the dimension measures to stays where the fillet touches the trait",
    );
}

#[test]
fn a_coincidence_hangs_a_point_on_a_point_of_a_fillet_beside_an_arc_of_ellipse() {
    let mut history = a_trait_and_an_arc_of_ellipse_rounded();
    let loose = next_point(&history);
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(60.0, 60.0)),
        end: PointRef::New(DVec2::new(70.0, 70.0)),
        construction: false,
    });
    let picks = [
        RulePick::Element(Element::Point(loose)),
        RulePick::CutPoint(CUT, CutPoint::Centre),
    ];
    let Some(RuleIntent::Constrain(rule)) =
        rule_intent(Rule::Coincident, &picks, &the_drawing(&history))
    else {
        panic!("a point and the centre of a fillet beside an arc of ellipse make no coincidence");
    };
    history.push(Operation::Constrain {
        sketch: 0,
        constraint: rule,
    });
    let (corner, far) = the_level_corner();
    near_place(
        the_drawing(&history).point(loose),
        by_hand_beside_an_ellipse(corner, far, LEVEL, 3.0).centre,
        "the coincidence brings the point onto the fillet's centre",
    );

    rounded_to(&mut history, 5.0);

    near_place(
        the_drawing(&history).point(loose),
        by_hand_beside_an_ellipse(corner, far, LEVEL, 5.0).centre,
        "the point follows the fillet's centre",
    );
}

#[test]
fn a_point_placed_on_a_fillet_beside_an_arc_of_ellipse_follows_its_arc() {
    let mut history = a_trait_and_an_arc_of_ellipse_rounded();
    let (corner, far) = the_level_corner();
    let fillet = by_hand_beside_an_ellipse(corner, far, LEVEL, 3.0);
    let halfway = ((fillet.on_the_trait - fillet.centre).normalize()
        + (fillet.on_the_curve - fillet.centre).normalize())
    .normalize();
    let placed = next_point(&history);
    history.push(Operation::AddPoint {
        sketch: 0,
        position: fillet.centre + halfway * 3.0,
        on: vec![Support::Cut(CUT)],
    });

    rounded_to(&mut history, 5.0);

    near(
        the_drawing(&history)
            .point(placed)
            .distance(by_hand_beside_an_ellipse(corner, far, LEVEL, 5.0).centre),
        5.0,
        "the point placed on the fillet stands a radius from its centre",
    );
}

#[test]
fn a_line_started_from_a_fillet_s_end_on_an_arc_of_ellipse_follows_the_ellipse_reshaped() {
    let mut history = a_trait_and_an_arc_of_ellipse_rounded();
    let hung = next_point(&history);
    a_line_from(&mut history, CutPoint::Second, DVec2::new(30.0, 50.0));
    let height = the_drawing(&history).ellipses()[0].second;

    history.push(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(height),
        value: 12.0.into(),
        placement: None,
    });

    let sketch = the_drawing(&history);
    near(
        the_oval(&sketch).second,
        6.0,
        "the ellipse takes the height it was given",
    );
    near_place(
        sketch.point(hung),
        by_hand_beside_the_ellipse_of(&sketch, 3.0).on_the_curve,
        "the line started where the fillet touches the arc of ellipse follows that touch",
    );
}

/// Lays a short free line square to the middle of the fillet `by_hand` of
/// `radius` works out, on the corner's side and three and a half further
/// from its centre than the fillet — touching nothing — then holds it tangent
/// to the fillet's arc as « Tangence » lays it, the fillet's arc clicked
/// first: the line comes to the fillet, the corner where it was.
fn a_line_held_tangent(history: &mut History, fillet: &ByHand, radius: f64) -> SegmentId {
    let middle = ((fillet.on_the_trait - fillet.centre).normalize()
        + (fillet.on_the_curve - fillet.centre).normalize())
    .normalize();
    let foot = fillet.centre + middle * (radius + 3.5);
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(foot - middle.perp() * 5.0),
        end: PointRef::New(foot + middle.perp() * 5.0),
        construction: false,
    });
    let line = SegmentId(the_drawing(history).segments().len() - 1);
    let picks = [
        RulePick::CutCurve(CUT),
        RulePick::Element(Element::Segment(line)),
    ];
    let Some(RuleIntent::Constrain(rule)) =
        rule_intent(Rule::Tangent, &picks, &the_drawing(history))
    else {
        panic!("the tangent rule between a line and a fillet beside a curve lays nothing");
    };
    history.push(Operation::Constrain {
        sketch: 0,
        constraint: rule,
    });
    let laid = the_drawing(history);
    assert_tangent(
        &laid,
        line,
        fillet.centre,
        radius,
        "the line just held tangent",
    );
    line
}

/// Holds a line, taken whole, a radius from a fillet's centre.
fn assert_tangent(sketch: &Sketch, line: SegmentId, centre: DVec2, radius: f64, what: &str) {
    let (start, end) = sketch.endpoints(line);
    near(
        (end - start).normalize().perp_dot(centre - start).abs(),
        radius,
        &format!("{what}: how far the line runs from the fillet's centre"),
    );
}

#[test]
fn a_line_held_tangent_to_a_fillet_beside_an_arc_follows_its_corner_moved() {
    let mut history = a_trait_and_an_arc_rounded();
    let fillet = by_hand_on(&the_drawing(&history), 5.0);
    let line = a_line_held_tangent(&mut history, &fillet, 5.0);
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
    near_place(
        sketch.point(CORNER),
        round_the_arc,
        "the corner slides round the arc to where it was taken",
    );
    assert_tangent(
        &sketch,
        line,
        by_hand_on(&sketch, 5.0).centre,
        5.0,
        "the corner moved",
    );
}

#[test]
fn a_line_held_tangent_to_a_fillet_beside_an_arc_follows_a_new_radius() {
    let mut history = a_trait_and_an_arc_rounded();
    let fillet = by_hand_on(&the_drawing(&history), 5.0);
    let line = a_line_held_tangent(&mut history, &fillet, 5.0);

    rounded_to(&mut history, 12.0);

    let sketch = the_drawing(&history);
    assert_tangent(
        &sketch,
        line,
        by_hand_on(&sketch, 12.0).centre,
        12.0,
        "rounded to 12",
    );
}

#[test]
fn a_line_held_tangent_to_a_fillet_beside_an_arc_follows_the_arc_given_a_new_radius() {
    let mut history = a_trait_and_an_arc_rounded();
    let fillet = by_hand_on(&the_drawing(&history), 5.0);
    let line = a_line_held_tangent(&mut history, &fillet, 5.0);

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
    assert_tangent(
        &sketch,
        line,
        by_hand_on(&sketch, 5.0).centre,
        5.0,
        "the arc given a radius of 10",
    );
}

#[test]
fn a_line_held_tangent_to_a_fillet_beside_an_arc_of_ellipse_follows_its_corner_moved() {
    let mut history = a_trait_and_an_arc_of_ellipse_rounded();
    let fillet = by_hand_beside_the_ellipse_of(&the_drawing(&history), 3.0);
    let line = a_line_held_tangent(&mut history, &fillet, 3.0);
    let round_the_curve = LEVEL.at(40_f64.to_radians());

    history.push(Operation::MovePoint {
        sketch: 0,
        point: CORNER,
        position: round_the_curve,
        merged_into: None,
        on: Vec::new(),
        let_go: false,
    });

    let sketch = the_drawing(&history);
    near_place(
        sketch.point(CORNER),
        round_the_curve,
        "the corner slides round the ellipse to where it was taken",
    );
    assert_tangent(
        &sketch,
        line,
        by_hand_beside_the_ellipse_of(&sketch, 3.0).centre,
        3.0,
        "the corner moved",
    );
}

#[test]
fn a_line_held_tangent_to_a_fillet_beside_an_arc_of_ellipse_follows_a_new_radius() {
    let mut history = a_trait_and_an_arc_of_ellipse_rounded();
    let fillet = by_hand_beside_the_ellipse_of(&the_drawing(&history), 3.0);
    let line = a_line_held_tangent(&mut history, &fillet, 3.0);

    rounded_to(&mut history, 5.0);

    let sketch = the_drawing(&history);
    assert_tangent(
        &sketch,
        line,
        by_hand_beside_the_ellipse_of(&sketch, 5.0).centre,
        5.0,
        "rounded to 5",
    );
}

#[test]
fn a_line_held_tangent_to_a_fillet_beside_an_arc_of_ellipse_follows_the_ellipse_reshaped() {
    let mut history = a_trait_and_an_arc_of_ellipse_rounded();
    let fillet = by_hand_beside_the_ellipse_of(&the_drawing(&history), 3.0);
    let line = a_line_held_tangent(&mut history, &fillet, 3.0);
    let height = the_drawing(&history).ellipses()[0].second;

    history.push(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(height),
        value: 12.0.into(),
        placement: None,
    });

    let sketch = the_drawing(&history);
    near(
        the_oval(&sketch).second,
        6.0,
        "the ellipse takes the height it was given",
    );
    assert_tangent(
        &sketch,
        line,
        by_hand_beside_the_ellipse_of(&sketch, 3.0).centre,
        3.0,
        "the ellipse given a height of 12",
    );
}

/// A corner at (10, 10) between a trait running east to (50, 10) and an arc
/// of radius 5 round (14, 13), hollow towards the corner, running
/// counter-clockwise from (9, 13) round to the corner: at the corner it
/// leaves three fifths west and four fifths north. The corner is named by its
/// trait then its arc, and rounded to 7/4 — the first value, which makes a
/// unit a millimetre.
///
/// A fillet of `r` there stands `r` above the trait and `5 − r` from the
/// arc's centre, inside its circle: round (14 − √(16 − 4r), 10 + r).
/// - Rounded to 7/4, it fits: round (11, 11.75), from (11, 10) on the trait to
///   (14 − 60/13, 13 − 25/13) on the arc.
/// - Rounded to 15/4, it still has a place, round (13, 13.75), from (13, 10)
///   to (10, 16) — past the arc's end at (9, 13), which it overruns: red.
/// - Rounded to 6, it has none: a fillet of 6 cannot stand inside a circle of
///   5. The stand-in #621 draws is the fillet of the tangents alone, which
///   open with a cosine of −3/5: it touches each three from the corner, at
///   (13, 10) and (8.2, 12.4), round (13, 16).
fn a_trait_and_a_hollow_arc_rounded() -> History {
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
        center: PointRef::New(DVec2::new(14.0, 13.0)),
        start: PointRef::New(DVec2::new(9.0, 13.0)),
        end: PointRef::Existing(CORNER),
        construction: false,
    });
    history.push(Operation::Fillet {
        sketch: 0,
        corners: vec![Corner::Between(SegmentId(0).into(), ARC.into())],
        radius: 1.75.into(),
        construction: false,
    });
    history
}

/// The fillet's centre, then where it touches the trait, then the arc, as
/// `a_trait_and_a_hollow_arc_rounded` works them out.
const FITTING: [DVec2; 3] = [
    DVec2::new(11.0, 11.75),
    DVec2::new(11.0, 10.0),
    DVec2::new(14.0 - 60.0 / 13.0, 13.0 - 25.0 / 13.0),
];
const OVERRUNNING: [DVec2; 3] = [
    DVec2::new(13.0, 13.75),
    DVec2::new(13.0, 10.0),
    DVec2::new(10.0, 16.0),
];
const STANDING_IN: [DVec2; 3] = [
    DVec2::new(13.0, 16.0),
    DVec2::new(13.0, 10.0),
    DVec2::new(8.2, 12.4),
];

/// A circle on the fillet's centre and a line from each of its touches: what
/// hangs on all three of its points, said by the point laid on each.
fn hang_on_all_three(history: &mut History) -> [PointId; 3] {
    let centre = next_point(history);
    let center = on_the_cut(history, CutPoint::Centre);
    history.push(Operation::AddCircle {
        sketch: 0,
        center,
        radius: 0.5,
        rim: Vec::new(),
        construction: false,
    });
    let on_the_trait = next_point(history);
    a_line_from(history, CutPoint::First, DVec2::new(11.0, -10.0));
    let on_the_curve = next_point(history);
    a_line_from(history, CutPoint::Second, DVec2::new(-10.0, 11.0));
    [centre, on_the_trait, on_the_curve]
}

fn assert_hung_at(sketch: &Sketch, hung: [PointId; 3], places: [DVec2; 3], what: &str) {
    for ((point, place), on) in hung.into_iter().zip(places).zip([
        "its centre",
        "its touch on the trait",
        "its touch on the arc",
    ]) {
        near_place(
            sketch.point(point),
            place,
            &format!("{what}: what hangs on {on}"),
        );
    }
}

fn assert_red(sketch: &Sketch, red: bool, what: &str) {
    let drawn = sketch.drawn_cut(CUT).expect("the fillet is drawn");
    assert_eq!(!drawn.fits, red, "{what}: {drawn:?}");
}

/// Holds the corner of `a_trait_and_a_hollow_arc_rounded` and what its sides
/// are drawn from where they stood `before`. The far end of a line hung on
/// the fillet is not among them: the solver keeps a line its length as its
/// hung end moves, between two traits as beside a curve.
fn assert_where_it_stood(sketch: &Sketch, before: &Sketch, what: &str) {
    for (point, which) in [
        (CORNER, "the corner"),
        (TRAIT_END, "the trait's far end"),
        (PointId(3), "the arc's centre"),
        (PointId(4), "the arc's far end"),
    ] {
        near_place(
            sketch.point(point),
            before.point(point),
            &format!("{what}: {which} is where it stood"),
        );
    }
}

#[test]
fn a_fillet_beside_a_curve_that_overruns_its_side_keeps_what_hangs_on_it_on_the_red_fillet() {
    let mut history = a_trait_and_a_hollow_arc_rounded();
    let hung = hang_on_all_three(&mut history);
    let before = the_drawing(&history);
    assert_hung_at(&before, hung, FITTING, "the fillet fitting");

    rounded_to(&mut history, 3.75);

    let sketch = the_drawing(&history);
    assert_red(&sketch, true, "the fillet overruns the arc");
    assert_hung_at(
        &sketch,
        hung,
        OVERRUNNING,
        "the fillet red, overrunning the arc",
    );

    rounded_to(&mut history, 1.75);

    let sketch = the_drawing(&history);
    assert_red(&sketch, false, "the fillet fits again");
    assert_hung_at(&sketch, hung, FITTING, "the fillet fitting again");
    assert_where_it_stood(&sketch, &before, "the fillet fitting again");
}

#[test]
fn what_hangs_on_a_fillet_with_no_place_in_the_hollow_of_an_arc_follows_its_stand_in_and_comes_back()
 {
    let mut history = a_trait_and_a_hollow_arc_rounded();
    let hung = hang_on_all_three(&mut history);
    let before = the_drawing(&history);
    assert_hung_at(&before, hung, FITTING, "the fillet fitting");

    rounded_to(&mut history, 6.0);

    let sketch = the_drawing(&history);
    assert_red(&sketch, true, "a fillet of 6 in the hollow of an arc of 5");
    assert_hung_at(
        &sketch,
        hung,
        STANDING_IN,
        "the fillet with no place, drawn where the tangents alone put it",
    );

    rounded_to(&mut history, 1.75);

    let sketch = the_drawing(&history);
    assert_red(&sketch, false, "the fillet finds a place again");
    assert_hung_at(&sketch, hung, FITTING, "the fillet back in its place");
    assert_where_it_stood(&sketch, &before, "the fillet back in its place");
}

/// The corners of `a_trait_and_an_arc_rounded` and of
/// `a_trait_and_an_arc_of_ellipse_rounded`, each with the fillet a hand works
/// out on it, its radius, and the rule a point on its curve is held by.
fn corners_rounded_beside_a_curve() -> [(History, ByHand, f64, Support, &'static str); 2] {
    let beside_an_arc = a_trait_and_an_arc_rounded();
    let beside_an_ellipse = a_trait_and_an_arc_of_ellipse_rounded();
    let (corner, far) = the_level_corner();
    [
        (
            beside_an_arc,
            by_hand(
                DVec2::new(10.0, 10.0),
                DVec2::new(50.0, 10.0),
                ROUND,
                REACH,
                5.0,
            ),
            5.0,
            Support::Arc(ARC),
            "beside an arc",
        ),
        (
            beside_an_ellipse,
            by_hand_beside_an_ellipse(corner, far, LEVEL, 3.0),
            3.0,
            Support::Ellipse(EllipseId(0)),
            "beside an arc of ellipse",
        ),
    ]
}

#[test]
fn taking_a_fillet_beside_a_curve_off_leaves_its_centre_free_and_its_ends_on_their_sides() {
    for (mut history, fillet, _, on_the_curve, what) in corners_rounded_beside_a_curve() {
        let [centre, on_the_trait, at_the_curve] = hang_on_all_three(&mut history);

        history.push(Operation::CutChange(CutChange::TakeOff {
            sketch: 0,
            cuts: vec![CUT],
        }));

        let sketch = the_drawing(&history);
        assert_eq!(
            sketch.live_corner_cuts().count(),
            0,
            "{what}: the fillet is off"
        );
        assert_hung_at(
            &sketch,
            [centre, on_the_trait, at_the_curve],
            [fillet.centre, fillet.on_the_trait, fillet.on_the_curve],
            &format!("{what}, the fillet taken off: each point stays where it was, and"),
        );
        assert_eq!(
            sketch.holds_on(centre),
            Vec::new(),
            "{what}: the centre becomes a plain point"
        );
        assert_eq!(
            sketch.holds_on(on_the_trait),
            vec![Support::Segment(SegmentId(0))],
            "{what}: the end on the trait stays held on the trait"
        );
        assert_eq!(
            sketch.holds_on(at_the_curve),
            vec![on_the_curve],
            "{what}: the end on the curve stays held on the curve"
        );
        assert_eq!(sketch.live_circles().count(), 1, "{what}: the circle stays");
    }
}

#[test]
fn a_press_on_a_point_hung_on_a_fillet_beside_a_curve_drags_its_corner() {
    for (mut history, fillet, _, _, what) in corners_rounded_beside_a_curve() {
        hang_on_all_three(&mut history);
        let sketch = the_drawing(&history);

        for (place, on) in [
            (fillet.centre, "its centre"),
            (fillet.on_the_trait, "its touch on the trait"),
            (fillet.on_the_curve, "its touch on the curve"),
        ] {
            assert_eq!(
                sketch.point_to_drag_at(place, 0.5),
                Some(Grip {
                    point: CORNER,
                    from_afar: true
                }),
                "{what}: a press on the point hung on {on} takes the fillet's corner"
            );
        }
    }
}

/// What a hand works out of a fillet, in the order `hang_on_all_three` hangs
/// on it: its centre, its touch on the trait, its touch on the curve.
fn places_of(fillet: &ByHand) -> [DVec2; 3] {
    [fillet.centre, fillet.on_the_trait, fillet.on_the_curve]
}

/// Holds what `hang_on_all_three` hung where the fillet `fillet` stands, each
/// point still hung on its place.
fn assert_still_hung(sketch: &Sketch, hung: [PointId; 3], fillet: &ByHand, what: &str) {
    assert_hung_at(sketch, hung, places_of(fillet), what);
    for (point, at) in hung
        .into_iter()
        .zip([CutPoint::Centre, CutPoint::First, CutPoint::Second])
    {
        assert_eq!(
            sketch.holds_on(point),
            vec![Support::CutPoint(CUT, at)],
            "{what}: the point at the fillet's {at:?} is still hung there"
        );
    }
}

#[test]
fn what_hangs_on_a_fillet_beside_an_arc_stays_hung_once_the_arc_is_divided() {
    let mut history = a_trait_and_an_arc_rounded();
    let hung = hang_on_all_three(&mut history);

    history.push(Operation::Split {
        sketch: 0,
        segments: Vec::new(),
        arcs: vec![ARC],
        at: ROUND + DVec2::from_angle(60_f64.to_radians()) * REACH,
    });

    let sketch = the_drawing(&history);
    assert_eq!(sketch.live_arcs().count(), 2, "the arc is divided in two");
    assert_still_hung(
        &sketch,
        hung,
        &by_hand_on(&sketch, 5.0),
        "the arc divided past the fillet",
    );
    rounded_to(&mut history, 12.0);
    let sketch = the_drawing(&history);
    assert_still_hung(
        &sketch,
        hung,
        &by_hand_on(&sketch, 12.0),
        "the arc divided, then the fillet rounded to 12",
    );
}

#[test]
fn what_hangs_on_a_fillet_beside_an_arc_stays_hung_once_the_arc_is_trimmed() {
    let mut history = a_trait_and_an_arc_rounded();
    let sixty = next_point(&history);
    history.push(Operation::AddPoint {
        sketch: 0,
        position: ROUND + DVec2::from_angle(60_f64.to_radians()) * REACH,
        on: Vec::new(),
    });
    let hung = hang_on_all_three(&mut history);

    history.push(Operation::TrimArc {
        sketch: 0,
        arc: ARC,
        from: sixty,
        to: PointId(4),
    });

    let sketch = the_drawing(&history);
    let CutSide::Arc(piece) = sketch.corner_cuts()[CUT.0].sides[1] else {
        panic!("the fillet still stands beside an arc");
    };
    let piece = sketch.arc(piece);
    assert_eq!(
        [piece.start, piece.end],
        [CORNER, sixty],
        "the stretch away from the corner is trimmed: {piece:?}"
    );
    assert_still_hung(
        &sketch,
        hung,
        &by_hand_on(&sketch, 5.0),
        "the arc trimmed past the fillet",
    );
    rounded_to(&mut history, 12.0);
    let sketch = the_drawing(&history);
    assert_still_hung(
        &sketch,
        hung,
        &by_hand_on(&sketch, 12.0),
        "the arc trimmed, then the fillet rounded to 12",
    );
}

#[test]
fn what_hangs_on_a_fillet_beside_an_arc_of_ellipse_stays_hung_once_it_is_trimmed() {
    let mut history = a_trait_and_an_arc_of_ellipse_rounded();
    let mut on_the_curve = Vec::new();
    for degrees in [120.0_f64, 150.0] {
        on_the_curve.push(next_point(&history));
        history.push(Operation::AddPoint {
            sketch: 0,
            position: LEVEL.at(degrees.to_radians()),
            on: Vec::new(),
        });
    }
    let hung = hang_on_all_three(&mut history);

    history.push(Operation::TrimEllipse {
        sketch: 0,
        ellipse: EllipseId(0),
        between: Some((on_the_curve[0], on_the_curve[1])),
    });

    let sketch = the_drawing(&history);
    assert_eq!(
        sketch.live_ellipses().count(),
        2,
        "a stretch is trimmed from the middle of the arc of ellipse"
    );
    assert_still_hung(
        &sketch,
        hung,
        &by_hand_beside_the_ellipse_of(&sketch, 3.0),
        "the arc of ellipse trimmed past the fillet",
    );
    rounded_to(&mut history, 5.0);
    let sketch = the_drawing(&history);
    assert_still_hung(
        &sketch,
        hung,
        &by_hand_beside_the_ellipse_of(&sketch, 5.0),
        "the arc of ellipse trimmed, then the fillet rounded to 5",
    );
}

#[test]
fn taking_off_a_fillet_with_no_place_lets_the_end_on_its_stand_in_go_where_it_stands() {
    let mut history = a_trait_and_a_hollow_arc_rounded();
    let [centre, on_the_trait, on_the_curve] = hang_on_all_three(&mut history);
    rounded_to(&mut history, 6.0);
    let red = the_drawing(&history);
    assert_hung_at(
        &red,
        [centre, on_the_trait, on_the_curve],
        STANDING_IN,
        "the fillet with no place",
    );

    history.push(Operation::CutChange(CutChange::TakeOff {
        sketch: 0,
        cuts: vec![CUT],
    }));

    let sketch = the_drawing(&history);
    assert_eq!(sketch.live_corner_cuts().count(), 0, "the fillet is off");
    for (rank, (is, was)) in sketch.points().iter().zip(red.points()).enumerate() {
        near_place(
            *is,
            *was,
            &format!("the fillet with no place taken off, point {rank} does not move"),
        );
    }
    assert_eq!(
        sketch.holds_on(on_the_curve),
        Vec::new(),
        "the end on the stand-in's touch, off the arc, is let go: a plain point"
    );
    assert_eq!(
        sketch.holds_on(centre),
        Vec::new(),
        "the centre is let go: a plain point"
    );
    assert_eq!(
        sketch.holds_on(on_the_trait),
        vec![Support::Segment(SegmentId(0))],
        "the stand-in's touch on the trait lies on it, and the end there stays held on it"
    );
}

#[test]
fn taking_off_a_fillet_with_no_place_moves_nothing_though_the_line_on_its_stand_in_is_fixed() {
    let mut history = a_trait_and_a_hollow_arc_rounded();
    hang_on_all_three(&mut history);
    rounded_to(&mut history, 6.0);
    let line_on_the_curve = SegmentId(the_drawing(&history).segments().len() - 1);
    history.push(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Fixed {
            element: Element::Segment(line_on_the_curve),
        },
    });
    let red = the_drawing(&history);

    history.push(Operation::CutChange(CutChange::TakeOff {
        sketch: 0,
        cuts: vec![CUT],
    }));

    let sketch = the_drawing(&history);
    for (point, which) in [
        (CORNER, "the corner"),
        (TRAIT_END, "the trait's far end"),
        (PointId(3), "the arc's centre"),
        (PointId(4), "the arc's far end"),
    ] {
        near_place(
            sketch.point(point),
            red.point(point),
            &format!("the fillet with no place taken off, {which} does not move"),
        );
    }
    near(sketch.arc_radius(ARC), 5.0, "the arc keeps its radius");
}

#[test]
fn a_line_held_tangent_to_a_red_fillet_beside_an_arc_stays_tangent_to_it_as_drawn() {
    let mut history = a_trait_and_a_hollow_arc_rounded();
    let fitting = by_hand_in_the_hollow_on(&the_drawing(&history), 1.75);
    let line = a_line_held_tangent(&mut history, &fitting, 1.75);

    for (radius, red, what) in [
        (3.75, true, "red, overrunning the arc"),
        (6.0, true, "red with no place, on its stand-in"),
        (1.75, false, "fitting again"),
    ] {
        rounded_to(&mut history, radius);
        let sketch = the_drawing(&history);
        assert_red(&sketch, red, what);
        assert_tangent(
            &sketch,
            line,
            by_hand_in_the_hollow_on(&sketch, radius).centre,
            radius,
            what,
        );
    }
}

/// The fillet of `radius` a hand works out beside the hollow arc of
/// `a_trait_and_a_hollow_arc_rounded`, from where its corner, its trait and
/// its arc stand now — a line held tangent lets them give: inside the arc's
/// circle, the arc leaving the corner clockwise, or where that has no place,
/// the stand-in of the tangents alone.
fn by_hand_in_the_hollow_on(sketch: &Sketch, radius: f64) -> ByHand {
    let CutSide::Arc(arc) = sketch.corner_cuts()[CUT.0].sides[1] else {
        panic!("the fillet stands beside an arc");
    };
    let (corner, far) = (sketch.point(CORNER), sketch.point(TRAIT_END));
    let round = sketch.point(sketch.arc(arc).center);
    by_hand_by_an_arc(
        corner,
        far,
        round,
        sketch.arc_radius(arc),
        radius,
        -1.0,
        -1.0,
    )
    .unwrap_or_else(|| {
        let leaving = [
            (far - corner).normalize(),
            -(corner - round).perp().normalize(),
        ];
        the_tangents_fillet(corner, leaving, radius)
    })
}

/// A corner on the ellipse `LEVEL` at the turn of 30°, between a trait
/// running west into the ellipse to (35, 32.5) and the arc of ellipse drawn
/// counter-clockwise from the ellipse's bottom, (40, 25), up to that corner:
/// it leaves the corner clockwise, and is hollow towards it. Named by its
/// trait then its curve, and rounded to 1, which fits; a fillet of 6 has no
/// place inside an ellipse five high.
fn a_trait_inside_an_arc_of_ellipse_rounded() -> History {
    let corner = LEVEL.at(30_f64.to_radians());
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
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(CORNER),
        end: PointRef::New(DVec2::new(35.0, corner.y)),
        construction: false,
    });
    let bottom = PointRef::New(LEVEL.centre - LEVEL.across());
    history.push(Operation::AddEllipse {
        sketch: 0,
        center: PointRef::New(LEVEL.centre),
        first: [
            PointRef::New(LEVEL.centre - LEVEL.first),
            PointRef::New(LEVEL.centre + LEVEL.first),
        ],
        second: [bottom.clone(), PointRef::New(LEVEL.centre + LEVEL.across())],
        construction: false,
        drawn: Some([bottom, PointRef::Existing(CORNER)]),
    });
    history.push(Operation::Fillet {
        sketch: 0,
        corners: vec![Corner::Between(SegmentId(0).into(), EllipseId(0).into())],
        radius: 1.0.into(),
        construction: false,
    });
    history
}

/// The fillet of `radius` the sides' tangents alone give at `corner`, the
/// sides leaving it along `leaving`, trait first: the stand-in #621 draws for
/// a fillet with no place. It touches each tangent the radius over the
/// tangent of half the opening from the corner.
fn the_tangents_fillet(corner: DVec2, leaving: [DVec2; 2], radius: f64) -> ByHand {
    let cos = leaving[0].dot(leaving[1]);
    let sin = leaving[0].perp_dot(leaving[1]).abs();
    let back = radius * (1.0 + cos) / sin;
    ByHand {
        centre: corner + (leaving[0] + leaving[1]) * (radius / sin),
        on_the_trait: corner + leaving[0] * back,
        on_the_curve: corner + leaving[1] * back,
    }
}

#[test]
fn what_hangs_on_a_fillet_with_no_place_inside_an_arc_of_ellipse_follows_its_stand_in_and_comes_back()
 {
    let turn = 30_f64.to_radians();
    let corner = LEVEL.at(turn);
    let far = DVec2::new(35.0, corner.y);
    let fitting = by_hand_by_an_ellipse(corner, far, LEVEL, 1.0, -1.0, -1.0);
    let stand_in = the_tangents_fillet(corner, [-DVec2::X, -LEVEL.onwards(turn)], 6.0);
    let mut history = a_trait_inside_an_arc_of_ellipse_rounded();
    let hung = hang_on_all_three(&mut history);
    assert_hung_at(
        &the_drawing(&history),
        hung,
        places_of(&fitting),
        "the fillet fitting",
    );

    rounded_to(&mut history, 6.0);

    let sketch = the_drawing(&history);
    assert_red(&sketch, true, "a fillet of 6 inside an ellipse five high");
    assert_hung_at(
        &sketch,
        hung,
        places_of(&stand_in),
        "the fillet with no place, drawn where the tangents alone put it",
    );

    rounded_to(&mut history, 1.0);

    let sketch = the_drawing(&history);
    assert_red(&sketch, false, "the fillet finds a place again");
    assert_hung_at(
        &sketch,
        hung,
        places_of(&fitting),
        "the fillet back in its place",
    );
}

#[test]
fn a_trim_past_a_fillet_counts_what_it_drops_and_not_what_hangs_on_the_fillet() {
    let beyond_on_the_arc = |degrees: f64| ROUND + DVec2::from_angle(degrees.to_radians()) * REACH;
    for (at, side, held_beyond, cut_from, what) in [
        (
            CutPoint::First,
            Support::Segment(SegmentId(0)),
            DVec2::new(45.0, 10.0),
            DVec2::new(40.0, 10.0),
            "the trait trimmed past the fillet",
        ),
        (
            CutPoint::Second,
            Support::Arc(ARC),
            beyond_on_the_arc(75.0),
            beyond_on_the_arc(60.0),
            "the arc trimmed past the fillet",
        ),
    ] {
        let mut history = a_trait_and_an_arc_rounded();
        let hung = next_point(&history);
        hang_a_point(&mut history, at);
        let held = next_point(&history);
        history.push(Operation::AddPoint {
            sketch: 0,
            position: held_beyond,
            on: vec![side],
        });
        let from = next_point(&history);
        history.push(Operation::AddPoint {
            sketch: 0,
            position: cut_from,
            on: Vec::new(),
        });
        let mut state = PartState::rebuild(&history);

        let outcome = state.apply(&match side {
            Support::Arc(arc) => Operation::TrimArc {
                sketch: 0,
                arc,
                from,
                to: PointId(4),
            },
            _ => Operation::Trim {
                sketch: 0,
                segment: SegmentId(0),
                from,
                to: TRAIT_END,
            },
        });

        let sketch = &state.sketches[0];
        assert_eq!(
            sketch.holds_on(hung),
            vec![Support::CutPoint(CUT, at)],
            "{what}: the point hung on the fillet is still hung there"
        );
        assert_eq!(
            sketch.holds_on(held),
            Vec::new(),
            "{what}: the point held on the stretch that went is let go"
        );
        assert_eq!(
            outcome,
            Some(Outcome::Cut {
                rules: 1,
                values: 0,
                refused: 0
            }),
            "{what}: the trim lost the rule holding the point on the stretch that went, and \
             nothing that hangs on the fillet"
        );
    }
}

#[test]
fn trimming_the_curve_beside_a_fillet_from_its_corner_takes_the_fillet_and_lets_go_of_what_hung_on_it()
 {
    let arc_from_its_corner = {
        let mut history = a_trait_and_an_arc_rounded();
        let sixty = next_point(&history);
        history.push(Operation::AddPoint {
            sketch: 0,
            position: ROUND + DVec2::from_angle(60_f64.to_radians()) * REACH,
            on: Vec::new(),
        });
        let hung = hang_on_all_three(&mut history);
        let trim = Operation::TrimArc {
            sketch: 0,
            arc: ARC,
            from: CORNER,
            to: sixty,
        };
        (history, hung, trim, "the arc trimmed from its corner")
    };
    let ellipse_from_its_corner = {
        let mut history = a_trait_and_an_arc_of_ellipse_rounded();
        let further = next_point(&history);
        history.push(Operation::AddPoint {
            sketch: 0,
            position: LEVEL.at(120_f64.to_radians()),
            on: Vec::new(),
        });
        let hung = hang_on_all_three(&mut history);
        let trim = Operation::TrimEllipse {
            sketch: 0,
            ellipse: EllipseId(0),
            between: Some((CORNER, further)),
        };
        (
            history,
            hung,
            trim,
            "the arc of ellipse trimmed from its corner",
        )
    };

    for (history, hung, trim, what) in [arc_from_its_corner, ellipse_from_its_corner] {
        let before = the_drawing(&history);
        let mut state = PartState::rebuild(&history);

        let outcome = state.apply(&trim);

        let sketch = &state.sketches[0];
        assert_eq!(
            sketch.live_corner_cuts().count(),
            0,
            "{what}: the corner went with the stretch, and the fillet with it"
        );
        assert!(
            !sketch.anything_hangs_on_a_cut(),
            "{what}: a rule still names the fillet gone: {:?}",
            sketch.constraints()
        );
        for point in hung {
            assert!(
                sketch.holds_on(point).iter().all(|on| !on.names_a_cut()),
                "{what}: {point:?} is still held on the fillet gone: {:?}",
                sketch.holds_on(point)
            );
            near_place(
                sketch.point(point),
                before.point(point),
                &format!("{what}: {point:?} is let go where it stood"),
            );
        }
        assert_eq!(
            outcome,
            Some(Outcome::Cut {
                rules: 3,
                values: 0,
                refused: 0
            }),
            "{what}: the three rules that hung on the fillet are lost to the trim"
        );
    }
}

/// A corner standing on its own at (10, 10): one trait right to (60, 10), one
/// up to (10, 50), rounded to 5 — #620's corner, between two traits.
fn two_traits_rounded() -> History {
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
    for end in [DVec2::new(60.0, 10.0), DVec2::new(10.0, 50.0)] {
        history.push(Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(CORNER),
            end: PointRef::New(end),
            construction: false,
        });
    }
    history.push(Operation::Fillet {
        sketch: 0,
        corners: vec![Corner::Between(SegmentId(0).into(), SegmentId(1).into())],
        radius: 5.0.into(),
        construction: false,
    });
    history
}

/// The corner of `a_trait_and_an_arc_of_ellipse_rounded`, its arc of ellipse
/// drawn only up to the turn of 70°, and rounded to 1, which fits: a fillet
/// of 3 there touches the ellipse a little short of 75°, past what is drawn.
fn a_trait_and_a_short_arc_of_ellipse_rounded() -> History {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddPoint {
        sketch: 0,
        position: LEVEL.at(0.8_f64.atan2(0.6)),
        on: Vec::new(),
    });
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(CORNER),
        end: PointRef::New(DVec2::new(46.0, 54.0)),
        construction: false,
    });
    history.push(Operation::AddEllipse {
        sketch: 0,
        center: PointRef::New(LEVEL.centre),
        first: [
            PointRef::New(LEVEL.centre - LEVEL.first),
            PointRef::New(LEVEL.centre + LEVEL.first),
        ],
        second: [
            PointRef::New(LEVEL.centre - LEVEL.across()),
            PointRef::New(LEVEL.centre + LEVEL.across()),
        ],
        construction: false,
        drawn: Some([
            PointRef::Existing(CORNER),
            PointRef::New(LEVEL.at(70_f64.to_radians())),
        ]),
    });
    history.push(Operation::Fillet {
        sketch: 0,
        corners: vec![Corner::Between(SegmentId(0).into(), EllipseId(0).into())],
        radius: 1.0.into(),
        construction: false,
    });
    history
}

#[test]
fn taking_off_a_red_fillet_that_overruns_its_side_keeps_the_end_past_it_held_on_its_side() {
    for (mut history, overrun, sides, what) in [
        (
            two_traits_rounded(),
            70.0,
            [
                Support::Segment(SegmentId(0)),
                Support::Segment(SegmentId(1)),
            ],
            "between two traits, past the end of each",
        ),
        (
            a_trait_and_a_hollow_arc_rounded(),
            3.75,
            [Support::Segment(SegmentId(0)), Support::Arc(ARC)],
            "beside an arc, past its end on its circle",
        ),
        (
            a_trait_and_a_short_arc_of_ellipse_rounded(),
            3.0,
            [
                Support::Segment(SegmentId(0)),
                Support::Ellipse(EllipseId(0)),
            ],
            "beside an arc of ellipse, past what is drawn of it",
        ),
    ] {
        let [centre, first, second] = hang_on_all_three(&mut history);
        rounded_to(&mut history, overrun);
        let red = the_drawing(&history);
        assert_red(&red, true, what);

        history.push(Operation::CutChange(CutChange::TakeOff {
            sketch: 0,
            cuts: vec![CUT],
        }));

        let sketch = the_drawing(&history);
        assert_eq!(sketch.holds_on(centre), Vec::new(), "{what}: the centre");
        assert_eq!(
            sketch.holds_on(first),
            vec![sides[0]],
            "{what}: the end on the first side stays held on it"
        );
        assert_eq!(
            sketch.holds_on(second),
            vec![sides[1]],
            "{what}: the end on the second side stays held on it"
        );
        for (rank, (is, was)) in sketch.points().iter().zip(red.points()).enumerate() {
            near_place(
                *is,
                *was,
                &format!("{what}: the fillet taken off, point {rank} does not move"),
            );
        }
    }
}
