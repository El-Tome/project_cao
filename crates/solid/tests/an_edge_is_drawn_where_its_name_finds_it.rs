//! Every edge of an exact body handed out as drawn: its name beside the line
//! it runs along, in the place its triangles stand in. What the view needs to
//! light the edge under the cursor and hand its name to a chamfer or a fillet;
//! which edge is nearest a cursor is the view's to decide, in pixels.
//!
//! Closes #920 — the whole transcription is in
//! `crates/part/tests/an_edge_is_chamfered_or_rounded.rs`; this bullet is the
//! kernel's half of picking an edge.
//! - With the panel open, hovering an edge of the part in the 3D view lights
//!   it, and clicking it chooses it — every edge drawn once under its name, a
//!   straight one from corner to corner, by
//!   `every_edge_of_a_block_is_drawn_once_under_its_name_from_corner_to_corner`;
//!   the name a chamfer is handed drawn along that edge, by
//!   `the_edge_named_between_the_top_and_the_front_is_drawn_along_the_top_front_line`;
//!   a round edge drawn closed on its circle, by
//!   `the_rims_of_a_disc_are_drawn_closed_on_their_circle_within_the_tolerance_of_the_triangles`;
//!   and the edges a step lays drawn as well, by
//!   `a_chamfered_block_draws_the_edges_its_chamfer_laid_under_their_names_too`;
//!   found by the mutants, a round edge drawn no finer than that tolerance
//!   asks, by
//!   `the_rims_of_a_disc_are_drawn_no_finer_than_the_tolerance_of_the_triangles_asks`

use std::collections::BTreeSet;
use std::f64::consts::TAU;

use cao_solid::profile::{Contour, Frame, Profile};
use cao_solid::{Body, DrawnEdge, EdgeName, Loop};
use glam::{DVec2, DVec3};

/// How far a point of an edge may stand from where the arithmetic puts it:
/// every point drawn lies on the edge's own curve, so what is left is rounding.
const PLACE: f64 = 1e-9;

/// How far a chord of a curved edge may sag inside it, as a share of the
/// body's reach: the tolerance its triangles are drawn at, so that the line
/// lies on the surface the view shades.
const DRAWN: f64 = 2e-4;

const LENGTH: f64 = 30.0;
const BREADTH: f64 = 10.0;
const HEIGHT: f64 = 20.0;

/// The numbers a raised block's top and front answer to.
const TOP: u32 = 1;
const FRONT: u32 = 2;

fn raised(outline: Contour, height: f64) -> Body {
    let ground = Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    };
    let profile = Profile {
        exact: Some((outline, Vec::new())),
        sampled: Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    let part = Body::default();
    let tool = part
        .tool_raised(&profile, ground, ground.normal() * height)
        .expect("the tool raises");
    part.union(&tool).expect("the part is built")
}

fn block() -> Body {
    raised(
        Contour::rectangle(DVec2::ZERO, DVec2::new(LENGTH, BREADTH)),
        HEIGHT,
    )
}

fn reach(body: &Body) -> f64 {
    let (low, high) = body.bounds().expect("some matter");
    low.abs().max(high.abs()).max_element().max(1.0)
}

fn drawn_once_each(body: &Body, what: &str) -> Vec<DrawnEdge> {
    let drawn = body.drawn_edges();
    let names: Vec<&EdgeName> = drawn.iter().map(|edge| &edge.name).collect();
    let distinct: BTreeSet<&EdgeName> = names.iter().copied().collect();
    assert_eq!(distinct.len(), names.len(), "{what}: a name drawn twice");
    let named = body.edge_names();
    assert_eq!(
        distinct,
        named.iter().collect::<BTreeSet<_>>(),
        "{what}: the names drawn are the names the body gives",
    );
    for edge in &drawn {
        assert!(
            edge.points.len() >= 2,
            "{what}: {:?} drawn as {} points",
            edge.name,
            edge.points.len(),
        );
    }
    drawn
}

fn straight_between(edge: &DrawnEdge, what: &str) -> [DVec3; 2] {
    let ends = [edge.points[0], *edge.points.last().expect("two points")];
    let along = (ends[1] - ends[0]).normalize();
    for point in &edge.points {
        let off = (*point - ends[0]) - along * (*point - ends[0]).dot(along);
        let at = (*point - ends[0]).dot(along);
        assert!(
            off.length() < PLACE && at > -PLACE && at < ends[0].distance(ends[1]) + PLACE,
            "{what}: {:?} strays off its line to {point}",
            edge.name,
        );
    }
    ends
}

fn is_a_corner_of_the_block(point: DVec3) -> bool {
    [(point.x, LENGTH), (point.y, BREADTH), (point.z, HEIGHT)]
        .iter()
        .all(|(at, far)| at.abs() < PLACE || (at - far).abs() < PLACE)
}

fn the_same_ends(ends: [DVec3; 2], expected: [DVec3; 2]) -> bool {
    let near = |a: DVec3, b: DVec3| a.distance(b) < PLACE;
    (near(ends[0], expected[0]) && near(ends[1], expected[1]))
        || (near(ends[0], expected[1]) && near(ends[1], expected[0]))
}

