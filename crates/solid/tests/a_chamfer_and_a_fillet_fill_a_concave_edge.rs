//! A chamfer and a fillet fill a concave straight edge of an exact body in:
//! where two planar faces meet inwards — the inner corner of an L, the foot of
//! a rib standing on a plate — the steps that cut a convex edge off (#696,
//! #697) join matter in instead. The chamfer lays a flat face set back by its
//! distance along each of the two faces; the fillet a cylinder of its radius
//! tangent to both, set back along each by r / tan(α / 2), α the angle between
//! them outside the matter — the radius itself where they meet square. The new
//! face's two sides are edges of the two faces, its two ends edges of the
//! faces at the edge's ends, and a value the faces cannot give is reduced to
//! the largest they can.
//!
//! Closes #778.
//! - A chamfer and a fillet given the name of the inner edge of an L, between
//!   two planar faces, and a value, leave a closed body with one more face: a
//!   flat set back by the distance along each face, or a cylinder of the
//!   radius tangent to both. The faces at the ends hold its ends, and the body
//!   passes the soundness checks —
//!   `a_chamfer_on_the_inner_corner_of_an_l_fills_it_with_one_flat_face_set_back_by_the_distance_along_each_face`,
//!   `a_fillet_on_the_inner_corner_of_an_l_fills_it_with_one_round_of_the_radius_tangent_to_both_faces`
//! - The same holds on a concave edge whose faces meet at an angle other than
//!   a right one —
//!   `a_chamfer_on_a_concave_edge_whose_faces_meet_at_a_slant_is_set_back_by_the_distance_along_each_face`,
//!   `a_fillet_on_a_concave_edge_whose_faces_meet_at_a_slant_is_set_back_by_the_radius_over_the_tangent_of_half_the_angle`
//! - The same holds where the edge ends on an oblique wall, and where a rib
//!   ends short of its plate's edge (the fill trimmed flush by the rib's end) —
//!   `a_chamfer_and_a_fillet_on_a_concave_edge_ending_on_an_oblique_wall_are_trimmed_flush_by_it`,
//!   `a_chamfer_and_a_fillet_at_the_foot_of_a_rib_ending_short_of_its_plate_s_edge_are_trimmed_flush_by_the_rib_s_end`
//! - A value larger than the neighbouring faces can give is reduced to the
//!   largest that fits, and the step reports the value asked and the value
//!   taken —
//!   `a_chamfer_on_a_concave_edge_asked_more_than_a_face_can_give_takes_the_whole_face_and_reports_both_values`,
//!   `a_fillet_on_a_concave_edge_asked_more_than_a_face_can_give_takes_the_radius_whose_set_back_is_the_whole_face_and_reports_both_radii`;
//!   and a value that fits is not reduced, however near a face's whole width —
//!   `a_chamfer_on_a_concave_edge_just_short_of_a_face_s_whole_width_is_laid_as_asked`
//! - After a change of size, a move or a turn upstream, the step fills the
//!   same edge, found by its name —
//!   `a_chamfer_and_a_fillet_fill_the_concave_edge_their_name_finds_again_after_a_change_of_size_a_move_and_a_turn`
//! - An edge its name no longer finds, gone or split, is declined `Lost` with
//!   which —
//!   `a_chamfer_and_a_fillet_on_a_concave_edge_a_cut_took_away_are_declined_saying_it_is_gone`,
//!   `a_chamfer_and_a_fillet_on_a_concave_edge_a_cut_parted_in_two_are_declined_saying_it_is_split`
//! - The face numbers of every existing part are unchanged; the new face takes
//!   the number after the steps' last one —
//!   `a_chamfer_and_a_fillet_on_a_concave_edge_leave_every_face_of_the_l_its_number_and_number_theirs_after_the_steps_last`;
//!   the parts with neither a chamfer nor a fillet keep theirs by
//!   `faces_keep_their_numbers.rs`, left as it was
//! - The exact STEP export of a filled L writes its new face on its true
//!   surface —
//!   `an_l_filled_at_its_inner_corner_written_to_step_writes_its_new_face_on_its_true_surface`
//! - The convex chamfer's and fillet's tests still pass — no test: the convex
//!   suites, `a_chamfer_cuts_one_straight_edge.rs`,
//!   `a_fillet_rounds_one_straight_edge.rs`, `src/body/tests/chamfer.rs` and
//!   `src/body/tests/fillet.rs`, are the answer, left as they were but for the
//!   two tests that declined the inner corner of an L, which this issue turns
//!   round and this file now speaks for

use std::collections::BTreeSet;
use std::f64::consts::PI;

use cao_solid::profile::{Contour, Frame, Profile};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, EdgeName, Loop};
use glam::{DVec2, DVec3};

/// How far a corner may stand from where the arithmetic puts it, on a plane
/// or on the round: each is placed exactly, so what is left is rounding.
const PLACE: f64 = 1e-9;

/// How near the arithmetic an exact volume comes, as a fraction of it.
const EXACT: f64 = 1e-9;

/// How far the triangles may stand from a round, as a share of the reach the
/// body carries: the drawing's tolerance.
const DRAWN: f64 = 2e-4;

/// The numbers a raised part's floor and top answer to.
const FLOOR: usize = 0;
const TOP: usize = 1;

/// The numbers an L's arm's back and its stem's right side answer to: the
/// two faces that meet inwards at its inner corner.
const ARM_BACK: usize = 4;
const STEM_RIGHT: usize = 5;

/// The ground's plane, which a part is drawn on.
fn ground() -> Frame {
    Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    }
}

/// An outline with its runs, as the drawing hands it to the exact kernel.
fn profile(outline: Contour) -> Profile<'static> {
    Profile {
        exact: Some((outline, Vec::new())),
        sampled: Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    }
}

/// An outline drawn on a frame raised along its normal by `height`, and
/// joined as `cao_part` builds a step: its floor numbered nought, its top
/// one, and its walls from two on, one for each run of the outline.
fn raised_on(frame: Frame, outline: Contour, height: f64) -> Body {
    let part = Body::default();
    let tool = part
        .tool_raised(&profile(outline), frame, frame.normal() * height)
        .expect("the tool raises");
    part.union(&tool).expect("the part is built")
}

/// The two corners an upright edge of a part raised from the ground stands
/// between, at the point of its outline `at`.
fn upright(at: DVec2, height: f64) -> [DVec3; 2] {
    [at.extend(0.0), at.extend(height)]
}

/// The corners of a part's floor or top, the points of its outline lifted to
/// `height`.
fn lifted_to(outline: &[DVec2], height: f64) -> Vec<DVec3> {
    outline.iter().map(|at| at.extend(height)).collect()
}

/// An L lying on the ground: an arm `length` long and `depth` deep along X,
/// and at the arm's left end a stem `stem` wide reaching back to `back`,
/// raised by `height`. Its walls are numbered from the front round: the front
/// two, the arm's right end three, the arm's back four, the stem's right side
/// five, the stem's back six and the left seven. The arm's back and the
/// stem's right side meet inwards, at its inner corner.
#[derive(Clone, Copy)]
struct Ell {
    length: f64,
    depth: f64,
    stem: f64,
    back: f64,
    height: f64,
}

const ELL: Ell = Ell {
    length: 30.0,
    depth: 6.0,
    stem: 12.0,
    back: 14.0,
    height: 10.0,
};

impl Ell {
    /// The L's outline, from the front's left end round, its inner corner
    /// fourth.
    fn outline(&self) -> Vec<DVec2> {
        let Ell {
            length,
            depth,
            stem,
            back,
            ..
        } = *self;
        vec![
            DVec2::ZERO,
            DVec2::new(length, 0.0),
            DVec2::new(length, depth),
            DVec2::new(stem, depth),
            DVec2::new(stem, back),
            DVec2::new(0.0, back),
        ]
    }

