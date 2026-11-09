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
//!   `a_chamfer_and_a_fillet_at_the_foot_of_a_rib_ending_short_of_its_plate_s_edge_are_trimmed_flush_by_the_rib_s_end`;
//!   a rib whose two ends both slant across its foot, the fill trimmed by
//!   each — `a_chamfer_and_a_fillet_at_the_foot_of_a_rib_whose_ends_both_slant_are_trimmed_flush_by_each`;
//!   the edges round the rib's ends made one again however the rib stands, a
//!   hole bored through it above the fill —
//!   `a_chamfer_and_a_fillet_at_the_foot_of_a_rib_bored_through_above_them_join_the_edges_round_its_ends_however_it_stands`;
//!   and a fill of 1 there as one of 3 —
//!   `a_chamfer_and_a_fillet_of_1_at_the_foot_of_a_rib_fill_it_as_one_of_3_does`
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
//!   round and this file now speaks for, and the filled bodies the last two now
//!   hold to their listing besides the cut ones
//!
//! What must not break holds too: a join merges its edges as it did before
//! the fill's corners came into it, so an existing part's edges, and their
//! names, stay where they were —
//! `two_blocks_joined_at_two_heights_keep_their_floor_s_front_and_back_edges_in_two_as_the_join_always_has`
//! — and a fill leaves them so wherever it does not reach —
//! `a_chamfer_and_a_fillet_at_the_step_between_two_blocks_leave_their_floor_s_edges_as_the_join_did`.
//! They sit beside the fill rather than with the joins because the rule they
//! guard was opened for the fill alone, and whoever opens it further reads
//! them here; and they ask nothing but the names a part's user sees.
//!
//! What the issue asks besides its Done when is held here too. An edge a join
//! bridged from two is filled whole, as #696 and #697 cut it —
//! `a_chamfer_and_a_fillet_on_the_concave_edge_a_join_bridged_from_two_fill_it_whole`.
//! An end the kernel cannot build is declined with its reason, `Unsupported`,
//! by both steps — an end on a curved face is #788's, and
//! `a_chamfer_and_a_fillet_fill_a_concave_edge_up_to_a_curved_end.rs` speaks
//! for it: an end on a face turned back along the edge, the foot of an L
//! standing on a plate, where three concave edges meet — #698 and #699's
//! corner —
//! `a_chamfer_and_a_fillet_on_a_concave_edge_ending_on_a_face_turned_back_along_it_are_declined_as_unsupported`;
//! an end where two faces meet,
//! `a_chamfer_and_a_fillet_on_a_concave_edge_ending_where_two_faces_meet_are_declined_as_unsupported`;
//! and an end whose plane would carry the fill out past the part,
//! `a_chamfer_and_a_fillet_whose_fill_a_rib_s_slanted_end_would_carry_past_the_plate_s_front_are_declined_as_unsupported`.
//! A fill whose prism, sunk into the matter by its margin to join it, would
//! come out through a wall thinner than that margin is a limit of the way the
//! fill is joined rather than of the geometry: it is laid and sound, or
//! declined `Unsupported`, and never a wrong body —
//! `a_chamfer_and_a_fillet_at_a_wall_thinner_than_their_margin_are_laid_soundly_or_declined_never_a_wrong_body`.
//! An edge nearly flat is filled with the
//! matter arithmetic says, as the convex side's bent fronts are cut —
//! `a_chamfer_and_a_fillet_on_a_concave_edge_nearly_flat_fill_it_with_the_matter_arithmetic_says`.

mod step_reader;

use std::collections::BTreeSet;
use std::f64::consts::{PI, TAU};

use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, Declined, EdgeName, Loop, Lost, step};
use glam::{DQuat, DVec2, DVec3};

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

/// The numbers the L's front, its arm's right end and its left answer to:
/// walls no fill of its inner corner reaches.
const ELL_FRONT: usize = 2;
const ARM_END: usize = 3;
const ELL_LEFT: usize = 7;

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

/// Where a part is moved to, whole.
const ELSEWHERE: [f64; 3] = [100.0, -40.0, 7.0];

/// The ground's plane with its origin moved to `to`.
fn moved(to: [f64; 3]) -> Frame {
    Frame {
        origin: DVec3::from(to),
        ..ground()
    }
}

/// The ground's plane turned about the vertical by `degrees`: a part drawn
/// on it stands upright, its walls square to no axis.
fn turned_about_the_vertical(degrees: f64) -> Frame {
    let turn = DQuat::from_rotation_z(degrees.to_radians());
    Frame {
        origin: DVec3::ZERO,
        u: turn * DVec3::X,
        v: turn * DVec3::Y,
    }
}

/// The ground's plane tipped by 40 degrees about an axis leaning off all
/// three, its origin moved to `to`: a part drawn on it has no face and no
/// edge square to an axis or along one.
fn tipped(to: [f64; 3]) -> Frame {
    let turn = DQuat::from_axis_angle(DVec3::new(1.0, 2.0, 2.0).normalize(), 40f64.to_radians());
    Frame {
        origin: DVec3::from(to),
        u: turn * DVec3::X,
        v: turn * DVec3::Y,
    }
}

/// A point given in the part's own axes, along, across and up, where the
/// frame puts it.
fn placed(frame: Frame, [along, across, up]: [f64; 3]) -> DVec3 {
    frame.at(DVec2::new(along, across)) + frame.normal() * up
}

/// The part with an outline drawn on the ground, raised from `from` up by
/// `height`, cut from it as `cao_part` cuts a step: the tool's faces numbered
/// past the part's.
fn cut_from(part: &Body, outline: Contour, from: f64, height: f64) -> Body {
    let frame = Frame {
        origin: DVec3::Z * from,
        ..ground()
    };
    let tool = part
        .tool_raised(&profile(outline), frame, DVec3::Z * height)
        .expect("the tool raises");
    part.difference(&tool).expect("the cut is built")
}

