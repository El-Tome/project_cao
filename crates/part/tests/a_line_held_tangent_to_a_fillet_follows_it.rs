//! « Tangence » links a line and a fillet's arc as it links a line and any arc:
//! the line is held on the whole circle the fillet is a piece of, its distance
//! to the fillet's centre being the radius. The centre is worked out again from
//! the corner, its sides and the value each time the drawing settles, so the
//! line follows them, and pulled, moves them.
//!
//! Closes #651.
//! - « Tangence » with a line and a fillet's arc, in either order, makes the
//!   line tangent to the fillet —
//!   `a_line_and_a_fillet_clicked_in_either_order_are_made_tangent`, the one
//!   clicked first staying where it is —
//!   `a_tangent_with_a_fillet_leaves_the_first_clicked_where_it_was`; the tool
//!   pointing at the fillet's arc is `cao_app`'s, held by
//!   `the_tangent_tool_points_at_a_fillet_s_arc_rather_than_the_side_under_it`
//! - The line stays tangent when the corner, its sides or the radius change,
//!   even once the touch is outside the drawn arc —
//!   `a_line_tangent_to_a_fillet_follows_a_new_radius`,
//!   `a_line_tangent_to_a_fillet_follows_its_corner_moved`,
//!   `a_line_tangent_to_a_fillet_follows_its_side_turned`,
//!   `a_line_tangent_to_a_fillet_stays_on_its_circle_once_the_touch_is_past_the_arc`
//! - Pulling the tangent line moves the corner or the radius as the rules
//!   allow — `pulling_a_line_tangent_to_a_fillet_moves_its_corner`
//! - « Tangence » between a line and a chamfer's cut lays nothing —
//!   `a_tangent_between_a_line_and_a_chamfer_lays_nothing`
//! - A red cut keeps the line tangent. A cut taken off, or a fillet become a
//!   chamfer, lets it go where it stands —
//!   `a_red_fillet_keeps_its_line_tangent`,
//!   `taking_a_fillet_off_lets_its_tangent_line_go_where_it_stands`,
//!   `a_fillet_become_a_chamfer_lets_its_tangent_line_go_where_it_stands`
//! - A cut nothing hangs on costs the solver nothing more than in #617 — no
//!   test: what the solver is asked is read only inside the sketch crate, so
//!   the tests holding it live there, in `corner_cut/hanging/tests.rs`:
//!   `a_cut_nothing_hangs_on_asks_the_solver_for_nothing` (#620), a lone
//!   fillet, and
//!   `a_fillet_nothing_hangs_on_asks_nothing_beside_one_a_line_is_tangent_to`,
//!   a fillet beside one carrying a tangent line
//!
//! Carried over from the owner's answers on #620, which apply to a tangent as
//! they are:
//! - a fillet cut again, the rule follows the cut —
//!   `a_corner_rounded_again_keeps_its_line_tangent`
//! - sides swapped, the rule follows the cut — a fillet's own sides cannot be
//!   swapped (`swap_cut_sides` leaves a fillet as it is), so the swap a tangent
//!   meets is the corner rounded again from its other side —
//!   `a_corner_rounded_again_from_its_other_side_keeps_its_line_tangent`
//! - a side trimmed, the rule follows the cut —
//!   `a_side_trimmed_keeps_its_line_tangent_to_the_fillet`
//!
//! Surfaced by review:
//! - a part holding a line tangent to a fillet is compacted without losing the
//!   rule — `a_compacted_part_keeps_its_line_tangent_to_a_fillet`
//! - a fillet clicked first under « Tangence » is drawn as picked, as a line
//!   clicked first is — no test: the drawing is `cao_app`'s, held there by
//!   `a_fillet_shown_to_a_tangent_is_drawn_as_picked` in
//!   `screens/viewport/render/cuts/tests.rs`
//!
//! Asked by the owner after trying the pull request — the line moves as it
//! does with a circle or an arc: a touch that would fall past an end of the
//! line is brought onto it, the second clicked sliding along the line by what
//! it lacks —
//! - the fillet clicked first, the line slides along itself —
//!   `a_line_short_of_a_fillet_slides_along_itself_to_touch_it`
//! - the line clicked first, the fillet slides along the line —
//!   `a_fillet_beyond_a_line_s_end_slides_along_it_to_touch_it`
//!
//! Asked by the owner after trying undo in the app, to keep it working — an
//! undo gives the rule back, and the line follows the fillet again:
//! - a fillet become a chamfer, undone —
//!   `undoing_a_fillet_become_a_chamfer_holds_its_line_tangent_again`
//! - a fillet taken off, undone —
//!   `undoing_a_fillet_taken_off_holds_its_line_tangent_again`

