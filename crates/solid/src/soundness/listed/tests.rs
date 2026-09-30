use std::f64::consts::PI;

use glam::DVec3;

use super::*;
use crate::brep::{Circle, Curve, Cylinder, Line, ListedEdge, ListedFace, Plane, Surface};
use crate::soundness::{Flaw, Rule};

#[test]
fn a_listing_that_does_not_hold_is_a_flaw_of_its_own_rule() {
    let flaw = Flaw::from(Mislisted::Unused { edge: 3 });
    assert_eq!(flaw.rule(), Rule::Listed);
    assert!(flaw.is_like(&Flaw::from(Mislisted::Backwards { face: 0, lap: 1 })));
}

/// The faces of a listing in the order they use each edge, filled into every
/// edge's sides as a body would list them.
fn with_sides(mut listing: Listing) -> Listing {
    for edge in &mut listing.edges {
        edge.sides.clear();
    }
    for (face, listed) in listing.faces.iter().enumerate() {
        for &(edge, forward) in listed.loops.iter().flatten() {
            listing.edges[edge].sides.push((face, forward));
        }
    }
    listing
}

/// A box between two corners: a vertex per corner numbered by which of its
/// coordinates are high, an edge per pair of corners one axis apart running
/// from the low one to the high one, and six faces turning anticlockwise
/// seen from outside.
fn block(low: DVec3, high: DVec3) -> Listing {
    let corner = |bits: usize| {
        DVec3::new(
            if bits & 1 != 0 { high.x } else { low.x },
            if bits & 2 != 0 { high.y } else { low.y },
            if bits & 4 != 0 { high.z } else { low.z },
        )
    };
    let vertices: Vec<DVec3> = (0..8).map(corner).collect();
    let mut pairs = Vec::new();
    for bits in 0..8 {
        for axis in 0..3 {
            if bits & (1 << axis) == 0 {
                pairs.push((bits, bits | (1 << axis), axis));
            }
        }
    }
    let edges = pairs
        .iter()
        .map(|&(from, to, axis)| {
            let direction = DVec3::AXES[axis];
            let origin = vertices[from] - direction * direction.dot(vertices[from]);
            ListedEdge {
                curve: Curve::Line(Line { origin, direction }),
                from: direction.dot(vertices[from]),
                to: direction.dot(vertices[to]),
                ends: Some([from, to]),
                sides: Vec::new(),
            }
        })
        .collect();
    let edge_between = |one: usize, other: usize| {
        let (low, high) = (one.min(other), one.max(other));
        let rank = pairs
            .iter()
            .position(|&(from, to, _)| (from, to) == (low, high))
            .expect("an edge between two corners one axis apart");
        (rank, one == low)
    };
    let mut faces = Vec::new();
    for axis in 0..3 {
        let (first, second) = (1 << ((axis + 1) % 3), 1 << ((axis + 2) % 3));
        for high_side in [false, true] {
            let base = if high_side { 1 << axis } else { 0 };
            let mut around = [base, base | first, base | first | second, base | second];
            if !high_side {
                around.reverse();
            }
            let point = if high_side { high } else { low };
            let (plane, _) = Plane::through(point, DVec3::AXES[axis]);
            faces.push(ListedFace {
                surface: Surface::Plane(plane),
                outward: high_side,
                loops: vec![
                    (0..4)
                        .map(|index| edge_between(around[index], around[(index + 1) % 4]))
                        .collect(),
                ],
            });
        }
    }
    with_sides(Listing {
        faces,
        edges,
        vertices,
    })
}

/// A round stock standing on the XY plane: a disc on top, a disc underneath,
/// and the wall between, bounded by two whole circles and no vertex.
fn round(center: DVec3, radius: f64, height: f64) -> Listing {
    let cylinder = Cylinder::about(center, DVec3::Z, radius);
    let rim = |h: f64| ListedEdge {
        curve: Curve::Circle(Circle::on(&cylinder, h)),
        from: -PI,
        to: PI,
        ends: None,
        sides: Vec::new(),
    };
    let disc = |h: f64| Plane::through(center + DVec3::Z * h, DVec3::Z).0;
    with_sides(Listing {
        faces: vec![
            ListedFace {
                surface: Surface::Plane(disc(height)),
                outward: true,
                loops: vec![vec![(1, true)]],
            },
            ListedFace {
                surface: Surface::Plane(disc(0.0)),
                outward: false,
                loops: vec![vec![(0, false)]],
            },
            ListedFace {
                surface: Surface::Cylinder(cylinder),
                outward: true,
                loops: vec![vec![(0, true)], vec![(1, false)]],
            },
        ],
        edges: vec![rim(0.0), rim(height)],
        vertices: Vec::new(),
    })
}

