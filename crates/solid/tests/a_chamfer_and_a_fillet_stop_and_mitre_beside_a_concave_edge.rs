//! A chamfer and a fillet on a convex straight edge that ends where a concave
//! edge also ends: the top and floor edges at the inner corner of an L, the
//! rim of a pocket, the uprights of a block standing on another. Past such an
//! end lies matter, not air, and the cut's prism, carried past the end along
//! the edge's line as #699 carries it past a block's corner, would take it.
//! It is trimmed flush on the face the edge runs into instead, as #778 trims
//! a fill, and that face grows by the cut's end section and keeps its number.
//! Two such edges meeting at a corner whose third edge is concave, that edge
//! left sharp, run on past the corner into a mitre. An operation of the kernel
//! on [`Body`], as #699's corners are.
//!
//! Part of #801, its first slice: two convex edges with a concave one left
//! sharp. The fills beside a convex edge and the three concave edges of a
//! pocket's bottom corner are its second and third slices, each on a branch
//! of its own; the issue stays open until the last one lands.
//!
//! - A convex straight edge ending where a concave edge also ends leaves a
//!   closed, sound body when chamfered or filleted alone. Its flat or round
//!   stops flush on the face it runs into, which keeps its number. Nothing
//!   past that face is taken. The volume is exact. This holds on the L's top
//!   and floor edges at its inner corner —
//!   `a_chamfer_and_a_fillet_on_a_top_or_floor_edge_at_the_l_s_inner_corner_stop_flush_on_the_side_they_run_into`;
//!   on each rim edge of a pocket —
//!   `a_chamfer_and_a_fillet_on_each_rim_edge_of_a_pocket_stop_flush_on_the_walls_they_run_into_at_both_ends`;
//!   and on each upright of a block standing on another —
//!   `a_chamfer_and_a_fillet_on_each_upright_of_a_block_standing_on_another_stop_flush_on_the_top_it_stands_on`.
//!   The first holds the body where
//!   `a_chamfer_whose_cut_carried_along_its_edge_would_run_into_the_l_s_stem_is_declined_as_unsupported`,
//!   in `a_chamfer_cuts_one_straight_edge.rs`, held the decline of the L's
//!   top edge on the arm's back; that test is gone
//! - Two convex edges meeting at a corner whose third edge is concave and
//!   left sharp leave a closed, sound body when chamfered or filleted at one
//!   value. The two flats or rounds run past the corner into a mitre: a
//!   straight line among flats, a planar ellipse arc among rounds. The mitre
//!   runs from where their set-backs cross on the face they share down to the
//!   concave edge. That edge stays sharp and ends one value short of the old
//!   corner. No patch is added. The volume is exact, and the body is the same
//!   in either order. This holds on the L's two top edges at its inner
//!   corner —
//!   `a_chamfer_on_the_two_top_edges_at_the_l_s_inner_corner_mitres_their_flats_down_to_its_upright_left_sharp_one_distance_short`,
//!   `a_fillet_on_the_two_top_edges_at_the_l_s_inner_corner_mitres_their_rounds_on_an_ellipse_down_to_its_upright_left_sharp_one_radius_short`;
//!   on two rim edges of a pocket above one of its uprights —
//!   `a_chamfer_and_a_fillet_on_two_rim_edges_of_a_pocket_mitre_them_down_to_the_upright_below_left_sharp`;
//!   and the four rim edges of a pocket mitre its four corners —
//!   `a_chamfer_and_a_fillet_on_the_four_rim_edges_of_a_pocket_mitre_its_four_corners`
//! - The same holds at a corner that is not square: an L whose inner corner
//!   is not a right angle —
//!   `a_chamfer_and_a_fillet_at_an_l_s_inner_corner_that_is_not_square_stop_flush_alone_and_mitre_together`,
//!   read as both bullets above: each top edge there alone, stopping flush on
//!   the slanted side it runs into, and the two together, mitred
//! - What must not break: #699's corner tests; the concave fill of one edge
//!   (`a_chamfer_and_a_fillet_fill_a_concave_edge`); the decline test
//!   `a_fillet_and_a_chamfer_on_edges_meeting_at_a_corner_with_a_concave_edge_are_declined_as_unsupported`,
//!   which loses the lists this slice builds, the L's two top edges and the
//!   pocket's two rim edges, and keeps the others — no test: each is held by
//!   its own file, `a_chamfer_and_a_fillet_blend_the_corner_their_edges_meet_at.rs`
//!   and `a_chamfer_and_a_fillet_fill_a_concave_edge.rs`, left as they were
//!   but for the two lists the decline test gives up to the tests above
//!
//! The volumes are arithmetic. Every edge here lies between two faces square
//! to each other, and every face it ends on is square to it. A flat set back
//! `d` along both faces takes a right triangle of legs `d` off the edge's
//! section, `d²/2`; a round of radius `r`, the square of side `r` less the
//! quarter of the disc, `r²(1 − π/4)`. Where air lies past an end, the cut
//! carried past it takes nothing beyond the end's plane; where matter does,
//! the cut stopped flush takes nothing beyond the end face. Either way the
//! cut takes its section along the edge's length, and the part keeps its
//! volume less that: the L, an arm 30 by 6 and a stem 12 by 8 raised 10,
//! holds 2760; the block 30 by 20 by 10 holds 5500 with its pocket 10 by 10
//! sunk 5, and 7000 with the block 10 by 10 by 10 standing on it. Held
//! another way, by the booleans, every edge here at both values agrees to the
//! last digit: the part less the prism over the cut's section, raised along
//! the edge from the face it runs into to past its other end, or from one
//! face run into to the other.