use cao_part::history::{ChamferAsked, CutAsked, CutChange, Operation, PointRef};
use cao_part::{History, PartDocument, PartState, compact};
use cao_sketch::{
    ArcDraft, Chamfer, Constraint, Corner, CornerCutId, CutCurve, Element, PointId, Rule,
    RuleIntent, RulePick, SegmentId, Sketch, Support, WorkPlane, rule_intent, sweep_of,
};
use glam::DVec2;

/// How near two places must stand to be the same place after a solve.
const SAME: f64 = 1e-6;

const CUT: CornerCutId = CornerCutId(0);
/// The corner, and the far ends of its two sides.
const CORNER: PointId = PointId(1);
const RIGHT_END: PointId = PointId(2);
const UP_END: PointId = PointId(3);
const RADIUS: f64 = 5.0;
/// The line drawn after the corner was rounded.
const LINE: SegmentId = SegmentId(2);

/// A corner standing on its own at (10, 10): one trait right to (60, 10),
/// one up to (10, 50), cut by `cut`. Its first value fixes the scale: a unit
/// is a millimetre. Beyond the corner a free line runs from (0, 18) to
/// (18, 0), square to the cut's middle: touching nothing, crossing nothing.
fn a_corner_cut_beside_a_line(cut: Operation) -> History {
    a_corner_cut_beside(cut, [DVec2::new(0.0, 18.0), DVec2::new(18.0, 0.0)])
}

/// The corner rounded to 5 about (15, 15), and a free line on the same
/// slant as the line beside it, from (−10, 28) to (−2, 20): it stops some
/// 15.6 short of where it would touch the fillet's circle, at (9, 9).
fn a_rounded_corner_beside_a_short_line() -> History {
    a_corner_cut_beside(
        Operation::Fillet {
            sketch: 0,
            corners: the_corner(),
            radius: RADIUS.into(),
            construction: false,
        },
        [DVec2::new(-10.0, 28.0), DVec2::new(-2.0, 20.0)],
    )
}

fn a_corner_cut_beside(cut: Operation, [start, end]: [DVec2; 2]) -> History {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(10.0, 10.0)),
        end: PointRef::New(DVec2::new(60.0, 10.0)),
        construction: false,
    });
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(CORNER),
        end: PointRef::New(DVec2::new(10.0, 50.0)),
        construction: false,
    });
    history.push(cut);
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(start),
        end: PointRef::New(end),
        construction: false,
    });
    history
}

fn the_corner() -> Vec<Corner> {
    vec![Corner::Between(SegmentId(0), SegmentId(1))]
}

/// The corner rounded to 5 about (15, 15), the line some 8.5 from the
/// fillet's centre.
fn a_rounded_corner_beside_a_line() -> History {
    a_corner_cut_beside_a_line(Operation::Fillet {
        sketch: 0,
        corners: the_corner(),
        radius: RADIUS.into(),
        construction: false,
    })
}

/// The corner chamfered by 5 on each side.
fn a_chamfered_corner_beside_a_line() -> History {
    a_corner_cut_beside_a_line(Operation::Chamfer {
        sketch: 0,
        corners: the_corner(),
        mode: ChamferAsked::from(Chamfer::Equal(5.0)),
        construction: false,
    })
}

/// What the constraint tool hands the rule, the line clicked first or the
/// fillet's arc clicked first.
fn clicked(line_first: bool) -> (&'static str, [RulePick; 2]) {
    let line = RulePick::Element(Element::Segment(LINE));
    let arc = RulePick::CutCurve(CUT);
    match line_first {
        true => ("the line clicked first", [line, arc]),
        false => ("the fillet clicked first", [arc, line]),
    }
}

/// The rule « Tangence » makes of the line and the fillet, clicked in the
/// order `picks` says.
fn the_tangent(sketch: &Sketch, picks: [RulePick; 2]) -> Constraint {
    let Some(RuleIntent::Constrain(rule)) = rule_intent(Rule::Tangent, &picks, sketch) else {
        panic!("the tangent rule between a line and a fillet's arc lays nothing");
    };
    rule
}