/// A box with a round hole bored through it from its top to its bottom.
fn bored(low: DVec3, high: DVec3, center: DVec3, radius: f64) -> Listing {
    let mut listing = block(low, high);
    let cylinder = Cylinder::about(center, DVec3::Z, radius);
    let rim = |h: f64| ListedEdge {
        curve: Curve::Circle(Circle::on(&cylinder, h)),
        from: -PI,
        to: PI,
        ends: None,
        sides: Vec::new(),
    };
    let (bottom, top) = (listing.edges.len(), listing.edges.len() + 1);
    listing.edges.push(rim(low.z));
    listing.edges.push(rim(high.z));
    listing.faces[4].loops.push(vec![(bottom, true)]);
    listing.faces[5].loops.push(vec![(top, false)]);
    listing.faces.push(ListedFace {
        surface: Surface::Cylinder(cylinder),
        outward: false,
        loops: vec![vec![(top, true)], vec![(bottom, false)]],
    });
    with_sides(listing)
}

fn plate() -> Listing {
    bored(
        DVec3::new(-10.0, -8.0, 0.0),
        DVec3::new(10.0, 8.0, 5.0),
        DVec3::new(2.0, 1.0, 0.0),
        3.0,
    )
}

/// The same listing with every loop run the other way round.
fn turned_round(mut listing: Listing) -> Listing {
    for face in &mut listing.faces {
        for uses in &mut face.loops {
            uses.reverse();
            for (_, forward) in uses.iter_mut() {
                *forward = !*forward;
            }
        }
    }
    with_sides(listing)
}

fn stock() -> Listing {
    round(DVec3::new(3.0, -2.0, 0.0), 5.0, 10.0)
}

fn brick() -> Listing {
    block(DVec3::new(-1.0, 2.0, 0.5), DVec3::new(4.0, 3.5, 7.0))
}

const REACH: f64 = 10.0;

#[test]
fn a_block_listed_by_hand_keeps_every_rule_of_a_listing() {
    assert_eq!(listed(&brick(), REACH), Ok(()));
}

#[test]
fn a_round_stock_listed_by_hand_keeps_every_rule_of_a_listing() {
    assert_eq!(listed(&stock(), REACH), Ok(()));
}

#[test]
fn a_loop_naming_an_edge_the_listing_does_not_hold_is_found() {
    let mut listing = brick();
    listing.faces[2].loops[0][1].0 = 12;
    assert_eq!(
        listed(&listing, REACH),
        Err(Mislisted::NoSuchEdge { face: 2, edge: 12 })
    );
}

#[test]
fn an_edge_naming_a_face_its_loops_do_not_is_found() {
    let mut listing = brick();
    listing.edges[4].sides.push((5, true));
    assert_eq!(listed(&listing, REACH), Err(Mislisted::Sides { edge: 4 }));
}

#[test]
fn a_use_turned_round_runs_twice_one_way_along_its_edge() {
    let mut listing = brick();
    let (edge, forward) = listing.faces[3].loops[0][2];
    listing.faces[3].loops[0][2].1 = !forward;
    let listing = with_sides(listing);
    let (one_way, other_way) = if forward { (0, 2) } else { (2, 0) };
    assert_eq!(
        listed(&listing, REACH),
        Err(Mislisted::Unbalanced {
            edge,
            forward: one_way,
            backward: other_way
        })
    );
}

#[test]
fn a_use_gone_from_its_loop_leaves_its_edge_one_way() {
    let mut listing = brick();
    let (edge, _) = listing.faces[1].loops[0].remove(3);
    let listing = with_sides(listing);
    let found = listed(&listing, REACH);
    assert!(
        matches!(found, Err(Mislisted::Unbalanced { edge: at, .. }) if at == edge),
        "{found:?}"
    );
}

#[test]
fn an_edge_no_face_uses_is_found() {
    let mut listing = brick();
    let spare = listing.edges[0].clone();
    listing.edges.push(ListedEdge {
        sides: Vec::new(),
        ..spare
    });
    assert_eq!(listed(&listing, REACH), Err(Mislisted::Unused { edge: 12 }));
}

