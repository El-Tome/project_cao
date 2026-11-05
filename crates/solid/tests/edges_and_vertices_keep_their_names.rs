//! Every edge and every vertex of an exact body is named from the numbers of
//! the faces it lies between (#582), so that a step pointing at one finds it
//! again once a size changes. An edge answers to the sides of its two faces
//! and to the faces met at its ends, read along the first side's loop, which
//! a move, a turn or a change of size leaves alone; a vertex answers to the
//! sides of every face at it and, where another vertex shares them, to the
//! names of the edges ending there. What nothing topological tells apart, a
//! ball's two poles or the two rims of a cross-bore, is ranked by where it
//! lies, read in the world to within the body's tolerance. A name is found as
//! a face is, by a number its sides share, and on one end kept when a cut has
//! moved the other; both ends moved, it is gone; gone on in more pieces than
//! there were edges between its faces when it was taken, it is split; and
//! with fewer, its pieces joined back, it is gone.
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
//!   `a_slot_s_two_edges_along_the_block_s_front_keep_their_names_through_a_change_of_size_a_move_and_a_change_of_width`;
//!   what tells apart two edges between the same two faces, which the issue
//!   left to the branch, by
//!   `two_rulings_between_a_block_s_back_and_a_post_keep_their_names_through_a_change_of_size_a_move_and_a_turn`,
//!   two vertices at the same faces by
//!   `four_vertices_where_a_post_meets_a_block_s_back_keep_their_names_through_a_change_of_size_a_move_and_a_turn`,
//!   and two that nothing topological tells apart by
//!   `a_ball_s_two_poles_keep_their_names_through_a_move_though_rounding_leaves_them_either_side_of_a_coordinate_plane`
//!   and `the_two_rims_of_a_cross_bore_through_a_shaft_keep_their_names_through_a_change_of_size_and_a_move`
//!   and `two_concentric_rims_between_the_same_two_faces_keep_their_names_through_a_change_of_size_and_a_move`;
//!   and a tool that leaves a face of its own in pieces names the part's
//!   edges as its areas cut one after the other do, by
//!   `a_tool_that_parts_a_face_of_its_own_names_the_part_s_edges_as_cutting_its_areas_one_by_one_does`
//! - A change that splits or removes an edge is reported for that edge —
//!   `an_edge_a_later_cut_parts_in_two_is_reported_split_into_its_pieces`,
//!   `an_edge_a_later_cut_parts_along_with_its_face_is_reported_split_rather_than_kept_on_its_first_piece`,
//!   `an_edge_a_later_cut_takes_away_is_reported_gone`,
//!   `an_edge_a_later_step_leaves_alone_is_not_reported`,
//!   `a_notch_moved_across_a_slot_reports_the_edge_it_bounded_rather_than_finding_another`,
//!   `a_notch_cut_on_one_side_of_a_slot_leaves_the_edge_on_the_other_side_found`,
//!   `an_edge_alone_between_its_faces_is_still_found_when_a_cut_moves_its_end`,
//!   `an_edge_whose_two_ends_a_change_replaces_is_reported_gone_rather_than_found_on_the_far_side`,
//!   `an_edge_of_a_face_a_later_join_merges_flush_is_still_found`,
//!   `an_edge_among_several_whose_one_end_a_cut_moves_is_still_found_on_its_own_piece`,
//!   `a_cross_bore_that_no_longer_comes_out_reports_both_its_rims_gone`,
//!   `an_edge_a_slot_parts_and_a_cut_shortens_at_its_end_is_reported_split_rather_than_found_on_one_piece`,
//!   `a_whole_rim_a_later_slot_interrupts_is_reported_split_rather_than_gone`,
//!   `two_pieces_a_shortened_slot_joins_back_into_one_edge_are_both_reported_gone_rather_than_found_on_it`,
//!   `a_hole_bored_on_through_a_shaft_reports_its_one_rim_split_in_two`,
//!   `an_arc_of_a_rim_looked_up_once_the_slot_is_gone_is_reported_gone_rather_than_found_on_the_whole_rim`,
//!   `every_name_is_reported_gone_on_a_part_the_flats_computed`
//! - The names are written in the body, read back from the geometry cache,
//!   and replayed the same — no test: held by
//!   `an_exact_body_s_edge_and_vertex_names_come_back_from_the_cache_as_the_replay_leaves_them`
//!   and `a_cache_written_before_edges_and_vertices_were_named_is_replayed`,
//!   in `crates/part/src/document/geometry_cache/tests.rs`, beside the cache,
//!   which is `cao_part`'s and out of this crate's sight, and by
//!   `an_exact_body_read_back_with_a_cycle_in_its_roots_still_answers_for_a_name`,
//!   in `crates/part/tests/an_exact_body_comes_back_from_json.rs`, where a
//!   body is read back from its text
//!
//! A face a cut leaves in pieces keeps its number on one piece, and each
//! other piece answers to a number of its own, from a range no step's
//! numbers reach, and to the face it was parted from (#682): a later step's
//! numbers start where the earlier steps' end, whatever any of them left in
//! pieces, and a sketch or a name laid on a face a later step made stays on
//! that face. Not on a piece: the pieces are counted one after the other
//! across the part, so the pieces a later cut leaves answer to other
//! numbers when an earlier cut starts or stops parting a face. A tool that
//! comes with pieces of its own is cut into a part holding some with its
//! pieces counted past the part's, each still answering to its own face —
//! `a_tool_s_piece_cut_into_a_part_that_holds_a_piece_answers_to_a_number_of_its_own_and_to_its_own_wall`
//! — and a face parted after that is counted on past them all —
//! `a_face_parted_after_a_tool_brought_its_pieces_answers_to_the_number_after_every_piece_before_it`.
//!
//! Closes #682.
//! - The notch of the #582 fixtures is numbered the same whether or not the
//!   slot before it parts the top face —
//!   `a_notch_after_a_slot_answers_to_the_same_numbers_whether_or_not_the_slot_parts_the_top`
//!
//! The faces a step made, which the variables panel blinks for each step a
//! change it refuses names, count the pieces of those a later cut parted,
//! and the cut counts none of them: held by
//! `a_step_of_matter_names_both_pieces_of_its_face_a_later_cut_parts_and_the_cut_names_neither`,
//! in `crates/part/tests/extruding.rs`, where a step's faces are read.

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

