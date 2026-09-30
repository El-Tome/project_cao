//! The sixteen cases of `docs/exact-kernel-journal.md` on the exact kernel of
//! #498, each held to what was reasoned out by hand before it ran: how many
//! faces, edges and vertices the result has, the volume arithmetic promises
//! to rounding, a listing that stands on its geometry, and triangles closed
//! and uncrossed.
//!
//! The kernel does not yet merge faces lying on one surface with one side
//! across an edge nothing else uses, nor edges meeting at a vertex only they
//! reach: the counts are those of the result as it stands, each such face and
//! edge counted.

use std::f64::consts::{PI, TAU};

use cao_solid::brep::{Body, Curve, Declined, Surface};
use cao_solid::exact::MESH;
use cao_solid::profile::{Contour, Frame, Run};
use cao_solid::soundness::{closed, listed, uncrossed};
use glam::{DVec2, DVec3};

const HEIGHT: f64 = 10.0;
const RADIUS: f64 = 20.0;
const HOLE: f64 = 5.0;

fn frame(origin: DVec3, u: DVec3, v: DVec3) -> Frame {
    Frame { origin, u, v }
}

fn raised(outline: Contour, frame: Frame, travel: DVec3) -> Body {
    Body::raised(&outline, &[], frame, travel).expect("a leaf raises")
}

/// A box between two corners.
fn boxed(low: [f64; 3], high: [f64; 3]) -> Body {
    raised(
        Contour::rectangle(DVec2::new(low[0], low[1]), DVec2::new(high[0], high[1])),
        frame(DVec3::Z * low[2], DVec3::X, DVec3::Y),
        DVec3::Z * (high[2] - low[2]),
    )
}

/// A whole circle, handed as one run all the way round from one corner.
fn circle(center: DVec2, radius: f64) -> Contour {
    Contour {
        corners: vec![center + DVec2::X * radius],
        runs: vec![Run::Round { center, turn: TAU }],
    }
}

/// A cylinder standing on the XY plane between two heights.
fn standing(center: [f64; 2], radius: f64, from: f64, to: f64) -> Body {
    raised(
        circle(DVec2::from(center), radius),
        frame(DVec3::Z * from, DVec3::X, DVec3::Y),
        DVec3::Z * (to - from),
    )
}

/// A cylinder lying along Y between two abscissae, its axis at `center` in
/// X and Z.
fn lying(center: [f64; 2], radius: f64, from: f64, to: f64) -> Body {
    raised(
        circle(DVec2::from(center), radius),
        frame(DVec3::Y * from, DVec3::X, DVec3::Z),
        DVec3::Y * (to - from),
    )
}

fn stock() -> Body {
    standing([0.0, 0.0], RADIUS, 0.0, HEIGHT)
}

fn block() -> Body {
    boxed([-20.0, -20.0, 0.0], [20.0, 20.0, HEIGHT])
}

fn hole(x: f64, from: f64, to: f64) -> Body {
    standing([x, 0.0], HOLE, from, to)
}

/// What a result is made of: faces, edges and vertices.
#[derive(Debug, PartialEq, Eq)]
struct Counts {
    faces: usize,
    edges: usize,
    vertices: usize,
}

fn counts(faces: usize, edges: usize, vertices: usize) -> Counts {
    Counts {
        faces,
        edges,
        vertices,
    }
}

/// A result held to its counts, the arithmetic, its listing and its
/// triangles.
fn holds(result: Result<Body, Declined>, expected: Counts, arithmetic: f64) -> Body {
    let body = answers(result, expected, arithmetic);
    assert_eq!(listed(&body.listing(), body.scale().reach()), Ok(()));
    body
}

/// A result held to its counts, the arithmetic and its triangles.
fn answers(result: Result<Body, Declined>, expected: Counts, arithmetic: f64) -> Body {
    let body = result.expect("the kernel answers");
    let listing = body.listing();
    assert_eq!(
        counts(
            listing.faces.len(),
            listing.edges.len(),
            listing.vertices.len()
        ),
        expected,
        "{listing:#?}"
    );
    let volume = body.volume();
    assert!(
        ((volume - arithmetic) / arithmetic).abs() <= 1e-9,
        "the exact volume is {volume}, the arithmetic {arithmetic}"
    );
    let triangles = body.triangles(MESH);
    assert_eq!(closed(&triangles), Ok(()));
    assert_eq!(uncrossed(&triangles), Ok(()));
    body
}

const SLAB: f64 = 40.0 * 40.0 * HEIGHT;
const WHOLE: f64 = PI * RADIUS * RADIUS * HEIGHT;
const BORED: f64 = PI * HOLE * HOLE * HEIGHT;
const BOSS: f64 = PI * HOLE * HOLE * 5.0;

#[test]
fn two_blocks_sharing_a_wall_flush_lose_the_wall_and_keep_each_its_own_faces() {
    let other = boxed([20.0, -20.0, 0.0], [60.0, 20.0, HEIGHT]);
    holds(block().joined(&other), counts(10, 20, 12), 2.0 * SLAB);
}

#[test]
fn a_pocket_flush_with_a_side_and_the_top_opens_both_and_leaves_a_floor_and_three_walls() {
    let pocket = boxed([10.0, -5.0, 5.0], [20.0, 5.0, HEIGHT]);
    holds(
        block().cut_by(&pocket),
        counts(10, 24, 16),
        SLAB - 10.0 * 10.0 * 5.0,
    );
}

#[test]
fn a_block_boss_overhanging_the_top_s_edge_stands_on_it_and_shows_its_underside_beyond() {
    let boss = boxed([15.0, -5.0, HEIGHT], [25.0, 5.0, 15.0]);
    holds(
        block().joined(&boss),
        counts(12, 28, 18),
        SLAB + 10.0 * 10.0 * 5.0,
    );
}