/// The drawing once « Tangence » is laid between the line and the fillet,
/// clicked in the order `picks` says, as the constraint tool lays it.
fn tangent_laid(history: &History, picks: [RulePick; 2]) -> Sketch {
    let mut state = PartState::rebuild(history);
    let rule = the_tangent(&state.sketches[0], picks);
    let outcome = state.apply(&Operation::Constrain {
        sketch: 0,
        constraint: rule,
    });
    assert!(outcome.is_none(), "the tangency was refused: {outcome:?}");
    state.sketches[0].clone()
}

/// The line of `a_rounded_corner_beside_a_line` held tangent to the fillet,
/// the fillet clicked first: the line came to it, square to its middle, the
/// corner where it was.
fn a_line_held_tangent() -> History {
    let mut history = a_rounded_corner_beside_a_line();
    let (_, picks) = clicked(false);
    let rule = the_tangent(&the_drawing(&history), picks);
    history.push(Operation::Constrain {
        sketch: 0,
        constraint: rule,
    });
    history
}

fn move_point(history: &mut History, point: PointId, to: DVec2) {
    history.push(Operation::MovePoint {
        sketch: 0,
        point,
        position: to,
        merged_into: None,
        on: Vec::new(),
        let_go: false,
    });
}

fn the_drawing(history: &History) -> Sketch {
    PartState::rebuild(history).sketches[0].clone()
}

/// The one cut standing on the corner, whichever name a change gave it.
fn the_cut(sketch: &Sketch) -> CornerCutId {
    let mut standing = sketch.live_corner_cuts().map(|(cut, _)| cut);
    let cut = standing.next().expect("a cut stands on the corner");
    assert!(standing.next().is_none(), "one cut stands on the corner");
    cut
}

fn rounded_to(history: &mut History, radius: f64) {
    history.push(Operation::CutChange(CutChange::Reshape {
        sketch: 0,
        cut: CUT,
        value: CutAsked::Fillet(radius.into()),
    }));
}

/// Whether a rule still holds a line tangent to a cut.
fn a_line_is_held_on_a_cut(sketch: &Sketch) -> bool {
    sketch
        .constraints()
        .iter()
        .any(|rule| matches!(rule, Constraint::CutTangent { .. }))
}

/// The fillet's arc as it is drawn: its centre and its radius.
fn the_fillet(sketch: &Sketch) -> (DVec2, f64) {
    match sketch
        .drawn_cut(the_cut(sketch))
        .expect("the fillet is drawn")
        .curve
    {
        CutCurve::Arc(arc) => (arc.centre, arc.centre.distance(arc.start)),
        CutCurve::Straight { .. } => panic!("the fillet is drawn straight"),
    }
}

/// How far the line, taken whole, runs from the fillet's centre.
fn reach_from_the_centre(sketch: &Sketch) -> f64 {
    let (centre, _) = the_fillet(sketch);
    let (start, end) = sketch.endpoints(LINE);
    (end - start).normalize().perp_dot(centre - start).abs()
}

fn assert_tangent(sketch: &Sketch, case: &str) {
    let reach = reach_from_the_centre(sketch);
    let (_, radius) = the_fillet(sketch);
    assert!(
        (reach - radius).abs() < SAME,
        "{case}: the line runs {reach} from the fillet's centre, its radius is {radius}"
    );
}

/// Where the line, taken whole, touches the fillet's circle: the foot of the
/// centre on it.
fn the_touch(sketch: &Sketch) -> DVec2 {
    let (centre, _) = the_fillet(sketch);
    let (start, end) = sketch.endpoints(LINE);
    let along = (end - start).normalize();
    start + along * along.dot(centre - start)
}

/// Whether a place lies within the sweep of the fillet's drawn arc, seen from
/// its centre.
fn within_the_drawn_arc(sketch: &Sketch, place: DVec2) -> bool {
    let CutCurve::Arc(arc) = sketch
        .drawn_cut(the_cut(sketch))
        .expect("the fillet is drawn")
        .curve
    else {
        panic!("the fillet is drawn straight");
    };
    sweep_of(ArcDraft { end: place, ..arc }) <= sweep_of(arc)
}