use std::collections::BTreeSet;
use std::f64::consts::PI;

use cao_solid::profile::{Contour, Frame, Profile};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, Declined, EdgeName, Loop};
use glam::{DVec2, DVec3};

/// How far a corner may stand from where the arithmetic puts it, on a plane:
/// each is placed exactly, so what is left is rounding.
const PLACE: f64 = 1e-9;

/// How near the arithmetic an exact volume comes, as a fraction of it.
const EXACT: f64 = 1e-9;

/// How far the triangles may stand from a round, as a share of the reach the
/// body carries: the drawing's tolerance.
const DRAWN: f64 = 2e-4;

/// The numbers a raised part's floor and top answer to.
const FLOOR: usize = 0;
const TOP: usize = 1;

/// The values every edge is cut at.
const VALUES: [f64; 2] = [1.0, 2.5];

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

/// An outline drawn on the ground raised `height` as `cao_part` builds a
/// step: its floor numbered nought, its top one, then its walls, one for each
/// run of the outline.
fn raised(outline: Contour, height: f64) -> Body {
    let part = Body::default();
    let tool = part
        .tool_raised(&profile(outline), ground(), DVec3::Z * height)
        .expect("the tool raises");
    part.union(&tool).expect("the part is built")
}

/// A block of a length, a breadth and a height standing on the ground's
/// origin, its sides numbered two to five from the front round to the left.
fn block([length, breadth, height]: [f64; 3]) -> Body {
    raised(
        Contour::rectangle(DVec2::ZERO, DVec2::new(length, breadth)),
        height,
    )
}

/// The block a step raises between the corners `from` and `to`, from
/// `from_height` by `height`: its faces numbered past the part's, its floor
/// first, then its top, then its sides from the front round to the left.
fn tool_between(part: &Body, [from, to]: [DVec2; 2], from_height: f64, height: f64) -> Body {
    part.tool_raised(
        &profile(Contour::rectangle(from, to)),
        Frame {
            origin: DVec3::Z * from_height,
            ..ground()
        },
        DVec3::Z * height,
    )
    .expect("the tool raises")
}