/// The same part built by the exact kernel alone, what a named edge is found
/// on: each tool's numbers moved above the numbers the steps before it
/// raised. The kernel alone parts no face, so it gives no number of its own
/// to a piece a cut leaves apart, as the body does; the body numbers such a
/// piece apart from the steps, so a tool cut after it is numbered here as the
/// body numbers it, and the names a test looks up on both are made of
/// numbers they agree on.
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

/// Sides as the tests write them: each face's numbers, face after face.
fn sides(faces: &[&[u32]]) -> Vec<Vec<u32>> {
    faces.iter().map(|side| side.to_vec()).collect()
}

/// The name of the one edge between two sides: the sides of its two faces,
/// and the faces met at its ends, where it starts and where it ends along the
/// first side's loop.
fn edge(first: &[u32], second: &[u32], start: &[&[u32]], end: &[&[u32]]) -> EdgeName {
    EdgeName {
        between: [first.to_vec(), second.to_vec()],
        ends: [sides(start), sides(end)],
        among: 1,
        rank: 0,
        of: 1,
    }
}

/// The name of an edge taken where `count` edges lay between its two sides.
fn among(count: u32, name: EdgeName) -> EdgeName {
    EdgeName {
        among: count,
        ..name
    }
}

/// The name of the `rank`th of `of` whole closed edges between two sides,
/// the only edges between them, which have no end to be told apart by.
fn closed(first: &[u32], second: &[u32], rank: u32, of: u32) -> EdgeName {
    EdgeName {
        between: [first.to_vec(), second.to_vec()],
        ends: [Vec::new(), Vec::new()],
        among: of,
        rank,
        of,
    }
}

/// The name of the one vertex at those faces.
fn vertex_alone(faces: &[&[u32]]) -> VertexName {
    VertexName {
        faces: sides(faces),
        edges: None,
        rank: 0,
        of: 1,
    }
}

/// The name of a vertex at faces another vertex shares, told apart by the
/// names of the edges ending there.
fn vertex_ending(faces: &[&[u32]], mut edges: Vec<EdgeName>) -> VertexName {
    edges.sort();
    VertexName {
        faces: sides(faces),
        edges: Some(edges),
        rank: 0,
        of: 1,
    }
}