/// How far the touch falls past the nearer end of the line: nothing while it
/// lies between the two.
fn past_the_line(sketch: &Sketch) -> f64 {
    let (start, end) = sketch.endpoints(LINE);
    let run = end - start;
    let along = (the_touch(sketch) - start).dot(run) / run.length_squared();
    (along - along.clamp(0.0, 1.0)).abs() * run.length()
}

fn the_line(sketch: &Sketch) -> [DVec2; 2] {
    let (start, end) = sketch.endpoints(LINE);
    [start, end]
}

fn the_corner_and_its_sides(sketch: &Sketch) -> [DVec2; 3] {
    [CORNER, RIGHT_END, UP_END].map(|point| sketch.point(point))
}

/// How far a set of places travelled, taken as the worst of them.
fn moved<const N: usize>(before: [DVec2; N], after: [DVec2; N]) -> f64 {
    before
        .iter()
        .zip(after)
        .map(|(was, is)| was.distance(is))
        .fold(0.0, f64::max)
}

#[test]
fn a_line_and_a_fillet_clicked_in_either_order_are_made_tangent() {
    let history = a_rounded_corner_beside_a_line();
    assert!(
        reach_from_the_centre(&the_drawing(&history)) - RADIUS > 1.0,
        "the line already touches the fillet before the rule is laid"
    );

    for line_first in [true, false] {
        let (order, picks) = clicked(line_first);
        let sketch = tangent_laid(&history, picks);

        assert_tangent(&sketch, order);
    }
}

#[test]
fn a_tangent_with_a_fillet_leaves_the_first_clicked_where_it_was() {
    let history = a_rounded_corner_beside_a_line();
    let before = the_drawing(&history);

    for line_first in [true, false] {
        let (order, picks) = clicked(line_first);
        let sketch = tangent_laid(&history, picks);

        let line_moved = moved(the_line(&before), the_line(&sketch));
        let fillet_moved = moved(
            the_corner_and_its_sides(&before),
            the_corner_and_its_sides(&sketch),
        );
        let (stayed, came) = match line_first {
            true => (line_moved, fillet_moved),
            false => (fillet_moved, line_moved),
        };
        assert!(stayed < SAME, "{order}: it moved by {stayed}");
        assert!(came > SAME, "{order}: the other stayed where it was");
        let (_, radius) = the_fillet(&sketch);
        assert!(
            (radius - RADIUS).abs() < SAME,
            "{order}: the fillet's value became {radius}"
        );
    }
}

#[test]
fn a_line_tangent_to_a_fillet_follows_a_new_radius() {
    let mut history = a_line_held_tangent();

    history.push(Operation::CutChange(CutChange::Reshape {
        sketch: 0,
        cut: CUT,
        value: CutAsked::Fillet(8.0.into()),
    }));

    let sketch = the_drawing(&history);
    let (_, radius) = the_fillet(&sketch);
    assert!(
        (radius - 8.0).abs() < SAME,
        "the fillet was to be rounded to 8, it is {radius}"
    );
    assert_tangent(&sketch, "rounded to 8");
}

#[test]
fn a_line_tangent_to_a_fillet_follows_its_corner_moved() {
    let mut history = a_line_held_tangent();
    let to = DVec2::new(0.0, 4.0);

    move_point(&mut history, CORNER, to);

    let sketch = the_drawing(&history);
    assert!(
        sketch.point(CORNER).distance(to) < SAME,
        "the corner was to be dragged to {to}, it is at {}",
        sketch.point(CORNER),
    );
    assert_tangent(&sketch, "the corner dragged");
}

#[test]
fn a_line_tangent_to_a_fillet_follows_its_side_turned() {
    let mut history = a_line_held_tangent();
    let to = DVec2::new(40.0, 50.0);

    move_point(&mut history, UP_END, to);

    let sketch = the_drawing(&history);
    assert!(
        sketch.point(UP_END).distance(to) < SAME,
        "the side's end was to be dragged to {to}, it is at {}",
        sketch.point(UP_END),
    );
    assert_tangent(&sketch, "a side turned");
}

#[test]
fn a_line_tangent_to_a_fillet_stays_on_its_circle_once_the_touch_is_past_the_arc() {
    let mut history = a_line_held_tangent();
    let held = the_drawing(&history);
    assert!(
        within_the_drawn_arc(&held, the_touch(&held)),
        "the line was to touch the drawn arc before the side turned"
    );
    let opened_wide = DVec2::new(-30.0, 20.0);

    move_point(&mut history, UP_END, opened_wide);

    let sketch = the_drawing(&history);
    assert!(
        sketch.point(UP_END).distance(opened_wide) < SAME,
        "the side's end was to be dragged to {opened_wide}, it is at {}",
        sketch.point(UP_END),
    );
    assert!(
        !within_the_drawn_arc(&sketch, the_touch(&sketch)),
        "the corner opened wide was to leave the touch past the drawn arc, at {}",
        the_touch(&sketch),
    );
    assert_tangent(&sketch, "the touch past the drawn arc");
}