/// The outline of an L: an arm 30 long and 6 deep along the front, and at its
/// left end a stem 12 wide reaching back to 14. Raised 10, its walls are
/// numbered from the front round: the front two, the arm's right end three,
/// the arm's back four, the stem's right side five, the stem's back six and
/// the left seven. The arm's back and the stem's right side meet inwards, in
/// the L's inner corner at (12, 6).
fn ell() -> Body {
    raised(
        Contour::straight(vec![
            DVec2::new(0.0, 0.0),
            DVec2::new(30.0, 0.0),
            DVec2::new(30.0, 6.0),
            DVec2::new(12.0, 6.0),
            DVec2::new(12.0, 14.0),
            DVec2::new(0.0, 14.0),
        ]),
        10.0,
    )
}
const ELL_VOLUME: f64 = (30.0 * 6.0 + 12.0 * 8.0) * 10.0;
const ARM_END: usize = 3;
const ARM_BACK: usize = 4;
const STEM_RIGHT: usize = 5;
const STEM_BACK: usize = 6;

/// Where the pocket is sunk and the standing block stands: 10 by 10 in the
/// middle of the block 30 by 20.
const FOOTPRINT: [DVec2; 2] = [DVec2::new(10.0, 5.0), DVec2::new(20.0, 15.0)];

/// A block 30 by 20 by 10 with a pocket 10 by 10 sunk 5 into the middle of
/// its top, as `cao_part` cuts a step: the pocket's floor numbered six, its
/// walls, front, right, back and left, eight to eleven, and the tool's top,
/// standing above the block, gone. The rim, where the walls meet the block's
/// top, meets outwards; the uprights, where two walls meet, inwards.
fn pocketed() -> Body {
    let block = block([30.0, 20.0, 10.0]);
    block
        .difference(&tool_between(&block, FOOTPRINT, 5.0, 6.0))
        .expect("the pocket is cut")
}
const POCKETED_VOLUME: f64 = 30.0 * 20.0 * 10.0 - 10.0 * 10.0 * 5.0;
const POCKET_FRONT: usize = 8;
const POCKET_RIGHT: usize = 9;
const POCKET_BACK: usize = 10;
const POCKET_LEFT: usize = 11;

/// A block 30 by 20 by 10 with another, 10 by 10 by 10, standing on the
/// middle of its top, as `cao_part` joins a step: the upper block's top
/// numbered seven, its walls, front, right, back and left, eight to eleven,
/// its floor gone into the lower block's top. Its uprights meet outwards; its
/// feet, where its walls meet the lower block's top, inwards.
fn stacked() -> Body {
    let block = block([30.0, 20.0, 10.0]);
    block
        .union(&tool_between(&block, FOOTPRINT, 10.0, 10.0))
        .expect("the standing block is joined")
}
const STACKED_VOLUME: f64 = 30.0 * 20.0 * 10.0 + 10.0 * 10.0 * 10.0;
const STANDING_TOP: usize = 7;
const STANDING_FRONT: usize = 8;
const STANDING_RIGHT: usize = 9;
const STANDING_BACK: usize = 10;
const STANDING_LEFT: usize = 11;

/// A convex edge between two faces square to each other: the faces it lies
/// between, the way each faces, out of the matter, and its two ends.
struct Convex {
    called: &'static str,
    between: [usize; 2],
    facing: [DVec3; 2],
    ends: [End; 2],
}

/// An end of an edge: where it is, the face the edge runs into there — the
/// one face at that corner that is neither of the edge's two, square to it —
/// and whether matter lies past that face along the edge's line, or air.
struct End {
    at: DVec3,
    on: usize,
    matter_past: bool,
}

impl Convex {
    fn length(&self) -> f64 {
        self.ends[0].at.distance(self.ends[1].at)
    }

    fn along(&self) -> DVec3 {
        (self.ends[1].at - self.ends[0].at).normalize()
    }

