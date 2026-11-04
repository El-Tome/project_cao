//! Every edge and every vertex of an exact body is named from the numbers of
//! the faces it lies between (#582), so that a step pointing at one finds it
//! again once a size changes. An edge answers to the sides of its two faces
//! and to its rank among the edges between those same two, read off the way
//! the loops run and what meets at its ends, which a move, a turn or a change
//! of size leaves alone; a vertex answers to the sides of every face at it.
//!
//! The slot is milled across a block's top from its front to its back, which
//! leaves the top in two pieces: its width stands for the diameter of a hole.
//! The chamfered shaft has no hole, and neither it nor the bored cylinder has
//! a vertex: every edge of theirs is a whole circle.
//!
//! Closes #582.
//! - The edges and vertices of a block, a bored cylinder, a chamfered shaft
//!   and a slot keep their names through a change of size, a move and a
//!   change of hole diameter —
//!   `a_block_s_edges_and_vertices_keep_their_names_through_a_change_of_size_and_a_move`,
//!   `a_block_s_named_edge_and_corner_follow_the_block_through_a_change_of_size_and_a_move`,
//!   `a_bored_cylinder_s_edges_keep_their_names_through_a_change_of_size_a_move_and_a_change_of_hole_diameter`,
//!   `a_bored_cylinder_s_named_hole_rim_follows_the_hole_through_a_change_of_size_a_move_and_a_change_of_hole_diameter`,
//!   `a_chamfered_shaft_s_edges_keep_their_names_through_a_change_of_size_and_a_move`,
//!   `a_chamfered_shaft_s_named_edge_follows_its_chamfer_through_a_change_of_size_and_a_move`,
//!   `a_slot_s_edges_and_vertices_keep_their_names_through_a_change_of_size_a_move_and_a_change_of_width`,
//!   `a_slot_s_two_edges_along_the_block_s_front_keep_their_ranks_through_a_change_of_size_a_move_and_a_change_of_width`;
//!   the order that tells apart two edges between the same two faces, which
//!   the issue left to the branch, by
//!   `two_rulings_between_a_block_s_back_and_a_post_keep_their_ranks_through_a_change_of_size_a_move_and_a_turn`,
//!   and the vertices that share their faces, ranked by the names of the
//!   edges ending there, by
//!   `four_vertices_where_a_post_meets_a_block_s_back_keep_their_ranks_through_a_change_of_size_a_move_and_a_turn`
//! - A change that splits or removes an edge is reported for that edge —
//!   `an_edge_a_later_cut_parts_in_two_is_reported_split_into_its_pieces`,
//!   `an_edge_a_later_cut_parts_along_with_its_face_is_reported_split_rather_than_kept_on_its_first_piece`,
//!   `an_edge_a_later_cut_takes_away_is_reported_gone`,
//!   `an_edge_a_later_step_leaves_alone_is_not_reported`,
//!   `every_name_is_reported_gone_on_a_part_the_flats_computed`
//! - The names are written in the body, read back from the geometry cache,
//!   and replayed the same — no test: held by
//!   `an_exact_body_s_edge_and_vertex_names_come_back_from_the_cache_as_the_replay_leaves_them`,
//!   in `crates/part/src/document/geometry_cache/tests.rs`, beside the cache,
//!   which is `cao_part`'s and out of this crate's sight

use std::collections::BTreeSet;
use std::f64::consts::TAU;

use cao_solid::brep::{self, EdgeId, VertexId};
use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::turning::{Axis, Corner, Straight, Turn};
use cao_solid::{Body, EdgeName, Loop, Lost, VertexName};
use glam::{DVec2, DVec3};

/// How far a named edge or vertex may stand from where the drawing put it.
const PLACE: f64 = 1e-9;

/// Where a part is moved to, whole.
const ELSEWHERE: [f64; 3] = [100.0, -40.0, 7.0];

/// The ground's plane, which a part is drawn on.
fn ground() -> Frame {
    Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    }
}

/// The ground's plane with its origin moved to `to`.
fn moved(to: [f64; 3]) -> Frame {
    Frame {
        origin: DVec3::from(to),
        ..ground()
    }
}