#[test]
fn pulling_a_line_tangent_to_a_fillet_moves_its_corner() {
    let mut history = a_line_held_tangent();
    let held = the_drawing(&history);
    let towards_the_corner = DVec2::new(-3.0, -3.0);

    history.push(Operation::MoveSegment {
        sketch: 0,
        segment: LINE,
        by: towards_the_corner,
    });

    let sketch = the_drawing(&history);
    let (start, end) = held.endpoints(LINE);
    let across = (end - start).normalize().perp();
    let travel = towards_the_corner.dot(across);
    for (was, is) in the_line(&held).into_iter().zip(the_line(&sketch)) {
        let went = (is - was).dot(across);
        assert!(
            (went - travel).abs() < SAME,
            "the line was pulled {travel} across, an end of it went {went}"
        );
    }
    let corner_moved = sketch.point(CORNER).distance(held.point(CORNER));
    assert!(
        corner_moved > 1.0,
        "the corner moved by {corner_moved}, to {}",
        sketch.point(CORNER),
    );
    let (_, radius) = the_fillet(&sketch);
    assert!(
        (radius - RADIUS).abs() < SAME,
        "the fillet's value became {radius}"
    );
    assert_tangent(&sketch, "the line pulled");
}

#[test]
fn a_tangent_between_a_line_and_a_chamfer_lays_nothing() {
    let mut history = a_chamfered_corner_beside_a_line();
    let chamfered = the_drawing(&history);
    for line_first in [true, false] {
        let (order, picks) = clicked(line_first);
        assert_eq!(
            rule_intent(Rule::Tangent, &picks, &chamfered),
            None,
            "{order}: a rule was laid between a line and a chamfer's cut"
        );
    }

    history.push(Operation::CutChange(CutChange::Reshape {
        sketch: 0,
        cut: CUT,
        value: CutAsked::Fillet(RADIUS.into()),
    }));

    let rounded = the_drawing(&history);
    for line_first in [true, false] {
        let (order, picks) = clicked(line_first);
        assert!(
            rule_intent(Rule::Tangent, &picks, &rounded).is_some(),
            "{order}: the same corner rounded takes no tangent either"
        );
    }
}

#[test]
fn a_red_fillet_keeps_its_line_tangent() {
    let mut history = a_line_held_tangent();

    rounded_to(&mut history, 45.0);

    let sketch = the_drawing(&history);
    assert!(
        !sketch.drawn_cut(CUT).expect("the fillet is drawn").fits,
        "a fillet of 45 was to be too big for a side of 40, and drawn red"
    );
    let (_, radius) = the_fillet(&sketch);
    assert!(
        (radius - 45.0).abs() < SAME,
        "the red fillet's value is {radius}"
    );
    assert_tangent(&sketch, "the fillet red");

    rounded_to(&mut history, RADIUS);
    let sketch = the_drawing(&history);
    assert!(sketch.drawn_cut(CUT).expect("the fillet is drawn").fits);
    assert_tangent(&sketch, "the fillet fitting again");
}

#[test]
fn taking_a_fillet_off_lets_its_tangent_line_go_where_it_stands() {
    let mut history = a_line_held_tangent();
    let held = the_drawing(&history);

    history.push(Operation::CutChange(CutChange::TakeOff {
        sketch: 0,
        cuts: vec![CUT],
    }));

    let sketch = the_drawing(&history);
    assert_eq!(sketch.live_corner_cuts().count(), 0, "the fillet stays");
    let line_moved = moved(the_line(&held), the_line(&sketch));
    assert!(line_moved < SAME, "the line moved by {line_moved}");
    assert!(
        !a_line_is_held_on_a_cut(&sketch),
        "the line is still held on the cut taken off: {:?}",
        sketch.constraints(),
    );
}

