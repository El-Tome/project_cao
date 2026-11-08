//! A chamfer and a fillet on straight edges of an exact body that meet at a
//! corner, at one value. Where the corner is convex — each of its three edges
//! meeting outwards — two of its edges cut trim each other in a mitre and
//! leave the third sharp, and all three are closed by a patch of their own
//! kind: a triangle among flats, a piece of sphere among rounds. A corner with
//! an edge meeting inwards is declined. An operation of the kernel on
//! [`Body`], as the step on several edges of #698 is; the step of the history
//! that calls it comes with the application, in #701.
//!
//! Closes #699.
//! - Three edges of a convex corner chamfered at one value leave a closed
//!   body with a triangular face at that corner, meeting the three chamfers;
//!   three filleted at one value leave a closed body with a sphere piece
//!   there, meeting the three fillets. Volume exact, soundness checks pass —
//!   on the block's square corner,
//!   `a_chamfer_on_the_three_edges_of_a_corner_closes_it_with_a_triangle_where_the_flats_edges_cross`,
//!   `a_fillet_on_the_three_edges_of_a_corner_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces`;
//!   and on a convex corner that is not square, as the third decision asks —
//!   `a_chamfer_on_the_three_edges_of_a_corner_that_is_not_square_closes_it_with_a_triangle_where_the_flats_edges_cross`,
//!   `a_fillet_on_the_three_edges_of_a_corner_that_is_not_square_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces`
//! - Decision: the triangle of three chamfers has its corners where the
//!   chamfer edges cross on each of the three faces; the sphere piece of three
//!   fillets is centred at the point one value from all three faces, and
//!   meets each fillet along that fillet's end section. Both hold for any
//!   convex trihedral corner, not only a square one — the four tests of the
//!   criterion above, which hold the triangle's three corners and the
//!   sphere's centre, radius and the arcs it shares with the rounds:
//!   `a_chamfer_on_the_three_edges_of_a_corner_closes_it_with_a_triangle_where_the_flats_edges_cross`,
//!   `a_fillet_on_the_three_edges_of_a_corner_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces`,
//!   `a_chamfer_on_the_three_edges_of_a_corner_that_is_not_square_closes_it_with_a_triangle_where_the_flats_edges_cross`,
//!   `a_fillet_on_the_three_edges_of_a_corner_that_is_not_square_closes_it_with_a_piece_of_sphere_one_radius_from_its_three_faces`
//! - Two of a convex corner's three edges chamfered or rounded leave a closed
//!   body mitred at that corner, the third edge sharp. Volume exact, soundness
//!   checks pass — on each of the corner's three pairs, named in either order,
//!   `a_chamfer_on_two_of_a_corner_s_three_edges_mitres_their_flats_and_leaves_the_third_edge_sharp_one_distance_short`,
//!   `a_fillet_on_two_of_a_corner_s_three_edges_mitres_their_rounds_on_an_ellipse_and_leaves_the_third_edge_sharp_one_radius_short`;
//!   and on the case the issue opens on, the four top edges of a block, a
//!   mitre at each of its four corners —
//!   `a_chamfer_and_a_fillet_on_the_four_top_edges_of_a_block_mitre_its_four_corners`;
//!   and on a convex corner that is not square, the end of a prism whose
//!   sides slant, every pair of its three edges — the slanted upright among
//!   them — mitred whatever the angles —
//!   `a_chamfer_and_a_fillet_on_two_of_the_three_edges_of_a_corner_that_is_not_square_mitre_them_the_third_left_sharp`
//! - Decision: two of three is a mitre. The two cuts trim each other: a
//!   straight mitre line for chamfers, a planar ellipse arc for fillets,
//!   running from where the two cuts' edges cross on their shared face down to
//!   the third edge, which stays sharp and ends one value short of the old
//!   vertex. No patch is added —
//!   `a_chamfer_on_two_of_a_corner_s_three_edges_mitres_their_flats_and_leaves_the_third_edge_sharp_one_distance_short`,
//!   `a_fillet_on_two_of_a_corner_s_three_edges_mitres_their_rounds_on_an_ellipse_and_leaves_the_third_edge_sharp_one_radius_short`
//! - A corner with a concave edge, and any corner the kernel cannot blend,
//!   gives a broken step with its reason, never an open body. Decision: a
//!   corner is blended only when every one of its edges is convex; the inner
//!   corner of an L-shaped block and the bottom of a pocket are declined —
//!   `a_fillet_and_a_chamfer_on_edges_meeting_at_a_corner_with_a_concave_edge_are_declined_as_unsupported`;
//!   the reason is `Unsupported`, the kernel's word for a shape it does not
//!   build, until the follow-up issue the decision promises builds it. At the
//!   L's inner corner and at a pocket's, each edge but the L's upright one is
//!   declined alone already, its cut carried along its line taking matter
//!   beyond it: those lists hold what the user sees, and not why. What holds
//!   the corner's own decline is a block standing on another, whose four feet
//!   are filled in each alone, and two of which meet at each of its corners,
//!   the third edge there its upright, meeting outwards
//! - Any other corner the kernel cannot blend — no test: a corner of more
//!   than three edges, of values that differ, or of a curved edge is out of
//!   the issue's scope, and no list of edges of a block, an L or a pocket
//!   reaches one; the concave corners above are the ones this file can build
//! - The tests of #698 and the single-edge issues still pass — no test: held
//!   by those files, left as they were but for the one test #699 takes back.
//!   `a_fillet_and_a_chamfer_on_two_edges_sharing_a_vertex_are_declined_as_unsupported`
//!   held that two edges meeting at the block's top front right corner are
//!   declined; they now meet in a mitre, and its two pairs are two of the
//!   three the mitre tests above hold
//!
//! The volumes are arithmetic, measured from the corner along its three
//! edges: a point stands `(a, b, c)` from it, `a` along the edge whose length
//! is `l1`, `b` along `l2`, `c` along `l3`, on a block of volume `V`. At a
//! square corner a chamfer of the first edge alone takes away the prism
//! `b + c < d` along all of it, `d²·l1/2`; a fillet of radius `r`, the strip
//! between the faces and the round, `r²(1 − π/4)·l1`.
//!
//! Two chamfers mitred, the first two edges: the prisms `b + c < d` and
//! `a + c < d` overlap at the corner where both cut, whose section at a depth
//! `c` below their shared face is a square of side `d − c`; it holds the
//! integral from 0 to `d` of `(d − c)²`, `d³/3`, counted twice. The body keeps
//! `V − d²(l1 + l2)/2 + d³/3`.
//!
//! Two fillets mitred: at a depth `c` below the shared face, each round takes
//! the width `w = r − √(r² − (r − c)²)` off its edge's other face; the overlap
//! is a square of side `w`, holding, with `u = r − c`, the integral from 0 to
//! `r` of `(r − √(r² − u²))²`, `2r³ − 2r·πr²/4 − r³/3 = r³(5/3 − π/2)`. The
//! body keeps `V − r²(1 − π/4)(l1 + l2) + r³(5/3 − π/2)`.
//!
//! Three chamfers: outside the cube of side `d` at the corner the three prisms
//! do not overlap — a point of the first's past `a = d` is beyond the other
//! two — each taking `d²(l − d)/2`. Inside it every point goes but those with
//! `a + b + c ≥ 2d`, a tetrahedron of `d³/6` at the cube's far corner, which
//! the triangle through the three crossings bounds: if `a`, `b` and `c` are at
//! most `d`, a sum of `2d` or more leaves each pair `d` or more. The body
//! keeps `V − d²(l1 + l2 + l3)/2 + 3d³/2 − (d³ − d³/6)`, that is
//! `V − d²(l1 + l2 + l3)/2 + 2d³/3`.
//!
//! Three fillets: outside the cube of side `r` the three rounds do not
//! overlap, each taking `r²(1 − π/4)(l − r)`. Inside it every point goes but
//! the eighth of the ball of radius `r` about the cube's far corner, `πr³/6`,
//! which lies inside all three rounds. The body keeps
//! `V − r²(1 − π/4)(l1 + l2 + l3) + 3r³(1 − π/4) − r³(1 − π/6)`, that is
//! `V − r²(1 − π/4)(l1 + l2 + l3) + r³(2 − 7π/12)`.
//!
//! The four top edges of a block, two of three at each of its four corners:
//! each pair overlaps once, so the chamfered body keeps
//! `V − d²(2L + 2B)/2 + 4d³/3`, and the filleted one
//! `V − r²(1 − π/4)(2L + 2B) + 4r³(5/3 − π/2)`, `L` and `B` its length and
//! breadth: 5676 for the chamfer at 3 on the block 30 by 10 by 20, and
//! 5855.84 for the fillet at 3. The corners that are not square are measured
//! where their tests stand.

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