#[test]
fn two_uses_swapped_in_their_loop_leave_it_open() {
    let mut listing = brick();
    listing.faces[4].loops[0].swap(1, 2);
    assert_eq!(
        listed(&listing, REACH),
        Err(Mislisted::Unclosed {
            face: 4,
            lap: 0,
            at: 1
        })
    );
}

#[test]
fn a_face_bounded_by_no_loop_is_found() {
    let mut listing = brick();
    listing.faces.push(ListedFace {
        surface: listing.faces[0].surface,
        outward: true,
        loops: Vec::new(),
    });
    assert_eq!(
        listed(&listing, REACH),
        Err(Mislisted::Unbounded { face: 6 })
    );
}

#[test]
fn a_whole_circle_sharing_its_loop_leaves_the_loop_open() {
    let mut listing = stock();
    listing.faces[2].loops = vec![vec![(0, true), (1, false)]];
    assert_eq!(
        listed(&listing, REACH),
        Err(Mislisted::Unclosed {
            face: 2,
            lap: 0,
            at: 0
        })
    );
}

/// The round stock with its top rim closed on one vertex rather than none.
fn stock_with_a_seam() -> Listing {
    let mut listing = stock();
    let Curve::Circle(rim) = listing.edges[1].curve else {
        unreachable!("the top rim of the stock is a circle")
    };
    listing.vertices = vec![rim.point(0.3)];
    listing.edges[1].ends = Some([0, 0]);
    listing.edges[1].from = 0.3;
    listing.edges[1].to = 0.3 + std::f64::consts::TAU;
    listing
}

#[test]
fn a_whole_circle_closed_on_one_vertex_keeps_every_rule_of_a_listing() {
    assert_eq!(listed(&stock_with_a_seam(), REACH), Ok(()));
}

#[test]
fn a_corner_moved_off_its_planes_is_found_off_the_faces_around_it() {
    let mut listing = brick();
    listing.vertices[7].z += 1e-6;
    let found = listed(&listing, REACH);
    assert!(
        matches!(
            found,
            Err(Mislisted::VertexOffFace {
                vertex: 7,
                face: 5,
                ..
            })
        ),
        "{found:?}"
    );
}

#[test]
fn a_corner_that_is_no_number_is_found_off_the_faces_around_it() {
    let mut listing = brick();
    listing.vertices[7] = DVec3::NAN;
    let found = listed(&listing, REACH);
    assert!(
        matches!(found, Err(Mislisted::VertexOffFace { vertex: 7, .. })),
        "{found:?}"
    );
}

#[test]
fn an_edge_along_a_line_with_no_direction_is_found_away_from_its_ends() {
    let mut listing = brick();
    let Curve::Line(line) = &mut listing.edges[0].curve else {
        unreachable!("the edges of a block are lines")
    };
    *line = Line::through(line.origin, DVec3::ZERO);
    let found = listed(&listing, REACH);
    assert!(
        matches!(found, Err(Mislisted::EndAway { edge: 0, .. })),
        "{found:?}"
    );
}

#[test]
fn a_face_on_a_plane_that_is_no_number_is_found_off_its_corners() {
    let mut listing = brick();
    let Surface::Plane(side) = &mut listing.faces[3].surface else {
        unreachable!("the faces of a block are planes")
    };
    side.normal = DVec3::NAN;
    let found = listed(&listing, REACH);
    assert!(
        matches!(found, Err(Mislisted::VertexOffFace { face: 3, .. })),
        "{found:?}"
    );
}

#[test]
fn a_corner_moved_by_less_than_a_billionth_of_the_reach_is_still_on_its_faces() {
    let mut listing = brick();
    listing.vertices[7].z += 0.5e-9 * REACH;
    assert_eq!(listed(&listing, REACH), Ok(()));
}

#[test]
fn a_vertex_slid_along_its_circle_is_found_away_from_the_end_of_its_edge() {
    let mut listing = stock_with_a_seam();
    let Curve::Circle(rim) = listing.edges[1].curve else {
        unreachable!("the top rim of the stock is a circle")
    };
    listing.vertices[0] = rim.point(0.3 + 1e-3);
    let found = listed(&listing, REACH);
    assert!(
        matches!(
            found,
            Err(Mislisted::EndAway {
                edge: 1,
                end: 0,
                vertex: 0,
                ..
            })
        ),
        "{found:?}"
    );
}

