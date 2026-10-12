//! Decision 11 (#552): a plane of one operand leaning a hair off square to,
//! or along, the axis of a wall of the other is laid exactly so, the second
//! operand turned rigidly about where the two meet, where the turn moves
//! none of it further than `Scale::HAIR` tolerances.

use glam::DQuat;

use super::*;
use crate::brep::canonical::squared::kept;

fn degrees(angle: f64) -> f64 {
    angle.to_radians()
}

/// The body turned about the middle of its box.
fn leaning(body: Body, turn: DQuat) -> Body {
    let (low, high) = body.bounds().expect("a body has a box");
    body.rotated((low + high) / 2.0, turn, DVec3::ZERO, &[], 0.0)
        .expect("a hair of a turn is made")
}

/// A post about the Z axis through `center`, from `low` to `high`.
fn post(center: [f64; 2], radius: f64, low: f64, high: f64) -> Body {
    let outline = Contour::circle(DVec2::from(center), radius);
    Body::raised(&outline, &[], ground(low), DVec3::Z * (high - low)).expect("a post raises")
}

/// Half a shaft of radius five along X, from -20 to 20, its flat side on
/// the plane z = 0 and its wall above it.
fn half_shaft() -> Body {
    let outline = Contour {
        corners: vec![DVec2::new(-5.0, 0.0), DVec2::new(5.0, 0.0)],
        runs: vec![
            Run::Straight,
            Run::Round {
                center: DVec2::ZERO,
                turn: PI,
            },
        ],
    };
    let frame = Frame {
        origin: DVec3::new(-20.0, 0.0, 0.0),
        u: DVec3::Y,
        v: DVec3::Z,
    };
    Body::raised(&outline, &[], frame, DVec3::X * 40.0).expect("half a shaft raises")
}

/// A point turned about Y: its base of radius five on the plane y = 0, its
/// apex at (0, 10, 0).
fn point() -> Body {
    use crate::turning::{Axis, Corner, Straight, Turn};
    let corners = [[0.0, 0.0], [10.0, 0.0], [0.0, 5.0]];
    let straight = Straight {
        side: -1.0,
        contours: vec![
            (0..corners.len())
                .map(|run| Corner {
                    at: DVec2::from(corners[run]),
                    run: run as u32,
                })
                .collect(),
        ],
        runs: corners.len() as u32,
        last_off_the_axis: Some(2),
    };
    let turn = Turn {
        axis: Axis {
            origin: DVec2::ZERO,
            direction: DVec2::Y,
        },
        angle: TAU,
        resolution: 0.0,
        on_the_axis: 0.0,
    };
    Body::turned(&straight, ground(0.0), &turn).expect("the point turns")
}

fn directions(body: &Body) -> Vec<DVec3> {
    body.surfaces
        .iter()
        .map(|surface| match surface {
            Surface::Plane(plane) => plane.normal,
            Surface::Cylinder(cylinder) => cylinder.axis,
            Surface::Cone(cone) => cone.axis,
        })
        .collect()
}

fn planes(body: &Body) -> Vec<Plane> {
    body.surfaces
        .iter()
        .filter_map(|surface| match surface {
            Surface::Plane(plane) => Some(*plane),
            Surface::Cylinder(_) | Surface::Cone(_) => None,
        })
        .collect()
}

/// Whether every direction of the body is the axis's own bits, or square
/// to it but for rounding.
fn square_or_along(body: &Body, axis: DVec3) -> bool {
    directions(body)
        .iter()
        .all(|direction| *direction == axis || direction.dot(axis).abs() <= 1e-15)
}

#[test]
fn a_block_leaning_a_ten_millionth_of_a_degree_off_square_to_a_post_is_turned_square_to_it() {
    let body = post([0.0, 0.0], 5.0, 0.0, 10.0);
    let tool = leaning(
        block([-10.0, -10.0, 8.0], [10.0, 10.0, 12.0]),
        DQuat::from_rotation_x(degrees(1e-7)),
    );
    assert!(!square_or_along(&tool, DVec3::Z));
    let squared = squared(&body, &tool, scale()).expect("the block is turned");
    assert!(square_or_along(&squared, DVec3::Z));
}