#[test]
fn a_fillet_become_a_chamfer_lets_its_tangent_line_go_where_it_stands() {
    let mut history = a_line_held_tangent();
    let held = the_drawing(&history);

    history.push(Operation::CutChange(CutChange::Reshape {
        sketch: 0,
        cut: CUT,
        value: CutAsked::Chamfer(ChamferAsked::from(Chamfer::Equal(7.0))),
    }));

    let sketch = the_drawing(&history);
    assert!(
        matches!(
            sketch.drawn_cut(CUT).expect("the cut is drawn").curve,
            CutCurve::Straight { .. }
        ),
        "the fillet was to become a chamfer"
    );
    let line_moved = moved(the_line(&held), the_line(&sketch));
    assert!(line_moved < SAME, "the line moved by {line_moved}");
    assert!(
        !a_line_is_held_on_a_cut(&sketch),
        "the line is still held on the chamfer: {:?}",
        sketch.constraints(),
    );

    rounded_to(&mut history, 8.0);
    let sketch = the_drawing(&history);
    let line_moved = moved(the_line(&held), the_line(&sketch));
    assert!(
        line_moved < SAME,
        "rounded again, the line followed the fillet by {line_moved}"
    );
}

#[test]
fn a_corner_rounded_again_keeps_its_line_tangent() {
    let mut history = a_line_held_tangent();

    history.push(Operation::Fillet {
        sketch: 0,
        corners: the_corner(),
        radius: 8.0.into(),
        construction: false,
    });

    let sketch = the_drawing(&history);
    let (_, radius) = the_fillet(&sketch);
    assert!(
        (radius - 8.0).abs() < SAME,
        "the corner was to be rounded again to 8, it is {radius}"
    );
    assert_tangent(&sketch, "the corner rounded again");
}

/// The corner rounded again to 8 as it stands, and the line tangent to it.
fn assert_follows_a_new_radius(history: &mut History, case: &str) {
    let cut = the_cut(&the_drawing(history));
    history.push(Operation::CutChange(CutChange::Reshape {
        sketch: 0,
        cut,
        value: CutAsked::Fillet(8.0.into()),
    }));
    let sketch = the_drawing(history);
    let (_, radius) = the_fillet(&sketch);
    assert!(
        (radius - 8.0).abs() < SAME,
        "{case}: the fillet was to be rounded to 8, it is {radius}"
    );
    assert_tangent(&sketch, &format!("{case}, then rounded to 8"));
}

#[test]
fn a_corner_rounded_again_from_its_other_side_keeps_its_line_tangent() {
    let mut history = a_line_held_tangent();

    history.push(Operation::Fillet {
        sketch: 0,
        corners: vec![Corner::Between(SegmentId(1), SegmentId(0))],
        radius: 8.0.into(),
        construction: false,
    });

    let sketch = the_drawing(&history);
    let (_, radius) = the_fillet(&sketch);
    assert!(
        (radius - 8.0).abs() < SAME,
        "the corner was to be rounded again to 8, it is {radius}"
    );
    assert_tangent(&sketch, "the corner rounded again from its other side");
}

#[test]
fn a_side_trimmed_keeps_its_line_tangent_to_the_fillet() {
    let mut history = a_line_held_tangent();
    let along_the_right_side = PointId(6);
    history.push(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(40.0, 10.0),
        on: vec![Support::Segment(SegmentId(0))],
    });

    history.push(Operation::Trim {
        sketch: 0,
        segment: SegmentId(0),
        from: along_the_right_side,
        to: RIGHT_END,
    });

    let sketch = the_drawing(&history);
    let cut = the_cut(&sketch);
    let side = sketch.corner_cuts()[cut.0].sides[0];
    let (start, end) = sketch.endpoints(side);
    assert!(
        (start.distance(end) - 30.0).abs() < SAME,
        "the right side was to be trimmed to 30, it runs {}",
        start.distance(end),
    );
    assert_tangent(&sketch, "a side trimmed");
    assert_follows_a_new_radius(&mut history, "a side trimmed");
}

#[test]
fn a_compacted_part_keeps_its_line_tangent_to_a_fillet() {
    let history = a_line_held_tangent();
    let before = the_drawing(&history);

    let mut compacted = compact(&history);

    let sketch = the_drawing(&compacted);
    assert!(
        a_line_is_held_on_a_cut(&sketch),
        "compacted, the line is no longer held on the fillet: {:?}",
        sketch.constraints(),
    );
    let line_moved = moved(the_line(&before), the_line(&sketch));
    assert!(
        line_moved < SAME,
        "compacted, the line moved by {line_moved}"
    );
    assert_tangent(&sketch, "compacted");
    assert_follows_a_new_radius(&mut compacted, "compacted");
}

