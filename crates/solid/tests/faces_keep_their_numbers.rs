//! The exact kernel names its faces the way the flats do, so that a drawing
//! laid on a face finds it again (#526): a raise numbers its floor nought,
//! its top one and each wall after the run of the profile it stands on, the
//! outline's runs first and then the holes'; a boolean keeps, on each face it
//! hands back, the numbers of the faces of its operands it lies on, the
//! second operand's moved above the first's by whoever calls it.

use std::f64::consts::{PI, TAU};

use cao_solid::brep::{Body, FaceId, Surface};
use cao_solid::profile::{Contour, Frame, Run};
use cao_solid::turning::{Axis, Corner, Straight, Turn};
use glam::{DVec2, DVec3};

/// How far a face's plane may stand from where the arithmetic puts it.
const PLACE: f64 = 1e-9;

fn ground(height: f64) -> Frame {
    Frame {
        origin: DVec3::Z * height,
        u: DVec3::X,
        v: DVec3::Y,
    }
}

fn raised(outline: Contour, holes: &[Contour], from: f64, height: f64) -> Body {
    Body::raised(&outline, holes, ground(from), DVec3::Z * height).expect("a profile raises")
}

/// A box between two corners, numbered from nought.
fn block(low: [f64; 3], high: [f64; 3]) -> Body {
    raised(
        Contour::rectangle(DVec2::new(low[0], low[1]), DVec2::new(high[0], high[1])),
        &[],
        low[2],
        high[2] - low[2],
    )
}

/// A whole circle, as one run all the way round.
fn circle(center: [f64; 2], radius: f64) -> Contour {
    let center = DVec2::from(center);
    Contour {
        corners: vec![center + DVec2::X * radius],
        runs: vec![Run::Round { center, turn: TAU }],
    }
}

/// The numbers of every face lying on the plane through `point` whose matter
/// stands behind `outward`, in the body's order.
fn on_plane(body: &Body, point: DVec3, outward: DVec3) -> Vec<Vec<u32>> {
    body.face_ids()
        .filter(|&face| {
            let lying = body.face(face);
            let Surface::Plane(plane) = body.surface(lying.surface) else {
                return false;
            };
            let normal = if lying.flipped {
                -plane.normal
            } else {
                plane.normal
            };
            normal.dot(outward) > 1.0 - PLACE && plane.distance(point).abs() < PLACE
        })
        .map(|face| body.numbers(face).to_vec())
        .collect()
}

/// The numbers of every face lying on a cylinder, in the body's order.
fn on_cylinders(body: &Body) -> Vec<Vec<u32>> {
    body.face_ids()
        .filter(|&face| matches!(body.surface(body.face(face).surface), Surface::Cylinder(_)))
        .map(|face| body.numbers(face).to_vec())
        .collect()
}

#[test]
fn a_raised_block_numbers_its_floor_nought_its_top_one_and_its_walls_by_run() {
    let body = raised(
        Contour::rectangle(DVec2::ZERO, DVec2::new(30.0, 10.0)),
        &[circle([15.0, 5.0], 2.0)],
        0.0,
        5.0,
    );
    assert_eq!(on_plane(&body, DVec3::ZERO, -DVec3::Z), [[0]]);
    assert_eq!(on_plane(&body, DVec3::Z * 5.0, DVec3::Z), [[1]]);
    assert_eq!(on_plane(&body, DVec3::ZERO, -DVec3::Y), [[2]]);
    assert_eq!(on_plane(&body, DVec3::X * 30.0, DVec3::X), [[3]]);
    assert_eq!(on_plane(&body, DVec3::Y * 10.0, DVec3::Y), [[4]]);
    assert_eq!(on_plane(&body, DVec3::ZERO, -DVec3::X), [[5]]);
    assert_eq!(on_cylinders(&body), [[6]]);
    assert_eq!(body.face_ids().count(), 7);
}