#[test]
fn every_edge_of_a_block_is_drawn_once_under_its_name_from_corner_to_corner() {
    let block = block();
    let drawn = drawn_once_each(&block, "the block");

    assert_eq!(drawn.len(), 12, "a block has twelve edges");
    let mut sides = BTreeSet::new();
    for edge in &drawn {
        let ends = straight_between(edge, "the block");
        assert!(
            ends.iter().all(|end| is_a_corner_of_the_block(*end)),
            "{:?} drawn from {} to {}, not between two corners",
            edge.name,
            ends[0],
            ends[1],
        );
        let apart = (ends[1] - ends[0]).abs();
        let axes = [apart.x, apart.y, apart.z]
            .iter()
            .filter(|step| **step > PLACE)
            .count();
        assert_eq!(
            axes, 1,
            "{:?} drawn across the block, from {} to {}",
            edge.name, ends[0], ends[1],
        );
        let key = |point: DVec3| [point.x, point.y, point.z].map(|at| at.round() as i64);
        let (a, b) = (key(ends[0]), key(ends[1]));
        sides.insert((a.min(b), a.max(b)));
    }
    assert_eq!(sides.len(), 12, "each edge of the block drawn by one name");
}

#[test]
fn the_edge_named_between_the_top_and_the_front_is_drawn_along_the_top_front_line() {
    let block = block();
    let sides = [vec![TOP], vec![FRONT]];
    let named: Vec<DrawnEdge> = block
        .drawn_edges()
        .into_iter()
        .filter(|edge| edge.name.between == sides)
        .collect();

    assert_eq!(named.len(), 1, "one edge between the top and the front");
    let ends = straight_between(&named[0], "the top front edge");
    assert!(
        the_same_ends(
            ends,
            [
                DVec3::new(0.0, 0.0, HEIGHT),
                DVec3::new(LENGTH, 0.0, HEIGHT)
            ],
        ),
        "the edge between the top and the front drawn from {} to {}",
        ends[0],
        ends[1],
    );
}

#[test]
fn the_rims_of_a_disc_are_drawn_closed_on_their_circle_within_the_tolerance_of_the_triangles() {
    let radius = 5.0;
    let height = 10.0;
    let disc = raised(Contour::circle(DVec2::ZERO, radius), height);
    let sag = DRAWN * reach(&disc);
    let drawn = drawn_once_each(&disc, "the disc");

    assert_eq!(drawn.len(), 2, "a disc has two rims");
    let mut levels = BTreeSet::new();
    for edge in &drawn {
        let what = format!("the rim {:?}", edge.name);
        let level = edge.points[0].z;
        levels.insert((level / height).round() as i64);
        for point in &edge.points {
            assert!(
                (point.z - level).abs() < PLACE
                    && (point.truncate().length() - radius).abs() < PLACE * radius,
                "{what}: {point} off its circle",
            );
        }
        let (first, last) = (edge.points[0], *edge.points.last().expect("points"));
        assert!(
            first.distance(last) < PLACE,
            "{what}: a whole circle drawn open, from {first} to {last}",
        );
        let mut turned: f64 = 0.0;
        for pair in edge.points.windows(2) {
            let middle = (pair[0] + pair[1]).truncate() / 2.0;
            assert!(
                radius - middle.length() <= sag,
                "{what}: a chord from {} to {} sags {} inside the circle, where the triangles \
                 are drawn within {sag}",
                pair[0],
                pair[1],
                radius - middle.length(),
            );
            turned += pair[0].truncate().angle_to(pair[1].truncate());
        }
        assert!(
            (turned.abs() - TAU).abs() < 1e-6,
            "{what}: drawn round {turned} radians, where a rim goes round once",
        );
    }
    assert_eq!(
        levels,
        BTreeSet::from([0, 1]),
        "one rim at the floor, one at the top"
    );
}

#[test]
fn a_chamfered_block_draws_the_edges_its_chamfer_laid_under_their_names_too() {
    let block = block();
    let edge = block
        .edge_names()
        .into_iter()
        .find(|name| name.between == [vec![TOP], vec![FRONT]])
        .expect("the top front edge");
    let size = 2.0;
    let chamfered = block
        .chamfer(&edge, size)
        .unwrap_or_else(|declined| panic!("declined, {declined:?}"))
        .body;

    let drawn = drawn_once_each(&chamfered, "the chamfered block");

    assert_eq!(drawn.len(), 15, "the chamfer swaps one edge for four");
    for line in [
        [
            DVec3::new(0.0, 0.0, HEIGHT - size),
            DVec3::new(LENGTH, 0.0, HEIGHT - size),
        ],
        [
            DVec3::new(0.0, size, HEIGHT),
            DVec3::new(LENGTH, size, HEIGHT),
        ],
    ] {
        assert!(
            drawn
                .iter()
                .any(|edge| the_same_ends(straight_between(edge, "the chamfered block"), line)),
            "no edge drawn from {} to {}, where the chamfer meets the front or the top",
            line[0],
            line[1],
        );
    }
}

#[test]
fn the_rims_of_a_disc_are_drawn_no_finer_than_the_tolerance_of_the_triangles_asks() {
    let radius = 5.0;
    let disc = raised(Contour::circle(DVec2::ZERO, radius), 10.0);
    let sag = DRAWN * reach(&disc);
    let fewest = (std::f64::consts::PI / (1.0 - sag / radius).acos()).ceil() as usize;

    for edge in disc.drawn_edges() {
        let chords = edge.points.len() - 1;
        assert!(
            chords <= 2 * fewest,
            "the rim {:?} is drawn in {chords} chords, where a sag of {sag} asks for {fewest}",
            edge.name,
        );
    }
}