    /// The L's outline with its inner corner given up for the two points
    /// `set_back` along the arm's back and along the stem's right side.
    fn outline_filled(&self, set_back: [DVec2; 2]) -> Vec<DVec2> {
        let mut outline = self.outline();
        outline.splice(3..4, set_back);
        outline
    }

    fn on(&self, frame: Frame) -> Body {
        raised_on(frame, Contour::straight(self.outline()), self.height)
    }

    fn volume(&self) -> f64 {
        (self.length * self.depth + self.stem * (self.back - self.depth)) * self.height
    }
}

/// The outline of a plate 50 long on the X axis, reaching back to Y = 12 at
/// its left and to Y = 4 at its right, and nowhere less than 4 deep: between
/// the two a riser leaning back 3 across for every 4 up, and in the high part
/// a V notch 8 wide at its mouth whose point stands 4 behind the front. The
/// step meets the riser at an obtuse concave corner and the notch's two walls
/// meet at an acute one: the cosines of the angles the air between them
/// makes are −0.6 and 0.6, their sines both 0.8, and the tangents of their
/// halves 2 and 1/2 — a round set back by its radius is off by half at the
/// one and by double at the other. Raised, its walls are numbered from the
/// front round: the front two, the right end three, the step four, the riser
/// five, the back of the high part six, the notch's right wall seven and its
/// left wall eight, the back left of the notch nine and the left ten.
const NOTCHED_STEP: [DVec2; 9] = [
    DVec2::new(0.0, 0.0),
    DVec2::new(50.0, 0.0),
    DVec2::new(50.0, 4.0),
    DVec2::new(38.0, 4.0),
    DVec2::new(32.0, 12.0),
    DVec2::new(24.0, 12.0),
    DVec2::new(20.0, 4.0),
    DVec2::new(16.0, 12.0),
    DVec2::new(0.0, 12.0),
];

/// The area [`NOTCHED_STEP`] encloses: 50 by 12, less the 120 the riser and
/// the step leave above the step, and the 32 of the notch.
const NOTCHED_STEP_AREA: f64 = 448.0;

/// The numbers the step, the riser and the notch's walls of the
/// [`NOTCHED_STEP`] plate answer to.
const STEP: usize = 4;
const RISER: usize = 5;
const NOTCH_RIGHT: usize = 7;
const NOTCH_LEFT: usize = 8;

/// The [`NOTCHED_STEP`] outline raised from the ground by `height`.
fn notched_step(height: f64) -> Body {
    raised_on(ground(), Contour::straight(NOTCHED_STEP.to_vec()), height)
}

/// A concave upright edge of the [`NOTCHED_STEP`] plate: the point of the
/// outline it stands on, its two walls, the one before that point and the one
/// after, and the way each wall faces, in the ground's plane.
struct Inner {
    what: &'static str,
    at: usize,
    walls: [usize; 2],
    outward: [DVec2; 2],
}

const INNER_EDGES: [Inner; 2] = [
    Inner {
        what: "the obtuse inner edge between the step and the riser",
        at: 3,
        walls: [STEP, RISER],
        outward: [DVec2::new(0.0, 1.0), DVec2::new(4.0, 3.0)],
    },
    Inner {
        what: "the acute inner edge at the point of the notch",
        at: 6,
        walls: [NOTCH_RIGHT, NOTCH_LEFT],
        outward: [DVec2::new(-2.0, 1.0), DVec2::new(2.0, 1.0)],
    },
];

impl Inner {
    fn corner(&self) -> DVec2 {
        NOTCHED_STEP[self.at]
    }

    /// The far end of each wall from the corner.
    fn far(&self) -> [DVec2; 2] {
        [NOTCHED_STEP[self.at - 1], NOTCHED_STEP[self.at + 1]]
    }

    /// The way along each wall from the corner.
    fn ways(&self) -> [DVec2; 2] {
        self.far().map(|far| (far - self.corner()).normalize())
    }

    /// The angle between the two walls, through the air.
    fn angle(&self) -> f64 {
        let [first, second] = self.ways();
        first.dot(second).acos()
    }

    /// The two points `set_back` from the corner along each wall.
    fn set_back(&self, set_back: f64) -> [DVec2; 2] {
        self.ways().map(|way| self.corner() + way * set_back)
    }

    /// The outline with the corner given up for the two points `set_back`
    /// from it along each wall.
    fn outline_filled(&self, set_back: f64) -> Vec<DVec2> {
        let mut outline = NOTCHED_STEP.to_vec();
        outline.splice(self.at..=self.at, self.set_back(set_back));
        outline
    }
}

/// The height over a point of the ground of the leaning plane
/// z = 8 − x / 5 + y / 10, which the [`leaning_l`] is cut down to: lower
/// toward the arm's right end and higher toward the stem's back, under the
/// L's top and over its floor everywhere above it.
fn leaning_height(at: DVec2) -> f64 {
    8.0 - at.x / 5.0 + at.y / 10.0
}

/// A point of the ground lifted onto the leaning plane.
fn on_the_lean(at: DVec2) -> DVec3 {
    at.extend(leaning_height(at))
}

/// The way the leaning plane faces, up and out of the matter: it holds the
/// points p where `LEANING_FACING` · p = 8.
const LEANING_FACING: DVec3 = DVec3::new(0.2, -0.1, 1.0);

/// The number the leaning top of the [`leaning_l`] answers to: the floor of
/// the box that cut it, numbered past the L's faces.
const LEANING: usize = 8;

/// The L of [`ELL`] with its top cut away down to the leaning plane, as a
/// part's next step cuts it: a box drawn on a sketch lying on the plane,
/// wider than the L, raised up off it and taken away. Its top is gone whole;
/// the leaning face caps it instead, at 6.2 over the inner corner — square
/// to nothing, the inner edge least of all.
fn leaning_l() -> Body {
    let ell = ELL.on(ground());
    let u = DVec3::new(1.0, 0.0, -0.2).normalize();
    let facing = LEANING_FACING.normalize();
    let frame = Frame {
        origin: DVec3::new(0.0, 0.0, 8.0),
        u,
        v: facing.cross(u),
    };
    let wider = Contour::rectangle(DVec2::new(-4.0, -4.0), DVec2::new(36.0, 20.0));
    let tool = ell
        .tool_raised(&profile(wider), frame, facing * 10.0)
        .expect("the tool raises");
    ell.difference(&tool).expect("the cut is built")
}

/// The matter the [`leaning_l`] holds: under a plane, an area holds itself
/// times the plane's height over its centre, the arm's 180 at (15, 3) and the
/// stem's 96 at (6, 10).
fn leaning_l_volume() -> f64 {
    180.0 * leaning_height(DVec2::new(15.0, 3.0)) + 96.0 * leaning_height(DVec2::new(6.0, 10.0))
}

/// The numbers the faces of the [`ribbed_plate`]'s rib answer to: its front,
/// its right side, its back and its left side. Its floor, six, sits on the
/// plate's top and the join takes it in; its top is seven.
const RIB_FRONT: usize = 8;
const RIB_RIGHT: usize = 9;
const RIB_BACK: usize = 10;
const RIB_LEFT: usize = 11;

/// Where the [`ribbed_plate`]'s rib stands: from x = 18 to its right side at
/// 22, from its front at y = 5 to its back at 22, from the plate's top at
/// z = 4 to 14.
const RIB_SIDE: f64 = 22.0;
const RIB_ENDS: [f64; 2] = [5.0, 22.0];
const PLATE: f64 = 4.0;