/// The ground's plane turned about the vertical by a quarter turn, `quarters`
/// times, its axes written out so that no rounding leans them.
fn quarter_turns(quarters: u32) -> Frame {
    let (u, v) = match quarters % 4 {
        0 => (DVec3::X, DVec3::Y),
        1 => (DVec3::Y, -DVec3::X),
        2 => (-DVec3::X, -DVec3::Y),
        _ => (-DVec3::Y, DVec3::X),
    };
    Frame {
        origin: DVec3::ZERO,
        u,
        v,
    }
}

/// The frame raised along its normal by `height`.
fn lifted(frame: Frame, height: f64) -> Frame {
    Frame {
        origin: frame.origin + frame.normal() * height,
        ..frame
    }
}

/// A point given in the part's own axes, along, across and up, where the
/// frame puts it.
fn placed(frame: Frame, [along, across, up]: [f64; 3]) -> DVec3 {
    frame.at(DVec2::new(along, across)) + frame.normal() * up
}

/// A whole circle, as one run all the way round.
fn circle(center: [f64; 2], radius: f64) -> Contour {
    let center = DVec2::from(center);
    Contour {
        corners: vec![center + DVec2::X * radius],
        runs: vec![Run::Round { center, turn: TAU }],
    }
}

#[derive(Clone, Copy)]
enum Mode {
    Join,
    Cut,
}

/// A step of a part: an outline raised square to the part's plane, from
/// `from` up for `height`, then joined to the matter so far or cut from it.
struct Raise {
    mode: Mode,
    outline: Contour,
    from: f64,
    height: f64,
}

fn join(outline: Contour, from: f64, height: f64) -> Raise {
    Raise {
        mode: Mode::Join,
        outline,
        from,
        height,
    }
}

fn cut(outline: Contour, from: f64, height: f64) -> Raise {
    Raise {
        mode: Mode::Cut,
        outline,
        from,
        height,
    }
}

/// The part as `cao_part` builds it: each step's tool raised against the
/// matter so far, then joined to it or cut from it.
fn part(steps: &[Raise], frame: Frame) -> Body {
    steps.iter().fold(Body::default(), |part, step| {
        let profile = Profile {
            exact: Some((step.outline.clone(), Vec::new())),
            sampled: Loop::straight(&[]),
            sampled_holes: Vec::new(),
            triangles: &[],
        };
        let tool = part
            .tool_raised(
                &profile,
                lifted(frame, step.from),
                frame.normal() * step.height,
            )
            .expect("the tool raises");
        match step.mode {
            Mode::Join => part.union(&tool),
            Mode::Cut => part.difference(&tool),
        }
        .expect("the step is built")
    })
}

/// The same part built by the exact kernel alone, each tool's numbers moved
/// above every number the steps before it named, as the body moves them:
/// what a named edge is found on.
fn exactly(steps: &[Raise], frame: Frame) -> brep::Body {
    let mut next = 0;
    let mut built: Option<brep::Body> = None;
    for step in steps {
        let tool = brep::Body::raised(
            &step.outline,
            &[],
            lifted(frame, step.from),
            frame.normal() * step.height,
        )
        .expect("the tool raises")
        .renumbered(next);
        next += brep::Body::numbers_raised(&step.outline, &[]);
        built = Some(match (built, step.mode) {
            (None, _) => tool,
            (Some(body), Mode::Join) => body.joined(&tool).expect("the tool joins"),
            (Some(body), Mode::Cut) => body.cut_by(&tool).expect("the tool cuts"),
        });
    }
    built.expect("a part of one step at least")
}

/// The name of the `rank`th of the `of` edges between two sides.
fn edge(first: &[u32], second: &[u32], rank: u32, of: u32) -> EdgeName {
    EdgeName {
        between: [first.to_vec(), second.to_vec()],
        rank,
        of,
    }
}

/// The name of the `rank`th of the `of` vertices at the same faces.
fn vertex(faces: &[&[u32]], rank: u32, of: u32) -> VertexName {
    VertexName {
        faces: faces.iter().map(|side| side.to_vec()).collect(),
        rank,
        of,
    }
}