    /// Where a cut of `value` crosses the face an end runs into: set back by
    /// the value along the edge's first face, then along its second.
    fn set_back(&self, end: &End, value: f64) -> [DVec3; 2] {
        let [first, second] = self.facing;
        [end.at - second * value, end.at - first * value]
    }

    /// Whether a point lies on the arc a round of `radius` ends in on the face
    /// an end runs into: on that face's plane, a radius from the round's axis,
    /// which stands the radius in from both faces, and on the edge's side of
    /// the axis, between the two set-backs.
    fn on_the_round_s_end(&self, end: &End, radius: f64, point: DVec3) -> bool {
        let [first, second] = self.facing;
        let off = point - (end.at - (first + second) * radius);
        (point - end.at).dot(self.along()).abs() < PLACE
            && (off.length() - radius).abs() < PLACE
            && off.dot(first) >= -PLACE
            && off.dot(second) >= -PLACE
    }
}

/// The L's four edges at its inner corner that meet outwards: the top's and
/// the floor's on the arm's back, running from the arm's right end, where air
/// lies past, to the stem's right side, where the stem does; and the top's
/// and the floor's on the stem's right side, from the stem's back to the
/// arm's back, where the arm does.
fn ell_edges() -> [Convex; 4] {
    [
        Convex {
            called: "the L's top edge on the arm's back",
            between: [TOP, ARM_BACK],
            facing: [DVec3::Z, DVec3::Y],
            ends: [
                End {
                    at: DVec3::new(30.0, 6.0, 10.0),
                    on: ARM_END,
                    matter_past: false,
                },
                End {
                    at: DVec3::new(12.0, 6.0, 10.0),
                    on: STEM_RIGHT,
                    matter_past: true,
                },
            ],
        },
        Convex {
            called: "the L's floor edge on the arm's back",
            between: [FLOOR, ARM_BACK],
            facing: [-DVec3::Z, DVec3::Y],
            ends: [
                End {
                    at: DVec3::new(30.0, 6.0, 0.0),
                    on: ARM_END,
                    matter_past: false,
                },
                End {
                    at: DVec3::new(12.0, 6.0, 0.0),
                    on: STEM_RIGHT,
                    matter_past: true,
                },
            ],
        },
        Convex {
            called: "the L's top edge on the stem's right side",
            between: [TOP, STEM_RIGHT],
            facing: [DVec3::Z, DVec3::X],
            ends: [
                End {
                    at: DVec3::new(12.0, 14.0, 10.0),
                    on: STEM_BACK,
                    matter_past: false,
                },
                End {
                    at: DVec3::new(12.0, 6.0, 10.0),
                    on: ARM_BACK,
                    matter_past: true,
                },
            ],
        },
        Convex {
            called: "the L's floor edge on the stem's right side",
            between: [FLOOR, STEM_RIGHT],
            facing: [-DVec3::Z, DVec3::X],
            ends: [
                End {
                    at: DVec3::new(12.0, 14.0, 0.0),
                    on: STEM_BACK,
                    matter_past: false,
                },
                End {
                    at: DVec3::new(12.0, 6.0, 0.0),
                    on: ARM_BACK,
                    matter_past: true,
                },
            ],
        },
    ]
}