/// A plate 40 long, 30 deep and 4 thick, raised from the ground and numbered
/// as [`raised_on`] numbers it, and a rib 4 wide and 10 high joined on its
/// top as a part's next step is: a rectangle sketched on the plate's top and
/// raised. The rib ends short of the plate's edges at both ends, 5 behind its
/// front and 8 before its back, so the plate's top runs on round it, and
/// meets it inwards along all four of its sides.
fn ribbed_plate() -> Body {
    let plate = raised_on(
        ground(),
        Contour::rectangle(DVec2::ZERO, DVec2::new(40.0, 30.0)),
        PLATE,
    );
    let on_the_plate = Frame {
        origin: DVec3::Z * PLATE,
        ..ground()
    };
    let rib = Contour::rectangle(
        DVec2::new(18.0, RIB_ENDS[0]),
        DVec2::new(RIB_SIDE, RIB_ENDS[1]),
    );
    let tool = plate
        .tool_raised(&profile(rib), on_the_plate, DVec3::Z * 10.0)
        .expect("the tool raises");
    plate.union(&tool).expect("the rib is joined")
}

/// The matter the [`ribbed_plate`] holds: the plate's 40 × 30 × 4 and the
/// rib's 4 × 17 × 10.
const RIBBED_VOLUME: f64 = 4800.0 + 680.0;

/// The numbers the L's stem's back and the [`NOTCHED_STEP`] plate's back of
/// its high part answer to: the walls past the stem's right side and past the
/// riser.
const STEM_BACK: usize = 6;
const HIGH_BACK: usize = 6;

/// A concave upright edge asked more than its faces give: the outline its
/// part was raised from by `height`, and the matter that part holds; the
/// point of the outline the edge stands on; its two walls, the one before
/// that point and the one after, which is the shorter and is taken whole; the
/// way each faces; and the face beyond the one taken whole, the wall after
/// it, and the way that one faces.
struct Overreach {
    what: &'static str,
    outline: Vec<DVec2>,
    height: f64,
    volume: f64,
    at: usize,
    walls: [usize; 2],
    outward: [DVec2; 2],
    beyond: usize,
    beyond_facing: DVec2,
}

/// The L's inner corner, whose stem's right side reaches 8 and arm's back 18,
/// and the plate's obtuse inner edge, whose riser reaches 10 and step 12: a
/// chamfer takes 8 and 10 there, and a fillet the radius whose set-back
/// r / tan(α / 2) is that whole width, 8 at the L's square corner and 20 at
/// the plate's, where tan(α / 2) is 2.
fn overreaching() -> [Overreach; 2] {
    [
        Overreach {
            what: "the inner corner of the L, its stem's right side 8 wide, its arm's back 18",
            outline: ELL.outline(),
            height: ELL.height,
            volume: ELL.volume(),
            at: 3,
            walls: [ARM_BACK, STEM_RIGHT],
            outward: [DVec2::Y, DVec2::X],
            beyond: STEM_BACK,
            beyond_facing: DVec2::Y,
        },
        Overreach {
            what: "the obtuse inner edge of the notched plate, its riser 10 wide, its step 12",
            outline: NOTCHED_STEP.to_vec(),
            height: 10.0,
            volume: NOTCHED_STEP_AREA * 10.0,
            at: 3,
            walls: [STEP, RISER],
            outward: [DVec2::Y, DVec2::new(4.0, 3.0)],
            beyond: HIGH_BACK,
            beyond_facing: DVec2::Y,
        },
    ]
}

impl Overreach {
    fn part(&self) -> Body {
        raised_on(
            ground(),
            Contour::straight(self.outline.clone()),
            self.height,
        )
    }

    fn corner(&self) -> DVec2 {
        self.outline[self.at]
    }

    /// The far end of each wall from the corner.
    fn far(&self) -> [DVec2; 2] {
        [self.outline[self.at - 1], self.outline[self.at + 1]]
    }

    /// The way along each wall from the corner.
    fn ways(&self) -> [DVec2; 2] {
        self.far().map(|far| (far - self.corner()).normalize())
    }

    /// The angle between the two walls, through the air.
    fn angle(&self) -> f64 {
        let [first, second] = self.ways();
        first.dot(second).acos()
    }

    /// The second wall's whole width: the most a set-back can be.
    fn reach(&self) -> f64 {
        self.corner().distance(self.far()[1])
    }

    /// Where a set-back of the second wall's whole width stops on the first.
    fn stop(&self) -> DVec2 {
        self.corner() + self.ways()[0] * self.reach()
    }

    /// The outline with the corner given up for that stop: the second wall's
    /// far end stays, where the fill meets the face beyond.
    fn outline_filled(&self) -> Vec<DVec2> {
        let mut outline = self.outline.clone();
        outline[self.at] = self.stop();
        outline
    }
}

/// The name of the one edge between the faces numbered `first` and `second`,
/// as the body hands it to whoever points at that edge.
fn edge_between(body: &Body, first: usize, second: usize) -> EdgeName {
    let sides = [vec![first as u32], vec![second as u32]];
    let named: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| name.between == sides)
        .collect();
    assert_eq!(
        named.len(),
        1,
        "one edge between {first} and {second}: {named:?}"
    );
    named[0].clone()
}

/// Holds that the faces numbered `first` and `second` meet along no edge.
fn no_longer_meet(body: &Body, first: usize, second: usize, what: &str) {
    let (first, second) = (vec![first as u32], vec![second as u32]);
    let meeting: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| name.between.contains(&first) && name.between.contains(&second))
        .collect();
    assert!(meeting.is_empty(), "{what}: {meeting:?}");
}

/// Every number some face of the body answers to: a step's, below where the
/// steps' numbers end, and a piece's, past them.
fn numbers(body: &Body) -> BTreeSet<usize> {
    (0..body.faces_end())
        .chain(body.pieces())
        .filter(|&number| body.has_face(number))
        .collect()
}

/// The sides of the faces the face numbered `face` meets, one for each edge
/// it has, in order.
fn sides_met(body: &Body, face: usize) -> Vec<Vec<u32>> {
    let own = vec![face as u32];
    let mut met: Vec<Vec<u32>> = body
        .edge_names()
        .into_iter()
        .filter_map(|name| {
            let [first, second] = name.between;
            if first == own {
                Some(second)
            } else if second == own {
                Some(first)
            } else {
                None
            }
        })
        .collect();
    met.sort();
    met
}

/// Holds that a face's corners are the points expected: each of them a
/// corner, and no corner anywhere else.
fn corners_are(body: &Body, face: usize, expected: &[DVec3], what: &str) {
    corners_are_or_on(body, face, expected, |_| false, what);
}

/// Holds that a face's corners are the points expected, each of them a
/// corner, and every other corner on the curve `on` says it is on.
fn corners_are_or_on(
    body: &Body,
    face: usize,
    expected: &[DVec3],
    on: impl Fn(DVec3) -> bool,
    what: &str,
) {
    let plane = body
        .plane_of(face)
        .unwrap_or_else(|| panic!("{what}: the body offers no plane on face {face}"));
    for corner in &plane.corners {
        assert!(
            on(*corner) || expected.iter().any(|point| point.distance(*corner) < PLACE),
            "{what}: a corner at {corner}, where the corners are {expected:?}"
        );
    }
    for point in expected {
        assert!(
            plane
                .corners
                .iter()
                .any(|corner| corner.distance(*point) < PLACE),
            "{what}: no corner at {point}, among {:?}",
            plane.corners
        );
    }
}

/// Holds that a body is exact and keeps the rules every solid must: its
/// surface closed, no face through another, and the matter its triangles
/// enclose, as the matter it holds, what arithmetic promised.
fn sound(body: &Body, promised: f64, what: &str) {
    sound_drawn_within(body, promised, EXACT * promised, what);
}