#[test]
fn a_line_short_of_a_fillet_slides_along_itself_to_touch_it() {
    let history = a_rounded_corner_beside_a_short_line();
    let before = the_drawing(&history);
    assert!(
        past_the_line(&before) > 1.0,
        "the line was to stop well short of the fillet"
    );
    let (_, picks) = clicked(false);

    let sketch = tangent_laid(&history, picks);

    assert_tangent(&sketch, "the fillet clicked first");
    let past = past_the_line(&sketch);
    assert!(
        past < SAME,
        "the line touches the fillet {past} past its end"
    );
    let fillet_moved = moved(
        the_corner_and_its_sides(&before),
        the_corner_and_its_sides(&sketch),
    );
    assert!(
        fillet_moved < SAME,
        "the fillet, clicked first, moved by {fillet_moved}"
    );
    let (_, radius) = the_fillet(&sketch);
    assert!(
        (radius - RADIUS).abs() < SAME,
        "the fillet's value became {radius}"
    );
}

#[test]
fn a_fillet_beyond_a_line_s_end_slides_along_it_to_touch_it() {
    let history = a_rounded_corner_beside_a_short_line();
    let before = the_drawing(&history);
    assert!(
        past_the_line(&before) > 1.0,
        "the fillet was to stand well beyond the line's end"
    );
    let (_, picks) = clicked(true);

    let sketch = tangent_laid(&history, picks);

    assert_tangent(&sketch, "the line clicked first");
    let past = past_the_line(&sketch);
    assert!(
        past < SAME,
        "the fillet touches the line {past} past its end"
    );
    let line_moved = moved(the_line(&before), the_line(&sketch));
    assert!(
        line_moved < SAME,
        "the line, clicked first, moved by {line_moved}"
    );
    let (_, radius) = the_fillet(&sketch);
    assert!(
        (radius - RADIUS).abs() < SAME,
        "the fillet's value became {radius}"
    );
}

/// The part of `a_line_held_tangent`, applied gesture by gesture as the app
/// applies them, so that it can be undone as the app undoes.
fn a_document_with_a_line_held_tangent() -> PartDocument {
    let mut document = PartDocument::new("Plaque", "2026-11-03T09:00:00Z".parse().expect("a date"));
    for operation in a_line_held_tangent().operations() {
        document.apply(operation.clone());
    }
    document
}

/// The change undone, and the line held tangent again: the rule is back, the
/// line touches the fillet, and follows it to a new radius.
fn assert_undone_holds_the_line_again(mut document: PartDocument, case: &str) {
    assert!(document.undo(), "{case}: there was nothing to undo");

    let sketch = &document.sketches()[0];
    assert!(
        a_line_is_held_on_a_cut(sketch),
        "{case}: the line is not held on the fillet again: {:?}",
        sketch.constraints(),
    );
    assert_tangent(sketch, case);
    assert_follows_a_new_radius(&mut document.history, case);
}

#[test]
fn undoing_a_fillet_become_a_chamfer_holds_its_line_tangent_again() {
    let mut document = a_document_with_a_line_held_tangent();
    document.apply(Operation::CutChange(CutChange::Reshape {
        sketch: 0,
        cut: CUT,
        value: CutAsked::Chamfer(ChamferAsked::from(Chamfer::Equal(7.0))),
    }));
    assert!(
        !a_line_is_held_on_a_cut(&document.sketches()[0]),
        "the chamfer was to let the line go"
    );

    assert_undone_holds_the_line_again(document, "the chamfer undone");
}

#[test]
fn undoing_a_fillet_taken_off_holds_its_line_tangent_again() {
    let mut document = a_document_with_a_line_held_tangent();
    document.apply(Operation::CutChange(CutChange::TakeOff {
        sketch: 0,
        cuts: vec![CUT],
    }));
    let taken_off = &document.sketches()[0];
    assert_eq!(taken_off.live_corner_cuts().count(), 0, "the fillet stays");
    assert!(
        !a_line_is_held_on_a_cut(taken_off),
        "taking the fillet off was to let the line go"
    );

    assert_undone_holds_the_line_again(document, "the fillet taken off, undone");
}