#[test]
fn a_plane_a_hair_off_along_a_bore_s_axis_is_laid_along_it() {
    let body = block([-10.0, -10.0, 0.0], [10.0, 10.0, 10.0])
        .cut_by(&post([0.0, 0.0], 5.0, 0.0, 10.0))
        .expect("the block is bored");
    let tool = leaning(
        block([3.0, -20.0, -5.0], [20.0, 20.0, 15.0]),
        DQuat::from_rotation_y(degrees(4e-8)),
    );
    let squared = squared(&body, &tool, scale()).expect("the side is turned");
    let [wall] = body
        .surfaces
        .iter()
        .filter(|surface| matches!(surface, Surface::Cylinder(_)))
        .collect::<Vec<_>>()[..]
    else {
        panic!("one bore");
    };
    let side = planes(&squared)
        .into_iter()
        .find(|plane| plane.normal.x > 0.5 && (plane.offset() - 3.0).abs() < 1e-6)
        .expect("a side through the bore");
    assert!(side.normal.dot(DVec3::Z).abs() <= 1e-15);
    assert!(matches!(
        relation(wall, &Surface::Plane(side), scale()),
        Relation::Lines(_)
    ));
}

#[test]
fn a_plane_a_hair_off_square_to_a_cone_s_axis_is_laid_square_to_it() {
    let body = chamfered_shaft();
    let cone = *body
        .surfaces
        .iter()
        .find(|surface| matches!(surface, Surface::Cone(_)))
        .expect("the chamfer is a cone");
    assert!(matches!(cone, Surface::Cone(Cone { axis, .. }) if axis == DVec3::Y));
    let tool = leaning(
        block([-40.0, 9.0, -40.0], [40.0, 20.0, 40.0]),
        DQuat::from_rotation_x(degrees(1e-7)),
    );
    let squared = squared(&body, &tool, scale()).expect("the block is turned");
    let floor = planes(&squared)
        .into_iter()
        .find(|plane| plane.normal == DVec3::Y)
        .expect("a floor square to the cone's axis");
    assert!(matches!(
        relation(&cone, &Surface::Plane(floor), scale()),
        Relation::Circle(_)
    ));
}

#[test]
fn a_lean_that_would_move_the_tool_further_than_the_hair_is_left_as_drawn() {
    let body = post([0.0, 0.0], 5.0, 0.0, 10.0);
    let tool = leaning(
        block([-10.0, -10.0, 8.0], [10.0, 10.0, 12.0]),
        DQuat::from_rotation_x(degrees(1e-5)),
    );
    assert_eq!(squared(&body, &tool, scale()), None);
}

#[test]
fn a_tool_exactly_square_or_along_the_walls_it_meets_is_not_turned() {
    let body = post([0.0, 0.0], 5.0, 0.0, 10.0);
    let tool = block([-10.0, 3.0, 8.0], [10.0, 10.0, 12.0]);
    assert_eq!(squared(&body, &tool, scale()), None);
}

#[test]
fn a_tool_lying_on_a_face_of_the_body_is_turned_about_that_face_s_normal_only() {
    let body = half_shaft();
    let tool = leaning(
        block([-10.0, -10.0, -5.0], [10.0, 10.0, 0.0]),
        DQuat::from_rotation_z(degrees(1e-7)),
    );
    let top = |body: &Body| {
        planes(body)
            .into_iter()
            .find(|plane| plane.normal == DVec3::Z && plane.offset() > -1.0)
            .expect("a top on the flat side")
    };
    let before = top(&tool);
    let squared = squared(&body, &tool, scale()).expect("the block is turned");
    assert!((top(&squared).offset() - before.offset()).abs() <= 1e-15);
    assert!(directions(&squared).contains(&DVec3::X));
    assert!(square_or_along(&squared, DVec3::Z));
}

#[test]
fn a_tool_whose_leans_no_one_turn_lays_exactly_is_left_as_drawn() {
    let turn = DQuat::from_rotation_x(degrees(1e-7));
    let far = leaning(block([20.0, -10.0, 0.0], [30.0, 10.0, 10.0]), turn);
    let body = post([0.0, 0.0], 5.0, 0.0, 10.0)
        .joined(&far)
        .expect("a post and a block far off join");
    let tool = leaning(block([-10.0, -10.0, 8.0], [30.0, 10.0, 12.0]), turn);
    assert_eq!(squared(&body, &tool, scale()), None);
}

#[test]
fn a_turn_that_would_take_a_wall_off_a_plane_it_touches_is_not_made() {
    let body = block([-10.0, -10.0, 0.0], [10.0, 10.0, 10.0]);
    let tool = post([15.0, 0.0], 5.0, 0.0, 10.0);
    let off = tool
        .rotated(
            DVec3::new(15.0, 0.0, 1000.0),
            DQuat::from_rotation_y(degrees(1e-8)),
            DVec3::ZERO,
            &[],
            0.0,
        )
        .expect("a hair of a turn is made");
    assert!(!kept(&body, &tool, &off, scale()));
    assert!(kept(&body, &tool, &tool, scale()));
}