/// Holds what [`sound`] holds of a body with curved faces, whose triangles
/// stand within the drawing's tolerance of them: the matter they enclose
/// within `drawn` of what arithmetic promised, the matter the body holds
/// exactly that.
fn sound_drawn_within(body: &Body, promised: f64, drawn: f64, what: &str) {
    assert!(
        body.is_exact(),
        "{what}: the exact kernel computed the body"
    );
    let triangles = body.triangles();
    assert_eq!(closed(&triangles), Ok(()), "{what}");
    assert_eq!(uncrossed(&triangles), Ok(()), "{what}");
    assert!(
        (body.volume() - promised).abs() <= EXACT * promised,
        "{what}: the body holds {}, where arithmetic promised {promised}",
        body.volume()
    );
    let enclosed = enclosed(&triangles);
    assert!(
        (enclosed - promised).abs() <= drawn,
        "{what}: its triangles enclose {enclosed}, where arithmetic promised {promised}"
    );
}

/// Holds that nothing of the body stands past the box of the part it was
/// made from: a fill held flush by the faces at its edge's ends.
fn within_the_box_of(part: &Body, body: &Body, what: &str) {
    let (low, high) = part.bounds().expect("a part has a box");
    let (reached_low, reached_high) = body.bounds().expect("a part has a box");
    assert!(
        reached_low.distance(low) < PLACE && reached_high.distance(high) < PLACE,
        "{what}: the body reaches from {reached_low} to {reached_high}, where the part it was \
         made from reached from {low} to {high}"
    );
}

/// Holds that no corner of the body's triangles stands in front of the plane
/// of the points p where `facing` · p = `offset`.
fn nothing_stands_past(body: &Body, facing: DVec3, offset: f64, what: &str) {
    let (facing, offset) = (facing.normalize(), offset / facing.length());
    for corner in body.triangles().into_iter().flatten() {
        let past = corner.dot(facing) - offset;
        assert!(
            past <= PLACE,
            "{what}: a corner at {corner} stands {past} past the plane"
        );
    }
}

/// Holds that a step was given the value it was asked.
fn taken_whole(asked: f64, taken: f64, value: f64, what: &str) {
    assert!(
        (asked - value).abs() < PLACE && (taken - value).abs() < PLACE,
        "{what}: asked {asked} and took {taken}, where {value} fits"
    );
}

/// Holds that a step left every face of the part it was given and made one
/// face more, and gives back the number that face answers to.
fn one_face_more(part: &Body, body: &Body, what: &str) -> usize {
    let (before, after) = (numbers(part), numbers(body));
    assert!(
        before.is_subset(&after),
        "{what}: a face went: {before:?} became {after:?}"
    );
    let made: Vec<usize> = after.difference(&before).copied().collect();
    assert_eq!(
        made.len(),
        1,
        "{what}: one face more: {before:?} became {after:?}"
    );
    made[0]
}

/// Holds what [`one_face_more`] holds, the face made flat.
fn one_flat_face_more(part: &Body, body: &Body, what: &str) -> usize {
    let made = one_face_more(part, body, what);
    assert!(
        body.is_flat(made),
        "{what}: the face the chamfer lays is flat"
    );
    made
}

/// Holds what [`one_face_more`] holds, the face made curved.
fn one_round_more(part: &Body, body: &Body, what: &str) -> usize {
    let made = one_face_more(part, body, what);
    assert!(
        !body.is_flat(made) && body.plane_of(made).is_none(),
        "{what}: the face the fillet lays is curved, and offers no plane to draw on"
    );
    made
}

/// Holds that a step reports the value it was asked, `asked`, and the smaller
/// one the edge's two faces gave it, `taken`.
fn reduced(asked: f64, taken: f64, [was_asked, gave]: [f64; 2], what: &str) {
    assert!(
        (asked - was_asked).abs() < PLACE,
        "{what}: reports {asked} asked, where {was_asked} was"
    );
    assert!(
        (taken - gave).abs() < PLACE,
        "{what}: took {taken}, where the faces give {gave}"
    );
}

/// Holds that a step took the face numbered `consumed` away whole, left every
/// other face of the part it was given, and made one face more; and gives
/// back the number that face answers to.
fn one_face_for_another(part: &Body, body: &Body, consumed: usize, what: &str) -> usize {
    let (mut kept, after) = (numbers(part), numbers(body));
    assert!(
        !after.contains(&consumed),
        "{what}: face {consumed} still answers: {after:?}"
    );
    kept.remove(&consumed);
    assert!(
        kept.is_subset(&after),
        "{what}: a face other than {consumed} went: {after:?}"
    );
    let made: Vec<usize> = after.difference(&kept).copied().collect();
    assert_eq!(
        made.len(),
        1,
        "{what}: one face for face {consumed}: {after:?}"
    );
    made[0]
}

/// Holds what [`one_face_for_another`] holds, the face made flat.
fn one_flat_face_for_another(part: &Body, body: &Body, consumed: usize, what: &str) -> usize {
    let made = one_face_for_another(part, body, consumed, what);
    assert!(
        body.is_flat(made),
        "{what}: the face the chamfer lays is flat"
    );
    made
}

/// Holds what [`one_face_for_another`] holds, the face made curved.
fn one_round_for_another(part: &Body, body: &Body, consumed: usize, what: &str) -> usize {
    let made = one_face_for_another(part, body, consumed, what);
    assert!(
        !body.is_flat(made) && body.plane_of(made).is_none(),
        "{what}: the face the fillet lays is curved, and offers no plane to draw on"
    );
    made
}

/// Holds that a flat face faces the way `toward` points.
fn faces_toward(body: &Body, face: usize, toward: DVec3, what: &str) {
    let facing = body
        .plane_of(face)
        .expect("a flat face offers a plane")
        .normal;
    assert!(
        facing.distance(toward.normalize()) < PLACE,
        "{what}: the face faces {facing}, where it faces {}",
        toward.normalize()
    );
}

/// The largest coordinate a body reaches, on either side of the origin.
fn reach_of(body: &Body) -> f64 {
    let (low, high) = body.bounds().expect("a part has a box");
    low.abs().max(high.abs()).max_element()
}

/// What the triangles of the body a step left on `part` may enclose apart
/// from its matter, its curved faces `area` in all: the drawing's tolerance
/// at the part's own reach, over them.
fn drawn_within(part: &Body, area: f64) -> f64 {
    DRAWN * reach_of(part) * area
}

/// The round a fillet lays in a concave corner: a cylinder of `radius` about
/// the line from `start` along `along` for `length`, that line standing in
/// the air the two faces leave between them. The round turns from facing the
/// way the edge's first face does to facing the way its second does, and
/// everywhere faces its axis, a radius away against the way it faces. Facing
/// each face's way where it meets it is what tangent to both comes to.
struct Hollow {
    start: DVec3,
    along: DVec3,
    length: f64,
    radius: f64,
    facing: [DVec3; 2],
}

impl Hollow {
    /// The point of the axis `share` of the way along it.
    fn axis_at(&self, share: f64) -> DVec3 {
        self.start + self.along * self.length * share
    }

    /// The way the round faces `share` of the way round it, from its first
    /// face's way to its second's.
    fn facing_at(&self, share: f64) -> DVec3 {
        let [first, second] = self.facing;
        let turn = first.dot(second).acos() * share;
        let across = (second - first * first.dot(second)).normalize();
        first * turn.cos() + across * turn.sin()
    }

    /// Whether a point lies on the round: on its cylinder, between its two
    /// ends, and between its two sides — no further round from the way it
    /// faces halfway than either side is.
    fn holds(&self, point: DVec3) -> bool {
        let off = point - self.start;
        let along = off.dot(self.along);
        let toward_the_axis = self.along * along - off;
        let halfway = self.facing_at(0.5);
        let least = self.radius * self.facing[0].dot(halfway);
        (toward_the_axis.length() - self.radius).abs() < PLACE
            && (-PLACE..=self.length + PLACE).contains(&along)
            && toward_the_axis.dot(halfway) >= least - PLACE
    }