#[test]
fn a_rim_left_below_its_disc_is_found_off_the_face() {
    let mut listing = stock();
    let Surface::Plane(top) = &mut listing.faces[0].surface else {
        unreachable!("the top of the stock is a plane")
    };
    top.origin.z += 1e-6;
    let found = listed(&listing, REACH);
    assert!(
        matches!(
            found,
            Err(Mislisted::OffFace {
                edge: 1,
                face: 0,
                ..
            })
        ),
        "{found:?}"
    );
}

#[test]
fn a_block_bored_through_listed_by_hand_keeps_every_rule_of_a_listing() {
    assert_eq!(listed(&plate(), REACH), Ok(()));
}

/// A block whose top holds a slit: a line inside it that a cylinder lying on
/// the top touches, run once each way by the top's own loop.
fn block_with_a_slit() -> Listing {
    let mut listing = block(DVec3::ZERO, DVec3::splat(10.0));
    let first = listing.vertices.len();
    listing.vertices.push(DVec3::new(3.0, 5.0, 10.0));
    listing.vertices.push(DVec3::new(7.0, 5.0, 10.0));
    listing.edges.push(ListedEdge {
        curve: Curve::Line(Line {
            origin: DVec3::new(0.0, 5.0, 10.0),
            direction: DVec3::X,
        }),
        from: 3.0,
        to: 7.0,
        ends: Some([first, first + 1]),
        sides: Vec::new(),
    });
    let slit = listing.edges.len() - 1;
    listing.faces[5]
        .loops
        .push(vec![(slit, true), (slit, false)]);
    with_sides(listing)
}

#[test]
fn a_slit_run_once_each_way_inside_a_face_is_no_hole_turned_the_wrong_way() {
    assert_eq!(listed(&block_with_a_slit(), 10.0), Ok(()));
}

#[test]
fn a_block_with_every_loop_turned_round_keeps_its_faces_on_its_right() {
    assert_eq!(
        listed(&turned_round(brick()), REACH),
        Err(Mislisted::Backwards { face: 0, lap: 0 })
    );
}

#[test]
fn a_round_stock_with_every_loop_turned_round_keeps_its_faces_on_its_right() {
    assert_eq!(
        listed(&turned_round(stock()), REACH),
        Err(Mislisted::Backwards { face: 0, lap: 0 })
    );
}

#[test]
fn a_face_said_to_look_into_its_matter_is_found_running_backwards() {
    let mut listing = brick();
    listing.faces[2].outward = !listing.faces[2].outward;
    assert_eq!(
        listed(&listing, REACH),
        Err(Mislisted::Backwards { face: 2, lap: 0 })
    );
}

#[test]
fn a_hole_turned_the_same_way_as_the_rim_around_it_is_found() {
    let mut listing = plate();
    listing.faces[5].loops[1][0].1 = true;
    listing.faces[6].loops[0][0].1 = false;
    assert_eq!(
        listed(&with_sides(listing), REACH),
        Err(Mislisted::Backwards { face: 5, lap: 1 })
    );
}

#[test]
fn a_bore_whose_wall_runs_round_one_way_at_both_ends_is_found() {
    let mut listing = plate();
    listing.faces[4].loops[1][0].1 = false;
    listing.faces[6].loops[1][0].1 = true;
    let found = listed(&with_sides(listing), REACH);
    assert!(
        matches!(found, Err(Mislisted::Backwards { face: 4, lap: 1 })),
        "{found:?}"
    );
}

#[test]
fn an_edge_with_no_vertex_short_of_a_whole_turn_is_found() {
    let mut listing = stock();
    listing.edges[1].to = 0.0;
    assert_eq!(listed(&listing, REACH), Err(Mislisted::Endless { edge: 1 }));
}