/// A D: a straight floor, a half round bulging along X, a straight top and a
/// straight back.
fn d_shape() -> Contour {
    Contour {
        corners: vec![
            DVec2::ZERO,
            DVec2::new(10.0, 0.0),
            DVec2::new(10.0, 10.0),
            DVec2::new(0.0, 10.0),
        ],
        runs: vec![
            Run::Straight,
            Run::Round {
                center: DVec2::new(10.0, 5.0),
                turn: PI,
            },
            Run::Straight,
            Run::Straight,
        ],
    }
}

#[test]
fn a_profile_raised_backwards_numbers_its_faces_as_raised_forwards() {
    for (height, top) in [(4.0, DVec3::Z), (-4.0, -DVec3::Z)] {
        let body = raised(d_shape(), &[], 0.0, height);
        let label = format!("raised by {height}");
        assert_eq!(on_plane(&body, DVec3::ZERO, -top), [[0]], "{label}");
        assert_eq!(on_plane(&body, DVec3::Z * height, top), [[1]], "{label}");
        assert_eq!(on_plane(&body, DVec3::ZERO, -DVec3::Y), [[2]], "{label}");
        assert_eq!(on_cylinders(&body), [[3]], "{label}");
        assert_eq!(on_plane(&body, DVec3::Y * 10.0, DVec3::Y), [[4]], "{label}");
        assert_eq!(on_plane(&body, DVec3::ZERO, -DVec3::X), [[5]], "{label}");
    }
}

#[test]
fn a_profile_drawn_clockwise_numbers_its_walls_by_its_own_runs() {
    let mut drawn = Contour::rectangle(DVec2::ZERO, DVec2::new(30.0, 10.0));
    drawn.corners.reverse();
    let body = raised(drawn, &[], 0.0, 5.0);
    assert_eq!(on_plane(&body, DVec3::Y * 10.0, DVec3::Y), [[2]]);
    assert_eq!(on_plane(&body, DVec3::X * 30.0, DVec3::X), [[3]]);
    assert_eq!(on_plane(&body, DVec3::ZERO, -DVec3::Y), [[4]]);
    assert_eq!(on_plane(&body, DVec3::ZERO, -DVec3::X), [[5]]);
}

#[test]
fn two_runs_on_one_line_make_one_wall_answering_to_both() {
    let drawn = Contour::straight(vec![
        DVec2::new(15.0, 0.0),
        DVec2::new(30.0, 0.0),
        DVec2::new(30.0, 10.0),
        DVec2::new(20.0, 10.0),
        DVec2::new(0.0, 10.0),
        DVec2::ZERO,
    ]);
    let body = raised(drawn, &[], 0.0, 5.0);
    assert_eq!(on_plane(&body, DVec3::ZERO, -DVec3::Y), [[2, 7]]);
    assert_eq!(on_plane(&body, DVec3::X * 30.0, DVec3::X), [[3]]);
    assert_eq!(on_plane(&body, DVec3::Y * 10.0, DVec3::Y), [[4, 5]]);
    assert_eq!(on_plane(&body, DVec3::ZERO, -DVec3::X), [[6]]);
}

/// What the part's own counter gives the tool: the six numbers of a block.
const BLOCK: u32 = 6;

/// A block thirty long cut across by a trench ten wide, from its top down to
/// `floor`, the trench's numbers above the block's.
fn trenched(floor: f64) -> Body {
    let part = block([0.0, 0.0, 0.0], [30.0, 10.0, 10.0]);
    let tool = block([10.0, -1.0, floor], [20.0, 11.0, 11.0]).renumbered(BLOCK);
    part.cut_by(&tool).expect("the trench cuts")
}

#[test]
fn a_piece_of_a_face_a_cut_leaves_keeps_its_number() {
    let body = trenched(5.0);
    assert_eq!(on_plane(&body, DVec3::Z * 10.0, DVec3::Z), [[1], [1]]);
    assert_eq!(on_plane(&body, DVec3::ZERO, -DVec3::Z), [[0]]);
    assert_eq!(on_plane(&body, DVec3::ZERO, -DVec3::Y), [[2]]);
    assert_eq!(on_plane(&body, DVec3::X * 30.0, DVec3::X), [[3]]);
}