    /// The round's area, its turn times its radius times its length.
    fn area(&self) -> f64 {
        let [first, second] = self.facing;
        first.dot(second).acos() * self.radius * self.length
    }
}

/// Holds that the face numbered `face` is the hollow round expected: every
/// corner of its triangles on it, and every ray aimed from its axis, in the
/// air the round leaves, out to it meeting it first, where it faces back
/// along the ray.
fn is_the_hollow(body: &Body, face: usize, hollow: &Hollow, what: &str) {
    let corners: Vec<DVec3> = body.triangles_of(face).flatten().collect();
    assert!(!corners.is_empty(), "{what}: the round is drawn");
    for corner in corners {
        assert!(
            hollow.holds(corner),
            "{what}: the round is drawn through {corner}, off the cylinder of {} about the line \
             from {} along {}",
            hollow.radius,
            hollow.start,
            hollow.along
        );
    }
    for along in [0.11, 0.47, 0.83] {
        for share in [0.1, 0.3, 0.5, 0.7, 0.9] {
            let facing = hollow.facing_at(share);
            let from = hollow.axis_at(along);
            let hit = body
                .ray_hit(from, -facing)
                .unwrap_or_else(|| panic!("{what}: a ray from {from} on the axis meets nothing"));
            assert_eq!(
                hit.face, face,
                "{what}: a ray from {from} on the axis meets face {} before the round",
                hit.face
            );
            assert!(
                hit.normal.distance(facing) < PLACE,
                "{what}: where a ray from {from} meets it, the round faces {}, not {facing}, \
                 toward its axis",
                hit.normal
            );
        }
    }
}

#[test]
fn a_chamfer_on_the_inner_corner_of_an_l_fills_it_with_one_flat_face_set_back_by_the_distance_along_each_face()
 {
    let distance = 3.0;
    let ell = ELL.on(ground());
    let inner = edge_between(&ell, ARM_BACK, STEM_RIGHT);

    let chamfered = ell
        .chamfer(&inner, distance)
        .unwrap_or_else(|declined| panic!("the inner corner of the L: declined, {declined:?}"));

    let what = "the inner corner of the L";
    taken_whole(chamfered.asked, chamfered.taken, distance, what);
    let body = &chamfered.body;
    let Ell {
        length,
        depth,
        stem,
        back,
        height,
    } = ELL;
    sound(
        body,
        ELL.volume() + distance * distance / 2.0 * height,
        what,
    );
    let chamfer = one_flat_face_more(&ell, body, what);
    faces_toward(
        body,
        chamfer,
        DVec3::new(1.0, 1.0, 0.0),
        "the chamfer, halfway between the arm's back and the stem's right side, out of the matter",
    );
    let on_arm_back = DVec2::new(stem + distance, depth);
    let on_stem_right = DVec2::new(stem, depth + distance);
    corners_are(
        body,
        chamfer,
        &[upright(on_arm_back, height), upright(on_stem_right, height)].concat(),
        "the chamfer, set back by the distance along the arm's back and the stem's right side",
    );
    corners_are(
        body,
        ARM_BACK,
        &[
            upright(on_arm_back, height),
            upright(DVec2::new(length, depth), height),
        ]
        .concat(),
        "what is left of the arm's back",
    );
    corners_are(
        body,
        STEM_RIGHT,
        &[
            upright(on_stem_right, height),
            upright(DVec2::new(stem, back), height),
        ]
        .concat(),
        "what is left of the stem's right side",
    );
    let filled = ELL.outline_filled([on_arm_back, on_stem_right]);
    for (cap, at, whose) in [(FLOOR, 0.0, "the floor"), (TOP, height, "the top")] {
        corners_are(
            body,
            cap,
            &lifted_to(&filled, at),
            &format!("{whose}, holding the chamfer's end flush, the inner corner gone"),
        );
    }
    assert_eq!(
        sides_met(body, chamfer),
        [FLOOR, TOP, ARM_BACK, STEM_RIGHT].map(|side| vec![side as u32]),
        "the chamfer meets the arm's back and the stem's right side along its sides, and the \
         floor and the top at its ends, by one edge each"
    );
    no_longer_meet(
        body,
        ARM_BACK,
        STEM_RIGHT,
        "the arm's back and the stem's right side no longer meet: the chamfer stands between them",
    );
    within_the_box_of(&ell, body, what);
}

#[test]
fn a_fillet_on_the_inner_corner_of_an_l_fills_it_with_one_round_of_the_radius_tangent_to_both_faces()
 {
    let radius = 3.0;
    let ell = ELL.on(ground());
    let inner = edge_between(&ell, ARM_BACK, STEM_RIGHT);

    let filleted = ell
        .fillet(&inner, radius)
        .unwrap_or_else(|declined| panic!("the inner corner of the L: declined, {declined:?}"));

    let what = "the inner corner of the L";
    taken_whole(filleted.asked, filleted.taken, radius, what);
    let body = &filleted.body;
    let Ell {
        length,
        depth,
        stem,
        back,
        height,
    } = ELL;
    let hollow = Hollow {
        start: DVec3::new(stem + radius, depth + radius, 0.0),
        along: DVec3::Z,
        length: height,
        radius,
        facing: [DVec3::Y, DVec3::X],
    };
    sound_drawn_within(
        body,
        ELL.volume() + radius * radius * (1.0 - PI / 4.0) * height,
        drawn_within(&ell, hollow.area()),
        what,
    );
    let fillet = one_round_more(&ell, body, what);
    is_the_hollow(
        body,
        fillet,
        &hollow,
        "the round, its axis the radius out from the arm's back and from the stem's right side",
    );
    let on_arm_back = DVec2::new(stem + radius, depth);
    let on_stem_right = DVec2::new(stem, depth + radius);
    corners_are(
        body,
        ARM_BACK,
        &[
            upright(on_arm_back, height),
            upright(DVec2::new(length, depth), height),
        ]
        .concat(),
        "what is left of the arm's back, set back by the radius",
    );
    corners_are(
        body,
        STEM_RIGHT,
        &[
            upright(on_stem_right, height),
            upright(DVec2::new(stem, back), height),
        ]
        .concat(),
        "what is left of the stem's right side, set back by the radius",
    );
    let filled = ELL.outline_filled([on_arm_back, on_stem_right]);
    for (cap, at, whose) in [(FLOOR, 0.0, "the floor"), (TOP, height, "the top")] {
        corners_are_or_on(
            body,
            cap,
            &lifted_to(&filled, at),
            |corner| hollow.holds(corner),
            &format!("{whose}, holding the round's end flush, the inner corner gone"),
        );
    }
    assert_eq!(
        sides_met(body, fillet),
        [FLOOR, TOP, ARM_BACK, STEM_RIGHT].map(|side| vec![side as u32]),
        "the round meets the arm's back and the stem's right side along its sides, and the floor \
         and the top at its ends, by one edge each"
    );
    no_longer_meet(
        body,
        ARM_BACK,
        STEM_RIGHT,
        "the arm's back and the stem's right side no longer meet: the round stands between them",
    );
    within_the_box_of(&ell, body, what);
}