/// Holds that names tell every edge apart: none given twice, each counting
/// the edges between its two sides, and the edges alike in their sides and
/// their ends ranked from nought to as many as there are.
fn told_apart(names: &[EdgeName]) {
    let distinct: BTreeSet<&EdgeName> = names.iter().collect();
    assert_eq!(distinct.len(), names.len(), "a name given twice: {names:?}");
    for name in names {
        let between = names
            .iter()
            .filter(|other| other.between == name.between)
            .count();
        assert_eq!(name.among as usize, between, "{name:?} among {names:?}");
        let alike = names
            .iter()
            .filter(|other| other.between == name.between && other.ends == name.ends)
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

/// The edge the body finds for a name, which a step pointing at it would
/// take.
fn found(body: &brep::Body, name: &EdgeName) -> EdgeId {
    body.find(name)
        .unwrap_or_else(|lost| panic!("{name:?} not found: {lost:?}"))
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
        edge(&[0], &[2], &[&[3]], &[&[5]]),
        edge(&[0], &[3], &[&[4]], &[&[2]]),
        edge(&[0], &[4], &[&[5]], &[&[3]]),
        edge(&[0], &[5], &[&[2]], &[&[4]]),
        edge(&[1], &[2], &[&[5]], &[&[3]]),
        edge(&[1], &[3], &[&[2]], &[&[4]]),
        edge(&[1], &[4], &[&[3]], &[&[5]]),
        edge(&[1], &[5], &[&[4]], &[&[2]]),
        edge(&[2], &[3], &[&[0]], &[&[1]]),
        edge(&[2], &[5], &[&[1]], &[&[0]]),
        edge(&[3], &[4], &[&[0]], &[&[1]]),
        edge(&[4], &[5], &[&[0]], &[&[1]]),
    ];
    let vertices = [
        vertex_alone(&[&[0], &[2], &[3]]),
        vertex_alone(&[&[0], &[2], &[5]]),
        vertex_alone(&[&[0], &[3], &[4]]),
        vertex_alone(&[&[0], &[4], &[5]]),
        vertex_alone(&[&[1], &[2], &[3]]),
        vertex_alone(&[&[1], &[2], &[5]]),
        vertex_alone(&[&[1], &[3], &[4]]),
        vertex_alone(&[&[1], &[4], &[5]]),
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
        let top_right = named_edge(&body, &edge(&[1], &[3], &[&[2]], &[&[4]]));
        near(
            middle(&body, top_right),
            placed(frame, [length, breadth / 2.0, height]),
            &format!("the edge between the top and the right side, {label}"),
        );
        near(
            named_vertex(&body, &vertex_alone(&[&[1], &[3], &[4]])),
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
        closed(&[0], &[2], 0, 1),
        closed(&[0], &[5], 0, 1),
        closed(&[1], &[2], 0, 1),
        closed(&[1], &[5], 0, 1),
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
        let rim = named_edge(&body, &closed(&[1], &[5], 0, 1));
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
        let foot = named_edge(&body, &closed(&[1], &[2], 0, 1));
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
    for apart in [
        left_of_the_slot(),
        right_of_the_slot(),
        among(2, edge(&[1], &[4], &[&[3]], &[&[9]])),
        among(2, edge(&[1], &[4], &[&[11]], &[&[5]])),
    ] {
        assert!(names.contains(&apart), "{apart:?} among {names:?}");
    }
}

#[test]
fn a_slot_s_two_edges_along_the_block_s_front_keep_their_names_through_a_change_of_size_a_move_and_a_change_of_width()
 {
    for (label, sizes, frame) in [
        ("as drawn", &SLOTTED, ground()),
        ("resized", &SLOTTED_RESIZED, ground()),
        ("moved", &SLOTTED, moved(ELSEWHERE)),
        ("with a narrower slot", &SLOTTED_NARROWER, ground()),
    ] {
        let [length, _, height] = sizes.block;
        let body = exactly(&slotted(sizes), frame);
        let left = named_edge(&body, &left_of_the_slot());
        near(
            middle(&body, left),
            placed(frame, [sizes.from / 2.0, 0.0, height]),
            &format!("the edge from the left side to the slot, {label}"),
        );
        let right = named_edge(&body, &right_of_the_slot());
        near(
            middle(&body, right),
            placed(
                frame,
                [(sizes.from + sizes.width + length) / 2.0, 0.0, height],
            ),
            &format!("the edge from the slot to the right side, {label}"),
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

/// The ruling the back's loop runs up, from the floor to the post's top.
fn rising() -> EdgeName {
    among(2, edge(&[4], &[8], &[&[0, 6]], &[&[7]]))
}

/// The ruling the back's loop runs down, from the post's top to the floor.
fn falling() -> EdgeName {
    among(2, edge(&[4], &[8], &[&[7]], &[&[0, 6]]))
}

#[test]
fn two_rulings_between_a_block_s_back_and_a_post_keep_their_names_through_a_change_of_size_a_move_and_a_turn()
 {
    for (label, sizes, frame) in posted_versions() {
        let breadth = sizes.block[1];
        let body = exactly(&posted(sizes), frame);
        for (name, along) in [
            (rising(), sizes.along + sizes.radius),
            (falling(), sizes.along - sizes.radius),
        ] {
            let ruling = named_edge(&body, &name);
            near(
                middle(&body, ruling),
                placed(frame, [along, breadth, sizes.height / 2.0]),
                &format!("{name:?}, {label}"),
            );
        }
    }
}

#[test]
fn four_vertices_where_a_post_meets_a_block_s_back_keep_their_names_through_a_change_of_size_a_move_and_a_turn()
 {
    let foot = [&[0, 6][..], &[4], &[8]];
    let head = [&[4][..], &[7], &[8]];
    let round_the_foot = edge(&[0, 6], &[8], &[&[4]], &[&[4]]);
    let under_the_head = || {
        vec![
            edge(&[4], &[7], &[&[8]], &[&[8]]),
            edge(&[7], &[8], &[&[4]], &[&[4]]),
        ]
    };
    let names = [
        vertex_ending(
            &foot,
            vec![
                among(2, edge(&[0, 6], &[4], &[&[5]], &[&[8]])),
                round_the_foot.clone(),
                falling(),
            ],
        ),
        vertex_ending(
            &foot,
            vec![
                among(2, edge(&[0, 6], &[4], &[&[8]], &[&[3]])),
                round_the_foot,
                rising(),
            ],
        ),
        vertex_ending(&head, [under_the_head(), vec![rising()]].concat()),
        vertex_ending(&head, [under_the_head(), vec![falling()]].concat()),
    ];
    for (label, sizes, frame) in posted_versions() {
        let breadth = sizes.block[1];
        let body = exactly(&posted(sizes), frame);
        let (near_side, far_side) = (sizes.along - sizes.radius, sizes.along + sizes.radius);
        for (name, along, up) in [
            (&names[0], near_side, 0.0),
            (&names[1], far_side, 0.0),
            (&names[2], far_side, sizes.height),
            (&names[3], near_side, sizes.height),
        ] {
            near(
                named_vertex(&body, name),
                placed(frame, [along, breadth, up]),
                &format!("{name:?}, {label}"),
            );
        }
    }
}

/// The edge between a block's top and its front, as the block alone names
/// it.
fn top_front() -> EdgeName {
    edge(&[1], &[2], &[&[5]], &[&[3]])
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

/// The edge between the top and the front from the left side to the slot.
fn left_of_the_slot() -> EdgeName {
    among(2, edge(&[1], &[2], &[&[5]], &[&[11]]))
}

/// The edge between the top and the front from the slot to the right side.
fn right_of_the_slot() -> EdgeName {
    among(2, edge(&[1], &[2], &[&[9]], &[&[3]]))
}

/// The slot, then a notch three wide cut into the edge between the top and
/// the front, from `from` along it.
fn slotted_and_notched(from: f64) -> Vec<Raise> {
    notched_after(&SLOTTED, from)
}

/// The slot `sizes` gives, then a notch three wide and two deep cut into the
/// edge between the top and the front, from `from` along it, down to eight.
fn notched_after(sizes: &Slotted, from: f64) -> Vec<Raise> {
    let mut steps = slotted(sizes);
    steps.push(cut(
        Contour::rectangle(DVec2::new(from, -1.0), DVec2::new(from + 3.0, 2.0)),
        8.0,
        3.0,
    ));
    steps
}

/// The numbers of the notch's walls across the edge, its left one and its
/// right one: the part numbers the notch from where the slot's numbers end,
/// the top's second piece answering to a number of its own, apart from them.
fn notch_walls() -> (u32, u32) {
    let notch = part(&slotted(&SLOTTED), ground()).faces_end() as u32;
    (notch + 5, notch + 3)
}

/// The block with its front right corner cut off from its floor to its top:
/// the edge between the top and the front ends at the cut, no longer at the
/// right side, and the front and the right side no longer meet.
fn cornered() -> Vec<Raise> {
    let mut steps = block(BLOCK);
    steps.push(cut(
        Contour::rectangle(DVec2::new(27.0, -1.0), DVec2::new(31.0, 3.0)),
        -1.0,
        12.0,
    ));
    steps
}

/// The face a ray sent from `origin` along `direction` meets first, which a
/// step pointing there would take, held to meet it `distance` along so that a
/// ray gone astray is not taken for a face numbered otherwise.
fn face_met(body: &Body, origin: [f64; 3], direction: DVec3, distance: f64) -> usize {
    let origin = DVec3::from(origin);
    let hit = body
        .ray_hit(origin, direction)
        .unwrap_or_else(|| panic!("a ray from {origin} along {direction} meets nothing"));
    assert!(
        (hit.distance - distance).abs() < PLACE,
        "a ray from {origin} along {direction} meets a face {} along, where \
         {distance} was expected",
        hit.distance
    );
    hit.face
}

/// Of the two faces rays sent from `origins` along `direction` meet,
/// `distance` along, the one that does not answer to `kept`: the piece of
/// `kept` a cut left apart, one ray meeting each piece.
fn piece_apart(
    body: &Body,
    kept: usize,
    origins: [[f64; 3]; 2],
    direction: DVec3,
    distance: f64,
) -> usize {
    let met = origins.map(|origin| face_met(body, origin, direction, distance));
    assert!(met.contains(&kept), "one piece answers to {kept}: {met:?}");
    met.into_iter()
        .find(|&face| face != kept)
        .unwrap_or_else(|| panic!("the other piece answers to a number of its own: {met:?}"))
}

/// The number the top's piece that did not keep the top's answers to, once
/// the slot has left the top in two: met from above right of the slot, and
/// the top's own left of it, or the other way round.
fn second_piece_of_the_top(body: &Body) -> usize {
    let [_, breadth, height] = SLOTTED.block;
    let above = |along: f64| [along, breadth / 2.0, height + 10.0];
    piece_apart(body, 1, [above(5.0), above(25.0)], DVec3::NEG_Z, 10.0)
}

/// Where the notch is cut along the block, left of the slot: from three to
/// six, where the top and the front stand alike whether the slot parts the
/// top or stops short of its right end.
const NOTCH: f64 = 3.0;

/// The numbers a later step would take the faces of the notch cut at
/// [`NOTCH`] by, each met by a ray sent into the notch: its floor, its back
/// wall, its left wall and its right wall.
fn notch_faces(body: &Body) -> [(&'static str, usize); 4] {
    let (along, across, up) = (NOTCH + 1.1, 1.3, 9.3);
    [
        (
            "floor",
            face_met(body, [along, 0.6, 20.0], DVec3::NEG_Z, 12.0),
        ),
        (
            "back wall",
            face_met(body, [along, -5.0, up], DVec3::Y, 7.0),
        ),
        (
            "left wall",
            face_met(body, [along, across, up], DVec3::NEG_X, 1.1),
        ),
        (
            "right wall",
            face_met(body, [along, across, up], DVec3::X, 1.9),
        ),
    ]
}

/// The names of the edges a face numbered from `from` on lies along: those
/// the notch bounds, in a part whose notch is numbered from `from`.
fn edges_numbered_from(body: &Body, from: usize) -> BTreeSet<EdgeName> {
    let numbers = from..body.faces_end();
    body.edge_names()
        .into_iter()
        .filter(|name| {
            name.between
                .iter()
                .flatten()
                .any(|&number| numbers.contains(&(number as usize)))
        })
        .collect()
}

#[test]
fn a_notch_after_a_slot_answers_to_the_same_numbers_whether_or_not_the_slot_parts_the_top() {
    let parting = part(&slotted(&SLOTTED), ground());
    second_piece_of_the_top(&parting);
    let parting_nothing = part(&slotted(&SLOTTED_PAST_THE_END), ground());
    let notched = part(&notched_after(&SLOTTED, NOTCH), ground());
    let notched_whole = part(&notched_after(&SLOTTED_PAST_THE_END, NOTCH), ground());

    let numbers = notch_faces(&notched);
    assert_eq!(
        numbers,
        notch_faces(&notched_whole),
        "the notch's faces answer to the same numbers whether the slot before \
         it left the top in two pieces or whole"
    );
    for (face, number) in numbers {
        assert!(
            (parting_nothing.faces_end()..notched_whole.faces_end()).contains(&number),
            "the notch's {face} answers to {number}, among the numbers it \
             raised past the slot's"
        );
    }
    assert_eq!(
        parting.faces_end(),
        parting_nothing.faces_end(),
        "a step after the slot numbers its faces from where the slot's end, \
         whether or not it parted the top"
    );
    let bounded = edges_numbered_from(&notched_whole, parting_nothing.faces_end());
    assert_eq!(
        bounded.len(),
        11,
        "the notch bounds four edges round its floor, two up its back \
         corners, and five along the top and the front: {bounded:?}"
    );
    assert_eq!(
        edges_numbered_from(&notched, parting.faces_end()),
        bounded,
        "the edges the notch bounds keep the names a later step takes them by"
    );
}

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
    let piece = second_piece_of_the_top(&trenched);
    assert_eq!(
        trenched.parted_from(piece),
        Some(1),
        "the slot leaves the top in two pieces, the second parted from it"
    );
    assert!(
        !drawn.has_face(piece) && !(0..trenched.faces_end()).contains(&piece),
        "the second piece answers to {piece}, which no step gives, where the \
         steps' numbers end at {}",
        trenched.faces_end()
    );
    for name in [top_front(), edge(&[1], &[4], &[&[3]], &[&[5]])] {
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
    let names = part(&slotted(&SLOTTED), ground()).edge_names();
    let (left, right) = (left_of_the_slot(), right_of_the_slot());
    assert!(names.contains(&left) && names.contains(&right), "{names:?}");
    let past_the_end = part(&slotted(&SLOTTED_PAST_THE_END), ground());
    assert_eq!(
        past_the_end.lost(&right),
        Some(Lost::Gone),
        "the edge from the slot to the right side went with the top past the slot"
    );
    assert_eq!(
        past_the_end.lost(&left),
        None,
        "the edge from the left side to the slot is still there"
    );
    let body = exactly(&slotted(&SLOTTED_PAST_THE_END), ground());
    let [_, _, height] = SLOTTED_PAST_THE_END.block;
    near(
        middle(&body, found(&body, &left)),
        placed(ground(), [SLOTTED_PAST_THE_END.from / 2.0, 0.0, height]),
        "the edge found from the left side to the slot",
    );
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

#[test]
fn a_notch_moved_across_a_slot_reports_the_edge_it_bounded_rather_than_finding_another() {
    let (_, notch_s_right_wall) = notch_walls();
    let notched_left = part(&slotted_and_notched(3.0), ground());
    let from_the_notch_to_the_slot = among(3, edge(&[1], &[2], &[&[notch_s_right_wall]], &[&[11]]));
    let names = notched_left.edge_names();
    assert!(
        names.contains(&from_the_notch_to_the_slot),
        "{from_the_notch_to_the_slot:?} among {names:?}"
    );
    let notched_right = part(&slotted_and_notched(24.0), ground());
    assert_eq!(
        notched_right.lost(&from_the_notch_to_the_slot),
        Some(Lost::Split { now: 3 }),
        "no edge runs from the notch to the slot any more: the notch's wall \
         starts the edge to the right side, and the slot's ends the one from \
         the left side"
    );
}

#[test]
fn a_notch_cut_on_one_side_of_a_slot_leaves_the_edge_on_the_other_side_found() {
    let names = part(&slotted(&SLOTTED), ground()).edge_names();
    let (left, right) = (left_of_the_slot(), right_of_the_slot());
    assert!(names.contains(&left) && names.contains(&right), "{names:?}");
    let notched = part(&slotted_and_notched(24.0), ground());
    assert_eq!(
        notched.lost(&left),
        None,
        "the edge from the left side to the slot is untouched"
    );
    let body = exactly(&slotted_and_notched(24.0), ground());
    near(
        middle(&body, found(&body, &left)),
        placed(ground(), [SLOTTED.from / 2.0, 0.0, SLOTTED.block[2]]),
        "the edge found from the left side to the slot",
    );
    assert_eq!(
        notched.lost(&right),
        Some(Lost::Split { now: 3 }),
        "the edge from the slot to the right side is the one notched"
    );
}

#[test]
fn an_edge_alone_between_its_faces_is_still_found_when_a_cut_moves_its_end() {
    let steps = cornered();
    let cornered = part(&steps, ground());
    assert_eq!(
        cornered.lost(&top_front()),
        None,
        "the top and the front still meet once"
    );
    let body = exactly(&steps, ground());
    near(
        middle(&body, found(&body, &top_front())),
        placed(ground(), [13.5, 0.0, BLOCK[2]]),
        "the edge found between the top and the front, from the left side to the cut",
    );
    assert_eq!(
        cornered.lost(&edge(&[2], &[3], &[&[0]], &[&[1]])),
        Some(Lost::Gone),
        "the front and the right side no longer meet"
    );
}

/// The radius of the ball.
const BALL: f64 = 8.0;

/// A ball, a half disc turned whole about the first axis of a plane turned a
/// quarter by the arithmetic, at `origin`: the plane's axes come out a
/// rounding off the world's, which leaves the two poles, drawn at the
/// origin, within rounding of the plane x = 0, one on either side of it. Its
/// one face is numbered one and holds both poles, which no edge reaches.
fn ball(origin: [f64; 3]) -> (brep::Body, Frame) {
    let (cos, sin) = DVec2::from_angle(-std::f64::consts::FRAC_PI_2).into();
    let frame = Frame {
        origin: DVec3::from(origin),
        u: DVec3::new(cos, sin, 0.0),
        v: DVec3::new(-sin, cos, 0.0),
    };
    let half = Contour {
        corners: vec![DVec2::new(-BALL, 0.0), DVec2::new(BALL, 0.0)],
        runs: vec![
            Run::Straight,
            Run::Round {
                center: DVec2::ZERO,
                turn: std::f64::consts::PI,
            },
        ],
    };
    let sampled: Vec<DVec2> =
        std::iter::once(DVec2::new(-BALL, 0.0))
            .chain((0..32).map(|step| {
                DVec2::from_angle(std::f64::consts::PI * f64::from(step) / 32.0) * BALL
            }))
            .collect();
    let profile = Profile {
        exact: Some((half.clone(), Vec::new())),
        sampled: Loop::straight(&sampled),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    let whole = Turn::of(first_axis(), TAU, 0.0, &profile);
    let straight =
        Straight::of(&half, &[], frame, &whole, 1.0).expect("the half disc lies on its axis");
    let body = brep::Body::turned(&straight, frame, &whole).expect("the ball turns");
    (body, frame)
}

#[test]
fn a_ball_s_two_poles_keep_their_names_through_a_move_though_rounding_leaves_them_either_side_of_a_coordinate_plane()
 {
    for (label, origin) in [
        ("as drawn", [0.0; 3]),
        ("moved a hair", [1e-3, 0.0, 0.0]),
        ("moved", ELSEWHERE),
    ] {
        let (body, frame) = ball(origin);
        for (rank, along) in [(0, BALL), (1, -BALL)] {
            let pole = VertexName {
                faces: vec![vec![1]],
                edges: Some(Vec::new()),
                rank,
                of: 2,
            };
            near(
                named_vertex(&body, &pole),
                placed(frame, [along, 0.0, 0.0]),
                &format!("{pole:?}, {label}"),
            );
        }
    }
}

/// The block with a notch cut into the edge between its top and its front,
/// from `from` to `to` along it, three deep.
fn notched_between(from: f64, to: f64) -> Vec<Raise> {
    let mut steps = block(BLOCK);
    steps.push(cut(
        Contour::rectangle(DVec2::new(from, -1.0), DVec2::new(to, 3.0)),
        7.0,
        4.0,
    ));
    steps
}

#[test]
fn an_edge_whose_two_ends_a_change_replaces_is_reported_gone_rather_than_found_on_the_far_side() {
    let stub = edge(&[1], &[2], &[&[5]], &[&[11]]);
    let names = part(&notched_between(10.0, 31.0), ground()).edge_names();
    assert!(names.contains(&stub), "{stub:?} among {names:?}");
    let widened = part(&notched_between(-1.0, 20.0), ground());
    let left = widened.edge_names();
    assert_eq!(
        left.iter()
            .filter(|name| name.between == stub.between)
            .count(),
        1,
        "the top and the front still meet once, at the far end: {left:?}"
    );
    assert_eq!(
        widened.lost(&stub),
        Some(Lost::Gone),
        "the edge from the left side to the notch is gone, and the stub from \
         the notch to the right side, both its ends other faces, is not it"
    );
}

#[test]
fn an_edge_of_a_face_a_later_join_merges_flush_is_still_found() {
    let floor_front = edge(&[0], &[2], &[&[3]], &[&[5]]);
    let names = part(&block(BLOCK), ground()).edge_names();
    assert!(
        names.contains(&floor_front),
        "{floor_front:?} among {names:?}"
    );
    let posted_part = part(&posted(&POSTED), ground());
    let merged = posted_part.edge_names();
    assert!(
        merged.contains(&edge(&[0, 6], &[2], &[&[3]], &[&[5]])),
        "the post's floor joins the block's, and the floor answers to both: {merged:?}"
    );
    assert_eq!(
        posted_part.lost(&floor_front),
        None,
        "the floor still answers to its own number"
    );
    let body = exactly(&posted(&POSTED), ground());
    near(
        middle(&body, found(&body, &floor_front)),
        placed(ground(), [BLOCK[0] / 2.0, 0.0, 0.0]),
        "the edge found between the floor and the front",
    );
}

/// The slot, then the block's front right corner cut off from its floor to
/// its top: the edge from the slot to the right side ends at the cut.
fn slotted_and_cornered() -> Vec<Raise> {
    let mut steps = slotted(&SLOTTED);
    steps.push(cut(
        Contour::rectangle(DVec2::new(27.0, -1.0), DVec2::new(31.0, 3.0)),
        -1.0,
        12.0,
    ));
    steps
}

#[test]
fn an_edge_among_several_whose_one_end_a_cut_moves_is_still_found_on_its_own_piece() {
    let cornered = part(&slotted_and_cornered(), ground());
    assert_eq!(
        cornered.lost(&right_of_the_slot()),
        None,
        "the edge from the slot keeps its start, and only its end was cut"
    );
    let body = exactly(&slotted_and_cornered(), ground());
    near(
        middle(&body, found(&body, &right_of_the_slot())),
        placed(ground(), [23.5, 0.0, SLOTTED.block[2]]),
        "the edge found from the slot to the cut corner",
    );
}

/// The shaft at `origin` with a hole of radius `hole` bored across it along
/// Z halfway along, from `from` below its axis up past its top. The hole's
/// wall, numbered six, meets the shaft's wall, numbered one, along two whole
/// rims when it comes out on both sides, which no end tells apart.
fn cross_bored(sizes: &Shaft, origin: [f64; 3], hole: f64, from: f64) -> (brep::Body, Frame) {
    let frame = lying(origin);
    let across = Frame {
        origin: frame.origin + DVec3::Z * from,
        u: DVec3::X,
        v: DVec3::Y,
    };
    let tool = brep::Body::raised(
        &circle([0.0, sizes.length / 2.0], hole),
        &[],
        across,
        DVec3::Z * (sizes.radius + 5.0 - from),
    )
    .expect("the hole raises")
    .renumbered(4);
    let body = shaft_exactly(sizes, frame)
        .cut_by(&tool)
        .expect("the hole bores across the shaft");
    (body, frame)
}

#[test]
fn the_two_rims_of_a_cross_bore_through_a_shaft_keep_their_names_through_a_change_of_size_and_a_move()
 {
    for (label, sizes, origin, hole) in [
        ("as drawn", &SHAFT, [0.0; 3], 3.0),
        ("resized", &SHAFT_RESIZED, [0.0; 3], 4.0),
        ("moved", &SHAFT, ELSEWHERE, 3.0),
    ] {
        let (body, frame) = cross_bored(sizes, origin, hole, -sizes.radius - 5.0);
        for (rank, below) in [(0, true), (1, false)] {
            let rim = named_edge(&body, &closed(&[1], &[6], rank, 2));
            let up = middle(&body, rim).z - frame.origin.z;
            assert!(
                if below { up < -hole } else { up > hole },
                "rim {rank} stands {up} off the shaft's axis, {label}"
            );
        }
    }
}

#[test]
fn a_cross_bore_that_no_longer_comes_out_reports_both_its_rims_gone() {
    let (through, _) = cross_bored(&SHAFT, [0.0; 3], 3.0, -SHAFT.radius - 5.0);
    let names = through.edge_names();
    let (blind, _) = cross_bored(&SHAFT, [0.0; 3], 3.0, 2.0);
    for rank in 0..2 {
        let rim = closed(&[1], &[6], rank, 2);
        assert!(names.contains(&rim), "{rim:?} among {names:?}");
        assert_eq!(
            blind.find(&rim),
            Err(Lost::Gone),
            "{rim:?}: one rim of the two is left, and nothing tells which"
        );
    }
}

#[test]
fn an_edge_a_slot_parts_and_a_cut_shortens_at_its_end_is_reported_split_rather_than_found_on_one_piece()
 {
    let names = part(&block(BLOCK), ground()).edge_names();
    assert!(names.contains(&top_front()), "{names:?}");
    assert_eq!(
        part(&slotted_and_cornered(), ground()).lost(&top_front()),
        Some(Lost::Split { now: 2 }),
        "the edge goes on either side of the slot, its far end cut back: \
         neither piece is the edge alone"
    );
}

/// The bored cylinder with a slot four wide cut across its top through the
/// hole, three deep: the top is left in two pieces, and the hole's rim on
/// it in two arcs.
fn bored_and_slotted() -> Vec<Raise> {
    let mut steps = bored(&BORED);
    steps.push(cut(
        Contour::rectangle(DVec2::new(6.0, -25.0), DVec2::new(10.0, 25.0)),
        7.0,
        4.0,
    ));
    steps
}

#[test]
fn a_whole_rim_a_later_slot_interrupts_is_reported_split_rather_than_gone() {
    let rim = closed(&[1], &[5], 0, 1);
    let names = part(&bored(&BORED), ground()).edge_names();
    assert!(names.contains(&rim), "{rim:?} among {names:?}");
    assert_eq!(
        part(&bored_and_slotted(), ground()).lost(&rim),
        Some(Lost::Split { now: 2 }),
        "the rim of the hole on the top goes on in two arcs, one each side \
         of the slot"
    );
}

/// A bead at `origin`: a half disc whose flat side stands square to the
/// first axis of its plane, from `inner` out to `outer` off the axis, its
/// round side away from the plane, turned whole about that axis. Its flat
/// side, numbered nought, meets its round, numbered one, along two whole
/// circles about one centre, which no end tells apart.
fn bead(inner: f64, outer: f64, origin: [f64; 3]) -> (brep::Body, Frame) {
    let middle = (inner + outer) / 2.0;
    let half = (outer - inner) / 2.0;
    let profile_of = Contour {
        corners: vec![DVec2::new(0.0, outer), DVec2::new(0.0, inner)],
        runs: vec![
            Run::Straight,
            Run::Round {
                center: DVec2::new(0.0, middle),
                turn: std::f64::consts::PI,
            },
        ],
    };
    let sampled: Vec<DVec2> = std::iter::once(DVec2::new(0.0, outer))
        .chain((0..32).map(|step| {
            DVec2::new(0.0, middle)
                + DVec2::from_angle(
                    -std::f64::consts::FRAC_PI_2 + std::f64::consts::PI * f64::from(step) / 32.0,
                ) * half
        }))
        .collect();
    let profile = Profile {
        exact: Some((profile_of.clone(), Vec::new())),
        sampled: Loop::straight(&sampled),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    let frame = lying(origin);
    let whole = Turn::of(first_axis(), TAU, 0.0, &profile);
    let straight = Straight::of(&profile_of, &[], frame, &whole, 1.0)
        .expect("the half disc lies off its axis");
    let body = brep::Body::turned(&straight, frame, &whole).expect("the bead turns");
    (body, frame)
}

#[test]
fn two_concentric_rims_between_the_same_two_faces_keep_their_names_through_a_change_of_size_and_a_move()
 {
    for (label, inner, outer, origin) in [
        ("as drawn", 5.0, 9.0, [0.0; 3]),
        ("resized", 7.0, 13.0, [0.0; 3]),
        ("moved", 5.0, 9.0, ELSEWHERE),
    ] {
        let (body, frame) = bead(inner, outer, origin);
        for (rank, radius) in [(0, inner), (1, outer)] {
            let name = closed(&[0], &[1], rank, 2);
            on_circle(
                middle(&body, named_edge(&body, &name)),
                frame.origin,
                frame.u,
                radius,
                &format!("{name:?}, {label}"),
            );
        }
    }
}

/// The slot drawn short of the front, from two along the block's depth on
/// past its back: the top is left whole, and meets the front along one edge.
fn slotted_short_of_the_front() -> Vec<Raise> {
    let [length, breadth, height] = SLOTTED.block;
    vec![
        join(
            Contour::rectangle(DVec2::ZERO, DVec2::new(length, breadth)),
            0.0,
            height,
        ),
        cut(
            Contour::rectangle(
                DVec2::new(SLOTTED.from, 2.0),
                DVec2::new(SLOTTED.from + SLOTTED.width, breadth + 1.0),
            ),
            height - SLOTTED.depth,
            SLOTTED.depth + 1.0,
        ),
    ]
}

#[test]
fn two_pieces_a_shortened_slot_joins_back_into_one_edge_are_both_reported_gone_rather_than_found_on_it()
 {
    let names = part(&slotted(&SLOTTED), ground()).edge_names();
    let (left, right) = (left_of_the_slot(), right_of_the_slot());
    assert!(names.contains(&left) && names.contains(&right), "{names:?}");
    let shortened = part(&slotted_short_of_the_front(), ground());
    let now = shortened.edge_names();
    assert_eq!(
        now.iter()
            .filter(|name| name.between == left.between)
            .count(),
        1,
        "the top and the front meet along one edge again: {now:?}"
    );
    for (piece, name) in [("left", &left), ("right", &right)] {
        assert_eq!(
            shortened.lost(name),
            Some(Lost::Gone),
            "the {piece} piece is not the whole edge, which the other piece \
             would find as well"
        );
    }
}

#[test]
fn a_hole_bored_on_through_a_shaft_reports_its_one_rim_split_in_two() {
    let (blind, _) = cross_bored(&SHAFT, [0.0; 3], 3.0, 2.0);
    let rim = closed(&[1], &[6], 0, 1);
    let names = blind.edge_names();
    assert!(names.contains(&rim), "{rim:?} among {names:?}");
    let (through, _) = cross_bored(&SHAFT, [0.0; 3], 3.0, -SHAFT.radius - 5.0);
    assert_eq!(
        through.find(&rim),
        Err(Lost::Split { now: 2 }),
        "the hole comes out the other side, and the shaft's wall meets it \
         along two rims alike in their ends where it met one"
    );
}

#[test]
fn an_arc_of_a_rim_looked_up_once_the_slot_is_gone_is_reported_gone_rather_than_found_on_the_whole_rim()
 {
    let arcs = [
        among(2, edge(&[1], &[5], &[&[11]], &[&[11]])),
        among(2, edge(&[1], &[5], &[&[9]], &[&[9]])),
    ];
    let names = part(&bored_and_slotted(), ground()).edge_names();
    let whole = part(&bored(&BORED), ground());
    for arc in &arcs {
        assert!(names.contains(arc), "{arc:?} among {names:?}");
        assert_eq!(
            whole.lost(arc),
            Some(Lost::Gone),
            "{arc:?}: the whole rim has no end where the slot's wall stood"
        );
    }
}

/// A tool raised against `part` as the part raises one, from `from` up for
/// `height`, on the ground.
fn tool(part: &Body, outline: Contour, from: f64, height: f64) -> Body {
    let profile = Profile {
        exact: Some((outline, Vec::new())),
        sampled: Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    part.tool_raised(&profile, lifted(ground(), from), DVec3::Z * height)
        .expect("the tool raises")
}

/// The bar of a T sunk four into a block forty by twenty: its back wall,
/// numbered four in the bar, where the stem stands.
fn bar() -> Contour {
    Contour::rectangle(DVec2::new(5.0, 5.0), DVec2::new(35.0, 9.0))
}

/// The stem of the T, standing on the middle of the bar's back wall.
fn stem() -> Contour {
    Contour::rectangle(DVec2::new(18.0, 9.0), DVec2::new(22.0, 15.0))
}

#[test]
fn a_tool_that_parts_a_face_of_its_own_names_the_part_s_edges_as_cutting_its_areas_one_by_one_does()
{
    let stock = block([40.0, 20.0, 10.0]);
    let drawn = part(&stock, ground());
    let tee = tool(&drawn, bar(), 6.0, 5.0)
        .union(&tool(&drawn, stem(), 6.0, 5.0))
        .expect("the bar and the stem join into one tool");
    let (bar_s_back, beside_the_stem) = (4, [10.0, 30.0]);
    let behind = |along: f64| [along, 30.0, 8.5];
    let piece = piece_apart(
        &tee,
        bar_s_back,
        beside_the_stem.map(behind),
        DVec3::NEG_Y,
        21.0,
    );
    assert_eq!(
        tee.parted_from(piece),
        Some(bar_s_back),
        "the stem leaves the bar's back wall in two pieces, the second parted \
         from it within the tool"
    );
    assert!(
        !(0..tee.faces_end()).contains(&piece),
        "the second piece answers to {piece}, which neither the bar nor the \
         stem gives, where their numbers end at {}",
        tee.faces_end()
    );
    let at_once = drawn.difference(&tee).expect("the T is cut at once");
    let wall = drawn.faces_end() + bar_s_back;
    let within = |along: f64| [along, 6.0, 8.5];
    let piece = piece_apart(&at_once, wall, beside_the_stem.map(within), DVec3::Y, 3.0);
    assert_eq!(
        at_once.parted_from(piece),
        Some(wall),
        "the piece is still parted from the bar's back wall, which answers to \
         its number moved above the block's"
    );
    assert!(
        !(0..at_once.faces_end()).contains(&piece),
        "the piece answers to {piece}, which no step gives, where the steps' \
         numbers end at {}",
        at_once.faces_end()
    );
    let number = u32::try_from(piece).expect("a face's number");
    let names = at_once.edge_names();
    assert!(
        names
            .iter()
            .all(|name| name.between.iter().all(|side| !side.contains(&number))),
        "the piece's edges are named by the wall it was parted from: {names:?}"
    );
    let mut one_by_one = stock;
    one_by_one.push(cut(bar(), 6.0, 5.0));
    one_by_one.push(cut(stem(), 6.0, 5.0));
    assert_eq!(
        names,
        part(&one_by_one, ground()).edge_names(),
        "as cutting the bar and then the stem names them"
    );
}

/// The block forty by twenty with a slot milled along it from its left side
/// to its right, behind where the T's stem ends: it leaves the top in two
/// pieces, and the T, cut after it, meets neither of its walls.
fn slotted_behind_the_stem() -> Vec<Raise> {
    let mut steps = block([40.0, 20.0, 10.0]);
    steps.push(cut(
        Contour::rectangle(DVec2::new(-1.0, 17.0), DVec2::new(41.0, 19.0)),
        7.0,
        4.0,
    ));
    steps
}

/// The T, its bar and its stem joined into one tool, raised against `part`.
fn tee(part: &Body) -> Body {
    tool(part, bar(), 6.0, 5.0)
        .union(&tool(part, stem(), 6.0, 5.0))
        .expect("the bar and the stem join into one tool")
}

/// The number of the top's piece the slot milled behind the stem left apart,
/// on whichever side of the slot it lies: met from above two along the
/// block, clear of the T, on both sides of the slot.
fn piece_of_the_top_behind_the_stem(body: &Body) -> usize {
    let above = |across: f64| [2.0, across, 20.0];
    piece_apart(body, 1, [above(10.0), above(19.5)], DVec3::NEG_Z, 10.0)
}

/// Where the bar's back wall stands along the bar, either side of the stem.
const BESIDE_THE_STEM: [f64; 2] = [10.0, 30.0];

/// The bar's back wall in a part the T was cut into, the numbers the steps
/// before the T gave ending at `before`, and the number its piece the stem
/// left apart answers to: met from inside the bar, either side of the stem.
fn bar_s_back_wall_and_its_piece(at_once: &Body, before: usize) -> (usize, usize) {
    let wall = before + 4;
    let within = |along: f64| [along, 6.0, 8.5];
    let piece = piece_apart(at_once, wall, BESIDE_THE_STEM.map(within), DVec3::Y, 3.0);
    (wall, piece)
}

#[test]
fn a_tool_s_piece_cut_into_a_part_that_holds_a_piece_answers_to_a_number_of_its_own_and_to_its_own_wall()
 {
    let stock = slotted_behind_the_stem();
    let slotted = part(&stock, ground());
    let top = 1;
    let strip = piece_of_the_top_behind_the_stem(&slotted);
    assert_eq!(
        slotted.parted_from(strip),
        Some(top),
        "the slot leaves the top in two pieces before the T is cut"
    );
    let tee = tee(&slotted);
    let bar_s_back = 4;
    let behind = |along: f64| [along, 30.0, 8.5];
    let piece = piece_apart(
        &tee,
        bar_s_back,
        BESIDE_THE_STEM.map(behind),
        DVec3::NEG_Y,
        21.0,
    );
    assert_eq!(
        tee.parted_from(piece),
        Some(bar_s_back),
        "the stem leaves the bar's back wall in two pieces within the tool"
    );

    let at_once = slotted.difference(&tee).expect("the T is cut at once");

    let strip = piece_of_the_top_behind_the_stem(&at_once);
    let (wall, piece) = bar_s_back_wall_and_its_piece(&at_once, slotted.faces_end());
    assert_eq!(
        at_once.parted_from(strip),
        Some(top),
        "the top's piece is still parted from the top"
    );
    assert_eq!(
        at_once.parted_from(piece),
        Some(wall),
        "the tool's piece is parted from the bar's back wall, which answers to \
         its number moved above the part's"
    );
    assert_eq!(
        piece,
        strip + 1,
        "the tool's piece is counted on after the part's own, the pieces \
         numbered one after the other across the part"
    );
    for (what, number) in [("the top's piece", strip), ("the tool's piece", piece)] {
        assert!(
            !(0..at_once.faces_end()).contains(&number),
            "{what} answers to {number}, which no step gives, where the steps' \
             numbers end at {}",
            at_once.faces_end()
        );
    }
    let mut one_by_one = stock;
    one_by_one.push(cut(bar(), 6.0, 5.0));
    one_by_one.push(cut(stem(), 6.0, 5.0));
    assert_eq!(
        at_once.edge_names(),
        part(&one_by_one, ground()).edge_names(),
        "as cutting the bar and then the stem after the slot names them"
    );
}

#[test]
fn a_face_parted_after_a_tool_brought_its_pieces_answers_to_the_number_after_every_piece_before_it()
{
    let slotted = part(&slotted_behind_the_stem(), ground());
    let at_once = slotted
        .difference(&tee(&slotted))
        .expect("the T is cut at once");
    let strip = piece_of_the_top_behind_the_stem(&at_once);
    let (wall, piece) = bar_s_back_wall_and_its_piece(&at_once, slotted.faces_end());
    let under = tool(
        &at_once,
        Contour::rectangle(DVec2::new(37.0, -1.0), DVec2::new(39.0, 21.0)),
        -1.0,
        3.0,
    );

    let cut_under = at_once
        .difference(&under)
        .expect("a slot milled across the floor cuts it");

    let floor = 0;
    let below = |along: f64| [along, 10.0, -10.0];
    let under_piece = piece_apart(
        &cut_under,
        floor,
        [below(20.0), below(39.5)],
        DVec3::Z,
        10.0,
    );
    assert_eq!(
        (
            piece_of_the_top_behind_the_stem(&cut_under),
            bar_s_back_wall_and_its_piece(&cut_under, slotted.faces_end()),
        ),
        (strip, (wall, piece)),
        "the earlier pieces keep their numbers"
    );
    for (what, number, from) in [
        ("the top's piece", strip, 1),
        ("the tool's piece", piece, wall),
        ("the floor's piece", under_piece, floor),
    ] {
        assert_eq!(
            cut_under.parted_from(number),
            Some(from),
            "{what}, {number}, is parted from its own face"
        );
    }
    assert_eq!(
        under_piece,
        piece + 1,
        "the floor's piece answers to the number after every piece parted \
         before it, the top's {strip} and the tool's {piece}"
    );
}
