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
//!
//! Then the cuts #552 brings, along the conic a plane at a slant meets a
//! cylinder or a cone along, each held to the arithmetic, its listing and
//! its triangles: a hole square to a slanted face out through the floor, a
//! shaft cut at a slant, a point cut across, through its tip and parallel to
//! its slope; a plane touching a shaft along a ruling, which is that line;
//! and what still declines, a skew bore and a radial hole in a cone.

use std::f64::consts::{PI, TAU};

use cao_solid::brep::{Body, Curve, Declined, Surface};
use cao_solid::profile::{Contour, Frame, Run};
use cao_solid::soundness::{closed, listed, uncrossed};
use cao_solid::turning::{Axis, Corner, Straight, Turn};
use glam::{DVec2, DVec3};

/// How far a triangle may stand from the surface it stands for.
const MESH: f64 = 0.01;

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

/// Their sides in one plane are one face each, with no seam where the wall
/// was (step 8).
#[test]
fn two_blocks_sharing_a_wall_flush_lose_the_wall_and_are_one_block() {
    let other = boxed([20.0, -20.0, 0.0], [60.0, 20.0, HEIGHT]);
    holds(block().joined(&other), counts(6, 12, 8), 2.0 * SLAB);
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

/// The top and the bottom are one face each, the outline of the two
/// together (step 8).
#[test]
fn two_blocks_sharing_part_of_a_wall_keep_what_each_does_not_share() {
    let other = boxed([20.0, -10.0, 0.0], [60.0, 30.0, HEIGHT]);
    holds(block().joined(&other), counts(10, 24, 16), 2.0 * SLAB);
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

/// A result held to the arithmetic, its listing and its triangles, whatever
/// it is made of.
fn exactly(result: Result<Body, Declined>, arithmetic: f64) -> Body {
    let body = result.expect("the kernel answers");
    assert_eq!(listed(&body.listing(), body.scale().reach()), Ok(()));
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

/// A rectangle from `low` to `high` on the plane through `origin` spanned by
/// `u` and `v`, raised `depth` along `u × v`.
fn slab(origin: DVec3, u: DVec3, v: DVec3, [low, high]: [[f64; 2]; 2], depth: f64) -> Body {
    raised(
        Contour::rectangle(DVec2::from(low), DVec2::from(high)),
        frame(origin, u, v),
        u.cross(v) * depth,
    )
}

/// The direction in the plane `YZ` leaning `degrees` from `Y` towards `Z`.
fn leaning(degrees: f64) -> DVec3 {
    let (sin, cos) = degrees.to_radians().sin_cos();
    DVec3::new(0.0, cos, sin)
}

/// A wedge twenty along X: a block of twenty by twenty under the slant
/// `z = 5 + y·tan 30°`.
fn wedge() -> Body {
    let top = 5.0 + 20.0 * 30f64.to_radians().tan();
    raised(
        Contour::straight(vec![
            DVec2::ZERO,
            DVec2::new(20.0, 0.0),
            DVec2::new(20.0, top),
            DVec2::new(0.0, 5.0),
        ]),
        frame(DVec3::ZERO, DVec3::Y, DVec3::Z),
        DVec3::X * 20.0,
    )
}

/// Its volume: twenty along X times the trapezoid under the slant.
fn wedge_volume() -> f64 {
    2000.0 + 4000.0 / 3f64.sqrt()
}

/// A point turned about Z: its tip at ten, its base of radius five on XY.
fn point() -> Body {
    let corner = |at: [f64; 2], run: u32| Corner {
        at: DVec2::from(at),
        run,
    };
    let straight = Straight {
        side: 1.0,
        contours: vec![vec![
            corner([0.0, 0.0], 0),
            corner([10.0, 0.0], 1),
            corner([0.0, 5.0], 2),
        ]],
        runs: 3,
        last_off_the_axis: Some(2),
    };
    let turn = Turn {
        axis: Axis {
            origin: DVec2::ZERO,
            direction: DVec2::X,
        },
        angle: TAU,
        resolution: 0.0,
        on_the_axis: 0.0,
    };
    Body::turned(&straight, frame(DVec3::ZERO, DVec3::Z, DVec3::X), &turn).expect("the point turns")
}

const POINT: f64 = 250.0 * PI / 3.0;

/// The area of the disc of radius `r` beyond a chord `d` from its centre.
fn segment(r: f64, d: f64) -> f64 {
    r * r * (d / r).acos() - d * (r * r - d * d).sqrt()
}

/// A hole of radius two square to the wedge's slant, from the middle of its
/// face down through the floor, which it leaves along an ellipse: what it
/// takes is a disc times the length of its axis inside.
fn holed_wedge() -> Result<Body, Declined> {
    let face = DVec3::new(10.0, 10.0, 5.0 + 10.0 * 30f64.to_radians().tan());
    let normal = DVec3::new(0.0, -0.5, 3f64.sqrt() / 2.0);
    let hole = raised(
        circle(DVec2::ZERO, 2.0),
        frame(face + normal, DVec3::X, (-normal).cross(DVec3::X)),
        -normal * 17.0,
    );
    wedge().cut_by(&hole)
}

#[test]
fn a_hole_square_to_a_slanted_face_out_through_the_floor_holds_its_volume() {
    let inside = 10.0 / 3f64.sqrt() + 20.0 / 3.0;
    exactly(holed_wedge(), wedge_volume() - 4.0 * PI * inside);
}

/// A shaft of radius five thirty high, all above a plane leaning thirty
/// degrees through its axis at fifteen cut away: it keeps its mean height.
fn shaft_cut_at_a_slant() -> Result<Body, Declined> {
    let above = slab(
        DVec3::Z * 15.0,
        DVec3::X,
        leaning(30.0),
        [[-10.0, -15.0], [10.0, 15.0]],
        20.0,
    );
    standing([0.0, 0.0], 5.0, 0.0, 30.0).cut_by(&above)
}

#[test]
fn a_shaft_cut_by_a_slanted_plane_holds_its_volume() {
    exactly(shaft_cut_at_a_slant(), 375.0 * PI);
}

/// The point, its tip cut off above a plane leaning fifteen degrees through
/// its axis at six: the tip's volume is a third of its height cubed times the
/// slope's square, over the slant's.
fn point_cut_obliquely() -> Result<Body, Declined> {
    let above = slab(
        DVec3::Z * 6.0,
        DVec3::X,
        leaning(15.0),
        [[-10.0, -10.0], [10.0, 10.0]],
        10.0,
    );
    point().cut_by(&above)
}

#[test]
fn a_cone_cut_obliquely_holds_its_volume() {
    let slant = 15f64.to_radians().tan();
    let tip = 16.0 * PI / 3.0 * (1.0 - slant * slant / 4.0).powf(-1.5);
    exactly(point_cut_obliquely(), POINT - tip);
}

/// The point cut by a plane through its tip, fifteen degrees off its axis,
/// steeper than its slope: it cuts two rulings, and takes the piece of the
/// point standing on the base's segment beyond the chord it leaves there.
fn point_cut_through_its_tip() -> Result<Body, Declined> {
    let chord = 10.0 * 15f64.to_radians().tan();
    let down = DVec3::new(0.0, chord, -10.0).normalize();
    let beyond = slab(
        DVec3::Z * 10.0,
        DVec3::X,
        down,
        [[-10.0, -5.0], [10.0, 15.0]],
        10.0,
    );
    point().cut_by(&beyond)
}

#[test]
fn a_plane_through_a_point_s_tip_at_a_slant_is_built_along_two_rulings() {
    let chord = 10.0 * 15f64.to_radians().tan();
    let body = exactly(
        point_cut_through_its_tip(),
        POINT - 10.0 / 3.0 * segment(5.0, chord),
    );
    let apex = DVec3::Z * 10.0;
    let rulings = body
        .listing()
        .edges
        .iter()
        .filter(|edge| match edge.curve {
            Curve::Line(line) => {
                let off = apex - line.origin;
                (off - line.direction * off.dot(line.direction)).length() <= 1e-9
            }
            _ => false,
        })
        .count();
    assert_eq!(rulings, 2, "{:#?}", body.listing());
}

/// The point cut by the plane `2y + z = 2`, parallel to its ruling on `+Y`:
/// the tip's side goes, a parabola between, the base's segment beyond the
/// chord at `y = 1` and the parabola's face adding up to it.
fn point_cut_parallel_to_its_slope() -> Result<Body, Declined> {
    let normal = DVec3::new(0.0, 2.0, 1.0).normalize();
    let beyond = slab(
        DVec3::Z * 2.0,
        DVec3::X,
        normal.cross(DVec3::X),
        [[-10.0, -12.0], [10.0, 8.0]],
        10.0,
    );
    point().cut_by(&beyond)
}

#[test]
fn a_plane_parallel_to_a_ruling_cuts_a_point_along_a_parabola() {
    let tip = (250.0 * 0.2f64.acos() + 44.0 * 6f64.sqrt()) / 3.0;
    let body = exactly(point_cut_parallel_to_its_slope(), POINT - tip);
    let parabolas = body
        .listing()
        .edges
        .iter()
        .filter(|edge| matches!(edge.curve, Curve::Conic(conic) if conic.k.abs() <= 1e-9))
        .count();
    assert_eq!(parabolas, 1, "{:#?}", body.listing());
}

/// A block standing against the shaft, its face leaning a tenth of a
/// billionth of a radian off its axis, within the angle the kernel reads as
/// square: the face touches the wall along a ruling.
fn shaft_and_block_touching_along_a_ruling() -> Result<Body, Declined> {
    let tilt: f64 = 1e-10;
    let normal = DVec3::new(tilt.cos(), 0.0, tilt.sin());
    let up = DVec3::new(-tilt.sin(), 0.0, tilt.cos());
    let block = raised(
        Contour::rectangle(DVec2::splat(-10.0), DVec2::splat(10.0)),
        frame(DVec3::new(5.0, 0.0, 15.0), DVec3::Y, up),
        normal * 10.0,
    );
    standing([0.0, 0.0], 5.0, 0.0, 30.0).joined(&block)
}

/// The line of touch is an edge of four uses, which the listing's check
/// takes for a backward hole on the block's face, as on a cylinder resting
/// on a flat one: what is held is the line itself, and no conic.
#[test]
fn a_plane_touching_a_shaft_along_a_ruling_lists_that_line_and_no_conic() {
    let body = shaft_and_block_touching_along_a_ruling().expect("the kernel answers");
    let volume = body.volume();
    let arithmetic = 750.0 * PI + 4000.0;
    assert!(
        ((volume - arithmetic) / arithmetic).abs() <= 1e-9,
        "the exact volume is {volume}, the arithmetic {arithmetic}"
    );
    let triangles = body.triangles(MESH);
    assert_eq!(closed(&triangles), Ok(()));
    assert_eq!(uncrossed(&triangles), Ok(()));
    let listing = body.listing();
    assert!(
        !listing
            .edges
            .iter()
            .any(|edge| matches!(edge.curve, Curve::Conic(_))),
        "{listing:#?}"
    );
    let along = listing.edges.iter().any(|edge| match edge.curve {
        Curve::Line(line) => {
            line.direction.abs().abs_diff_eq(DVec3::Z, 1e-9)
                && line.origin.truncate().distance(glam::DVec2::new(5.0, 0.0)) <= 1e-8
        }
        _ => false,
    });
    assert!(along, "the ruling of touch: {listing:#?}");
}

#[test]
fn every_conic_body_above_is_drawn_closed_and_uncrossed() {
    for (what, made) in [
        ("the holed wedge", holed_wedge()),
        ("the shaft cut at a slant", shaft_cut_at_a_slant()),
        ("the point cut obliquely", point_cut_obliquely()),
        ("the point cut through its tip", point_cut_through_its_tip()),
        (
            "the point cut parallel to its slope",
            point_cut_parallel_to_its_slope(),
        ),
        (
            "the shaft and the block touching it",
            shaft_and_block_touching_along_a_ruling(),
        ),
    ] {
        let body = made.unwrap_or_else(|declined| panic!("{what}: {declined:?}"));
        let reach = body.scale().reach();
        for tolerance in [1e-2 * reach, 1e-3 * reach, 1e-4 * reach] {
            let triangles = body.triangles(tolerance);
            assert_eq!(closed(&triangles), Ok(()), "{what} at {tolerance}");
            assert_eq!(uncrossed(&triangles), Ok(()), "{what} at {tolerance}");
        }
    }
}

#[test]
fn a_skew_bore_and_a_radial_hole_in_a_cone_still_decline() {
    let along = DVec3::new(0.5, 0.0, 3f64.sqrt() / 2.0);
    let skew = raised(
        circle(DVec2::ZERO, 3.0),
        frame(DVec3::new(15.0, 0.0, -5.0), DVec3::Y, DVec3::Y.cross(along)),
        along * 25.0,
    );
    assert_eq!(stock().cut_by(&skew), Err(Declined::Unsupported));
    let radial = lying([0.0, 5.0], 1.0, -10.0, 10.0);
    assert_eq!(point().cut_by(&radial), Err(Declined::Unsupported));
}