/// The pocket's four rim edges, each running between the two walls beside
/// its own, with the block's matter past both.
fn rim_edges() -> [Convex; 4] {
    let [front_left, front_right, back_right, back_left] = [
        DVec3::new(10.0, 5.0, 10.0),
        DVec3::new(20.0, 5.0, 10.0),
        DVec3::new(20.0, 15.0, 10.0),
        DVec3::new(10.0, 15.0, 10.0),
    ];
    let flush = |at: DVec3, on: usize| End {
        at,
        on,
        matter_past: true,
    };
    [
        Convex {
            called: "the pocket's rim on its front wall",
            between: [TOP, POCKET_FRONT],
            facing: [DVec3::Z, DVec3::Y],
            ends: [
                flush(front_left, POCKET_LEFT),
                flush(front_right, POCKET_RIGHT),
            ],
        },
        Convex {
            called: "the pocket's rim on its right wall",
            between: [TOP, POCKET_RIGHT],
            facing: [DVec3::Z, -DVec3::X],
            ends: [
                flush(front_right, POCKET_FRONT),
                flush(back_right, POCKET_BACK),
            ],
        },
        Convex {
            called: "the pocket's rim on its back wall",
            between: [TOP, POCKET_BACK],
            facing: [DVec3::Z, -DVec3::Y],
            ends: [
                flush(back_right, POCKET_RIGHT),
                flush(back_left, POCKET_LEFT),
            ],
        },
        Convex {
            called: "the pocket's rim on its left wall",
            between: [TOP, POCKET_LEFT],
            facing: [DVec3::Z, DVec3::X],
            ends: [
                flush(back_left, POCKET_BACK),
                flush(front_left, POCKET_FRONT),
            ],
        },
    ]
}

/// The standing block's four uprights, each running from its top, where air
/// lies past, down to the lower block's top, where that block does.
fn uprights() -> [Convex; 4] {
    let upright =
        |called: &'static str, between: [usize; 2], facing: [DVec3; 2], [x, y]: [f64; 2]| Convex {
            called,
            between,
            facing,
            ends: [
                End {
                    at: DVec3::new(x, y, 20.0),
                    on: STANDING_TOP,
                    matter_past: false,
                },
                End {
                    at: DVec3::new(x, y, 10.0),
                    on: TOP,
                    matter_past: true,
                },
            ],
        };
    [
        upright(
            "the standing block's front right upright",
            [STANDING_FRONT, STANDING_RIGHT],
            [-DVec3::Y, DVec3::X],
            [20.0, 5.0],
        ),
        upright(
            "the standing block's back right upright",
            [STANDING_RIGHT, STANDING_BACK],
            [DVec3::X, DVec3::Y],
            [20.0, 15.0],
        ),
        upright(
            "the standing block's back left upright",
            [STANDING_BACK, STANDING_LEFT],
            [DVec3::Y, -DVec3::X],
            [10.0, 15.0],
        ),
        upright(
            "the standing block's front left upright",
            [STANDING_LEFT, STANDING_FRONT],
            [-DVec3::X, -DVec3::Y],
            [10.0, 5.0],
        ),
    ]
}

/// The two cuts a step lays on an edge.
#[derive(Clone, Copy)]
enum Cut {
    Flat,
    Round,
}

/// The body a step left, the value it was asked and the value it took.
struct Laid {
    body: Body,
    asked: f64,
    taken: f64,
}

impl Cut {
    fn called(self) -> &'static str {
        match self {
            Cut::Flat => "chamfer",
            Cut::Round => "fillet",
        }
    }

    /// The step on the edge a name finds, alone, at `value`.
    fn laid(self, part: &Body, edge: &EdgeName, value: f64) -> Result<Laid, Declined> {
        match self {
            Cut::Flat => part.chamfer(edge, value).map(|step| Laid {
                body: step.body,
                asked: step.asked,
                taken: step.taken,
            }),
            Cut::Round => part.fillet(edge, value).map(|step| Laid {
                body: step.body,
                asked: step.asked,
                taken: step.taken,
            }),
        }
    }

    /// The section the cut takes off an edge whose faces meet square.
    fn section(self, value: f64) -> f64 {
        match self {
            Cut::Flat => value * value / 2.0,
            Cut::Round => value * value * (1.0 - PI / 4.0),
        }
    }
}

/// The name of the one edge between the faces numbered `first` and `second`,
/// as the body hands it to whoever points at that edge.
fn edge_between(body: &Body, first: usize, second: usize) -> EdgeName {
    let named = edges_between(body, first, second);
    assert_eq!(
        named.len(),
        1,
        "one edge between {first} and {second}: {named:?}"
    );
    named[0].clone()
}