#[test]
fn a_turn_that_would_lay_a_plane_a_hair_beside_a_cone_s_apex_is_not_made() {
    let body = point();
    let tool = block([0.0, -5.0, -5.0], [10.0, 15.0, 5.0]);
    let off_the_apex = |angle: f64| {
        tool.rotated(
            DVec3::new(0.0, 1000.0, 0.0),
            DQuat::from_rotation_z(angle),
            DVec3::ZERO,
            &[],
            0.0,
        )
        .expect("a hair of a turn is made")
    };
    let leaning = off_the_apex(1e-9);
    assert!(!kept(&body, &leaning, &off_the_apex(3e-11), scale()));
    assert!(kept(&body, &leaning, &tool, scale()));
}

#[test]
fn a_tool_turned_twice_from_the_same_operands_has_the_same_bits() {
    let body = post([0.0, 0.0], 5.0, 0.0, 10.0);
    let tool = leaning(
        block([-10.0, -10.0, 8.0], [10.0, 10.0, 12.0]),
        DQuat::from_rotation_x(degrees(1e-7)),
    );
    assert_eq!(
        squared(&body, &tool, scale()),
        squared(&body, &tool, scale())
    );
}

#[test]
fn a_lean_of_a_ten_millionth_of_a_degree_is_turned_by_that_angle_and_not_by_none() {
    let body = post([0.0, 0.0], 5.0, 0.0, 12.0);
    let drawn = block([-10.0, -10.0, 8.0], [10.0, 10.0, 12.0]);
    let tool = leaning(drawn.clone(), DQuat::from_rotation_x(degrees(1e-7)));
    let squared = squared(&body, &tool, scale()).expect("the block is turned");
    let off = |body: &Body| {
        body.vertices
            .iter()
            .zip(&drawn.vertices)
            .map(|(one, other)| one.point.distance(other.point))
            .fold(0.0, f64::max)
    };
    assert!(off(&tool) > 1e-9, "the block leans by {}", off(&tool));
    assert!(
        off(&squared) < 1e-12,
        "the block stands {} off square",
        off(&squared)
    );
}

#[test]
fn a_floor_laid_square_to_a_leaning_post_lies_exactly_on_the_end_it_was_drawn_on() {
    let body = leaning(
        post([0.0, 0.0], 5.0, 0.0, 10.0),
        DQuat::from_rotation_x(degrees(1e-7)),
    );
    let tool = block([-10.0, 2.0, 10.0], [10.0, 30.0, 14.0]);
    let end = planes(&body)
        .into_iter()
        .find(|plane| plane.offset() > 5.0)
        .expect("the post's upper end");
    let squared = squared(&body, &tool, scale()).expect("the block is turned");
    let floor = planes(&squared)
        .into_iter()
        .find(|plane| (plane.offset().abs() - 10.0).abs() < 1e-6)
        .expect("the block's floor");
    let apart = floor.offset() * floor.normal.dot(end.normal).signum() - end.offset();
    assert!(apart.abs() < 1e-12, "the floor stands {apart} off the end");
}

#[test]
fn a_side_laid_along_a_leaning_post_keeps_its_distance_to_the_axis_halfway_up_where_they_meet() {
    let body = leaning(
        post([0.0, 0.0], 5.0, 0.0, 10.0),
        DQuat::from_rotation_x(degrees(1e-7)),
    );
    let tool = block([-10.0, 3.0, 2.0], [10.0, 20.0, 30.0]);
    let Some(Surface::Cylinder(wall)) = body
        .surfaces
        .iter()
        .find(|surface| matches!(surface, Surface::Cylinder(_)))
    else {
        panic!("the post has a wall");
    };
    let from_the_axis = |plane: &Plane, height: f64| {
        let on_the_axis = wall.origin + wall.axis * (height - wall.origin.z) / wall.axis.z;
        plane.distance(on_the_axis).abs()
    };
    let side = |body: &Body| {
        planes(body)
            .into_iter()
            .find(|plane| plane.normal.y.abs() > 0.5 && (plane.offset().abs() - 3.0).abs() < 1e-6)
            .expect("the block's side by the post")
    };
    let squared = squared(&body, &tool, scale()).expect("the block is turned");
    let drawn = from_the_axis(&side(&tool), 6.0);
    let laid = from_the_axis(&side(&squared), 6.0);
    assert!(
        (laid - drawn).abs() < 1e-12,
        "the side moved {} off the axis",
        laid - drawn
    );
}