/// Half a round stock, the half on the negative side of X: its arcs run from
/// a quarter turn to three quarters, across the angle where a turn is read
/// again from minus a half, and its round wall is a piece of cylinder bounded
/// by two arcs and two rulings.
fn half_round() -> Listing {
    let (radius, height) = (2.0, 3.0);
    let cylinder = Cylinder::about(DVec3::ZERO, DVec3::Z, radius);
    let vertices = vec![
        DVec3::new(0.0, -radius, 0.0),
        DVec3::new(0.0, radius, 0.0),
        DVec3::new(0.0, -radius, height),
        DVec3::new(0.0, radius, height),
    ];
    let arc = |h: f64, ends: [usize; 2]| ListedEdge {
        curve: Curve::Circle(Circle::on(&cylinder, h)),
        from: PI / 2.0,
        to: 3.0 * PI / 2.0,
        ends: Some(ends),
        sides: Vec::new(),
    };
    let line = |from: usize, to: usize| {
        let direction = (vertices[to] - vertices[from]).normalize();
        let line = Line::through(vertices[from], direction);
        ListedEdge {
            curve: Curve::Line(line),
            from: line.parameter(vertices[from]),
            to: line.parameter(vertices[to]),
            ends: Some([from, to]),
            sides: Vec::new(),
        }
    };
    let edges = vec![
        arc(0.0, [1, 0]),
        arc(height, [3, 2]),
        line(0, 1),
        line(2, 3),
        line(0, 2),
        line(1, 3),
    ];
    let plane = |point: DVec3, normal: DVec3| Surface::Plane(Plane::through(point, normal).0);
    let face = |surface: Surface, outward: bool, uses: Vec<(usize, bool)>| ListedFace {
        surface,
        outward,
        loops: vec![uses],
    };
    with_sides(Listing {
        faces: vec![
            face(
                plane(DVec3::Z * height, DVec3::Z),
                true,
                vec![(1, true), (3, true)],
            ),
            face(
                plane(DVec3::ZERO, DVec3::Z),
                false,
                vec![(2, false), (0, false)],
            ),
            face(
                plane(DVec3::ZERO, DVec3::X),
                true,
                vec![(2, true), (5, true), (3, false), (4, false)],
            ),
            face(
                Surface::Cylinder(cylinder),
                true,
                vec![(0, true), (4, true), (1, false), (5, false)],
            ),
        ],
        edges,
        vertices,
    })
}

#[test]
fn half_a_round_stock_whose_arcs_run_across_half_a_turn_keeps_every_rule_of_a_listing() {
    assert_eq!(listed(&half_round(), REACH), Ok(()));
}

#[test]
fn an_arc_listed_round_the_other_half_of_its_circle_is_found_running_backwards() {
    let mut listing = half_round();
    listing.edges[0].to = -PI / 2.0;
    let found = listed(&listing, REACH);
    assert!(
        matches!(found, Err(Mislisted::Backwards { .. })),
        "{found:?}"
    );
}

/// A block whose top holds a sliver `thickness` thick, a face of its own on
/// the top's plane: both its loops turned the wrong way, the top's hole round
/// it anticlockwise and the sliver's own clockwise.
fn block_with_a_sliver(thickness: f64) -> Listing {
    let mut listing = block(DVec3::ZERO, DVec3::splat(10.0));
    let first = listing.vertices.len();
    let corners = [
        DVec3::new(3.0, 5.0, 10.0),
        DVec3::new(7.0, 5.0, 10.0),
        DVec3::new(5.0, 5.0 + thickness, 10.0),
    ];
    listing.vertices.extend(corners);
    for (rank, &from) in corners.iter().enumerate() {
        let to = corners[(rank + 1) % 3];
        let line = Line::through(from, to - from);
        listing.edges.push(ListedEdge {
            curve: Curve::Line(line),
            from: line.parameter(from),
            to: line.parameter(to),
            ends: Some([first + rank, first + (rank + 1) % 3]),
            sides: Vec::new(),
        });
    }
    let sides = listing.edges.len() - 3;
    let round: Vec<(usize, bool)> = (sides..sides + 3).map(|edge| (edge, true)).collect();
    listing.faces[5].loops.push(round.clone());
    listing.faces.push(ListedFace {
        loops: vec![round.iter().rev().map(|&(edge, _)| (edge, false)).collect()],
        ..listing.faces[5].clone()
    });
    with_sides(listing)
}

/// Every place of a listing may stand a billionth of the reach off where
/// its geometry says, so a loop's area is known only to that much times its
/// length: a loop sweeping less turns neither way that can be read. Seeds
/// 2790, 4901 and 10495 of the exact campaigns left such a loop, turned the
/// right way, round a sliver between a side and a circle tangent to it, and
/// rounding read it backwards.
#[test]
fn a_sliver_thinner_than_a_listing_can_be_read_is_turned_neither_way_and_a_thicker_one_is() {
    assert_eq!(listed(&block_with_a_sliver(1e-12), 10.0), Ok(()));
    assert_eq!(
        listed(&block_with_a_sliver(1e-3), 10.0),
        Err(Mislisted::Backwards { face: 5, lap: 1 })
    );
}

#[test]
fn half_a_round_stock_with_every_loop_turned_round_keeps_its_faces_on_its_right() {
    let found = listed(&turned_round(half_round()), REACH);
    assert!(
        matches!(found, Err(Mislisted::Backwards { .. })),
        "{found:?}"
    );
}