/// The numbers a raised block's faces answer to: its floor nought, its top
/// one, and its sides from the front round to the left, two to five.
const FLOOR: usize = 0;
const TOP: usize = 1;
const FRONT: usize = 2;
const RIGHT: usize = 3;
const BACK: usize = 4;
const LEFT: usize = 5;

/// The block the corners are held on: 30 along X, 10 across, 20 high — three
/// lengths apart, so that each edge of a corner is told by its length.
const LENGTH: f64 = 30.0;
const BREADTH: f64 = 10.0;
const HEIGHT: f64 = 20.0;
const BLOCK_VOLUME: f64 = LENGTH * BREADTH * HEIGHT;

/// The two edges a step names among a corner's three, and the third it leaves,
/// as indices into [`Corner`]'s edges: each of the three pairs, in either
/// order.
const TWO_OF_THREE: [[usize; 3]; 6] = [
    [0, 1, 2],
    [1, 0, 2],
    [0, 2, 1],
    [2, 0, 1],
    [1, 2, 0],
    [2, 1, 0],
];

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

/// A corner where three edges of a block meet, square: where it stands, and
/// for each of its three edges the way it runs from the corner, how long it
/// is, the face across from it — the one of the corner's three faces it does
/// not lie on —, the face it ends on at its far end, and what it is called.
struct Corner {
    at: DVec3,
    along: [DVec3; 3],
    length: [f64; 3],
    across: [usize; 3],
    far: [usize; 3],
    called: [&'static str; 3],
}

/// The block's top front right corner: the top's front edge running 30 to the
/// left wall, the top's right edge 10 to the back, and the front's right edge
/// 20 down to the floor.
fn top_front_right() -> Corner {
    Corner {
        at: DVec3::new(LENGTH, 0.0, HEIGHT),
        along: [-DVec3::X, DVec3::Y, -DVec3::Z],
        length: [LENGTH, BREADTH, HEIGHT],
        across: [RIGHT, FRONT, TOP],
        far: [LEFT, BACK, FLOOR],
        called: [
            "the top's front edge",
            "the top's right edge",
            "the front's right edge",
        ],
    }
}

impl Corner {
    /// The point `by` from the corner along the edges `order` names: the first
    /// length along the first, the second along the second, the third along
    /// the third.
    fn point(&self, order: [usize; 3], by: [f64; 3]) -> DVec3 {
        order
            .into_iter()
            .zip(by)
            .fold(self.at, |point, (edge, length)| {
                point + self.along[edge] * length
            })
    }