#[test]
fn a_face_laid_along_a_leaning_half_shaft_holds_the_edge_of_its_flat_side_it_was_drawn_through() {
    let body = leaning(half_shaft(), DQuat::from_rotation_z(degrees(1e-7)));
    let tool = block([0.0, 5.0, -5.0], [30.0, 20.0, 5.0]);
    let edge: Vec<DVec3> = body
        .vertices
        .iter()
        .map(|vertex| vertex.point)
        .filter(|point| point.y > 4.0 && point.z.abs() < 1e-6)
        .collect();
    assert_eq!(edge.len(), 2, "the edge of the flat side at y = 5");
    let squared = squared(&body, &tool, scale()).expect("the block is turned");
    let face = planes(&squared)
        .into_iter()
        .find(|plane| plane.normal.y.abs() > 0.5 && (plane.offset().abs() - 5.0).abs() < 1e-6)
        .expect("the block's face at y = 5");
    for corner in edge {
        let off = face.distance(corner);
        assert!(off.abs() < 1e-12, "the edge stands {off} off the face");
    }
}

#[test]
fn a_side_drawn_tangent_to_a_leaning_post_is_laid_along_it_still_tangent() {
    let body = leaning(
        post([0.0, 0.0], 5.0, 0.0, 10.0),
        DQuat::from_rotation_x(degrees(1e-7)),
    );
    let tool = block([-10.0, 5.0, 2.0], [10.0, 20.0, 30.0]);
    let Some(Surface::Cylinder(wall)) = body
        .surfaces
        .iter()
        .find(|surface| matches!(surface, Surface::Cylinder(_)))
    else {
        panic!("the post has a wall");
    };
    let squared = squared(&body, &tool, scale()).expect("the block is turned");
    let side = planes(&squared)
        .into_iter()
        .find(|plane| plane.normal.y.abs() > 0.5 && (plane.offset().abs() - 5.0).abs() < 1e-6)
        .expect("the block's side on the post");
    let off = side.distance(wall.origin).abs() - wall.radius;
    assert!(off.abs() < 1e-12, "the side stands {off} off the wall");
}

#[test]
fn an_edge_drawn_touching_a_leaning_post_s_wall_is_laid_across_it_still_touching() {
    let body = leaning(
        post([0.0, 0.0], 5.0, 0.0, 10.0),
        DQuat::from_rotation_x(degrees(1e-7)),
    );
    let outline = Contour {
        corners: vec![
            DVec2::new(5.0, 6.0),
            DVec2::new(15.0, 6.0),
            DVec2::new(15.0, 10.0),
        ],
        runs: vec![Run::Straight, Run::Straight, Run::Straight],
    };
    let frame = Frame {
        origin: DVec3::new(-10.0, 0.0, 0.0),
        u: DVec3::Y,
        v: DVec3::Z,
    };
    let wedge = Body::raised(&outline, &[], frame, DVec3::X * 20.0).expect("a wedge raises");
    let Some(Surface::Cylinder(wall)) = body
        .surfaces
        .iter()
        .find(|surface| matches!(surface, Surface::Cylinder(_)))
    else {
        panic!("the post has a wall");
    };
    let squared = squared(&body, &wedge, scale()).expect("the wedge is turned");
    let edge = squared
        .edges
        .iter()
        .filter_map(|edge| match squared.curve(edge.curve) {
            Curve::Line(line) if line.direction.x.abs() > 0.5 => Some(*line),
            _ => None,
        })
        .min_by(|one, other| one.origin.y.total_cmp(&other.origin.y))
        .expect("the wedge's edge on the post");
    let across = edge.direction.cross(wall.axis).normalize();
    let off = (edge.origin - wall.origin).dot(across).abs() - wall.radius;
    assert!(off.abs() < 1e-12, "the edge stands {off} off the wall");
}

#[test]
fn a_tool_reaching_far_past_the_body_is_turned_about_its_own_middle_where_turned_about_the_meeting_it_would_move_past_the_hair()
 {
    let body = post([0.0, 0.0], 5.0, 0.0, 10.0);
    let tool = leaning(
        block([-10.0, -10.0, 8.0], [10.0, 190.0, 12.0]),
        DQuat::from_rotation_x(degrees(5e-7)),
    );
    let squared = squared(&body, &tool, scale()).expect("the block is turned");
    assert!(square_or_along(&squared, DVec3::Z));
}