/// The part with an outline drawn on the ground raised up by `height`,
/// joined to it as `cao_part` joins a step: the tool's faces numbered past
/// the part's.
fn joined_to(part: &Body, outline: Contour, height: f64) -> Body {
    let tool = part
        .tool_raised(&profile(outline), ground(), DVec3::Z * height)
        .expect("the tool raises");
    part.union(&tool).expect("the join is built")
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

const ELL_RESIZED: Ell = Ell {
    length: 40.0,
    depth: 8.0,
    stem: 16.0,
    back: 20.0,
    height: 12.0,
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
    cut_down_to(&ELL.on(ground()), LEANING_FACING, 8.0)
}

/// The part with what stands in front of the plane of the points p where
/// `facing` · p = `offset` cut away, as a part's next step cuts it: a box
/// drawn on a sketch lying on the plane, wider than an L, raised up off it
/// along `facing`, which leans up, and taken away.
fn cut_down_to(part: &Body, facing: DVec3, offset: f64) -> Body {
    let u = DVec3::new(1.0, 0.0, -facing.x / facing.z).normalize();
    let up = facing.normalize();
    let frame = Frame {
        origin: DVec3::Z * offset / facing.z,
        u,
        v: up.cross(u),
    };
    let wider = Contour::rectangle(DVec2::new(-4.0, -4.0), DVec2::new(36.0, 20.0));
    let tool = part
        .tool_raised(&profile(wider), frame, up * 10.0)
        .expect("the tool raises");
    part.difference(&tool).expect("the cut is built")
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
    let plate = plate(PLATE);
    let rib = Contour::rectangle(
        DVec2::new(18.0, RIB_ENDS[0]),
        DVec2::new(RIB_SIDE, RIB_ENDS[1]),
    );
    on_the_plate(&plate, PLATE, rib)
}

/// A plate 40 long and 30 deep, `thickness` thick, raised from the ground
/// and numbered as [`raised_on`] numbers it.
fn plate(thickness: f64) -> Body {
    raised_on(
        ground(),
        Contour::rectangle(DVec2::ZERO, DVec2::new(40.0, 30.0)),
        thickness,
    )
}

/// The part with an outline sketched on the plate's top, `thickness` up,
/// raised 10 and joined as a part's next step is: its floor, which the join
/// takes in, and its top numbered past the part's faces, then its walls.
fn on_the_plate(part: &Body, thickness: f64, outline: Contour) -> Body {
    let frame = Frame {
        origin: DVec3::Z * thickness,
        ..ground()
    };
    let tool = part
        .tool_raised(&profile(outline), frame, DVec3::Z * 10.0)
        .expect("the tool raises");
    part.union(&tool).expect("the join is built")
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
    drawn_on_the_hollow(body, face, hollow, what);
    met_from_its_axis(body, face, hollow, what);
}

/// Holds that every corner of the triangles of the face numbered `face` lies
/// on the hollow round.
fn drawn_on_the_hollow(body: &Body, face: usize, hollow: &Hollow, what: &str) {
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
}

/// Holds that every ray aimed from the hollow round's axis, a tenth to nine
/// tenths of the way along it, out to it meets the face numbered `face`
/// first, where it faces back along the ray.
fn met_from_its_axis(body: &Body, face: usize, hollow: &Hollow, what: &str) {
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

/// The faces of a rib standing on the plate — its front, its right side, its
/// back and its left side — each as the numbers it answers to: one, or two
/// where a join merged two ribs' faces into one.
type RibFaces = [&'static [u32]; 4];

/// The faces of the [`ribbed_plate`]'s rib.
const RIB: RibFaces = [
    &[RIB_FRONT as u32],
    &[RIB_RIGHT as u32],
    &[RIB_BACK as u32],
    &[RIB_LEFT as u32],
];

/// Holds what a fill set back by `set_back` along the plate's top and the
/// right side of a rib standing from y = 5 to 22, its faces `rib`, leaves of
/// the faces round it: the plate's top meeting the rib's front, back and left
/// side along one edge each still, the fill turning round no end of the rib
/// and leaving each of those edges whole, for a name to find; the rib's right
/// side cut short; the plate's top one face round the rib and the fill's
/// footprint; the rib's front and back one face each, grown by the fill's
/// section, flush — its corners on the fill, `on_the_fill` says, besides; and
/// the fill meeting those four faces and nothing else.
fn filled_along_the_foot_of_the_rib(
    part: &Body,
    body: &Body,
    (made, rib): (usize, RibFaces),
    set_back: f64,
    on_the_fill: impl Fn(DVec3) -> bool,
    what: &str,
) {
    let (out, up, top) = (RIB_SIDE + set_back, PLATE + set_back, PLATE + 10.0);
    let [front_faces, right_faces, back_faces, left_faces] = rib;
    let plate_top = vec![TOP as u32];
    let meeting = |faces: &[u32]| -> Vec<EdgeName> {
        body.edge_names()
            .into_iter()
            .filter(|name| {
                name.between.contains(&plate_top) && name.between.contains(&faces.to_vec())
            })
            .collect()
    };
    for faces in [front_faces, back_faces, left_faces] {
        let meeting = meeting(faces);
        assert_eq!(
            meeting.len(),
            1,
            "{what}: the plate's top meets face {faces:?} along one edge still, unfilled: \
             {meeting:?}"
        );
    }
    corners_are(
        body,
        right_faces[0] as usize,
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
    for (faces, y, whose) in [(front_faces, front, "front"), (back_faces, back, "back")] {
        corners_are_or_on(
            body,
            faces[0] as usize,
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
    let mut sides = [&plate_top[..], front_faces, right_faces, back_faces].map(<[u32]>::to_vec);
    sides.sort();
    assert_eq!(
        sides_met(body, made),
        sides,
        "{what}: the fill meets the plate's top and the rib's right side along its sides, and \
         the rib's front and back at its ends, by one edge each"
    );
    let still = meeting(right_faces);
    assert!(
        still.is_empty(),
        "{what}: the plate's top and the rib's right side no longer meet: {still:?}"
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
    filled_along_the_foot_of_the_rib(&part, body, (chamfer, RIB), value, |_| false, what);

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
        (fillet, RIB),
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

/// Holds what a fill set back by `set_back` at the inner corner of `ell`,
/// drawn on `frame`, leaves of the faces round it, as criterion 1 holds it
/// of the L drawn on the ground: the arm's back and the stem's right side
/// cut short; the floor and the top one face each, their outlines taking the
/// fill's section in where the inner corner was — its corners on the fill,
/// `on_the_fill` says, besides; the fill meeting those four faces and nothing
/// else; and nothing standing past the part's box.
fn filled_where_the_frame_puts_it(
    part: &Body,
    body: &Body,
    made: usize,
    (ell, frame, set_back): (Ell, Frame, f64),
    on_the_fill: impl Fn(DVec3) -> bool,
    what: &str,
) {
    let Ell {
        length,
        depth,
        stem,
        back,
        height,
    } = ell;
    let upright_at = |at: DVec2| [0.0, height].map(|up| placed(frame, [at.x, at.y, up]));
    let on_arm_back = DVec2::new(stem + set_back, depth);
    let on_stem_right = DVec2::new(stem, depth + set_back);
    for (wall, near, far, whose) in [
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
    ] {
        corners_are(
            body,
            wall,
            &[upright_at(near), upright_at(far)].concat(),
            &format!("{what}: what is left of {whose}"),
        );
    }
    let filled = ell.outline_filled([on_arm_back, on_stem_right]);
    for (cap, up, whose) in [(FLOOR, 0.0, "the floor"), (TOP, height, "the top")] {
        let corners: Vec<DVec3> = filled
            .iter()
            .map(|at| placed(frame, [at.x, at.y, up]))
            .collect();
        corners_are_or_on(
            body,
            cap,
            &corners,
            &on_the_fill,
            &format!("{what}: {whose}, holding the fill's end flush, the inner corner gone"),
        );
    }
    assert_eq!(
        sides_met(body, made),
        [FLOOR, TOP, ARM_BACK, STEM_RIGHT].map(|side| vec![side as u32]),
        "{what}: the fill meets the arm's back and the stem's right side along its sides, and the \
         floor and the top at its ends, by one edge each"
    );
    no_longer_meet(
        body,
        ARM_BACK,
        STEM_RIGHT,
        &format!("{what}: the arm's back and the stem's right side no longer meet"),
    );
    within_the_box_of(part, body, what);
}

#[test]
fn a_chamfer_and_a_fillet_fill_the_concave_edge_their_name_finds_again_after_a_change_of_size_a_move_and_a_turn()
 {
    let value = 2.0;
    let name = edge_between(&ELL.on(ground()), ARM_BACK, STEM_RIGHT);

    for (drawn, ell, frame) in [
        ("as drawn", ELL, ground()),
        ("resized", ELL_RESIZED, ground()),
        ("moved", ELL, moved(ELSEWHERE)),
        (
            "turned 30 degrees about the vertical",
            ELL,
            turned_about_the_vertical(30.0),
        ),
        ("tipped off every axis", ELL, tipped([0.0; 3])),
        ("resized, moved and tipped", ELL_RESIZED, tipped(ELSEWHERE)),
    ] {
        let part = ell.on(frame);
        let at = |point: [f64; 3]| placed(frame, point);
        let Ell {
            depth,
            stem,
            height,
            ..
        } = ell;

        let chamfered = part
            .chamfer(&name, value)
            .unwrap_or_else(|declined| panic!("the chamfer, {drawn}: declined, {declined:?}"));

        let what = &format!("the chamfer, {drawn}");
        taken_whole(chamfered.asked, chamfered.taken, value, what);
        let body = &chamfered.body;
        sound(body, ell.volume() + value * value / 2.0 * height, what);
        let chamfer = one_flat_face_more(&part, body, what);
        faces_toward(
            body,
            chamfer,
            at([1.0, 1.0, 0.0]) - at([0.0; 3]),
            &format!("{what}: halfway between the arm's back and the stem's right side"),
        );
        let ends = [[stem + value, depth], [stem, depth + value]];
        corners_are(
            body,
            chamfer,
            &ends
                .iter()
                .flat_map(|&[along, across]| [0.0, height].map(|up| at([along, across, up])))
                .collect::<Vec<_>>(),
            &format!("{what}: the chamfer, set back by the distance along each face"),
        );
        filled_where_the_frame_puts_it(&part, body, chamfer, (ell, frame, value), |_| false, what);

        let filleted = part
            .fillet(&name, value)
            .unwrap_or_else(|declined| panic!("the fillet, {drawn}: declined, {declined:?}"));

        let what = &format!("the fillet, {drawn}");
        taken_whole(filleted.asked, filleted.taken, value, what);
        let body = &filleted.body;
        let hollow = Hollow {
            start: at([stem + value, depth + value, 0.0]),
            along: frame.normal(),
            length: height,
            radius: value,
            facing: [
                at([0.0, 1.0, 0.0]) - at([0.0; 3]),
                at([1.0, 0.0, 0.0]) - at([0.0; 3]),
            ],
        };
        sound_drawn_within(
            body,
            ell.volume() + value * value * (1.0 - PI / 4.0) * height,
            drawn_within(&part, hollow.area()),
            what,
        );
        let fillet = one_round_more(&part, body, what);
        is_the_hollow(
            body,
            fillet,
            &hollow,
            &format!("{what}: the round, its axis the radius out from both faces"),
        );
        filled_where_the_frame_puts_it(
            &part,
            body,
            fillet,
            (ell, frame, value),
            |corner| hollow.holds(corner),
            what,
        );
    }
}

/// The two cuts round the L's inner corner: a square 3 across, from
/// (11, 5) to (14, 8), taking the corner's arm and stem both a little.
fn round_the_inner_corner() -> Contour {
    Contour::rectangle(DVec2::new(11.0, 5.0), DVec2::new(14.0, 8.0))
}

#[test]
fn a_chamfer_and_a_fillet_on_a_concave_edge_a_cut_took_away_are_declined_saying_it_is_gone() {
    let drawn = ELL.on(ground());
    let name = edge_between(&drawn, ARM_BACK, STEM_RIGHT);
    let notched = cut_from(&drawn, round_the_inner_corner(), -1.0, ELL.height + 2.0);
    assert_eq!(
        notched.lost(&name),
        Some(Lost::Gone),
        "a square cut down through the inner corner leaves the arm's back and the stem's right \
         side meeting nowhere"
    );

    let declined = [
        ("the chamfer", notched.chamfer(&name, 1.0).err()),
        ("the fillet", notched.fillet(&name, 1.0).err()),
    ];

    for (step, declined) in declined {
        assert_eq!(declined, Some(Declined::Lost(Lost::Gone)), "{step}");
    }
}

#[test]
fn a_chamfer_and_a_fillet_on_a_concave_edge_a_cut_parted_in_two_are_declined_saying_it_is_split() {
    let drawn = ELL.on(ground());
    let name = edge_between(&drawn, ARM_BACK, STEM_RIGHT);
    let grooved = cut_from(&drawn, round_the_inner_corner(), 4.0, 2.0);
    assert_eq!(
        grooved.lost(&name),
        Some(Lost::Split { now: 2 }),
        "a groove round the inner corner halfway up leaves the edge in two, under it and over it"
    );

    let declined = [
        ("the chamfer", grooved.chamfer(&name, 1.0).err()),
        ("the fillet", grooved.fillet(&name, 1.0).err()),
    ];

    for (step, declined) in declined {
        assert_eq!(
            declined,
            Some(Declined::Lost(Lost::Split { now: 2 })),
            "{step}"
        );
    }
}

#[test]
fn a_chamfer_and_a_fillet_on_a_concave_edge_leave_every_face_of_the_l_its_number_and_number_theirs_after_the_steps_last()
 {
    for (whose, part, steps_end) in [
        ("the L", ELL.on(ground()), 8),
        ("the L cut down to a lean", leaning_l(), 14),
    ] {
        assert_eq!(
            part.faces_end(),
            steps_end,
            "{whose}: its steps numbered their faces up to {steps_end}"
        );
        let inner = edge_between(&part, ARM_BACK, STEM_RIGHT);
        let filled = [
            (
                "chamfer",
                part.chamfer(&inner, 3.0).map(|chamfered| chamfered.body),
                true,
            ),
            (
                "fillet",
                part.fillet(&inner, 3.0).map(|filleted| filleted.body),
                false,
            ),
        ];

        for (step, body, flat) in filled {
            let what = &format!("the {step} on {whose}");
            let body = body.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
            let before = numbers(&part);
            let mut kept_and_made = before.clone();
            kept_and_made.insert(steps_end);
            assert_eq!(
                numbers(&body),
                kept_and_made,
                "{what}: every number its faces answered to, and the fill's, {steps_end}, where \
                 the steps' numbers ended"
            );
            assert!(
                body.has_face(steps_end) && body.is_flat(steps_end) == flat,
                "{what}: face {steps_end} is the {step}'s"
            );
            assert_eq!(
                body.faces_end(),
                steps_end + 1,
                "{what}: the fill names one face, and the next step numbers its own past it"
            );
            assert_eq!(
                body.pieces(),
                part.pieces(),
                "{what}: nothing of the fill's making is numbered as a piece"
            );
            for number in before {
                let was = part.plane_of(number).expect("the L's faces are flat");
                let is = body
                    .plane_of(number)
                    .unwrap_or_else(|| panic!("{what}: face {number} offers no plane"));
                assert!(
                    is.normal.distance(was.normal) < PLACE,
                    "{what}: face {number} faces {}, where it faced {}",
                    is.normal,
                    was.normal
                );
                for corner in &is.corners {
                    let off = (*corner - was.corners[0]).dot(was.normal);
                    assert!(
                        off.abs() < PLACE,
                        "{what}: face {number} has a corner at {corner}, {off} off the plane it \
                         lay on"
                    );
                }
            }
            for untouched in [ELL_FRONT, ARM_END, STEM_BACK, ELL_LEFT] {
                let was = part.plane_of(untouched).expect("the L's faces are flat");
                corners_are(
                    &body,
                    untouched,
                    &was.corners,
                    &format!("{what}: face {untouched}, which the fill does not reach"),
                );
            }

            let then = joined_to(
                &body,
                Contour::rectangle(DVec2::new(40.0, 0.0), DVec2::new(50.0, 10.0)),
                5.0,
            );

            let mut after_the_next_step = kept_and_made.clone();
            after_the_next_step.extend(steps_end + 1..steps_end + 7);
            assert_eq!(
                numbers(&then),
                after_the_next_step,
                "{what}: a block joined after it numbers its six faces past the fill's"
            );
            assert!(
                then.has_face(steps_end) && then.is_flat(steps_end) == flat,
                "{what}: the fill still answers to {steps_end} after the next step"
            );
        }
    }
}

/// Holds that a STEP file read back is one exact solid of the body's faces,
/// all its loops on their surfaces, enclosing the matter arithmetic promised
/// and the body holds, and covering the area arithmetic promised, within the
/// file's tolerance over the part's edges, `edges` long in all.
fn written_whole(
    solid: &step_reader::Solid,
    body: &Body,
    [volume, area, edges]: [f64; 3],
    what: &str,
) {
    assert_eq!(
        solid.faces,
        numbers(body).len(),
        "{what}: the file holds the faces the body does"
    );
    assert!(
        solid.worst <= solid.uncertainty,
        "{what}: a loop stands {} off its face or its edge, the file's tolerance is {}",
        solid.worst,
        solid.uncertainty
    );
    for (whose, volume) in [
        ("arithmetic promised", volume),
        ("the body holds", body.volume()),
    ] {
        assert!(
            (solid.volume - volume).abs() <= solid.uncertainty * solid.area,
            "{what}: the file encloses {}, where {whose} {volume}",
            solid.volume
        );
    }
    assert!(
        (solid.area - area).abs() <= solid.uncertainty * edges,
        "{what}: the file's faces cover {}, where arithmetic promised {area}",
        solid.area
    );
}

#[test]
fn an_l_filled_at_its_inner_corner_written_to_step_writes_its_new_face_on_its_true_surface() {
    let value = 3.0;
    let ell = ELL.on(ground());
    let inner = edge_between(&ell, ARM_BACK, STEM_RIGHT);
    let Ell {
        length,
        depth,
        stem,
        back,
        height,
    } = ELL;
    let perimeter = 2.0 * (length + back);
    let footprint = ELL.volume() / height;
    let walls_short = 2.0 * value * height;
    let edges = 2.0 * (2.0 * perimeter + 6.0 * height);

    let chamfered = ell.chamfer(&inner, value).expect("the chamfer is laid");
    let text = step::of(&chamfered.body, 1.0).expect("an exact body is written");

    let what = "the chamfered L";
    let solid = step_reader::read(&text)
        .unwrap_or_else(|fault| panic!("{what}: the file does not read back: {fault}"));
    assert_eq!(
        solid.faces, 9,
        "{what}: the L's eight faces and the chamfer"
    );
    let section = value * value / 2.0;
    let area = 2.0 * (footprint + section) + perimeter * height - walls_short
        + value * 2f64.sqrt() * height;
    written_whole(
        &solid,
        &chamfered.body,
        [ELL.volume() + section * height, area, edges],
        what,
    );
    let facing = DVec3::new(1.0, 1.0, 0.0).normalize();
    let through = [
        DVec3::new(stem + value, depth, 0.0),
        DVec3::new(stem, depth + value, height),
    ];
    assert!(
        solid.planes.iter().any(|&(origin, normal)| {
            normal.distance(facing) < PLACE
                && through
                    .iter()
                    .all(|point| (*point - origin).dot(normal).abs() <= solid.uncertainty)
        }),
        "{what}: no face of the file faces {facing} through {through:?}: {:?}",
        solid.planes
    );

    let filleted = ell.fillet(&inner, value).expect("the fillet is laid");
    let text = step::of(&filleted.body, 1.0).expect("an exact body is written");

    let what = "the filleted L";
    let solid = step_reader::read(&text)
        .unwrap_or_else(|fault| panic!("{what}: the file does not read back: {fault}"));
    assert_eq!(solid.faces, 9, "{what}: the L's eight faces and the round");
    let section = value * value * (1.0 - PI / 4.0);
    let area =
        2.0 * (footprint + section) + perimeter * height - walls_short + PI / 2.0 * value * height;
    written_whole(
        &solid,
        &filleted.body,
        [ELL.volume() + section * height, area, edges],
        what,
    );
    let [(origin, axis, written)] = solid.cylinders[..] else {
        panic!(
            "{what}: one cylinder, under the round: {:?}",
            solid.cylinders
        );
    };
    assert!(
        (written - value).abs() <= solid.uncertainty,
        "{what}: the round is written on a cylinder of radius {written}, where it is {value}"
    );
    assert!(
        axis.cross(DVec3::Z).length() < PLACE,
        "{what}: the round's cylinder runs along {axis}, where the inner edge stood upright"
    );
    let off = origin - DVec3::new(stem + value, depth + value, 0.0);
    assert!(
        (off - DVec3::Z * off.z).length() <= solid.uncertainty,
        "{what}: the round's cylinder stands about the line through {origin}, where its axis is \
         the radius out from the arm's back and from the stem's right side"
    );
}

/// The faces of two ribs joined end to end on the plate, from y = 5 to 13
/// and from 13 to 22, as [`two_ribs`] numbers them once they abut: the
/// first's front, the right sides merged, the second's back, the left sides
/// merged.
const BRIDGED: RibFaces = [&[8], &[9, 15], &[16], &[11, 17]];

/// A plate 4 thick and on it two ribs joined one after the other, as two
/// steps would: from y = 5 to 13, then from `from` 9 long, both from x = 18
/// to 22 and 10 high. At 13 they abut and the join merges their tops and
/// their sides; apart, each rib's faces answer to its own numbers, the
/// first's from 6 and the second's from 12.
fn two_ribs(from: f64) -> Body {
    let first = on_the_plate(
        &plate(PLATE),
        PLATE,
        Contour::rectangle(DVec2::new(18.0, 5.0), DVec2::new(RIB_SIDE, 13.0)),
    );
    on_the_plate(
        &first,
        PLATE,
        Contour::rectangle(DVec2::new(18.0, from), DVec2::new(RIB_SIDE, from + 9.0)),
    )
}

#[test]
fn a_chamfer_and_a_fillet_on_the_concave_edge_a_join_bridged_from_two_fill_it_whole() {
    let value = 3.0;
    let apart = two_ribs(15.0);
    let names = [
        ("the first rib's foot", edge_between(&apart, TOP, RIB_RIGHT)),
        (
            "the second rib's foot",
            edge_between(&apart, TOP, RIB_RIGHT + 6),
        ),
    ];
    let abutting = two_ribs(13.0);
    let merged = [vec![TOP as u32], BRIDGED[1].to_vec()];
    let bridged: Vec<EdgeName> = abutting
        .edge_names()
        .into_iter()
        .filter(|name| name.between == merged)
        .collect();
    assert_eq!(
        bridged.len(),
        1,
        "the two ribs' right sides merge, and meet the plate's top along one edge: {bridged:?}"
    );
    let long = RIB_ENDS[1] - RIB_ENDS[0];

    for (whose, name) in names {
        assert_eq!(
            abutting.lost(&name),
            None,
            "by {whose}: the name finds the edge the join bridged"
        );

        let chamfered = abutting
            .chamfer(&name, value)
            .unwrap_or_else(|declined| panic!("the chamfer by {whose}: declined, {declined:?}"));

        let what = &format!("the chamfer by {whose}");
        taken_whole(chamfered.asked, chamfered.taken, value, what);
        let body = &chamfered.body;
        sound(body, RIBBED_VOLUME + value * value / 2.0 * long, what);
        let chamfer = one_flat_face_more(&abutting, body, what);
        faces_toward(
            body,
            chamfer,
            DVec3::new(1.0, 0.0, 1.0),
            &format!("{what}: halfway between the plate's top and the ribs' right sides"),
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
            &format!("{what}: the chamfer, from the first rib's front to the second's back"),
        );
        filled_along_the_foot_of_the_rib(
            &abutting,
            body,
            (chamfer, BRIDGED),
            value,
            |_| false,
            what,
        );

        let filleted = abutting
            .fillet(&name, value)
            .unwrap_or_else(|declined| panic!("the fillet by {whose}: declined, {declined:?}"));

        let what = &format!("the fillet by {whose}");
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
            drawn_within(&abutting, hollow.area()),
            what,
        );
        let fillet = one_round_more(&abutting, body, what);
        is_the_hollow(
            body,
            fillet,
            &hollow,
            &format!("{what}: the round, from the first rib's front to the second's back"),
        );
        filled_along_the_foot_of_the_rib(
            &abutting,
            body,
            (fillet, BRIDGED),
            value,
            |corner| hollow.holds(corner),
            what,
        );
    }
}

/// Holds that a chamfer and a fillet of `value` on the edge between the
/// faces numbered `first` and `second` are both declined as a fill the
/// kernel cannot build.
fn both_declined(part: &Body, [first, second]: [usize; 2], value: f64, what: &str) {
    let name = edge_between(part, first, second);

    let declined = [
        ("the chamfer", part.chamfer(&name, value).err()),
        ("the fillet", part.fillet(&name, value).err()),
    ];

    for (step, declined) in declined {
        assert_eq!(
            declined,
            Some(Declined::Unsupported),
            "{step} of {value} {what}"
        );
    }
}

#[test]
fn a_chamfer_and_a_fillet_on_a_concave_edge_ending_on_a_face_turned_back_along_it_are_declined_as_unsupported()
 {
    let ell = Contour::straight(vec![
        DVec2::new(5.0, 5.0),
        DVec2::new(30.0, 5.0),
        DVec2::new(30.0, 11.0),
        DVec2::new(17.0, 11.0),
        DVec2::new(17.0, 19.0),
        DVec2::new(5.0, 19.0),
    ]);
    let part = on_the_plate(&plate(PLATE), PLATE, ell);
    let (arm_back, stem_right) = (10, 11);

    both_declined(
        &part,
        [arm_back, stem_right],
        3.0,
        "on the inner corner of an L standing on a plate: its foot ends on the plate's top, \
         which faces up along it, where the L's arm and stem meet the plate inwards too",
    );
}

#[test]
fn a_chamfer_and_a_fillet_on_a_concave_edge_ending_where_two_faces_meet_are_declined_as_unsupported()
 {
    let creased = cut_down_to(&ELL.on(ground()), DVec3::new(0.25, 0.25, 1.0), 14.5);
    let top_end = DVec3::new(ELL.stem, ELL.depth, ELL.height);
    for (face, whose) in [(TOP, "the top"), (LEANING, "the leaning face")] {
        let corners = creased
            .plane_of(face)
            .unwrap_or_else(|| panic!("{whose} is flat"))
            .corners;
        assert!(
            corners
                .iter()
                .any(|corner| corner.distance(top_end) < PLACE),
            "{whose} reaches the inner edge's top end, where the crease between the two runs \
             through it: {corners:?}"
        );
    }

    both_declined(
        &creased,
        [ARM_BACK, STEM_RIGHT],
        3.0,
        "on the inner corner of an L whose top is creased right through the corner's top end",
    );
}

#[test]
fn a_chamfer_and_a_fillet_whose_fill_a_rib_s_slanted_end_would_carry_past_the_plate_s_front_are_declined_as_unsupported()
 {
    let slanted_in_front = Contour::straight(vec![
        DVec2::new(18.0, 5.0),
        DVec2::new(RIB_SIDE, 1.0),
        DVec2::new(RIB_SIDE, RIB_ENDS[1]),
        DVec2::new(18.0, RIB_ENDS[1]),
    ]);
    let part = on_the_plate(&plate(PLATE), PLATE, slanted_in_front);

    both_declined(
        &part,
        [TOP, RIB_RIGHT],
        3.0,
        "at the foot of a rib whose front slants back at 45 degrees from 1 behind the plate's \
         front: carried 3 out along the plate, its plane passes the plate's front 2 before it",
    );
}

/// A part with a wall 0.3 thin beside a concave edge, the two faces of that
/// edge, the matter the part holds, and how long the edge runs.
struct Thin {
    what: &'static str,
    part: Body,
    walls: [usize; 2],
    volume: f64,
    long: f64,
}

#[test]
fn a_chamfer_and_a_fillet_at_a_wall_thinner_than_their_margin_are_laid_soundly_or_declined_never_a_wrong_body()
 {
    let (thin, value) = (0.3, 3.0);
    let rib = |left: f64| {
        Contour::rectangle(
            DVec2::new(left, RIB_ENDS[0]),
            DVec2::new(RIB_SIDE, RIB_ENDS[1]),
        )
    };
    let long = RIB_ENDS[1] - RIB_ENDS[0];
    let thin_arm = Ell { depth: thin, ..ELL };
    let parts = [
        Thin {
            what: "at the inner corner of an L whose arm is 0.3 deep",
            part: thin_arm.on(ground()),
            walls: [ARM_BACK, STEM_RIGHT],
            volume: thin_arm.volume(),
            long: ELL.height,
        },
        Thin {
            what: "at the foot of a rib on a plate 0.3 thick",
            part: on_the_plate(&plate(thin), thin, rib(18.0)),
            walls: [TOP, RIB_RIGHT],
            volume: 40.0 * 30.0 * thin + 4.0 * long * 10.0,
            long,
        },
        Thin {
            what: "at the foot of a rib 0.3 wide",
            part: on_the_plate(&plate(PLATE), PLATE, rib(RIB_SIDE - thin)),
            walls: [TOP, RIB_RIGHT],
            volume: 40.0 * 30.0 * PLATE + thin * long * 10.0,
            long,
        },
    ];
    let mut faults = Vec::new();

    for Thin {
        what,
        part,
        walls,
        volume,
        long,
    } in parts
    {
        let name = edge_between(&part, walls[0], walls[1]);
        let filled = [
            (
                "the chamfer",
                part.chamfer(&name, value)
                    .map(|chamfered| (chamfered.body, chamfered.taken)),
                value * value / 2.0,
                true,
            ),
            (
                "the fillet",
                part.fillet(&name, value)
                    .map(|filleted| (filleted.body, filleted.taken)),
                value * value * (1.0 - PI / 4.0),
                false,
            ),
        ];
        for (step, filled, section, flat) in filled {
            if let Some(fault) =
                laid_soundly_or_declined(&part, filled, volume + section * long, flat)
            {
                faults.push(format!("{step} of {value} {what}: {fault}"));
            }
        }
    }

    assert!(
        faults.is_empty(),
        "a fill beside a wall thinner than its margin that is neither declined Unsupported nor \
         laid soundly:\n{}",
        faults.join("\n")
    );
}

/// What is wrong with what a fill of 3 on `part` gave back, if anything: a
/// decline other than `Unsupported`, or a body that is not exact, closed and
/// uncrossed, holding the matter `promised` with one face more, `flat` or
/// curved, numbered where the part's steps ended — nothing, where it is
/// declined `Unsupported` or laid so.
fn laid_soundly_or_declined(
    part: &Body,
    filled: Result<(Body, f64), Declined>,
    promised: f64,
    flat: bool,
) -> Option<String> {
    let (body, taken) = match filled {
        Err(Declined::Unsupported) => return None,
        Err(other) => return Some(format!("declined {other:?}")),
        Ok(laid) => laid,
    };
    if (taken - 3.0).abs() > PLACE {
        return Some(format!("took {taken}, where 3 fits"));
    }
    if !body.is_exact() {
        return Some("left a body the exact kernel did not compute".to_string());
    }
    let triangles = body.triangles();
    if let Err(fault) = closed(&triangles) {
        return Some(format!("left a body that is not closed: {fault:?}"));
    }
    if let Err(fault) = uncrossed(&triangles) {
        return Some(format!(
            "left a body with a face through another: {fault:?}"
        ));
    }
    if (body.volume() - promised).abs() > EXACT * promised {
        return Some(format!(
            "left {} of matter, where arithmetic promised {promised}",
            body.volume()
        ));
    }
    let made = part.faces_end();
    let mut kept_and_made = numbers(part);
    kept_and_made.insert(made);
    if numbers(&body) != kept_and_made || body.is_flat(made) != flat {
        return Some(format!(
            "left the faces {:?}, where the part's and one more, {made}, {}, were promised",
            numbers(&body),
            if flat { "flat" } else { "curved" }
        ));
    }
    None
}

/// The outline of a block 30 by 10 whose front bends forward, out of the
/// matter, by `turn` halfway along: its first half from the origin to
/// (15, 0), its second half 15 long and `turn` under the X axis, a right
/// wall up to (30, 10), the back and the left. The two halves meet inwards,
/// the air between them 180 degrees less `turn`.
fn bent_in(turn: f64) -> Vec<DVec2> {
    vec![
        DVec2::ZERO,
        DVec2::new(15.0, 0.0),
        DVec2::new(15.0, 0.0) + DVec2::new(turn.cos(), -turn.sin()) * 15.0,
        DVec2::new(30.0, 10.0),
        DVec2::new(0.0, 10.0),
    ]
}

/// Holds what [`sound`] holds of a body but for the matter its triangles
/// enclose: on a round this shallow and this wide the kernel draws coarser
/// than its tolerance, whatever made it, as the convex side's bent fronts
/// found.
fn sound_but_for_its_drawing(body: &Body, promised: f64, what: &str) {
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
}

#[test]
fn a_chamfer_and_a_fillet_on_a_concave_edge_nearly_flat_fill_it_with_the_matter_arithmetic_says() {
    let (height, set_back) = (5.0, 3.0);
    let (front, second_half) = (2, 3);

    for degrees in [10.0, 1.0] {
        let turn = f64::to_radians(degrees);
        let outline = bent_in(turn);
        let part = raised_on(ground(), Contour::straight(outline.clone()), height);
        let name = edge_between(&part, front, second_half);
        let corner = outline[1];
        let far = [outline[0], outline[2]];
        let stops = far.map(|far| corner + (far - corner).normalize() * set_back);
        let outward = [DVec2::new(0.0, -1.0), DVec2::new(-turn.sin(), -turn.cos())];
        let mut filled = outline.clone();
        filled.splice(1..2, stops);
        let walls_and_caps = |body: &Body,
                              made: usize,
                              on_the_fill: &dyn Fn(DVec3) -> bool,
                              what: &str| {
            for (wall, stop, end) in [(front, stops[0], far[0]), (second_half, stops[1], far[1])] {
                corners_are(
                    body,
                    wall,
                    &[upright(stop, height), upright(end, height)].concat(),
                    &format!("{what}: what is left of wall {wall}, set back by {set_back}"),
                );
            }
            for (cap, at) in [(FLOOR, 0.0), (TOP, height)] {
                corners_are_or_on(
                    body,
                    cap,
                    &lifted_to(&filled, at),
                    on_the_fill,
                    &format!("{what}: face {cap}, the fill's end flush on it"),
                );
            }
            assert_eq!(
                sides_met(body, made),
                [FLOOR, TOP, front, second_half].map(|side| vec![side as u32]),
                "{what}: the fill meets the two halves and the floor and the top, by one edge each"
            );
        };

        let chamfered = part
            .chamfer(&name, set_back)
            .unwrap_or_else(|declined| panic!("the chamfer, {degrees}°: declined, {declined:?}"));

        let what = &format!("the chamfer on a front bent in by {degrees}°");
        taken_whole(chamfered.asked, chamfered.taken, set_back, what);
        let body = &chamfered.body;
        let section = set_back * set_back * turn.sin() / 2.0;
        sound(body, part.volume() + section * height, what);
        let chamfer = one_flat_face_more(&part, body, what);
        faces_toward(
            body,
            chamfer,
            (outward[0] + outward[1].normalize()).extend(0.0),
            &format!("{what}: halfway between the two halves"),
        );
        corners_are(
            body,
            chamfer,
            &stops.map(|at| upright(at, height)).concat(),
            &format!("{what}: the chamfer, set back by {set_back} along each half"),
        );
        walls_and_caps(body, chamfer, &|_| false, what);

        let radius = set_back / (turn / 2.0).tan();
        let filleted = part
            .fillet(&name, radius)
            .unwrap_or_else(|declined| panic!("the fillet, {degrees}°: declined, {declined:?}"));

        let what = &format!("the fillet of {radius} on a front bent in by {degrees}°");
        taken_whole(filleted.asked, filleted.taken, radius, what);
        let body = &filleted.body;
        let bisector = (far[0] - corner).normalize() + (far[1] - corner).normalize();
        let axis = corner + bisector.normalize() * radius / (turn / 2.0).cos();
        let hollow = Hollow {
            start: axis.extend(0.0),
            along: DVec3::Z,
            length: height,
            radius,
            facing: outward.map(|way| way.extend(0.0)),
        };
        let section = radius * set_back - radius * radius * turn / 2.0;
        sound_but_for_its_drawing(body, part.volume() + section * height, what);
        let fillet = one_round_more(&part, body, what);
        is_the_hollow(
            body,
            fillet,
            &hollow,
            &format!("{what}: the round, its axis the radius out from both halves"),
        );
        walls_and_caps(body, fillet, &|corner| hollow.holds(corner), what);
    }
}

/// Holds that no corner of the triangles of the face numbered `face` stands
/// in front of the plane of the points p where `facing` · p = `offset`.
fn drawn_behind(body: &Body, face: usize, facing: DVec3, offset: f64, what: &str) {
    let (facing, offset) = (facing.normalize(), offset / facing.length());
    for corner in body.triangles_of(face).flatten() {
        let past = corner.dot(facing) - offset;
        assert!(
            past <= PLACE,
            "{what}: the fill is drawn through {corner}, {past} past the plane"
        );
    }
}

/// The faces' planes at the two ends of the rib of
/// [`rib_slanting_at_both_ends`]: its front, x + 2y = 32, and its back,
/// x + y = 42, each as the way it faces and its offset that way.
const SLANTED_ENDS: [(DVec3, f64); 2] = [
    (DVec3::new(-1.0, -2.0, 0.0), -32.0),
    (DVec3::new(1.0, 1.0, 0.0), 42.0),
];

/// The plate of [`ribbed_plate`] and on it a rib 4 wide whose two ends slant
/// across its foot: its front from (18, 7) to (22, 5), its right side from
/// there back to (22, 20), its back from there to (18, 24). A fill of 3
/// along its right foot is trimmed by the front 1.5 short of the foot's end
/// where it meets the plate, 3.5 behind the plate's front, and by the back 3
/// short — both planes leaning across the edge, neither square to it.
fn rib_slanting_at_both_ends() -> Body {
    on_the_plate(
        &plate(PLATE),
        PLATE,
        Contour::straight(vec![
            DVec2::new(18.0, 7.0),
            DVec2::new(RIB_SIDE, 5.0),
            DVec2::new(RIB_SIDE, 20.0),
            DVec2::new(18.0, 24.0),
        ]),
    )
}

/// How long a fill at the foot of the [`rib_slanting_at_both_ends`] runs, a
/// way `x` out from the rib: from its front's plane to its back's.
fn between_the_slants(x: f64) -> f64 {
    (42.0 - x) - (32.0 - x) / 2.0
}

#[test]
fn a_chamfer_and_a_fillet_at_the_foot_of_a_rib_whose_ends_both_slant_are_trimmed_flush_by_each() {
    let value = 3.0;
    let part = rib_slanting_at_both_ends();
    let foot = edge_between(&part, TOP, RIB_RIGHT);
    let volume = 40.0 * 30.0 * PLATE + (17.0 + 15.0) / 2.0 * 4.0 * 10.0;
    let (out, up, top) = (RIB_SIDE + value, PLATE + value, PLATE + 10.0);
    let on_the_plate_at = |x: f64| [(32.0 - x) / 2.0, 42.0 - x];
    let [front_out, back_out] = on_the_plate_at(out);
    let [front_in, back_in] = on_the_plate_at(RIB_SIDE);
    let ends = |body: &Body, made: usize, on_the_fill: &dyn Fn(DVec3) -> bool, what: &str| {
        for ((wall, y_in, y_out, far), (facing, offset)) in [
            (RIB_FRONT, front_in, front_out, 7.0),
            (RIB_BACK, back_in, back_out, 24.0),
        ]
        .into_iter()
        .zip(SLANTED_ENDS)
        {
            corners_are_or_on(
                body,
                wall,
                &[
                    DVec3::new(18.0, far, PLATE),
                    DVec3::new(out, y_out, PLATE),
                    DVec3::new(RIB_SIDE, y_in, up),
                    DVec3::new(RIB_SIDE, y_in, top),
                    DVec3::new(18.0, far, top),
                ],
                on_the_fill,
                &format!("{what}: face {wall}, grown by the fill's section in its own plane"),
            );
            drawn_behind(
                body,
                made,
                facing,
                offset,
                &format!("{what}: trimmed flush by face {wall}"),
            );
        }
        corners_are(
            body,
            TOP,
            &[
                DVec3::new(0.0, 0.0, PLATE),
                DVec3::new(40.0, 0.0, PLATE),
                DVec3::new(40.0, 30.0, PLATE),
                DVec3::new(0.0, 30.0, PLATE),
                DVec3::new(18.0, 7.0, PLATE),
                DVec3::new(out, front_out, PLATE),
                DVec3::new(out, back_out, PLATE),
                DVec3::new(18.0, 24.0, PLATE),
            ],
            &format!("{what}: the plate's top, one face round the rib and the fill's footprint"),
        );
        assert_eq!(
            sides_met(body, made),
            [TOP, RIB_FRONT, RIB_RIGHT, RIB_BACK].map(|side| vec![side as u32]),
            "{what}: the fill meets the plate's top and the rib's right side along its sides, and \
             the rib's slanted front and back at its ends, by one edge each"
        );
        for wall in [RIB_FRONT, RIB_BACK, RIB_LEFT] {
            let sides = [vec![TOP as u32], vec![wall as u32]];
            let meeting = body
                .edge_names()
                .into_iter()
                .filter(|name| name.between == sides)
                .count();
            assert_eq!(
                meeting, 1,
                "{what}: the plate's top meets face {wall} along one edge still"
            );
        }
    };

    let chamfered = part
        .chamfer(&foot, value)
        .unwrap_or_else(|declined| panic!("the chamfer: declined, {declined:?}"));

    let what = "the chamfer at the foot of a rib slanting at both ends";
    taken_whole(chamfered.asked, chamfered.taken, value, what);
    let body = &chamfered.body;
    let section = value * value / 2.0;
    let centre = RIB_SIDE + value / 3.0;
    sound(body, volume + section * between_the_slants(centre), what);
    let chamfer = one_flat_face_more(&part, body, what);
    corners_are(
        body,
        chamfer,
        &[
            DVec3::new(out, front_out, PLATE),
            DVec3::new(RIB_SIDE, front_in, up),
            DVec3::new(out, back_out, PLATE),
            DVec3::new(RIB_SIDE, back_in, up),
        ],
        &format!("{what}: the chamfer, its ends in the planes of the rib's front and back"),
    );
    ends(body, chamfer, &|_| false, what);

    let filleted = part
        .fillet(&foot, value)
        .unwrap_or_else(|declined| panic!("the fillet: declined, {declined:?}"));

    let what = "the fillet at the foot of a rib slanting at both ends";
    taken_whole(filleted.asked, filleted.taken, value, what);
    let body = &filleted.body;
    let around = |from: f64, to: f64| Hollow {
        start: DVec3::new(out, from, up),
        along: DVec3::Y,
        length: to - from,
        radius: value,
        facing: [DVec3::Z, DVec3::X],
    };
    let (whole, clear_of_the_ends) = (around(front_out, back_in), around(front_in, back_out));
    let square = value * value * between_the_slants(RIB_SIDE + value / 2.0);
    let quarter_disc =
        PI * value * value / 4.0 * between_the_slants(RIB_SIDE + value - 4.0 * value / (3.0 * PI));
    sound_drawn_within(
        body,
        volume + square - quarter_disc,
        drawn_within(&part, whole.area()),
        what,
    );
    let fillet = one_round_more(&part, body, what);
    drawn_on_the_hollow(body, fillet, &whole, what);
    met_from_its_axis(body, fillet, &clear_of_the_ends, what);
    ends(body, fillet, &|corner| whole.holds(corner), what);
}

/// The [`ribbed_plate`] drawn on `frame` — its plate 4 thick, its rib from
/// (18, 5) to (22, 22) and 10 high — and a hole of radius 3 bored through
/// the rib square to its sides, about the line 13 along it and 10.5 over
/// the plate's floor: the rib's right side holds the hole's rim, from 7.5 to
/// 13.5 up, all of it above where a fill of 3 at the rib's foot reaches.
/// The hole's wall answers to fourteen.
fn bored_rib(frame: Frame) -> Body {
    let plate = raised_on(
        frame,
        Contour::rectangle(DVec2::ZERO, DVec2::new(40.0, 30.0)),
        PLATE,
    );
    let on_the_plate = Frame {
        origin: placed(frame, [0.0, 0.0, PLATE]),
        ..frame
    };
    let rib = Contour::rectangle(
        DVec2::new(18.0, RIB_ENDS[0]),
        DVec2::new(RIB_SIDE, RIB_ENDS[1]),
    );
    let tool = plate
        .tool_raised(&profile(rib), on_the_plate, frame.normal() * 10.0)
        .expect("the tool raises");
    let ribbed = plate.union(&tool).expect("the rib is joined");
    let across = Frame {
        origin: placed(frame, [17.0, 0.0, 0.0]),
        u: frame.v,
        v: frame.normal(),
    };
    let hole = Contour {
        corners: vec![DVec2::new(16.0, 10.5)],
        runs: vec![Run::Round {
            center: DVec2::new(13.0, 10.5),
            turn: TAU,
        }],
    };
    let tool = ribbed
        .tool_raised(&profile(hole), across, across.normal() * 6.0)
        .expect("the tool raises");
    ribbed.difference(&tool).expect("the hole is bored")
}

#[test]
fn a_chamfer_and_a_fillet_at_the_foot_of_a_rib_bored_through_above_them_join_the_edges_round_its_ends_however_it_stands()
 {
    let value = 3.0;
    let long = RIB_ENDS[1] - RIB_ENDS[0];
    let hole_wall = TAU * 3.0 * 4.0;
    let volume = 40.0 * 30.0 * PLATE + 4.0 * long * 10.0 - PI * 9.0 * 4.0;
    let stands = [
        ("standing on the ground", ground()),
        (
            "hanging under its plate, above the ground",
            Frame {
                origin: DVec3::new(40.0, 0.0, 18.0),
                u: -DVec3::X,
                v: DVec3::Y,
            },
        ),
        (
            "standing far below the origin",
            Frame {
                origin: DVec3::splat(-100.0),
                ..ground()
            },
        ),
        (
            "hanging under its plate, far below the origin",
            Frame {
                origin: DVec3::new(-60.0, -100.0, -82.0),
                u: -DVec3::X,
                v: DVec3::Y,
            },
        ),
    ];

    for (stands, frame) in stands {
        let part = bored_rib(frame);
        let foot = edge_between(&part, TOP, RIB_RIGHT);
        let ends_of_the_foot = RIB_ENDS.map(|y| placed(frame, [RIB_SIDE, y, PLATE]));
        let filled = [
            (
                "the chamfer",
                part.chamfer(&foot, value)
                    .map(|chamfered| (chamfered.body, chamfered.taken)),
                value * value / 2.0,
                0.0,
            ),
            (
                "the fillet",
                part.fillet(&foot, value)
                    .map(|filleted| (filleted.body, filleted.taken)),
                value * value * (1.0 - PI / 4.0),
                PI / 2.0 * value * long,
            ),
        ];

        for (step, filled, section, round) in filled {
            let what = &format!("{step} at the foot of the bored rib, {stands}");
            let (body, taken) =
                filled.unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));
            assert!(
                (taken - value).abs() < PLACE,
                "{what}: took {taken}, where {value} fits"
            );
            sound_drawn_within(
                &body,
                volume + section * long,
                drawn_within(&part, hole_wall + round),
                what,
            );
            one_face_more(&part, &body, what);
            for wall in [RIB_FRONT, RIB_BACK] {
                let sides = [vec![TOP as u32], vec![wall as u32]];
                let meeting: Vec<EdgeName> = body
                    .edge_names()
                    .into_iter()
                    .filter(|name| name.between == sides)
                    .collect();
                assert_eq!(
                    meeting.len(),
                    1,
                    "{what}: the plate's top meets face {wall} along one edge again, the two \
                     either side of where the foot ended made one, though the rib's side holds \
                     the hole's rim: {meeting:?}"
                );
            }
            let corners = body.plane_of(TOP).expect("the plate's top is flat").corners;
            for end in ends_of_the_foot {
                assert!(
                    corners.iter().all(|corner| corner.distance(end) > PLACE),
                    "{what}: the plate's top keeps a corner where the foot ended, at {end}"
                );
            }
        }
    }
}

#[test]
fn a_chamfer_and_a_fillet_of_1_at_the_foot_of_a_rib_fill_it_as_one_of_3_does() {
    let value = 1.0;
    let part = ribbed_plate();
    let foot = edge_between(&part, TOP, RIB_RIGHT);
    let long = RIB_ENDS[1] - RIB_ENDS[0];

    let chamfered = part
        .chamfer(&foot, value)
        .unwrap_or_else(|declined| panic!("the chamfer of 1: declined, {declined:?}"));

    let what = "the chamfer of 1 at the rib's foot";
    taken_whole(chamfered.asked, chamfered.taken, value, what);
    let body = &chamfered.body;
    sound(body, RIBBED_VOLUME + value * value / 2.0 * long, what);
    let chamfer = one_flat_face_more(&part, body, what);
    filled_along_the_foot_of_the_rib(&part, body, (chamfer, RIB), value, |_| false, what);

    let filleted = part
        .fillet(&foot, value)
        .unwrap_or_else(|declined| panic!("the fillet of 1: declined, {declined:?}"));

    let what = "the fillet of 1 at the rib's foot";
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
    is_the_hollow(body, fillet, &hollow, what);
    filled_along_the_foot_of_the_rib(
        &part,
        body,
        (fillet, RIB),
        value,
        |corner| hollow.holds(corner),
        what,
    );
}

/// A block 10 by 10 and 10 high, and beside it a block 10 by 10 and 5 high
/// joined as a part's next step is, from x = 10 to 20: their floors merge,
/// numbered nought and six, their fronts two and eight, their backs four and
/// ten. The taller one's right side, three, stands on above the lower one's
/// top, seven, from 5 up. Its plane runs through the corners where the lower
/// one's front and back meet the floor, at x = 10, and the join keeps both as
/// vertices: the floor meets the front, and the back, along two edges each.
fn stepped_blocks() -> Body {
    let taller = raised_on(
        ground(),
        Contour::rectangle(DVec2::ZERO, DVec2::new(10.0, 10.0)),
        10.0,
    );
    let lower = Contour::rectangle(DVec2::new(10.0, 0.0), DVec2::new(20.0, 10.0));
    joined_to(&taller, lower, 5.0)
}

/// The numbers the [`stepped_blocks`]' merged floor, fronts and backs answer
/// to, and the taller one's right side and the lower one's top, which meet
/// inwards at the step.
const STEPPED_FLOOR: [u32; 2] = [0, 6];
const STEPPED_FRONT: [u32; 2] = [2, 8];
const STEPPED_BACK: [u32; 2] = [4, 10];
const TALLER_RIGHT: usize = 3;
const LOWER_TOP: usize = 7;

/// The names the edges between the [`stepped_blocks`]' floor and their
/// fronts, and their backs, answer to as the join leaves them: two edges
/// each, alike in what they recorded but for the end each keeps — at the
/// taller block's left, five, or the lower one's right, nine.
fn the_floor_s_edges_as_joined() -> Vec<EdgeName> {
    let floor = STEPPED_FLOOR.to_vec();
    let name = |side: [u32; 2], ends: [Vec<Vec<u32>>; 2]| EdgeName {
        between: [floor.clone(), side.to_vec()],
        ends,
        among: 2,
        rank: 0,
        of: 1,
    };
    vec![
        name(STEPPED_FRONT, [vec![], vec![vec![5]]]),
        name(STEPPED_FRONT, [vec![vec![9]], vec![]]),
        name(STEPPED_BACK, [vec![], vec![vec![9]]]),
        name(STEPPED_BACK, [vec![vec![5]], vec![]]),
    ]
}

/// Holds that the body's floor meets its fronts and backs along the edges
/// the join left, by the names they had, and keeps its corners where the
/// lower block's front and back meet it.
fn the_floor_s_edges_kept(body: &Body, what: &str) {
    let floor = STEPPED_FLOOR.to_vec();
    let names: Vec<EdgeName> = body
        .edge_names()
        .into_iter()
        .filter(|name| {
            name.between[0] == floor
                && [STEPPED_FRONT.to_vec(), STEPPED_BACK.to_vec()].contains(&name.between[1])
        })
        .collect();
    assert_eq!(
        names,
        the_floor_s_edges_as_joined(),
        "{what}: the floor meets the front and the back along two edges each, either side of \
         where the lower block meets the taller one, by the names the join gave them"
    );
    let corners = body
        .plane_of(STEPPED_FLOOR[0] as usize)
        .expect("the floor is flat")
        .corners;
    for kept in [DVec3::new(10.0, 0.0, 0.0), DVec3::new(10.0, 10.0, 0.0)] {
        assert!(
            corners.iter().any(|corner| corner.distance(kept) < PLACE),
            "{what}: the floor keeps its corner at {kept}, under the taller block's right side"
        );
    }
}

#[test]
fn two_blocks_joined_at_two_heights_keep_their_floor_s_front_and_back_edges_in_two_as_the_join_always_has()
 {
    let stepped = stepped_blocks();

    the_floor_s_edges_kept(&stepped, "two blocks of 10 and 5 joined side by side");
}

#[test]
fn a_chamfer_and_a_fillet_at_the_step_between_two_blocks_leave_their_floor_s_edges_as_the_join_did()
{
    let value = 3.0;
    let part = stepped_blocks();
    let step = edge_between(&part, TALLER_RIGHT, LOWER_TOP);
    let volume = 10.0 * 10.0 * 10.0 + 10.0 * 10.0 * 5.0;

    let chamfered = part
        .chamfer(&step, value)
        .unwrap_or_else(|declined| panic!("the chamfer at the step: declined, {declined:?}"));

    let what = "the chamfer at the step between the two blocks";
    taken_whole(chamfered.asked, chamfered.taken, value, what);
    let body = &chamfered.body;
    sound(body, volume + value * value / 2.0 * 10.0, what);
    one_flat_face_more(&part, body, what);
    the_floor_s_edges_kept(body, what);

    let filleted = part
        .fillet(&step, value)
        .unwrap_or_else(|declined| panic!("the fillet at the step: declined, {declined:?}"));

    let what = "the fillet at the step between the two blocks";
    taken_whole(filleted.asked, filleted.taken, value, what);
    let body = &filleted.body;
    sound_drawn_within(
        body,
        volume + value * value * (1.0 - PI / 4.0) * 10.0,
        drawn_within(&part, PI / 2.0 * value * 10.0),
        what,
    );
    one_round_more(&part, body, what);
    the_floor_s_edges_kept(body, what);
}