/// The names of every edge between the faces numbered `first` and `second`.
fn edges_between(body: &Body, first: usize, second: usize) -> Vec<EdgeName> {
    let sides = [vec![first as u32], vec![second as u32]];
    let [one, other] = sides.clone();
    body.edge_names()
        .into_iter()
        .filter(|name| name.between == sides || name.between == [other.clone(), one.clone()])
        .collect()
}

/// Every number some face of the body answers to: a step's, below where the
/// steps' numbers end, and a piece's, past them.
fn numbers(body: &Body) -> BTreeSet<usize> {
    (0..body.faces_end())
        .chain(body.pieces())
        .filter(|&number| body.has_face(number))
        .collect()
}

/// The numbers of the faces a step made: those the body answers to that the
/// part it was given did not.
fn faces_made(part: &Body, body: &Body) -> Vec<usize> {
    numbers(body).difference(&numbers(part)).copied().collect()
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

/// The sides a face is expected to meet, one edge each, in the order
/// [`sides_met`] lists them.
fn sides(faces: &[usize]) -> Vec<Vec<u32>> {
    let mut sorted: Vec<Vec<u32>> = faces.iter().map(|&face| vec![face as u32]).collect();
    sorted.sort();
    sorted
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
/// surface closed, no face through another, the matter it holds what
/// arithmetic promised, and the matter its triangles enclose within `drawn`
/// of it.
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

/// Holds that a step reports the value it was asked, and took it whole.
fn taken_as_asked(asked: f64, taken: f64, value: f64, what: &str) {
    assert!(
        (asked - value).abs() < PLACE && (taken - value).abs() < PLACE,
        "{what}: asked {asked} and took {taken}, where {value} fits"
    );
}

/// The largest coordinate a body reaches, on either side of the origin.
fn reach_of(body: &Body) -> f64 {
    let (low, high) = body.bounds().expect("a part has a box");
    low.abs().max(high.abs()).max_element()
}

/// Holds that each of `edges`, chamfered and filleted alone at every one of
/// [`VALUES`], leaves what [`stopped_flush`] holds, the part holding `volume`
/// before the step. Every edge is laid before any body is read, so that a
/// decline names every case that declines.
fn each_stops_flush(part: &Body, volume: f64, edges: &[Convex], called: &str) {
    assert!(
        (part.volume() - volume).abs() <= EXACT * volume,
        "{called}: the part holds {}, where arithmetic gives {volume}",
        part.volume()
    );
    let mut laid = Vec::new();
    let mut declined = Vec::new();
    for edge in edges {
        let name = edge_between(part, edge.between[0], edge.between[1]);
        for cut in [Cut::Flat, Cut::Round] {
            for value in VALUES {
                let what = format!("the {} of {value} on {}", cut.called(), edge.called);
                match cut.laid(part, &name, value) {
                    Ok(step) => laid.push((edge, cut, value, step, what)),
                    Err(reason) => declined.push(format!("{what}: {reason:?}")),
                }
            }
        }
    }
    assert!(
        declined.is_empty(),
        "{called}: each edge cut alone, its cut stopping flush where matter lies past its end; \
         declined:\n{}",
        declined.join("\n")
    );
    for (edge, cut, value, step, what) in &laid {
        stopped_flush(part, volume, edge, *cut, *value, step, what);
    }
}

/// Holds what a cut of `value` on a convex edge, alone, leaves of a part
/// holding `volume`: the value taken whole; a sound body holding the part's
/// volume less the cut's section along the edge's length, and so nothing past
/// the faces the edge runs into taken; one face laid, numbered where the
/// steps' numbers ended, flat or curved as the cut is, a flat running from
/// the face at one end of the edge to the face at the other, and either cut
/// meeting the edge's two faces and the face at each of its ends by one edge
/// each. At each end where matter lies past, the face the edge runs into
/// answers to its number still: its outline the part's with the corner the
/// edge ended at given up for the cut's end section, and meeting each of the
/// edge's two faces along one edge still, for a name to find.
fn stopped_flush(
    part: &Body,
    volume: f64,
    edge: &Convex,
    cut: Cut,
    value: f64,
    step: &Laid,
    what: &str,
) {
    taken_as_asked(step.asked, step.taken, value, what);
    let body = &step.body;
    let length = edge.length();
    let promised = volume - cut.section(value) * length;
    let drawn = match cut {
        Cut::Flat => EXACT * promised,
        Cut::Round => DRAWN * reach_of(part) * PI / 2.0 * value * length,
    };
    sound_drawn_within(body, promised, drawn, what);
    let made = part.faces_end();
    assert_eq!(
        faces_made(part, body),
        vec![made],
        "{what}: one face laid, numbered where the steps' numbers ended, and the faces the edge \
         runs into answering to theirs still: {:?}",
        numbers(body)
    );
    assert!(
        numbers(part).is_subset(&numbers(body)),
        "{what}: a face of the part went: {:?}",
        numbers(body)
    );
    let [start, end] = &edge.ends;
    match cut {
        Cut::Flat => {
            assert!(body.is_flat(made), "{what}: the chamfer lays a flat face");
            corners_are_or_on(
                body,
                made,
                &[edge.set_back(start, value), edge.set_back(end, value)].concat(),
                |_| false,
                &format!("{what}: the flat, from the face at one end of the edge to the other's"),
            );
        }
        Cut::Round => assert!(
            !body.is_flat(made) && body.plane_of(made).is_none(),
            "{what}: the fillet lays a curved face, which offers no plane to draw on"
        ),
    }
    let [one, other] = edge.between;
    assert_eq!(
        sides_met(body, made),
        sides(&[one, other, start.on, end.on]),
        "{what}: the cut meets the edge's two faces along its sides, and the face at each end of \
         the edge across its ends, by one edge each"
    );
    for end in edge.ends.iter().filter(|end| end.matter_past) {
        let face = end.on;
        let kept = part
            .plane_of(face)
            .unwrap_or_else(|| panic!("{what}: the part offers no plane on face {face}"))
            .corners
            .into_iter()
            .filter(|corner| corner.distance(end.at) >= PLACE);
        let expected: Vec<DVec3> = kept.chain(edge.set_back(end, value)).collect();
        corners_are_or_on(
            body,
            face,
            &expected,
            |corner| matches!(cut, Cut::Round) && edge.on_the_round_s_end(end, value, corner),
            &format!(
                "{what}: face {face}, which the edge runs into, grown by the cut's end section in \
                 place of the corner the edge ended at"
            ),
        );
        for side in edge.between {
            let meeting = edges_between(body, face, side);
            assert_eq!(
                meeting.len(),
                1,
                "{what}: face {face}, which the edge runs into, meets face {side} along one edge \
                 still: {meeting:?}"
            );
        }
    }
}

#[test]
fn a_chamfer_and_a_fillet_on_a_top_or_floor_edge_at_the_l_s_inner_corner_stop_flush_on_the_side_they_run_into()
 {
    each_stops_flush(
        &ell(),
        ELL_VOLUME,
        &ell_edges(),
        "the L's top and floor edges at its inner corner",
    );
}

#[test]
fn a_chamfer_and_a_fillet_on_each_rim_edge_of_a_pocket_stop_flush_on_the_walls_they_run_into_at_both_ends()
 {
    each_stops_flush(
        &pocketed(),
        POCKETED_VOLUME,
        &rim_edges(),
        "the pocket's rim edges",
    );
}

#[test]
fn a_chamfer_and_a_fillet_on_each_upright_of_a_block_standing_on_another_stop_flush_on_the_top_it_stands_on()
 {
    each_stops_flush(
        &stacked(),
        STACKED_VOLUME,
        &uprights(),
        "the standing block's uprights",
    );
}