    /// How far a point stands from the corner along the edge `edge`.
    fn along_edge(&self, point: DVec3, edge: usize) -> f64 {
        (point - self.at).dot(self.along[edge])
    }

    /// The round a fillet of `radius` lays on the edge `edge`, whole: its axis
    /// the radius in from the edge's two faces — the one it shares with the
    /// edge `beside` and the one across from `beside` — and running from the
    /// corner's plane to the edge's far end, turning from facing the way the
    /// shared face does to facing the way the other does.
    fn round(&self, edge: usize, [beside, shared]: [usize; 2], radius: f64) -> Round {
        Round {
            start: self.at + (self.along[beside] + self.along[shared]) * radius,
            along: self.along[edge],
            length: self.length[edge],
            radius,
            facing: [-self.along[shared], -self.along[beside]],
        }
    }

    /// The name of the corner's edge `edge` on the body: the edge between the
    /// two faces across from the corner's two other edges.
    fn name(&self, body: &Body, edge: usize) -> EdgeName {
        let [one, other] = [(edge + 1) % 3, (edge + 2) % 3];
        edge_between(body, self.across[one], self.across[other])
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
    body.edge_names()
        .into_iter()
        .filter(|name| lies_between(name, first, second))
        .collect()
}

/// Whether a name is that of an edge between the faces numbered `first` and
/// `second`, whichever it lists first.
fn lies_between(name: &EdgeName, first: usize, second: usize) -> bool {
    let sides = [vec![first as u32], vec![second as u32]];
    let [one, other] = sides.clone();
    name.between == sides || name.between == [other, one]
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

/// Holds that a face's corners are the points expected: each of them a
/// corner, and no corner anywhere else.
fn corners_are(body: &Body, face: usize, expected: &[DVec3], what: &str) {
    let plane = body
        .plane_of(face)
        .unwrap_or_else(|| panic!("{what}: the body offers no plane on face {face}"));
    for corner in &plane.corners {
        assert!(
            expected.iter().any(|point| point.distance(*corner) < PLACE),
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

/// Holds that the face numbered `face` is flat and faces the way `facing`
/// points.
fn faces_toward(body: &Body, face: usize, facing: DVec3, what: &str) {
    let plane = body
        .plane_of(face)
        .unwrap_or_else(|| panic!("{what}: the body offers no plane on face {face}"));
    assert!(
        plane.normal.distance(facing) < PLACE,
        "{what}: face {face} faces {}, not {facing}",
        plane.normal
    );
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

/// What a block keeps once two of a square corner's edges, `lengths` long,
/// are chamfered at `distance` and mitred: each flat's prism taken whole, and
/// the pyramid where both cut given back once, as the module's documentation
/// derives it.
fn mitred_by_flats(lengths: [f64; 2], distance: f64) -> f64 {
    let [first, second] = lengths;
    BLOCK_VOLUME - distance * distance * (first + second) / 2.0 + distance.powi(3) / 3.0
}

#[test]
fn a_chamfer_on_two_of_a_corner_s_three_edges_mitres_their_flats_and_leaves_the_third_edge_sharp_one_distance_short()
 {
    let distance = 3.0;
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let corner = top_front_right();
    let steps_end = block.faces_end();

    for [first, second, third] in TWO_OF_THREE {
        let what = format!(
            "the chamfer on {} then {}, {} left",
            corner.called[first], corner.called[second], corner.called[third]
        );
        let names = [corner.name(&block, first), corner.name(&block, second)];

        let chamfered = block
            .chamfer_edges(&names, distance)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        taken_as_asked(chamfered.asked, chamfered.taken, distance, &what);
        let body = &chamfered.body;
        let [long_first, long_second, long_third] =
            [first, second, third].map(|edge| corner.length[edge]);
        let promised = mitred_by_flats([long_first, long_second], distance);
        sound_drawn_within(body, promised, EXACT * promised, &what);
        let [flat_first, flat_second] = [steps_end, steps_end + 1];
        assert_eq!(
            faces_made(&block, body),
            vec![flat_first, flat_second],
            "{what}: one flat for each edge, numbered in the order the step names them, and no \
             face laid at the corner: {:?}",
            numbers(body)
        );
        assert!(
            numbers(&block).is_subset(&numbers(body)),
            "{what}: a face of the block went: {:?}",
            numbers(body)
        );
        let at = |by: [f64; 3]| corner.point([first, second, third], by);
        let d = distance;
        let [across_first, across_second, shared] =
            [first, second, third].map(|edge| corner.across[edge]);
        let [far_first, far_second, far_third] =
            [first, second, third].map(|edge| corner.far[edge]);
        let crossing = at([d, d, 0.0]);
        let short_of_the_corner = at([0.0, 0.0, d]);

        corners_are(
            body,
            flat_first,
            &[
                at([long_first, d, 0.0]),
                at([long_first, 0.0, d]),
                short_of_the_corner,
                crossing,
            ],
            &format!(
                "{what}: the flat of the first edge, from its far end to the mitre, which runs \
                 from where the two flats cross the shared face down to the third edge"
            ),
        );
        faces_toward(
            body,
            flat_first,
            -(corner.along[second] + corner.along[third]).normalize(),
            &format!("{what}: the flat of the first edge"),
        );
        assert_eq!(
            sides_met(body, flat_first),
            sides(&[shared, across_second, far_first, flat_second]),
            "{what}: the flat of the first edge meets its edge's two faces along its sides, the \
             face its edge ends on, and the other flat along the mitre, by one edge each"
        );
        corners_are(
            body,
            flat_second,
            &[
                at([d, long_second, 0.0]),
                at([0.0, long_second, d]),
                short_of_the_corner,
                crossing,
            ],
            &format!("{what}: the flat of the second edge, from its far end to the mitre"),
        );
        faces_toward(
            body,
            flat_second,
            -(corner.along[first] + corner.along[third]).normalize(),
            &format!("{what}: the flat of the second edge"),
        );
        assert_eq!(
            sides_met(body, flat_second),
            sides(&[shared, across_first, far_second, flat_first]),
            "{what}: the flat of the second edge meets its edge's two faces along its sides, the \
             face its edge ends on, and the other flat along the mitre, by one edge each"
        );
        corners_are(
            body,
            shared,
            &[
                crossing,
                at([long_first, d, 0.0]),
                at([long_first, long_second, 0.0]),
                at([d, long_second, 0.0]),
            ],
            &format!(
                "{what}: the face the two edges share, set back by the distance from each, its \
                 corner at the crossing"
            ),
        );
        assert_eq!(
            sides_met(body, shared),
            sides(&[flat_first, flat_second, far_first, far_second]),
            "{what}: the face the two edges share meets the two flats, and no longer the third \
             edge's faces"
        );
        let third_s_far_end = at([0.0, 0.0, long_third]);
        for (face, other, [beside, beyond], far, flat, which) in [
            (
                across_second,
                across_first,
                [at([long_first, 0.0, d]), at([long_first, 0.0, long_third])],
                far_first,
                flat_first,
                "the face the first edge and the third share",
            ),
            (
                across_first,
                across_second,
                [
                    at([0.0, long_second, d]),
                    at([0.0, long_second, long_third]),
                ],
                far_second,
                flat_second,
                "the face the second edge and the third share",
            ),
        ] {
            corners_are(
                body,
                face,
                &[short_of_the_corner, beside, beyond, third_s_far_end],
                &format!(
                    "{what}: {which}, set back by the distance from the flat, the third edge \
                     along it ending one distance short of the corner"
                ),
            );
            assert_eq!(
                sides_met(body, face),
                sides(&[flat, far, far_third, other]),
                "{what}: {which} meets its flat, the faces the edges end on, and the third \
                 edge's other face, sharp"
            );
        }
        assert_eq!(
            edges_between(body, across_first, across_second).len(),
            1,
            "{what}: {} is still one sharp edge, from where it ends one distance short of the \
             corner to its far end",
            corner.called[third]
        );
    }
}

/// What a block keeps once two of a square corner's edges, `lengths` long,
/// are rounded at `radius` and mitred: each round's strip taken whole, and the
/// overlap where both take given back once, as the module's documentation
/// derives it.
fn mitred_by_rounds(lengths: [f64; 2], radius: f64) -> f64 {
    let [first, second] = lengths;
    BLOCK_VOLUME - radius * radius * (1.0 - PI / 4.0) * (first + second)
        + radius.powi(3) * (5.0 / 3.0 - PI / 2.0)
}

/// What the block keeps once its four top edges are chamfered at `distance`,
/// mitred at its four top corners: each flat's prism taken whole, and at each
/// corner the pyramid where two cut given back once, as the module's
/// documentation derives it.
fn ringed_by_flats(distance: f64) -> f64 {
    BLOCK_VOLUME - distance * distance * (2.0 * LENGTH + 2.0 * BREADTH) / 2.0
        + 4.0 * distance.powi(3) / 3.0
}

/// What the block keeps once its four top edges are rounded at `radius`,
/// mitred at its four top corners, as [`ringed_by_flats`] counts it.
fn ringed_by_rounds(radius: f64) -> f64 {
    BLOCK_VOLUME - radius * radius * (1.0 - PI / 4.0) * (2.0 * LENGTH + 2.0 * BREADTH)
        + 4.0 * radius.powi(3) * (5.0 / 3.0 - PI / 2.0)
}

/// An edge of the block's top: the wall it runs along and the way that wall
/// faces, the corner it starts at, the way it runs from it — round the top
/// anticlockwise, seen from above — and how long it is.
struct TopEdge {
    wall: usize,
    facing: DVec3,
    start: DVec3,
    along: DVec3,
    length: f64,
}

/// The block's four top edges, round the top from the front: the front, the
/// right, the back and the left.
fn top_edges() -> [TopEdge; 4] {
    [
        TopEdge {
            wall: FRONT,
            facing: -DVec3::Y,
            start: DVec3::new(0.0, 0.0, HEIGHT),
            along: DVec3::X,
            length: LENGTH,
        },
        TopEdge {
            wall: RIGHT,
            facing: DVec3::X,
            start: DVec3::new(LENGTH, 0.0, HEIGHT),
            along: DVec3::Y,
            length: BREADTH,
        },
        TopEdge {
            wall: BACK,
            facing: DVec3::Y,
            start: DVec3::new(LENGTH, BREADTH, HEIGHT),
            along: -DVec3::X,
            length: LENGTH,
        },
        TopEdge {
            wall: LEFT,
            facing: -DVec3::X,
            start: DVec3::new(0.0, BREADTH, HEIGHT),
            along: -DVec3::Y,
            length: BREADTH,
        },
    ]
}

impl TopEdge {
    /// The corner it ends at.
    fn end(&self) -> DVec3 {
        self.start + self.along * self.length
    }

    /// The edge's name on the part.
    fn name(&self, part: &Body) -> EdgeName {
        edge_between(part, TOP, self.wall)
    }

    /// Where a cut set back by `value` on the top from this edge crosses the
    /// cut of the edge before it, at its start, and of the edge after it, at
    /// its end.
    fn crossings(&self, value: f64) -> [DVec3; 2] {
        let into = -self.facing * value;
        [
            self.start + self.along * value + into,
            self.end() - self.along * value + into,
        ]
    }

    /// The flat a chamfer of `distance` lays on the edge, mitred at both its
    /// ends: from the two crossings on the top down to the two uprights, one
    /// distance below the top, and the way it faces.
    fn flat(&self, distance: f64) -> ([DVec3; 4], DVec3) {
        let [at_start, at_end] = self.crossings(distance);
        let down = DVec3::Z * distance;
        (
            [at_start, at_end, self.end() - down, self.start - down],
            (self.facing + DVec3::Z).normalize(),
        )
    }

    /// The round a fillet of `radius` lays on the edge, whole: its axis the
    /// radius in from the top and from the wall, along the whole edge.
    fn round(&self, radius: f64) -> Round {
        Round {
            start: self.start - (self.facing + DVec3::Z) * radius,
            along: self.along,
            length: self.length,
            radius,
            facing: [DVec3::Z, self.facing],
        }
    }

    /// Whether a point stands on this edge's side of the plane halfway between
    /// it and the edge before it, at its start, and of the plane halfway
    /// between it and the edge after it, at its end: no nearer either corner
    /// along this edge than in from this edge's wall.
    fn between_its_mitres(&self, point: DVec3) -> bool {
        let inwards = |corner: DVec3| (point - corner).dot(-self.facing);
        (point - self.start).dot(self.along) >= inwards(self.start) - PLACE
            && (self.end() - point).dot(self.along) >= inwards(self.end()) - PLACE
    }
}

/// The largest coordinate a body reaches, on either side of the origin.
fn reach_of(body: &Body) -> f64 {
    let (low, high) = body.bounds().expect("a part has a box");
    low.abs().max(high.abs()).max_element()
}

/// The round a fillet is expected to lay: a cylinder of `radius` about the
/// line from `start` along `along` for `length`, turning about it from facing
/// the way the edge's first face does to facing the way its second does.
/// Facing each face's way where it meets it is what tangent to both comes to.
struct Round {
    start: DVec3,
    along: DVec3,
    length: f64,
    radius: f64,
    facing: [DVec3; 2],
}

impl Round {
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
    /// ends, and between its two sides.
    fn holds(&self, point: DVec3) -> bool {
        let off = point - self.start;
        let along = off.dot(self.along);
        let across = off - self.along * along;
        let [first, second] = self.facing;
        let least = self.radius * first.dot(second);
        (across.length() - self.radius).abs() < PLACE
            && (-PLACE..=self.length + PLACE).contains(&along)
            && across.dot(first) >= least - PLACE
            && across.dot(second) >= least - PLACE
    }

    /// The part of the round from `from` along its axis to `to`.
    fn part(&self, from: f64, to: f64) -> Round {
        Round {
            start: self.start + self.along * from,
            length: to - from,
            ..*self
        }
    }

    /// The round's area, its turn times its radius times its length.
    fn area(&self) -> f64 {
        let [first, second] = self.facing;
        first.dot(second).acos() * self.radius * self.length
    }
}

/// Holds that every ray aimed square at the round's axis from outside, a
/// tenth to nine tenths of the way along it, meets the face numbered `face`
/// first, where it faces away from the axis along the ray.
fn met_from_outside(body: &Body, face: usize, round: &Round, what: &str) {
    for along in [0.11, 0.47, 0.83] {
        for share in [0.1, 0.3, 0.5, 0.7, 0.9] {
            let facing = round.facing_at(share);
            let from = round.axis_at(along) + facing * (round.radius + 10.0);
            let hit = body.ray_hit(from, -facing).unwrap_or_else(|| {
                panic!("{what}: a ray from {from} toward the axis meets nothing")
            });
            assert_eq!(
                hit.face, face,
                "{what}: a ray from {from} toward the axis meets face {} before the round",
                hit.face
            );
            assert!(
                hit.normal.distance(facing) < PLACE,
                "{what}: where a ray from {from} meets it, the round faces {}, not {facing}, \
                 away from its axis",
                hit.normal
            );
        }
    }
}

/// The part with a block standing on the ground between the corners `from`
/// and `to`, raised from `from_height` by `height`, cut from it as `cao_part`
/// cuts a step: the tool's faces numbered past the part's, its floor first,
/// then its top, then its sides from the front round to the left.
fn cut_from(part: &Body, [from, to]: [DVec2; 2], from_height: f64, height: f64) -> Body {
    part.difference(&tool_between(part, [from, to], from_height, height))
        .expect("the cut is built")
}

/// The part with a block between the corners `from` and `to`, raised from
/// `from_height` by `height`, joined to it as `cao_part` joins a step,
/// numbered as [`cut_from`] numbers it.
fn joined_to(part: &Body, [from, to]: [DVec2; 2], from_height: f64, height: f64) -> Body {
    part.union(&tool_between(part, [from, to], from_height, height))
        .expect("the join is built")
}

/// The block a step raises between the corners `from` and `to`, from
/// `from_height` by `height`.
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

/// A block 30 by 20 by 10 with another, 10 by 10 by 10, standing on the
/// middle of its top: the upper block's walls, front, right, back and left,
/// numbered eight to eleven, its top seven, its floor gone into the lower
/// block's top. Its four feet, where its walls meet that top, meet inwards;
/// two meet at each of its four corners, where its upright edge meets
/// outwards.
fn stacked() -> Body {
    joined_to(
        &block([30.0, 20.0, 10.0]),
        [DVec2::new(10.0, 5.0), DVec2::new(20.0, 15.0)],
        10.0,
        10.0,
    )
}

/// The numbers of the walls of the block standing on the other, from the
/// front round to the left.
const STANDING: [usize; 4] = [8, 9, 10, 11];

/// The outline of an L: an arm 30 long and 6 deep along the front, and at its
/// left end a stem 12 wide reaching back to 14. Raised 10, its walls are
/// numbered from the front round: the front two, the arm's right end three,
/// the arm's back four, the stem's right side five, the stem's back six and
/// the left seven. The arm's back and the stem's right side meet inwards, in
/// the L's inner corner.
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
const ARM_BACK: usize = 4;
const STEM_RIGHT: usize = 5;

/// A block 30 by 20 by 10 with a pocket 10 by 10 sunk 5 into the middle of
/// its top: the pocket's floor numbered six, its walls, front, right, back
/// and left, eight to eleven. Its floor's edges and its uprights meet
/// inwards, its rim outwards.
fn pocketed() -> Body {
    cut_from(
        &block([30.0, 20.0, 10.0]),
        [DVec2::new(10.0, 5.0), DVec2::new(20.0, 15.0)],
        5.0,
        6.0,
    )
}
const POCKET_FLOOR: usize = 6;
const POCKET_FRONT: usize = 8;
const POCKET_RIGHT: usize = 9;

#[test]
fn a_fillet_and_a_chamfer_on_edges_meeting_at_a_corner_with_a_concave_edge_are_declined_as_unsupported()
 {
    let value = 1.0;
    let stacked = stacked();
    let feet: Vec<EdgeName> = STANDING
        .iter()
        .map(|&wall| edge_between(&stacked, TOP, wall))
        .collect();
    for (foot, name) in STANDING.iter().zip(&feet) {
        let which = format!("the foot of the standing block's wall {foot}, alone");
        stacked
            .fillet(name, value)
            .unwrap_or_else(|declined| panic!("{which}: the fillet declined, {declined:?}"));
        stacked
            .chamfer(name, value)
            .unwrap_or_else(|declined| panic!("{which}: the chamfer declined, {declined:?}"));
    }
    let ell = ell();
    let pocketed = pocketed();
    let names = |body: &Body, pairs: &[[usize; 2]]| -> Vec<EdgeName> {
        pairs
            .iter()
            .map(|&[first, second]| edge_between(body, first, second))
            .collect()
    };

    for (what, body, list) in [
        (
            "the front and right feet of the standing block, meeting at its front right corner",
            &stacked,
            vec![feet[0].clone(), feet[1].clone()],
        ),
        (
            "the right and back feet of the standing block, meeting at its back right corner",
            &stacked,
            vec![feet[1].clone(), feet[2].clone()],
        ),
        (
            "the four feet of the standing block, two meeting at each of its corners",
            &stacked,
            feet.clone(),
        ),
        (
            "the two top edges at the L's inner corner",
            &ell,
            names(&ell, &[[TOP, ARM_BACK], [TOP, STEM_RIGHT]]),
        ),
        (
            "a top edge at the L's inner corner and the L's inner upright",
            &ell,
            names(&ell, &[[TOP, ARM_BACK], [ARM_BACK, STEM_RIGHT]]),
        ),
        (
            "the three edges at the top of the L's inner corner",
            &ell,
            names(
                &ell,
                &[[TOP, ARM_BACK], [TOP, STEM_RIGHT], [ARM_BACK, STEM_RIGHT]],
            ),
        ),
        (
            "two edges of the pocket's floor, meeting at its front right corner",
            &pocketed,
            names(
                &pocketed,
                &[[POCKET_FLOOR, POCKET_FRONT], [POCKET_FLOOR, POCKET_RIGHT]],
            ),
        ),
        (
            "an edge of the pocket's floor and the upright it meets",
            &pocketed,
            names(
                &pocketed,
                &[[POCKET_FLOOR, POCKET_FRONT], [POCKET_FRONT, POCKET_RIGHT]],
            ),
        ),
        (
            "two edges of the pocket's rim, meeting above its front right upright",
            &pocketed,
            names(&pocketed, &[[TOP, POCKET_FRONT], [TOP, POCKET_RIGHT]]),
        ),
    ] {
        let filleted = body.fillet_edges(&list, value).err();
        let chamfered = body.chamfer_edges(&list, value).err();

        assert_eq!(
            filleted,
            Some(Declined::Unsupported),
            "the fillet on {what}: a broken step and its reason, and no body"
        );
        assert_eq!(
            chamfered,
            Some(Declined::Unsupported),
            "the chamfer on {what}: a broken step and its reason, and no body"
        );
    }
}

#[test]
fn a_fillet_on_two_of_a_corner_s_three_edges_mitres_their_rounds_on_an_ellipse_and_leaves_the_third_edge_sharp_one_radius_short()
 {
    let radius = 3.0;
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let corner = top_front_right();
    let steps_end = block.faces_end();

    for [first, second, third] in TWO_OF_THREE {
        let what = format!(
            "the fillet on {} then {}, {} left",
            corner.called[first], corner.called[second], corner.called[third]
        );
        let names = [corner.name(&block, first), corner.name(&block, second)];

        let filleted = block
            .fillet_edges(&names, radius)
            .unwrap_or_else(|declined| panic!("{what}: declined, {declined:?}"));

        taken_as_asked(filleted.asked, filleted.taken, radius, &what);
        let body = &filleted.body;
        let [long_first, long_second, long_third] =
            [first, second, third].map(|edge| corner.length[edge]);
        let rounds = [
            corner.round(first, [second, third], radius),
            corner.round(second, [first, third], radius),
        ];
        let area: f64 = rounds.iter().map(Round::area).sum();
        sound_drawn_within(
            body,
            mitred_by_rounds([long_first, long_second], radius),
            DRAWN * reach_of(&block) * area,
            &what,
        );
        let [round_first, round_second] = [steps_end, steps_end + 1];
        assert_eq!(
            faces_made(&block, body),
            vec![round_first, round_second],
            "{what}: one round for each edge, numbered in the order the step names them, and no \
             face laid at the corner: {:?}",
            numbers(body)
        );
        assert!(
            numbers(&block).is_subset(&numbers(body)),
            "{what}: a face of the block went: {:?}",
            numbers(body)
        );
        let at = |by: [f64; 3]| corner.point([first, second, third], by);
        let r = radius;
        let [across_first, across_second, shared] =
            [first, second, third].map(|edge| corner.across[edge]);
        let [far_first, far_second, far_third] =
            [first, second, third].map(|edge| corner.far[edge]);

        for (face, round, own, other, sides_expected, which) in [
            (
                round_first,
                &rounds[0],
                first,
                second,
                sides(&[shared, across_second, far_first, round_second]),
                "the round of the first edge",
            ),
            (
                round_second,
                &rounds[1],
                second,
                first,
                sides(&[shared, across_first, far_second, round_first]),
                "the round of the second edge",
            ),
        ] {
            assert!(
                !body.is_flat(face) && body.plane_of(face).is_none(),
                "{what}: {which} is curved, and offers no plane to draw on"
            );
            let corners: Vec<DVec3> = body.triangles_of(face).flatten().collect();
            assert!(!corners.is_empty(), "{what}: {which} is drawn");
            for point in corners {
                assert!(
                    round.holds(point),
                    "{what}: {which} is drawn through {point}, off its cylinder of {r} about the \
                     line from {} along {}",
                    round.start,
                    round.along
                );
                assert!(
                    corner.along_edge(point, own) >= corner.along_edge(point, other) - PLACE,
                    "{what}: {which} is drawn through {point}, past the plane halfway between the \
                     two edges, where the other round meets it in the mitre"
                );
            }
            met_from_outside(
                body,
                face,
                &round.part(r, round.length),
                &format!("{what}: {which}, past the mitre"),
            );
            assert_eq!(
                sides_met(body, face),
                sides_expected,
                "{what}: {which} meets its edge's two faces along its sides, the face its edge \
                 ends on, and the other round along the mitre, by one edge each"
            );
        }
        let short_of_the_corner = at([0.0, 0.0, r]);
        corners_are(
            body,
            shared,
            &[
                at([r, r, 0.0]),
                at([long_first, r, 0.0]),
                at([long_first, long_second, 0.0]),
                at([r, long_second, 0.0]),
            ],
            &format!(
                "{what}: the face the two edges share, set back by the radius from each, its \
                 corner where the two rounds' sides cross"
            ),
        );
        assert_eq!(
            sides_met(body, shared),
            sides(&[round_first, round_second, far_first, far_second]),
            "{what}: the face the two edges share meets the two rounds, and no longer the third \
             edge's faces"
        );
        let third_s_far_end = at([0.0, 0.0, long_third]);
        for (face, other, [beside, beyond], far, round, which) in [
            (
                across_second,
                across_first,
                [at([long_first, 0.0, r]), at([long_first, 0.0, long_third])],
                far_first,
                round_first,
                "the face the first edge and the third share",
            ),
            (
                across_first,
                across_second,
                [
                    at([0.0, long_second, r]),
                    at([0.0, long_second, long_third]),
                ],
                far_second,
                round_second,
                "the face the second edge and the third share",
            ),
        ] {
            corners_are(
                body,
                face,
                &[short_of_the_corner, beside, beyond, third_s_far_end],
                &format!(
                    "{what}: {which}, set back by the radius from the round, the third edge \
                     along it ending one radius short of the corner"
                ),
            );
            assert_eq!(
                sides_met(body, face),
                sides(&[round, far, far_third, other]),
                "{what}: {which} meets its round, the faces the edges end on, and the third \
                 edge's other face, sharp"
            );
        }
        assert_eq!(
            edges_between(body, across_first, across_second).len(),
            1,
            "{what}: {} is still one sharp edge, from where it ends one radius short of the \
             corner to its far end",
            corner.called[third]
        );
    }
}

#[test]
fn a_chamfer_and_a_fillet_on_the_four_top_edges_of_a_block_mitre_its_four_corners() {
    let value = 3.0;
    let block = block([LENGTH, BREADTH, HEIGHT]);
    let edges = top_edges();
    let steps_end = block.faces_end();

    for (order, called) in [
        ([0, 1, 2, 3], "named round the top"),
        ([0, 2, 1, 3], "named the front and the back first"),
    ] {
        let names: Vec<EdgeName> = order.iter().map(|&edge| edges[edge].name(&block)).collect();

        let chamfered = block
            .chamfer_edges(&names, value)
            .unwrap_or_else(|declined| panic!("the chamfer, {called}: declined, {declined:?}"));
        let filleted = block
            .fillet_edges(&names, value)
            .unwrap_or_else(|declined| panic!("the fillet, {called}: declined, {declined:?}"));

        let laid_for = |edge: usize| {
            steps_end
                + order
                    .iter()
                    .position(|&named| named == edge)
                    .expect("every top edge is named")
        };
        let laid: Vec<usize> = (0..4).map(laid_for).collect();
        let rounds: Vec<Round> = edges.iter().map(|edge| edge.round(value)).collect();
        let area: f64 = rounds.iter().map(Round::area).sum();
        for (what, asked, taken, body, promised, drawn) in [
            (
                format!("the chamfer on the four top edges, {called}"),
                chamfered.asked,
                chamfered.taken,
                &chamfered.body,
                ringed_by_flats(value),
                EXACT * ringed_by_flats(value),
            ),
            (
                format!("the fillet on the four top edges, {called}"),
                filleted.asked,
                filleted.taken,
                &filleted.body,
                ringed_by_rounds(value),
                DRAWN * reach_of(&block) * area,
            ),
        ] {
            taken_as_asked(asked, taken, value, &what);
            sound_drawn_within(body, promised, drawn, &what);
            let mut made = laid.clone();
            made.sort();
            assert_eq!(
                faces_made(&block, body),
                made,
                "{what}: one face for each edge, numbered in the order the step names them, and \
                 no face laid at a corner: {:?}",
                numbers(body)
            );
            assert!(
                numbers(&block).is_subset(&numbers(body)),
                "{what}: a face of the block went: {:?}",
                numbers(body)
            );
            let top: Vec<DVec3> = edges.iter().map(|edge| edge.crossings(value)[0]).collect();
            corners_are(
                body,
                TOP,
                &top,
                &format!("{what}: the top, set back by the value from its four edges"),
            );
            assert_eq!(
                sides_met(body, TOP),
                sides(&laid),
                "{what}: the top meets the four faces laid, and no wall"
            );
            for (index, edge) in edges.iter().enumerate() {
                let [before, after] = [(index + 3) % 4, (index + 1) % 4];
                let down = DVec3::Z * value;
                let floor = DVec3::Z * HEIGHT;
                corners_are(
                    body,
                    edge.wall,
                    &[
                        edge.start - down,
                        edge.end() - down,
                        edge.end() - floor,
                        edge.start - floor,
                    ],
                    &format!(
                        "{what}: wall {}, set back by the value from the top, its uprights \
                         ending one value below it",
                        edge.wall
                    ),
                );
                assert_eq!(
                    sides_met(body, edge.wall),
                    sides(&[FLOOR, laid[index], edges[before].wall, edges[after].wall]),
                    "{what}: wall {} meets the floor, the face laid on its top edge, and the \
                     two walls beside it along their uprights, sharp",
                    edge.wall
                );
                assert_eq!(
                    sides_met(body, laid[index]),
                    sides(&[TOP, edge.wall, laid[before], laid[after]]),
                    "{what}: the face laid on wall {}'s top edge meets the top and the wall \
                     along its sides, and the faces laid beside it along the mitres at its two \
                     ends",
                    edge.wall
                );
            }
        }
        for (index, edge) in edges.iter().enumerate() {
            let which = format!(
                "the chamfer on the four top edges, {called}: the flat on wall {}'s top edge",
                edge.wall
            );
            let (corners, facing) = edge.flat(value);
            corners_are(&chamfered.body, laid[index], &corners, &which);
            faces_toward(&chamfered.body, laid[index], facing, &which);
        }
        for (index, (edge, round)) in edges.iter().zip(&rounds).enumerate() {
            let which = format!(
                "the fillet on the four top edges, {called}: the round on wall {}'s top edge",
                edge.wall
            );
            let body = &filleted.body;
            let face = laid[index];
            assert!(
                !body.is_flat(face) && body.plane_of(face).is_none(),
                "{which} is curved, and offers no plane to draw on"
            );
            let corners: Vec<DVec3> = body.triangles_of(face).flatten().collect();
            assert!(!corners.is_empty(), "{which} is drawn");
            for point in corners {
                assert!(
                    round.holds(point),
                    "{which} is drawn through {point}, off its cylinder of {value} about the \
                     line from {} along {}",
                    round.start,
                    round.along
                );
                assert!(
                    edge.between_its_mitres(point),
                    "{which} is drawn through {point}, past a plane halfway between its edge and \
                     the one beside it, where the round beside it meets it in a mitre"
                );
            }
            met_from_outside(
                body,
                face,
                &round.part(value, round.length - value),
                &format!("{which}, between its mitres"),
            );
        }
    }
}