#[test]
fn a_chamfer_on_a_concave_edge_whose_faces_meet_at_a_slant_is_set_back_by_the_distance_along_each_face()
 {
    let (height, distance) = (10.0, 3.0);

    for inner in INNER_EDGES {
        let what = inner.what;
        let part = notched_step(height);
        let edge = edge_between(&part, inner.walls[0], inner.walls[1]);

        let chamfered = part
            .chamfer(&edge, distance)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        taken_whole(chamfered.asked, chamfered.taken, distance, what);
        let body = &chamfered.body;
        let filled_in = distance * distance * inner.angle().sin() / 2.0 * height;
        sound(body, NOTCHED_STEP_AREA * height + filled_in, what);
        let chamfer = one_flat_face_more(&part, body, what);
        let [first, second] = inner.outward.map(DVec2::normalize);
        faces_toward(
            body,
            chamfer,
            (first + second).extend(0.0),
            &format!("{what}: the chamfer, halfway between its walls, out of the matter"),
        );
        let set_back = inner.set_back(distance);
        corners_are(
            body,
            chamfer,
            &set_back.map(|at| upright(at, height)).concat(),
            &format!("{what}: the chamfer, set back by the distance along each wall"),
        );
        for ((wall, near), far) in inner.walls.into_iter().zip(set_back).zip(inner.far()) {
            corners_are(
                body,
                wall,
                &[upright(near, height), upright(far, height)].concat(),
                &format!("{what}: what is left of wall {wall}"),
            );
        }
        let filled = inner.outline_filled(distance);
        for (cap, at, whose) in [(FLOOR, 0.0, "the floor"), (TOP, height, "the top")] {
            corners_are(
                body,
                cap,
                &lifted_to(&filled, at),
                &format!("{what}: {whose}, holding the chamfer's end flush, the corner gone"),
            );
        }
        assert_eq!(
            sides_met(body, chamfer),
            [FLOOR, TOP, inner.walls[0], inner.walls[1]].map(|side| vec![side as u32]),
            "{what}: the chamfer meets its two walls along its sides, and the floor and the top \
             at its ends, by one edge each"
        );
        no_longer_meet(
            body,
            inner.walls[0],
            inner.walls[1],
            &format!("{what}: the two walls no longer meet, the chamfer stands between them"),
        );
        within_the_box_of(&part, body, what);
    }
}

#[test]
fn a_fillet_on_a_concave_edge_whose_faces_meet_at_a_slant_is_set_back_by_the_radius_over_the_tangent_of_half_the_angle()
 {
    let (height, radius) = (10.0, 3.0);

    for inner in INNER_EDGES {
        let what = inner.what;
        let part = notched_step(height);
        let edge = edge_between(&part, inner.walls[0], inner.walls[1]);

        let filleted = part
            .fillet(&edge, radius)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        taken_whole(filleted.asked, filleted.taken, radius, what);
        let body = &filleted.body;
        let (angle, [first_way, second_way]) = (inner.angle(), inner.ways());
        let set_back = radius / (angle / 2.0).tan();
        let axis =
            inner.corner() + (first_way + second_way).normalize() * radius / (angle / 2.0).sin();
        let hollow = Hollow {
            start: axis.extend(0.0),
            along: DVec3::Z,
            length: height,
            radius,
            facing: inner.outward.map(|way| way.normalize().extend(0.0)),
        };
        let filled_in = (radius * set_back - radius * radius * (PI - angle) / 2.0) * height;
        sound_drawn_within(
            body,
            NOTCHED_STEP_AREA * height + filled_in,
            drawn_within(&part, hollow.area()),
            what,
        );
        let fillet = one_round_more(&part, body, what);
        is_the_hollow(
            body,
            fillet,
            &hollow,
            &format!("{what}: the round, its axis the radius out from each wall"),
        );
        let stops = inner.set_back(set_back);
        for ((wall, near), far) in inner.walls.into_iter().zip(stops).zip(inner.far()) {
            corners_are(
                body,
                wall,
                &[upright(near, height), upright(far, height)].concat(),
                &format!(
                    "{what}: what is left of wall {wall}, set back by the radius over the tangent \
                     of half the angle, {set_back}"
                ),
            );
        }
        let filled = inner.outline_filled(set_back);
        for (cap, at, whose) in [(FLOOR, 0.0, "the floor"), (TOP, height, "the top")] {
            corners_are_or_on(
                body,
                cap,
                &lifted_to(&filled, at),
                |corner| hollow.holds(corner),
                &format!("{what}: {whose}, holding the round's end flush, the corner gone"),
            );
        }
        assert_eq!(
            sides_met(body, fillet),
            [FLOOR, TOP, inner.walls[0], inner.walls[1]].map(|side| vec![side as u32]),
            "{what}: the round meets its two walls along its sides, and the floor and the top at \
             its ends, by one edge each"
        );
        no_longer_meet(
            body,
            inner.walls[0],
            inner.walls[1],
            &format!("{what}: the two walls no longer meet, the round stands between them"),
        );
        within_the_box_of(&part, body, what);
    }
}

/// Holds what a fill set back by `set_back` along the [`leaning_l`]'s arm's
/// back and stem's right side leaves of the faces round it: the two walls cut
/// short, the floor and the leaning top one face each, their outlines taking
/// the fill's section in where the inner corner was — its corners on the
/// fill, `on_the_fill` says, besides — the fill meeting the two walls and the
/// two ends and nothing else, and nothing standing past the leaning top.
fn filled_up_to_the_lean(
    part: &Body,
    body: &Body,
    made: usize,
    set_back: f64,
    on_the_fill: impl Fn(DVec3) -> bool,
    what: &str,
) {
    let Ell {
        length,
        depth,
        stem,
        back,
        ..
    } = ELL;
    let on_arm_back = DVec2::new(stem + set_back, depth);
    let on_stem_right = DVec2::new(stem, depth + set_back);
    let walls = [
        (
            ARM_BACK,
            on_arm_back,
            DVec2::new(length, depth),
            "the arm's back",
        ),
        (
            STEM_RIGHT,
            on_stem_right,
            DVec2::new(stem, back),
            "the stem's right side",
        ),
    ];
    for (wall, near, far, whose) in walls {
        corners_are(
            body,
            wall,
            &[
                near.extend(0.0),
                on_the_lean(near),
                far.extend(0.0),
                on_the_lean(far),
            ],
            &format!("{what}: what is left of {whose}, under the leaning top"),
        );
    }
    let filled = ELL.outline_filled([on_arm_back, on_stem_right]);
    corners_are_or_on(
        body,
        FLOOR,
        &lifted_to(&filled, 0.0),
        &on_the_fill,
        &format!("{what}: the floor, holding the fill's lower end flush, the inner corner gone"),
    );
    let on_the_leaning_top: Vec<DVec3> = filled.iter().map(|&at| on_the_lean(at)).collect();
    corners_are_or_on(
        body,
        LEANING,
        &on_the_leaning_top,
        &on_the_fill,
        &format!(
            "{what}: the leaning top, holding the fill's upper end flush, the inner corner gone"
        ),
    );
    faces_toward(
        body,
        LEANING,
        LEANING_FACING,
        &format!("{what}: the leaning top, one plane still"),
    );
    assert_eq!(
        sides_met(body, made),
        [FLOOR, ARM_BACK, STEM_RIGHT, LEANING].map(|side| vec![side as u32]),
        "{what}: the fill meets the arm's back and the stem's right side along its sides, and the \
         floor and the leaning top at its ends, by one edge each"
    );
    no_longer_meet(
        body,
        ARM_BACK,
        STEM_RIGHT,
        &format!("{what}: the arm's back and the stem's right side no longer meet"),
    );
    nothing_stands_past(
        body,
        LEANING_FACING,
        8.0,
        &format!("{what}: trimmed flush by the leaning top"),
    );
    within_the_box_of(part, body, what);
}