/// Holds that names tell every edge apart: none given twice, and the edges
/// between the same two sides ranked from nought to as many as there are.
fn told_apart(names: &[EdgeName]) {
    let distinct: BTreeSet<&EdgeName> = names.iter().collect();
    assert_eq!(distinct.len(), names.len(), "a name given twice: {names:?}");
    for name in names {
        let alike = names
            .iter()
            .filter(|other| other.between == name.between)
            .count();
        assert_eq!(name.of as usize, alike, "{name:?} among {names:?}");
        assert!(name.rank < name.of, "{name:?} ranked past its count");
    }
}

/// Holds that every version of a part names its edges and its vertices as
/// the first does, the first naming `edges` edges and `vertices` vertices,
/// every edge told apart.
fn names_kept(versions: &[(&str, Body)], edges: usize, vertices: usize) {
    let (_, drawn) = &versions[0];
    let (edge_names, vertex_names) = (drawn.edge_names(), drawn.vertex_names());
    assert_eq!(edge_names.len(), edges, "{edge_names:?}");
    assert_eq!(vertex_names.len(), vertices, "{vertex_names:?}");
    told_apart(&edge_names);
    for (label, version) in &versions[1..] {
        assert_eq!(version.edge_names(), edge_names, "the edges, {label}");
        assert_eq!(
            version.vertex_names(),
            vertex_names,
            "the vertices, {label}"
        );
    }
}

/// The one edge of the body carrying the name.
fn named_edge(body: &brep::Body, name: &EdgeName) -> EdgeId {
    let names = body.edge_names();
    assert_eq!(names.len(), body.edge_ids().count(), "a name for each edge");
    let carrying: Vec<EdgeId> = body
        .edge_ids()
        .filter(|id| names[id.0 as usize] == *name)
        .collect();
    assert_eq!(
        carrying.len(),
        1,
        "{name:?} carried by {carrying:?} in {names:?}"
    );
    carrying[0]
}

/// The point of the one vertex of the body carrying the name.
fn named_vertex(body: &brep::Body, name: &VertexName) -> DVec3 {
    let names = body.vertex_names();
    assert_eq!(
        names.len(),
        body.vertex_ids().count(),
        "a name for each vertex"
    );
    let carrying: Vec<VertexId> = body
        .vertex_ids()
        .filter(|id| names[id.0 as usize] == *name)
        .collect();
    assert_eq!(
        carrying.len(),
        1,
        "{name:?} carried by {carrying:?} in {names:?}"
    );
    body.vertex(carrying[0]).point
}

/// The point halfway along an edge.
fn middle(body: &brep::Body, edge: EdgeId) -> DVec3 {
    let stretch = body.edge(edge);
    body.point_on(edge, (stretch.from + stretch.to) / 2.0)
}

fn near(found: DVec3, expected: DVec3, what: &str) {
    assert!(
        found.distance(expected) < PLACE,
        "{what}: expected at {expected}, found at {found}"
    );
}

/// Holds that a point lies on the circle of `radius` about the axis through
/// `center` along `axis`, in the plane square to it.
fn on_circle(point: DVec3, center: DVec3, axis: DVec3, radius: f64, what: &str) {
    let away = point - center;
    let along = away.dot(axis);
    let out = (away - axis * along).length();
    assert!(
        along.abs() < PLACE && (out - radius).abs() < PLACE,
        "{what}: {point} stands {along} along the axis and {out} from it, \
         where the circle is {radius} round {center}"
    );
}

/// A block of a length, a breadth and a height, standing on its plane's
/// origin: its floor numbered nought, its top one, and its sides two to five
/// from the front round to the left.
fn block([length, breadth, height]: [f64; 3]) -> Vec<Raise> {
    vec![join(
        Contour::rectangle(DVec2::ZERO, DVec2::new(length, breadth)),
        0.0,
        height,
    )]
}

const BLOCK: [f64; 3] = [30.0, 10.0, 10.0];
const BLOCK_RESIZED: [f64; 3] = [42.0, 14.0, 6.0];

