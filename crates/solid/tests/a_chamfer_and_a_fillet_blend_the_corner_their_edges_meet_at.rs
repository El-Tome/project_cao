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
//!   `a_chamfer_and_a_fillet_on_the_four_top_edges_of_a_block_mitre_its_four_corners`
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
//!   build, until the follow-up issue the decision promises builds it
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
//! breadth. The corners that are not square are measured where their tests
//! stand.

use std::collections::BTreeSet;

use cao_solid::profile::{Contour, Frame, Profile};
use cao_solid::soundness::{closed, enclosed, uncrossed};
use cao_solid::{Body, EdgeName, Loop};
use glam::{DVec2, DVec3};

/// How far a corner may stand from where the arithmetic puts it, on a plane:
/// each is placed exactly, so what is left is rounding.
const PLACE: f64 = 1e-9;

/// How near the arithmetic an exact volume comes, as a fraction of it.
const EXACT: f64 = 1e-9;

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