#[test]
fn a_chamfer_and_a_fillet_on_a_concave_edge_ending_on_an_oblique_wall_are_trimmed_flush_by_it() {
    let value = 3.0;
    let part = leaning_l();
    let inner = edge_between(&part, ARM_BACK, STEM_RIGHT);
    let corner = DVec2::new(ELL.stem, ELL.depth);

    let chamfered = part.chamfer(&inner, value).unwrap_or_else(|declined| {
        panic!("the chamfer up to the leaning top: declined, {declined:?}")
    });

    let what = "the chamfer up to the leaning top";
    taken_whole(chamfered.asked, chamfered.taken, value, what);
    let body = &chamfered.body;
    let section = value * value / 2.0;
    let filled_in = section * leaning_height(corner + DVec2::splat(value / 3.0));
    sound(body, leaning_l_volume() + filled_in, what);
    let chamfer = one_flat_face_more(&part, body, what);
    faces_toward(
        body,
        chamfer,
        DVec3::new(1.0, 1.0, 0.0),
        &format!("{what}: halfway between the arm's back and the stem's right side"),
    );
    let ends = [
        corner + DVec2::new(value, 0.0),
        corner + DVec2::new(0.0, value),
    ];
    corners_are(
        body,
        chamfer,
        &ends
            .iter()
            .flat_map(|&at| [at.extend(0.0), on_the_lean(at)])
            .collect::<Vec<_>>(),
        &format!("{what}: the chamfer, its upper end on the leaning top"),
    );
    filled_up_to_the_lean(&part, body, chamfer, value, |_| false, what);

    let filleted = part.fillet(&inner, value).unwrap_or_else(|declined| {
        panic!("the fillet up to the leaning top: declined, {declined:?}")
    });

    let what = "the fillet up to the leaning top";
    taken_whole(filleted.asked, filleted.taken, value, what);
    let body = &filleted.body;
    let hollow = Hollow {
        start: (corner + DVec2::splat(value)).extend(0.0),
        along: DVec3::Z,
        length: leaning_height(ends[1]),
        radius: value,
        facing: [DVec3::Y, DVec3::X],
    };
    let square = value * value * leaning_height(corner + DVec2::splat(value / 2.0));
    let quarter_disc_centre = corner + DVec2::splat(value - 4.0 * value / (3.0 * PI));
    let quarter_disc = PI * value * value / 4.0 * leaning_height(quarter_disc_centre);
    sound_drawn_within(
        body,
        leaning_l_volume() + square - quarter_disc,
        drawn_within(&part, hollow.area()),
        what,
    );
    let fillet = one_round_more(&part, body, what);
    is_the_hollow(
        body,
        fillet,
        &hollow,
        &format!("{what}: the round, its axis the radius out from both walls"),
    );
    filled_up_to_the_lean(
        &part,
        body,
        fillet,
        value,
        |corner| hollow.holds(corner),
        what,
    );
}

/// Holds what a fill set back by `set_back` along the plate's top and the
/// rib's right side of the [`ribbed_plate`] leaves of the faces round it: the
/// plate's top meeting the rib's front, back and left side along one edge
/// each still, the fill turning round no end of the rib and leaving each of
/// those edges whole, for a name to find; the rib's right side cut short;
/// the plate's top one face round the rib and the fill's footprint; the
/// rib's front and back one face each, grown by the fill's section, flush —
/// its corners on the fill, `on_the_fill` says, besides; and the fill
/// meeting those four faces and nothing else.
fn filled_along_the_foot_of_the_rib(
    part: &Body,
    body: &Body,
    made: usize,
    set_back: f64,
    on_the_fill: impl Fn(DVec3) -> bool,
    what: &str,
) {
    let (out, up, top) = (RIB_SIDE + set_back, PLATE + set_back, PLATE + 10.0);
    for wall in [RIB_FRONT, RIB_BACK, RIB_LEFT] {
        let sides = [vec![TOP as u32], vec![wall as u32]];
        let meeting: Vec<EdgeName> = body
            .edge_names()
            .into_iter()
            .filter(|name| name.between == sides)
            .collect();
        assert_eq!(
            meeting.len(),
            1,
            "{what}: the plate's top meets face {wall} along one edge still, unfilled: {meeting:?}"
        );
    }
    corners_are(
        body,
        RIB_RIGHT,
        &RIB_ENDS
            .iter()
            .flat_map(|&y| [DVec3::new(RIB_SIDE, y, up), DVec3::new(RIB_SIDE, y, top)])
            .collect::<Vec<_>>(),
        &format!("{what}: what is left of the rib's right side"),
    );
    let [front, back] = RIB_ENDS;
    corners_are(
        body,
        TOP,
        &[
            DVec3::new(0.0, 0.0, PLATE),
            DVec3::new(40.0, 0.0, PLATE),
            DVec3::new(40.0, 30.0, PLATE),
            DVec3::new(0.0, 30.0, PLATE),
            DVec3::new(18.0, front, PLATE),
            DVec3::new(out, front, PLATE),
            DVec3::new(out, back, PLATE),
            DVec3::new(18.0, back, PLATE),
        ],
        &format!("{what}: the plate's top, one face round the rib and the fill's footprint"),
    );
    for (wall, y, whose) in [(RIB_FRONT, front, "front"), (RIB_BACK, back, "back")] {
        corners_are_or_on(
            body,
            wall,
            &[
                DVec3::new(18.0, y, PLATE),
                DVec3::new(out, y, PLATE),
                DVec3::new(RIB_SIDE, y, up),
                DVec3::new(RIB_SIDE, y, top),
                DVec3::new(18.0, y, top),
            ],
            &on_the_fill,
            &format!("{what}: the rib's {whose}, grown by the fill's section, flush"),
        );
    }
    assert_eq!(
        sides_met(body, made),
        [TOP, RIB_FRONT, RIB_RIGHT, RIB_BACK].map(|side| vec![side as u32]),
        "{what}: the fill meets the plate's top and the rib's right side along its sides, and \
         the rib's front and back at its ends, by one edge each"
    );
    no_longer_meet(
        body,
        TOP,
        RIB_RIGHT,
        &format!("{what}: the plate's top and the rib's right side no longer meet"),
    );
    within_the_box_of(part, body, what);
}

#[test]
fn a_chamfer_and_a_fillet_at_the_foot_of_a_rib_ending_short_of_its_plate_s_edge_are_trimmed_flush_by_the_rib_s_end()
 {
    let value = 3.0;
    let part = ribbed_plate();
    let foot = edge_between(&part, TOP, RIB_RIGHT);
    let long = RIB_ENDS[1] - RIB_ENDS[0];

    let chamfered = part
        .chamfer(&foot, value)
        .unwrap_or_else(|declined| panic!("the chamfer at the rib's foot: declined, {declined:?}"));

    let what = "the chamfer at the rib's foot";
    taken_whole(chamfered.asked, chamfered.taken, value, what);
    let body = &chamfered.body;
    sound(body, RIBBED_VOLUME + value * value / 2.0 * long, what);
    let chamfer = one_flat_face_more(&part, body, what);
    faces_toward(
        body,
        chamfer,
        DVec3::new(1.0, 0.0, 1.0),
        &format!("{what}: halfway between the plate's top and the rib's right side"),
    );
    corners_are(
        body,
        chamfer,
        &RIB_ENDS
            .iter()
            .flat_map(|&y| {
                [
                    DVec3::new(RIB_SIDE + value, y, PLATE),
                    DVec3::new(RIB_SIDE, y, PLATE + value),
                ]
            })
            .collect::<Vec<_>>(),
        &format!("{what}: the chamfer, from the rib's front to its back"),
    );
    filled_along_the_foot_of_the_rib(&part, body, chamfer, value, |_| false, what);

    let filleted = part
        .fillet(&foot, value)
        .unwrap_or_else(|declined| panic!("the fillet at the rib's foot: declined, {declined:?}"));

    let what = "the fillet at the rib's foot";
    taken_whole(filleted.asked, filleted.taken, value, what);
    let body = &filleted.body;
    let hollow = Hollow {
        start: DVec3::new(RIB_SIDE + value, RIB_ENDS[0], PLATE + value),
        along: DVec3::Y,
        length: long,
        radius: value,
        facing: [DVec3::Z, DVec3::X],
    };
    sound_drawn_within(
        body,
        RIBBED_VOLUME + value * value * (1.0 - PI / 4.0) * long,
        drawn_within(&part, hollow.area()),
        what,
    );
    let fillet = one_round_more(&part, body, what);
    is_the_hollow(
        body,
        fillet,
        &hollow,
        &format!("{what}: the round, its axis the radius out from the plate's top and the rib"),
    );
    filled_along_the_foot_of_the_rib(
        &part,
        body,
        fillet,
        value,
        |corner| hollow.holds(corner),
        what,
    );
}