#[test]
fn a_block_s_edges_and_vertices_keep_their_names_through_a_change_of_size_and_a_move() {
    let edges = [
        edge(&[0], &[2], 0, 1),
        edge(&[0], &[3], 0, 1),
        edge(&[0], &[4], 0, 1),
        edge(&[0], &[5], 0, 1),
        edge(&[1], &[2], 0, 1),
        edge(&[1], &[3], 0, 1),
        edge(&[1], &[4], 0, 1),
        edge(&[1], &[5], 0, 1),
        edge(&[2], &[3], 0, 1),
        edge(&[2], &[5], 0, 1),
        edge(&[3], &[4], 0, 1),
        edge(&[4], &[5], 0, 1),
    ];
    let vertices = [
        vertex(&[&[0], &[2], &[3]], 0, 1),
        vertex(&[&[0], &[2], &[5]], 0, 1),
        vertex(&[&[0], &[3], &[4]], 0, 1),
        vertex(&[&[0], &[4], &[5]], 0, 1),
        vertex(&[&[1], &[2], &[3]], 0, 1),
        vertex(&[&[1], &[2], &[5]], 0, 1),
        vertex(&[&[1], &[3], &[4]], 0, 1),
        vertex(&[&[1], &[4], &[5]], 0, 1),
    ];
    for (label, body) in [
        ("as drawn", part(&block(BLOCK), ground())),
        ("resized", part(&block(BLOCK_RESIZED), ground())),
        ("moved", part(&block(BLOCK), moved(ELSEWHERE))),
    ] {
        assert_eq!(body.edge_names(), edges, "{label}");
        assert_eq!(body.vertex_names(), vertices, "{label}");
    }
}

#[test]
fn a_block_s_named_edge_and_corner_follow_the_block_through_a_change_of_size_and_a_move() {
    for (label, size, frame) in [
        ("as drawn", BLOCK, ground()),
        ("resized", BLOCK_RESIZED, ground()),
        ("moved", BLOCK, moved(ELSEWHERE)),
    ] {
        let [length, breadth, height] = size;
        let body = exactly(&block(size), frame);
        let top_right = named_edge(&body, &edge(&[1], &[3], 0, 1));
        near(
            middle(&body, top_right),
            placed(frame, [length, breadth / 2.0, height]),
            &format!("the edge between the top and the right side, {label}"),
        );
        near(
            named_vertex(&body, &vertex(&[&[1], &[3], &[4]], 0, 1)),
            placed(frame, [length, breadth, height]),
            &format!("the corner of the top, the right side and the back, {label}"),
        );
    }
}

/// A cylinder standing on its plane's origin, bored through by a hole whose
/// axis stands `off` along the plane's first axis: the cylinder's floor,
/// top and wall numbered nought to two, the hole's wall five.
struct Bored {
    radius: f64,
    height: f64,
    hole: f64,
    off: f64,
}

fn bored(sizes: &Bored) -> Vec<Raise> {
    vec![
        join(circle([0.0, 0.0], sizes.radius), 0.0, sizes.height),
        cut(
            circle([sizes.off, 0.0], sizes.hole),
            -1.0,
            sizes.height + 2.0,
        ),
    ]
}

const BORED: Bored = Bored {
    radius: 20.0,
    height: 10.0,
    hole: 5.0,
    off: 8.0,
};

const BORED_RESIZED: Bored = Bored {
    radius: 26.0,
    height: 14.0,
    ..BORED
};

const BORED_NARROWER: Bored = Bored { hole: 3.0, ..BORED };

#[test]
fn a_bored_cylinder_s_edges_keep_their_names_through_a_change_of_size_a_move_and_a_change_of_hole_diameter()
 {
    let edges = [
        edge(&[0], &[2], 0, 1),
        edge(&[0], &[5], 0, 1),
        edge(&[1], &[2], 0, 1),
        edge(&[1], &[5], 0, 1),
    ];
    for (label, body) in [
        ("as drawn", part(&bored(&BORED), ground())),
        ("resized", part(&bored(&BORED_RESIZED), ground())),
        ("moved", part(&bored(&BORED), moved(ELSEWHERE))),
        (
            "with a narrower hole",
            part(&bored(&BORED_NARROWER), ground()),
        ),
    ] {
        assert_eq!(body.edge_names(), edges, "{label}");
        let vertices = body.vertex_names();
        assert!(vertices.is_empty(), "{label}: {vertices:?}");
    }
}