#[test]
fn the_pieces_of_a_face_a_cut_leaves_apart_are_told_apart_by_the_edges_they_share() {
    let body = trenched(5.0);
    let pieces = body.pieces_of(1);
    assert_eq!(pieces.len(), 2, "{pieces:?}");
    assert!(pieces.iter().all(|piece| piece.len() == 1), "{pieces:?}");
    assert_eq!(body.pieces_of(0).len(), 1);
    assert!(body.pieces_of(BLOCK * 2).is_empty());
    let pocketed = block([0.0, 0.0, 0.0], [30.0, 10.0, 10.0])
        .cut_by(&block([10.0, 2.0, 5.0], [20.0, 8.0, 11.0]).renumbered(BLOCK))
        .expect("the pocket cuts");
    assert_eq!(pocketed.pieces_of(1).len(), 1);
}

#[test]
fn a_piece_renamed_answers_to_its_new_name_alone() {
    let mut body = trenched(5.0);
    let pieces = body.pieces_of(1);
    body.rename(&pieces[1], 1, 40);
    assert_eq!(on_plane(&body, DVec3::Z * 10.0, DVec3::Z), [[1], [40]]);
    assert_eq!(body.pieces_of(1), pieces[..1]);
}

#[test]
fn a_tools_face_kept_in_the_result_keeps_its_number_above_the_parts() {
    let body = trenched(5.0);
    assert_eq!(on_plane(&body, DVec3::Z * 5.0, DVec3::Z), [[BLOCK]]);
    assert_eq!(on_plane(&body, DVec3::X * 10.0, DVec3::X), [[BLOCK + 5]]);
    assert_eq!(on_plane(&body, DVec3::X * 20.0, -DVec3::X), [[BLOCK + 3]]);
}

#[test]
fn two_coplanar_tops_joined_are_one_face_answering_to_both_first_operand_first() {
    let part = block([0.0, 0.0, 0.0], [20.0, 10.0, 10.0]);
    let tool = block([10.0, 0.0, 0.0], [30.0, 10.0, 10.0]).renumbered(BLOCK);
    let body = part.joined(&tool).expect("the blocks join");
    assert_eq!(on_plane(&body, DVec3::Z * 10.0, DVec3::Z), [[1, BLOCK + 1]]);
    assert_eq!(on_plane(&body, DVec3::ZERO, -DVec3::Z), [[0, BLOCK]]);
    assert_eq!(on_plane(&body, DVec3::ZERO, -DVec3::Y), [[2, BLOCK + 2]]);
    assert_eq!(on_plane(&body, DVec3::X * 30.0, DVec3::X), [[BLOCK + 3]]);
    assert_eq!(on_plane(&body, DVec3::ZERO, -DVec3::X), [[5]]);
    assert_eq!(body.face_ids().count(), 6);
}

/// The sizes a part is drawn at: its height, how deep a trench cuts it, the
/// radius of a boss on its top.
struct Sizes {
    height: f64,
    depth: f64,
    boss: f64,
}

/// A block trenched across, a boss joined on its top and a hole bored
/// through it, each step's numbers above the steps' before it, as a part's
/// counter hands them.
fn part(sizes: &Sizes) -> Body {
    let height = sizes.height;
    let mut next = 0;
    let mut numbered = |body: Body, count: u32| {
        let shifted = body.renumbered(next);
        next += count;
        shifted
    };
    let stock = numbered(block([0.0, 0.0, 0.0], [30.0, 10.0, height]), BLOCK);
    let trench = numbered(
        block(
            [10.0, -1.0, height - sizes.depth],
            [20.0, 11.0, height + 1.0],
        ),
        BLOCK,
    );
    let boss = numbered(raised(circle([5.0, 5.0], sizes.boss), &[], height, 4.0), 3);
    let hole = numbered(raised(circle([25.0, 5.0], 1.5), &[], -1.0, height + 2.0), 3);
    stock
        .cut_by(&trench)
        .and_then(|body| body.joined(&boss))
        .and_then(|body| body.cut_by(&hole))
        .expect("the part is built")
}