/// Holds what a fill that took an [`Overreach`]'s second wall whole leaves of
/// the faces round it: the first wall cut short where the fill stops; the
/// face beyond whole, the fill meeting it where the second wall did; the
/// floor and the top one face each, their outlines taking the fill's section
/// in where the corner was — its corners on the fill, `on_the_fill` says,
/// besides; the fill meeting the first wall, the face beyond and the two ends
/// and nothing else; and nothing standing past the face beyond, into the air
/// past the wall taken whole.
fn filled_to_the_face_beyond(
    part: &Body,
    body: &Body,
    edge: &Overreach,
    made: usize,
    on_the_fill: impl Fn(DVec3) -> bool,
    what: &str,
) {
    let height = edge.height;
    corners_are(
        body,
        edge.walls[0],
        &[upright(edge.stop(), height), upright(edge.far()[0], height)].concat(),
        &format!("{what}: what is left of the first wall, set back by the second's whole width"),
    );
    let beyond = [edge.outline[edge.at + 1], edge.outline[edge.at + 2]];
    corners_are(
        body,
        edge.beyond,
        &beyond.map(|at| upright(at, height)).concat(),
        &format!("{what}: the face beyond the wall taken whole, whole"),
    );
    let filled = edge.outline_filled();
    for (cap, at, whose) in [(FLOOR, 0.0, "the floor"), (TOP, height, "the top")] {
        corners_are_or_on(
            body,
            cap,
            &lifted_to(&filled, at),
            &on_the_fill,
            &format!("{what}: {whose}, holding the fill's end flush, the corner gone"),
        );
    }
    let mut sides = [FLOOR, TOP, edge.walls[0], edge.beyond].map(|side| vec![side as u32]);
    sides.sort();
    assert_eq!(
        sides_met(body, made),
        sides,
        "{what}: the fill meets the first wall and the face beyond along its sides, and the floor \
         and the top at its ends, by one edge each"
    );
    let facing = edge.beyond_facing.extend(0.0);
    nothing_stands_past(
        body,
        facing,
        facing.dot(beyond[0].extend(0.0)),
        &format!("{what}: nothing past the face beyond, where the wall taken whole ended"),
    );
    within_the_box_of(part, body, what);
}

#[test]
fn a_chamfer_on_a_concave_edge_asked_more_than_a_face_can_give_takes_the_whole_face_and_reports_both_values()
 {
    let asked = 50.0;

    for edge in overreaching() {
        let (what, gives) = (edge.what, edge.reach());
        let part = edge.part();
        let name = edge_between(&part, edge.walls[0], edge.walls[1]);

        let chamfered = part
            .chamfer(&name, asked)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        reduced(chamfered.asked, chamfered.taken, [asked, gives], what);
        let body = &chamfered.body;
        let filled_in = gives * gives * edge.angle().sin() / 2.0 * edge.height;
        sound(body, edge.volume + filled_in, what);
        let chamfer = one_flat_face_for_another(&part, body, edge.walls[1], what);
        let [first, second] = edge.outward.map(DVec2::normalize);
        faces_toward(
            body,
            chamfer,
            (first + second).extend(0.0),
            &format!("{what}: the chamfer, halfway between its walls"),
        );
        corners_are(
            body,
            chamfer,
            &[
                upright(edge.stop(), edge.height),
                upright(edge.far()[1], edge.height),
            ]
            .concat(),
            &format!("{what}: the chamfer, from the first wall to where the second ended"),
        );
        filled_to_the_face_beyond(&part, body, &edge, chamfer, |_| false, what);
    }
}

#[test]
fn a_fillet_on_a_concave_edge_asked_more_than_a_face_can_give_takes_the_radius_whose_set_back_is_the_whole_face_and_reports_both_radii()
 {
    let asked = 50.0;

    for edge in overreaching() {
        let what = edge.what;
        let part = edge.part();
        let name = edge_between(&part, edge.walls[0], edge.walls[1]);

        let filleted = part
            .fillet(&name, asked)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        let half = edge.angle() / 2.0;
        let gives = edge.reach() * half.tan();
        reduced(filleted.asked, filleted.taken, [asked, gives], what);
        let body = &filleted.body;
        let [first_way, second_way] = edge.ways();
        let axis = edge.corner() + (first_way + second_way).normalize() * gives / half.sin();
        let hollow = Hollow {
            start: axis.extend(0.0),
            along: DVec3::Z,
            length: edge.height,
            radius: gives,
            facing: edge.outward.map(|way| way.normalize().extend(0.0)),
        };
        let section = gives * edge.reach() - gives * gives * (PI - edge.angle()) / 2.0;
        sound_drawn_within(
            body,
            edge.volume + section * edge.height,
            drawn_within(&part, hollow.area()),
            what,
        );
        let fillet = one_round_for_another(&part, body, edge.walls[1], what);
        is_the_hollow(
            body,
            fillet,
            &hollow,
            &format!("{what}: the round of {gives}, tangent to the first wall and the second"),
        );
        filled_to_the_face_beyond(
            &part,
            body,
            &edge,
            fillet,
            |corner| hollow.holds(corner),
            what,
        );
    }
}

#[test]
fn a_chamfer_on_a_concave_edge_just_short_of_a_face_s_whole_width_is_laid_as_asked() {
    for edge in overreaching() {
        let (part, height) = (edge.part(), edge.height);
        let distance = edge.reach() - 0.5;
        let what = &format!("{distance} asked on {}", edge.what);
        let name = edge_between(&part, edge.walls[0], edge.walls[1]);

        let chamfered = part
            .chamfer(&name, distance)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        taken_whole(chamfered.asked, chamfered.taken, distance, what);
        let body = &chamfered.body;
        let filled_in = distance * distance * edge.angle().sin() / 2.0 * height;
        sound(body, edge.volume + filled_in, what);
        let chamfer = one_flat_face_more(&part, body, what);
        let stops = edge.ways().map(|way| edge.corner() + way * distance);
        corners_are(
            body,
            chamfer,
            &stops.map(|at| upright(at, height)).concat(),
            &format!("{what}: the chamfer, set back by the distance along each wall"),
        );
        for ((wall, stop), far) in edge.walls.into_iter().zip(stops).zip(edge.far()) {
            corners_are(
                body,
                wall,
                &[upright(stop, height), upright(far, height)].concat(),
                &format!("{what}: what is left of wall {wall}, cut short and still there"),
            );
        }
        let beyond = [edge.outline[edge.at + 1], edge.outline[edge.at + 2]];
        corners_are(
            body,
            edge.beyond,
            &beyond.map(|at| upright(at, height)).concat(),
            &format!("{what}: the face beyond the second wall, whole"),
        );
        assert_eq!(
            sides_met(body, chamfer),
            [FLOOR, TOP, edge.walls[0], edge.walls[1]].map(|side| vec![side as u32]),
            "{what}: the chamfer meets its two walls along its sides, and the floor and the top at \
             its ends, by one edge each"
        );
        let facing = edge.beyond_facing.extend(0.0);
        nothing_stands_past(
            body,
            facing,
            facing.dot(beyond[0].extend(0.0)),
            &format!("{what}: nothing past the face beyond the second wall"),
        );
        within_the_box_of(&part, body, what);
    }
}