#[test]
fn a_bored_cylinder_s_named_hole_rim_follows_the_hole_through_a_change_of_size_a_move_and_a_change_of_hole_diameter()
 {
    for (label, sizes, frame) in [
        ("as drawn", &BORED, ground()),
        ("resized", &BORED_RESIZED, ground()),
        ("moved", &BORED, moved(ELSEWHERE)),
        ("with a narrower hole", &BORED_NARROWER, ground()),
    ] {
        let body = exactly(&bored(sizes), frame);
        let rim = named_edge(&body, &edge(&[1], &[5], 0, 1));
        on_circle(
            middle(&body, rim),
            placed(frame, [sizes.off, 0.0, sizes.height]),
            frame.normal(),
            sizes.hole,
            &format!("the rim of the hole on the top, {label}"),
        );
    }
}

/// A shaft turned whole about its plane's first axis, its far end chamfered.
struct Shaft {
    radius: f64,
    length: f64,
    chamfer: f64,
}

const SHAFT: Shaft = Shaft {
    radius: 10.0,
    length: 30.0,
    chamfer: 1.0,
};

const SHAFT_RESIZED: Shaft = Shaft {
    radius: 14.0,
    length: 45.0,
    chamfer: 2.5,
};

/// The plane the shaft is drawn on, at `origin`: its first axis, which the
/// shaft turns about, along Y.
fn lying(origin: [f64; 3]) -> Frame {
    Frame {
        origin: DVec3::from(origin),
        u: DVec3::Y,
        v: DVec3::X,
    }
}

/// The shaft's half drawn on its plane, its last side on the axis: its foot,
/// its wall, its chamfer, its end.
fn shaft_outline(sizes: &Shaft) -> Vec<DVec2> {
    let Shaft {
        radius,
        length,
        chamfer,
    } = *sizes;
    vec![
        DVec2::new(0.0, 0.0),
        DVec2::new(0.0, radius),
        DVec2::new(length - chamfer, radius),
        DVec2::new(length, radius - chamfer),
        DVec2::new(length, 0.0),
    ]
}

fn first_axis() -> Axis {
    Axis {
        origin: DVec2::ZERO,
        direction: DVec2::X,
    }
}