#[test]
fn two_blocks_sharing_part_of_a_wall_keep_what_each_does_not_share() {
    let other = boxed([20.0, -10.0, 0.0], [60.0, 30.0, HEIGHT]);
    holds(block().joined(&other), counts(12, 26, 16), 2.0 * SLAB);
}

#[test]
fn a_hole_bored_flush_off_the_axis_leaves_two_rings_and_two_walls_with_no_vertex() {
    holds(
        stock().cut_by(&hole(8.0, 0.0, HEIGHT)),
        counts(4, 4, 0),
        WHOLE - BORED,
    );
}

#[test]
fn a_hole_bored_through_off_the_axis_leaves_what_the_flush_one_does() {
    holds(
        stock().cut_by(&hole(8.0, -1.0, HEIGHT + 1.0)),
        counts(4, 4, 0),
        WHOLE - BORED,
    );
}

#[test]
fn a_hole_bored_flush_on_the_axis_leaves_two_rings_and_two_walls_with_no_vertex() {
    holds(
        stock().cut_by(&hole(0.0, 0.0, HEIGHT)),
        counts(4, 4, 0),
        WHOLE - BORED,
    );
}

#[test]
fn a_hole_bored_through_on_the_axis_leaves_what_the_flush_one_does() {
    holds(
        stock().cut_by(&hole(0.0, -1.0, HEIGHT + 1.0)),
        counts(4, 4, 0),
        WHOLE - BORED,
    );
}

#[test]
fn a_boss_standing_on_the_top_joins_it_along_a_whole_circle() {
    holds(
        stock().joined(&hole(8.0, HEIGHT, HEIGHT + 5.0)),
        counts(5, 4, 0),
        WHOLE + BOSS,
    );
}

#[test]
fn a_boss_sunk_into_the_top_comes_out_of_it_along_a_whole_circle() {
    holds(
        stock().joined(&hole(8.0, HEIGHT - 1.0, HEIGHT + 5.0)),
        counts(5, 4, 0),
        WHOLE + BOSS,
    );
}

#[test]
fn a_hole_bored_flush_but_for_a_hair_leaves_a_floor_a_hair_above_the_bottom() {
    holds(
        stock().cut_by(&hole(8.0, 1e-7, HEIGHT)),
        counts(5, 4, 0),
        WHOLE - PI * HOLE * HOLE * (HEIGHT - 1e-7),
    );
}

#[test]
fn a_round_boss_overhanging_the_top_s_edge_crosses_it_at_two_corners() {
    holds(
        stock().joined(&hole(18.0, HEIGHT, HEIGHT + 5.0)),
        counts(6, 6, 2),
        WHOLE + BOSS,
    );
}

#[test]
fn a_hole_tangent_to_a_side_of_a_block_parts_that_side_along_an_edge_of_four_uses() {
    holds(
        block().cut_by(&hole(15.0, 0.0, HEIGHT)),
        counts(8, 17, 10),
        SLAB - BORED,
    );
}

#[test]
fn two_holes_whose_circles_touch_share_the_line_they_touch_along() {
    let first = holds(
        stock().cut_by(&hole(-5.0, 0.0, HEIGHT)),
        counts(4, 4, 0),
        WHOLE - BORED,
    );
    holds(
        first.cut_by(&hole(5.0, 0.0, HEIGHT)),
        counts(5, 7, 2),
        WHOLE - 2.0 * BORED,
    );
}

#[test]
fn a_hole_bored_flush_tangent_to_the_wall_leaves_a_crescent_and_an_edge_of_four_uses() {
    holds(
        stock().cut_by(&hole(RADIUS - HOLE, 0.0, HEIGHT)),
        counts(4, 5, 2),
        WHOLE - BORED,
    );
}

#[test]
fn a_hole_bored_through_tangent_to_the_wall_leaves_what_the_flush_one_does() {
    holds(
        stock().cut_by(&hole(RADIUS - HOLE, -1.0, HEIGHT + 1.0)),
        counts(4, 5, 2),
        WHOLE - BORED,
    );
}

/// The line the cylinder rests along is an edge of the block's top, run
/// once each way by it as a slit, and of the cylinder's wall, run once each
/// way by it as a seam. The listing's check takes the slit, which sweeps no
/// area, for a hole turned the wrong way.
#[test]
fn a_cylinder_resting_against_a_flat_face_touches_it_along_an_edge_the_listing_takes_for_a_backward_hole()
 {
    let resting = lying([0.0, HEIGHT + HOLE], HOLE, -15.0, 15.0);
    let body = answers(
        block().joined(&resting),
        counts(9, 15, 10),
        SLAB + PI * HOLE * HOLE * 30.0,
    );
    let listing = body.listing();
    let contact: Vec<usize> = (0..listing.edges.len())
        .filter(|&edge| listing.edges[edge].sides.len() == 4)
        .collect();
    let [contact] = contact.as_slice() else {
        panic!("one edge of four uses: {contact:?}");
    };
    let Curve::Line(line) = listing.edges[*contact].curve else {
        panic!("the contact is a line: {:?}", listing.edges[*contact]);
    };
    assert!(
        line.direction.abs().abs_diff_eq(DVec3::Y, 1e-12),
        "{line:?}"
    );
    let slit = listing.faces.iter().any(|listed| {
        matches!(listed.surface, Surface::Plane(_))
            && listed.loops.iter().any(|lap| {
                lap == &vec![(*contact, true), (*contact, false)]
                    || lap == &vec![(*contact, false), (*contact, true)]
            })
    });
    assert!(
        slit,
        "the top runs the contact both ways as a loop of its own"
    );
    assert_eq!(listed(&listing, body.scale().reach()), Ok(()));
}