/// Each face's numbers beside the way it faces, sorted: what a drawing laid
/// on one finds it by, whatever its size.
fn names(body: &Body) -> Vec<(Vec<u32>, [i64; 3], bool)> {
    let mut names: Vec<(Vec<u32>, [i64; 3], bool)> = body
        .face_ids()
        .map(|face: FaceId| {
            let lying = body.face(face);
            let (facing, curved) = match body.surface(lying.surface) {
                Surface::Plane(plane) if lying.flipped => (-plane.normal, false),
                Surface::Plane(plane) => (plane.normal, false),
                Surface::Cylinder(cylinder) => (cylinder.axis, true),
                Surface::Cone(cone) => (cone.axis, true),
            };
            let facing = facing.round().as_i64vec3().to_array();
            (body.numbers(face).to_vec(), facing, curved)
        })
        .collect();
    names.sort();
    names
}

#[test]
fn the_numbers_of_a_parts_faces_do_not_move_when_a_size_changes() {
    let drawn = part(&Sizes {
        height: 10.0,
        depth: 5.0,
        boss: 2.0,
    });
    let resized = part(&Sizes {
        height: 14.0,
        depth: 3.0,
        boss: 3.5,
    });
    assert_eq!(names(&resized), names(&drawn));
    assert_eq!(
        on_plane(&drawn, DVec3::Z * 14.0, DVec3::Z),
        [[BLOCK * 2 + 1]]
    );
    assert_eq!(on_plane(&drawn, DVec3::Z * 10.0, DVec3::Z), [[1], [1]]);
}

#[test]
fn a_circle_handed_as_two_halves_makes_one_wall_answering_to_both() {
    let body = raised(Contour::circle(DVec2::ZERO, 10.0), &[], 0.0, 5.0);
    assert_eq!(on_cylinders(&body), [[2, 3]]);
}

/// A shaft of radius ten along Y, thirty long, its far end chamfered by
/// `chamfer`, turned whole: its runs number its foot nought, its wall one,
/// the chamfer two and its end three, the run on the axis none.
fn chamfered_shaft(chamfer: f64) -> Body {
    let corners = [
        [0.0, 0.0],
        [0.0, 10.0],
        [30.0 - chamfer, 10.0],
        [30.0, 10.0 - chamfer],
        [30.0, 0.0],
    ];
    let straight = Straight {
        side: 1.0,
        contours: vec![
            corners
                .iter()
                .zip(0..)
                .map(|(&at, run)| Corner {
                    at: DVec2::from(at),
                    run,
                })
                .collect(),
        ],
        runs: 5,
        last_off_the_axis: Some(3),
    };
    let along_y = Turn {
        axis: Axis {
            origin: DVec2::ZERO,
            direction: DVec2::X,
        },
        angle: TAU,
        resolution: 0.0,
        on_the_axis: 0.0,
    };
    let frame = Frame {
        origin: DVec3::ZERO,
        u: DVec3::Y,
        v: DVec3::X,
    };
    Body::turned(&straight, frame, &along_y).expect("the shaft turns")
}

#[test]
fn a_drawing_on_a_chamfered_shaft_s_shoulder_keeps_its_face_when_the_chamfer_grows() {
    let boss = |chamfer: f64| {
        let frame = Frame {
            origin: DVec3::Y * 30.0,
            u: DVec3::Z,
            v: DVec3::X,
        };
        let boss = Body::raised(&circle([0.0, 0.0], 3.0), &[], frame, DVec3::Y * 4.0)
            .expect("the boss raises")
            .renumbered(5);
        chamfered_shaft(chamfer)
            .joined(&boss)
            .expect("the boss joins the shaft's end")
    };
    let [small, large] = [1.0, 2.0].map(boss);
    assert_eq!(names(&large), names(&small));
    for body in [&small, &large] {
        assert_eq!(on_plane(body, DVec3::Y * 30.0, DVec3::Y), [[3]]);
        let chamfers: Vec<&[u32]> = body
            .face_ids()
            .filter(|&face| matches!(body.surface(body.face(face).surface), Surface::Cone(_)))
            .map(|face| body.numbers(face))
            .collect();
        assert_eq!(chamfers, [[2]]);
    }
}