/// The shaft as `cao_part` turns one.
fn shaft_part(sizes: &Shaft, frame: Frame) -> Body {
    let corners = shaft_outline(sizes);
    let profile = Profile {
        exact: Some((Contour::straight(corners.clone()), Vec::new())),
        sampled: Loop::straight(&corners),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    let turn = Turn::of(first_axis(), TAU, 0.0, &profile);
    let part = Body::default();
    let tool = part
        .tool_turned(&profile, frame, &turn)
        .expect("the shaft turns");
    part.union(&tool).expect("the part holds the shaft")
}

/// The shaft turned by the exact kernel alone: its foot numbered nought, its
/// wall one, its chamfer two and its end three.
fn shaft_exactly(sizes: &Shaft, frame: Frame) -> brep::Body {
    let straight = Straight {
        side: 1.0,
        contours: vec![
            shaft_outline(sizes)
                .into_iter()
                .zip(0..)
                .map(|(at, run)| Corner {
                    at,
                    run,
                    bend: None,
                })
                .collect(),
        ],
        runs: 5,
        last_off_the_axis: Some(3),
    };
    let whole = Turn {
        axis: first_axis(),
        angle: TAU,
        resolution: 0.0,
        on_the_axis: 0.0,
    };
    brep::Body::turned(&straight, frame, &whole).expect("the shaft turns")
}

#[test]
fn a_chamfered_shaft_s_edges_keep_their_names_through_a_change_of_size_and_a_move() {
    let versions = [
        ("as drawn", shaft_part(&SHAFT, lying([0.0; 3]))),
        ("resized", shaft_part(&SHAFT_RESIZED, lying([0.0; 3]))),
        ("moved", shaft_part(&SHAFT, lying(ELSEWHERE))),
    ];
    names_kept(&versions, 3, 0);
}

#[test]
fn a_chamfered_shaft_s_named_edge_follows_its_chamfer_through_a_change_of_size_and_a_move() {
    for (label, sizes, frame) in [
        ("as drawn", &SHAFT, lying([0.0; 3])),
        ("resized", &SHAFT_RESIZED, lying([0.0; 3])),
        ("moved", &SHAFT, lying(ELSEWHERE)),
    ] {
        let body = shaft_exactly(sizes, frame);
        let foot = named_edge(&body, &edge(&[1], &[2], 0, 1));
        on_circle(
            middle(&body, foot),
            placed(frame, [sizes.length - sizes.chamfer, 0.0, 0.0]),
            frame.u,
            sizes.radius,
            &format!("the edge between the wall and the chamfer, {label}"),
        );
    }
}

/// A block milled across its top by a slot running from its front through
/// to its back, `from` along it: the slot's floor numbered six, its wall at
/// `from` eleven and its far wall nine, the top's two pieces both one.
struct Slotted {
    block: [f64; 3],
    from: f64,
    width: f64,
    depth: f64,
}

fn slotted(sizes: &Slotted) -> Vec<Raise> {
    let [length, breadth, height] = sizes.block;
    vec![
        join(
            Contour::rectangle(DVec2::ZERO, DVec2::new(length, breadth)),
            0.0,
            height,
        ),
        cut(
            Contour::rectangle(
                DVec2::new(sizes.from, -1.0),
                DVec2::new(sizes.from + sizes.width, breadth + 1.0),
            ),
            height - sizes.depth,
            sizes.depth + 1.0,
        ),
    ]
}

const SLOTTED: Slotted = Slotted {
    block: BLOCK,
    from: 10.0,
    width: 10.0,
    depth: 5.0,
};

const SLOTTED_RESIZED: Slotted = Slotted {
    block: [40.0, 12.0, 14.0],
    from: 12.0,
    width: 10.0,
    depth: 8.0,
};

const SLOTTED_NARROWER: Slotted = Slotted {
    width: 6.0,
    ..SLOTTED
};

#[test]
fn a_slot_s_edges_and_vertices_keep_their_names_through_a_change_of_size_a_move_and_a_change_of_width()
 {
    let versions = [
        ("as drawn", part(&slotted(&SLOTTED), ground())),
        ("resized", part(&slotted(&SLOTTED_RESIZED), ground())),
        ("moved", part(&slotted(&SLOTTED), moved(ELSEWHERE))),
        (
            "with a narrower slot",
            part(&slotted(&SLOTTED_NARROWER), ground()),
        ),
    ];
    names_kept(&versions, 24, 16);
    let names = versions[0].1.edge_names();
    for split in [
        edge(&[1], &[2], 0, 2),
        edge(&[1], &[2], 1, 2),
        edge(&[1], &[4], 0, 2),
        edge(&[1], &[4], 1, 2),
    ] {
        assert!(names.contains(&split), "{split:?} among {names:?}");
    }
}

#[test]
fn a_slot_s_two_edges_along_the_block_s_front_keep_their_ranks_through_a_change_of_size_a_move_and_a_change_of_width()
 {
    for (label, sizes, frame) in [
        ("as drawn", &SLOTTED, ground()),
        ("resized", &SLOTTED_RESIZED, ground()),
        ("moved", &SLOTTED, moved(ELSEWHERE)),
        ("with a narrower slot", &SLOTTED_NARROWER, ground()),
    ] {
        let [length, _, height] = sizes.block;
        let body = exactly(&slotted(sizes), frame);
        let left = named_edge(&body, &edge(&[1], &[2], 0, 2));
        near(
            middle(&body, left),
            placed(frame, [sizes.from / 2.0, 0.0, height]),
            &format!("the first edge between the top and the front, {label}"),
        );
        let right = named_edge(&body, &edge(&[1], &[2], 1, 2));
        near(
            middle(&body, right),
            placed(
                frame,
                [(sizes.from + sizes.width + length) / 2.0, 0.0, height],
            ),
            &format!("the second edge between the top and the front, {label}"),
        );
    }
}

/// A block with a post standing on its floor's plane, the post's axis on
/// the block's back, the post lower than the block: the back, numbered
/// four, meets the post's wall, numbered eight, along two rulings, and
/// stays one face.
struct Posted {
    block: [f64; 3],
    along: f64,
    radius: f64,
    height: f64,
}

fn posted(sizes: &Posted) -> Vec<Raise> {
    let [length, breadth, height] = sizes.block;
    vec![
        join(
            Contour::rectangle(DVec2::ZERO, DVec2::new(length, breadth)),
            0.0,
            height,
        ),
        join(
            circle([sizes.along, breadth], sizes.radius),
            0.0,
            sizes.height,
        ),
    ]
}

const POSTED: Posted = Posted {
    block: BLOCK,
    along: 15.0,
    radius: 3.0,
    height: 6.0,
};

const POSTED_RESIZED: Posted = Posted {
    block: [40.0, 14.0, 12.0],
    along: 22.0,
    radius: 4.0,
    height: 8.0,
};

/// The block and its post as drawn, resized, moved, and turned about the
/// vertical a quarter and a half.
fn posted_versions() -> [(&'static str, &'static Posted, Frame); 5] {
    [
        ("as drawn", &POSTED, ground()),
        ("resized", &POSTED_RESIZED, ground()),
        ("moved", &POSTED, moved(ELSEWHERE)),
        ("turned a quarter", &POSTED, quarter_turns(1)),
        ("turned a half", &POSTED, quarter_turns(2)),
    ]
}

#[test]
fn two_rulings_between_a_block_s_back_and_a_post_keep_their_ranks_through_a_change_of_size_a_move_and_a_turn()
 {
    for (label, sizes, frame) in posted_versions() {
        let breadth = sizes.block[1];
        let body = exactly(&posted(sizes), frame);
        for (rank, along) in [
            (0, sizes.along + sizes.radius),
            (1, sizes.along - sizes.radius),
        ] {
            let ruling = named_edge(&body, &edge(&[4], &[8], rank, 2));
            near(
                middle(&body, ruling),
                placed(frame, [along, breadth, sizes.height / 2.0]),
                &format!("ruling {rank} between the back and the post, {label}"),
            );
        }
    }
}

#[test]
fn four_vertices_where_a_post_meets_a_block_s_back_keep_their_ranks_through_a_change_of_size_a_move_and_a_turn()
 {
    for (label, sizes, frame) in posted_versions() {
        let breadth = sizes.block[1];
        let body = exactly(&posted(sizes), frame);
        let (near_side, far_side) = (sizes.along - sizes.radius, sizes.along + sizes.radius);
        for (name, along, up) in [
            (vertex(&[&[0, 6], &[4], &[8]], 0, 2), near_side, 0.0),
            (vertex(&[&[0, 6], &[4], &[8]], 1, 2), far_side, 0.0),
            (vertex(&[&[4], &[7], &[8]], 0, 2), far_side, sizes.height),
            (vertex(&[&[4], &[7], &[8]], 1, 2), near_side, sizes.height),
        ] {
            near(
                named_vertex(&body, &name),
                placed(frame, [along, breadth, up]),
                &format!("{name:?}, {label}"),
            );
        }
    }
}

/// The edge between a block's top and its front, as the block alone names
/// it.
fn top_front() -> EdgeName {
    edge(&[1], &[2], 0, 1)
}

/// The block with a notch cut into the edge between its top and its front,
/// a third of the way along: the top and the front stay one face each.
fn notched() -> Vec<Raise> {
    let mut steps = block(BLOCK);
    steps.push(cut(
        Contour::rectangle(DVec2::new(10.0, -1.0), DVec2::new(20.0, 3.0)),
        7.0,
        4.0,
    ));
    steps
}

/// The block with a step cut all along the edge between its top and its
/// front: the top and the front no longer meet.
fn rabbeted() -> Vec<Raise> {
    let mut steps = block(BLOCK);
    steps.push(cut(
        Contour::rectangle(DVec2::new(-1.0, -1.0), DVec2::new(31.0, 2.0)),
        8.0,
        3.0,
    ));
    steps
}

/// The slot moved out to the block's right end and past it: the top is no
/// longer left in two pieces.
const SLOTTED_PAST_THE_END: Slotted = Slotted {
    from: 20.0,
    width: 11.0,
    ..SLOTTED
};

/// Every name of `names` the body reports lost, with what it reports.
fn reported(body: &Body, names: &[EdgeName]) -> Vec<(EdgeName, Lost)> {
    names
        .iter()
        .filter_map(|name| Some((name.clone(), body.lost(name)?)))
        .collect()
}

#[test]
fn an_edge_a_later_cut_parts_in_two_is_reported_split_into_its_pieces() {
    let drawn = part(&block(BLOCK), ground());
    assert!(drawn.edge_names().contains(&top_front()));
    let notched = part(&notched(), ground());
    assert_eq!(notched.lost(&top_front()), Some(Lost::Split { now: 2 }));
}

#[test]
fn an_edge_a_later_cut_parts_along_with_its_face_is_reported_split_rather_than_kept_on_its_first_piece()
 {
    let drawn = part(&block(BLOCK), ground());
    let trenched = part(&slotted(&SLOTTED), ground());
    assert!(
        trenched.has_face(12) && !drawn.has_face(12),
        "the slot leaves the top in two pieces, the second numbered afresh"
    );
    for name in [top_front(), edge(&[1], &[4], 0, 1)] {
        assert!(drawn.edge_names().contains(&name), "{name:?}");
        assert_eq!(
            trenched.lost(&name),
            Some(Lost::Split { now: 2 }),
            "{name:?}"
        );
    }
}

#[test]
fn an_edge_a_later_cut_takes_away_is_reported_gone() {
    let rabbeted = part(&rabbeted(), ground());
    assert_eq!(
        rabbeted.lost(&top_front()),
        Some(Lost::Gone),
        "the top and the front no longer meet"
    );
    let trenched = part(&slotted(&SLOTTED), ground());
    let past_the_end = part(&slotted(&SLOTTED_PAST_THE_END), ground());
    for rank in 0..2 {
        let name = edge(&[1], &[2], rank, 2);
        assert!(trenched.edge_names().contains(&name), "{name:?}");
        assert_eq!(
            past_the_end.lost(&name),
            Some(Lost::Gone),
            "{name:?}: one edge of the two is left, and which went cannot be told"
        );
    }
}

#[test]
fn an_edge_a_later_step_leaves_alone_is_not_reported() {
    let block_names = part(&block(BLOCK), ground()).edge_names();
    let slot_names = part(&slotted(&SLOTTED), ground()).edge_names();
    for (label, body, names) in [
        (
            "a block resized",
            part(&block(BLOCK_RESIZED), ground()),
            &block_names,
        ),
        (
            "a block moved",
            part(&block(BLOCK), moved(ELSEWHERE)),
            &block_names,
        ),
        (
            "a slot resized",
            part(&slotted(&SLOTTED_RESIZED), ground()),
            &slot_names,
        ),
        (
            "a slot moved",
            part(&slotted(&SLOTTED), moved(ELSEWHERE)),
            &slot_names,
        ),
        (
            "a slot narrower",
            part(&slotted(&SLOTTED_NARROWER), ground()),
            &slot_names,
        ),
    ] {
        let lost = reported(&body, names);
        assert!(lost.is_empty(), "{label}: {lost:?}");
    }
    assert_eq!(
        reported(&part(&notched(), ground()), &block_names),
        [(top_front(), Lost::Split { now: 2 })],
        "a notch reports the edge it cuts, and no other"
    );
}

/// The block raised by the flats: the drawing handed its outline sampled,
/// and no runs.
fn flat_block() -> Body {
    let corners = [
        DVec2::ZERO,
        DVec2::new(BLOCK[0], 0.0),
        DVec2::new(BLOCK[0], BLOCK[1]),
        DVec2::new(0.0, BLOCK[1]),
    ];
    let triangles = [
        [corners[0], corners[1], corners[2]],
        [corners[0], corners[2], corners[3]],
    ];
    let profile = Profile {
        exact: None,
        sampled: Loop::straight(&corners),
        sampled_holes: Vec::new(),
        triangles: &triangles,
    };
    let part = Body::default();
    let tool = part
        .tool_raised(&profile, ground(), DVec3::Z * BLOCK[2])
        .expect("the flats never decline");
    part.union(&tool).expect("the flats never decline")
}

#[test]
fn every_name_is_reported_gone_on_a_part_the_flats_computed() {
    let names = part(&block(BLOCK), ground()).edge_names();
    assert_eq!(names.len(), 12);
    let flats = flat_block();
    assert!(!flats.is_exact());
    for name in &names {
        assert_eq!(flats.lost(name), Some(Lost::Gone), "{name:?}");
    }
}
