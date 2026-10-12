//! Decision 11 of `docs/exact-kernel.md` (#552): a plane of one operand a
//! hair off square to, or along, the axis of a wall of the other is laid
//! exactly so before anything else is decided, and the kernel then builds
//! the circle or the lines it holds exactly rather than an ellipse within
//! the tolerance of the rim beside it.

use std::f64::consts::PI;

use cao_solid::brep::{Body, Curve, Surface};
use cao_solid::profile::{Contour, Frame};
use cao_solid::soundness::{closed, listed, uncrossed};
use glam::{DVec2, DVec3};

const MESH: f64 = 0.01;

/// A box between two corners, its floor laid on a frame turned from the
/// world's by `turn`.
fn leaning(low: [f64; 3], high: [f64; 3], turn: impl Fn(DVec3) -> DVec3) -> Body {
    let outline = Contour::rectangle(DVec2::new(low[0], low[1]), DVec2::new(high[0], high[1]));
    let frame = Frame {
        origin: turn(DVec3::Z * low[2]),
        u: turn(DVec3::X),
        v: turn(DVec3::Y),
    };
    Body::raised(&outline, &[], frame, turn(DVec3::Z) * (high[2] - low[2])).expect("a block raises")
}

fn upright(point: DVec3) -> DVec3 {
    point
}

/// A post about Z through `center`, from `from` to `to`, its floor laid on
/// a frame turned by `turn`.
fn post(center: [f64; 2], radius: f64, from: f64, to: f64, turn: impl Fn(DVec3) -> DVec3) -> Body {
    let frame = Frame {
        origin: turn(DVec3::Z * from),
        u: turn(DVec3::X),
        v: turn(DVec3::Y),
    };
    Body::raised(
        &Contour::circle(DVec2::from(center), radius),
        &[],
        frame,
        turn(DVec3::Z) * (to - from),
    )
    .expect("a post raises")
}

/// A turn about X by `degrees`.
fn about_x(degrees: f64) -> impl Fn(DVec3) -> DVec3 {
    let (sin, cos) = degrees.to_radians().sin_cos();
    move |point: DVec3| {
        DVec3::new(
            point.x,
            point.y * cos - point.z * sin,
            point.y * sin + point.z * cos,
        )
    }
}

/// A turn about Y by `degrees`.
fn about_y(degrees: f64) -> impl Fn(DVec3) -> DVec3 {
    let (sin, cos) = degrees.to_radians().sin_cos();
    move |point: DVec3| {
        DVec3::new(
            point.x * cos + point.z * sin,
            point.y,
            point.z * cos - point.x * sin,
        )
    }
}

fn bored() -> Body {
    leaning([-10.0, -10.0, 0.0], [10.0, 10.0, 10.0], upright)
        .cut_by(&post([0.0, 0.0], 5.0, 0.0, 10.0, upright))
        .expect("the block is bored")
}

/// The body held to its listing and its triangles, and its volume.
fn sound(body: &Body) -> f64 {
    assert_eq!(listed(&body.listing(), body.scale().reach()), Ok(()));
    let triangles = body.triangles(MESH);
    assert_eq!(closed(&triangles), Ok(()));
    assert_eq!(uncrossed(&triangles), Ok(()));
    body.volume()
}

fn conics(body: &Body) -> usize {
    body.listing()
        .edges
        .iter()
        .filter(|edge| matches!(edge.curve, Curve::Conic(_)))
        .count()
}

#[test]
fn a_post_cut_by_a_block_leaning_1e_7_degrees_off_its_end_holds_the_volume_of_a_square_cut() {
    let cut = post([0.0, 0.0], 5.0, 0.0, 10.0, upright)
        .cut_by(&leaning(
            [-10.0, -10.0, 6.0],
            [10.0, 10.0, 16.0],
            about_x(1e-7),
        ))
        .expect("the kernel answers");
    let volume = sound(&cut);
    let square = PI * 25.0 * 6.0;
    assert!(
        ((volume - square) / square).abs() <= 1e-8,
        "the post keeps {volume}, a square cut {square}"
    );
    assert_eq!(conics(&cut), 0);
    let floors = cut
        .listing()
        .faces
        .iter()
        .filter(|face| matches!(face.surface, Surface::Plane(plane) if plane.normal == DVec3::Z))
        .count();
    assert_eq!(
        floors, 2,
        "the cut is a floor square to the post's axis, as its foot is"
    );
}

#[test]
fn a_bore_cut_by_a_side_leaning_4e_8_degrees_along_its_axis_is_cut_along_two_lines() {
    let cut = bored()
        .cut_by(&leaning(
            [3.0, -20.0, -5.0],
            [20.0, 20.0, 15.0],
            about_y(4e-8),
        ))
        .expect("the kernel answers");
    let volume = sound(&cut);
    let expected = 13.0 * 20.0 * 10.0 - (25.0 * (PI - (0.6f64).acos()) + 3.0 * 4.0) * 10.0;
    assert!(
        ((volume - expected) / expected).abs() <= 1e-8,
        "the block keeps {volume}, {expected} reasoned"
    );
    assert_eq!(conics(&cut), 0);
    let along = cut
        .listing()
        .edges
        .iter()
        .filter(|edge| {
            matches!(edge.curve, Curve::Line(line)
                if line.direction == DVec3::Z
                    && (line.origin.x - 3.0).abs() < 1e-6
                    && (line.origin.y.abs() - 4.0).abs() < 1e-6)
        })
        .count();
    assert_eq!(
        along, 2,
        "the side cuts the bore along two lines up its axis"
    );
}

#[test]
fn a_turn_1e_7_degrees_off_a_bore_of_its_radius_is_merged_onto_it() {
    let block = bored();
    let cut = block
        .cut_by(&post([0.0, 0.0], 5.0, -1.0, 11.0, about_x(1e-7)))
        .expect("the kernel answers");
    let volume = sound(&cut);
    let before = block.volume();
    assert!(
        ((volume - before) / before).abs() <= 1e-9,
        "the block keeps {volume} of {before}"
    );
    let mut walls = Vec::new();
    for face in cut.listing().faces {
        if let Surface::Cylinder(wall) = face.surface
            && !walls.contains(&wall)
        {
            walls.push(wall);
        }
    }
    assert_eq!(walls.len(), 1, "the bore is one wall: {walls:?}");
}
