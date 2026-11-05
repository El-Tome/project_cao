//! What is left of an exact part behind a plane, cut by the exact kernel: a
//! solid closed by a true planar face on the plane, its outline on the part's
//! own surfaces, and the triangles clipped only where the kernel declines.
//!
//! Closes #578.
//! - `Body::behind` no longer calls `as_flats` or `Mesh::behind`, held by
//!   what it shows: an exact body stays exact behind a plane, and on both
//!   sides of a plane it is parted by — `an_exact_body_stays_exact_behind_a_plane`,
//!   `an_exact_body_stays_exact_on_both_sides_of_a_plane_it_is_parted_by`
//! - the section of an exact part is filled. Its face lies on the cutting
//!   plane, and its outline lies on the part's surfaces within the tolerance.
//!   This holds for a block, a bored cylinder, a cone, a conic face and an
//!   elliptic wall — `a_block_s_section_is_a_filled_face_on_the_plane`,
//!   `a_bored_cylinder_s_section_is_a_filled_face_on_the_plane`,
//!   `a_cone_s_section_is_a_filled_face_on_the_plane`,
//!   `a_shaft_cut_at_a_slant_s_section_is_a_filled_face_on_the_plane`,
//!   `an_elliptic_wall_s_section_is_a_filled_face_on_the_plane`. The outline
//!   is read off the section face's drawn triangles: its corners, which stand
//!   on the exact edges, are held to the part's surfaces as the fixture writes
//!   them, at the kernel's widest tolerance. The matter strictly in front,
//!   which `Body::parted` hands the see-through layer, is filled the same way,
//!   its cap a drawing tolerance ahead of the plane so that none of it lies on
//!   a sketch drawn there — `a_block_parted_by_a_plane_is_filled_on_both_sides`
//! - a section the kernel declines falls back to clipping the drawn
//!   triangles, with no flats involved —
//!   `a_section_the_kernel_declines_is_clipped_from_the_drawn_triangles`,
//!   which holds what is drawn: the body's own triangles, those behind the
//!   plane kept whole and those across it cut at it. Clipping the flats the
//!   same triangles make draws the same picture, so whether the flats were
//!   gone through is not something it can tell
//! - the matter in front of the plane is still hidden while a sketch is
//!   edited — no test: in this file; it is `cao_app`'s, held by
//!   `the_matter_between_the_eye_and_the_sketch_plane_is_not_drawn` in
//!   `crates/app/src/screens/viewport/matter/tests.rs`, kept green
//! - must not break: the section is never the document's, the `.caopart` is
//!   unchanged and leaving the view puts the part back whole — no test: in
//!   this file; `behind` and `parted` borrow the body and hand back another,
//!   and leaving the view is `cao_app`'s, held by
//!   `closing_the_sketch_puts_the_whole_body_back` in the same matter tests
//! - must not break: a part still on the flats is clipped as today —
//!   `a_part_on_the_flats_is_clipped_as_before`
//! - from What it does: the cut is computed once each time the plane changes,
//!   never every frame — no test: in this file; it is the view's, in
//!   `cao_app`, held by `the_section_is_cut_once_while_the_plane_stays_put`,
//!   `the_section_is_cut_again_when_the_plane_moves` and
//!   `the_section_is_cut_again_when_the_part_the_side_or_the_state_changes`
//!   in the same matter tests, through the view's `Sections`
//!
//! Closes #193.
//! - slicing a cube at mid-height gives the expected cross-section, with no
//!   window — `a_block_sliced_at_mid_height_leaves_its_footprint_on_the_plane`;
//!   the rest of #193 is the view's, transcribed in
//!   `crates/app/src/screens/viewport/render/tests.rs`

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use cao_solid::brep::Scale;
use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::{closed, uncrossed};
use cao_solid::turning::{Axis, Turn};
use cao_solid::{Body, Loop};
use glam::{DVec2, DVec3};

const GROUND: Frame = Frame {
    origin: DVec3::ZERO,
    u: DVec3::X,
    v: DVec3::Y,
};

fn exact(outline: Contour) -> Profile<'static> {
    Profile {
        exact: Some((outline, Vec::new())),
        sampled: Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    }
}

fn on_a_part(tool: Body) -> Body {
    Body::default()
        .union(&tool)
        .expect("the part takes the tool")
}

/// 30 by 20 by 10, from the origin.
fn block() -> Body {
    let tool = Body::default()
        .tool_raised(
            &exact(Contour::rectangle(DVec2::ZERO, DVec2::new(30.0, 20.0))),
            GROUND,
            DVec3::Z * 10.0,
        )
        .expect("the exact kernel raises it");
    on_a_part(tool)
}

/// A plane standing upright, turned thirty degrees from the block's end:
/// it crosses the block from `(17.32, 0)` to `(5.77, 20)` on the ground.
fn upright_at_thirty_degrees() -> (DVec3, f64) {
    let (sin, cos) = 30f64.to_radians().sin_cos();
    (DVec3::new(cos, sin, 0.0), 15.0)
}

/// The kernel's widest tolerance at the body's reach.
fn tolerance(body: &Body) -> f64 {
    Scale::HAIR * Scale::of(reach(body)).eps()
}

fn area(triangles: &[[DVec3; 3]]) -> f64 {
    triangles
        .iter()
        .map(|[a, b, c]| (*b - *a).cross(*c - *a).length() / 2.0)
        .sum()
}

/// The faces of `body` that are flat, face out along `normal`, and lie on the
/// plane `normal · p = offset` within `tolerance`.
fn faces_on_the_plane(body: &Body, normal: DVec3, offset: f64, tolerance: f64) -> Vec<usize> {
    (0..body.faces_end())
        .chain(body.pieces())
        .filter(|&face| body.has_face(face) && body.is_flat(face))
        .filter(|&face| {
            body.plane_of(face)
                .is_some_and(|plane| plane.normal.dot(normal) > 1.0 - 1e-9)
        })
        .filter(|&face| {
            body.triangles_of(face)
                .flatten()
                .all(|point| (point.dot(normal) - offset).abs() <= tolerance)
        })
        .collect()
}

/// The edges of a set of triangles that no other triangle runs back along:
/// the outline of the face they draw.
fn outline(triangles: &[[DVec3; 3]], tolerance: f64) -> Vec<(DVec3, DVec3)> {
    let edges: Vec<(DVec3, DVec3)> = triangles
        .iter()
        .flat_map(|&[a, b, c]| [(a, b), (b, c), (c, a)])
        .collect();
    edges
        .iter()
        .copied()
        .filter(|&(from, to)| {
            !edges.iter().any(|&(other_from, other_to)| {
                other_from.distance(to) <= tolerance && other_to.distance(from) <= tolerance
            })
        })
        .collect()
}

/// Whether `point` lies on one of the flat faces of `part`, within
/// `tolerance`: on its plane, and inside the part's box.
fn on_a_flat_face_of(part: &Body, point: DVec3, tolerance: f64) -> bool {
    let (low, high) = part.bounds().expect("the part holds matter");
    let inside = point.cmpge(low - tolerance).all() && point.cmple(high + tolerance).all();
    inside
        && (0..part.faces_end())
            .chain(part.pieces())
            .filter(|&face| part.has_face(face) && part.is_flat(face))
            .filter_map(|face| part.plane_of(face))
            .any(|plane| {
                (plane.normal.dot(point) - plane.normal.dot(plane.corners[0])).abs() <= tolerance
            })
}

#[test]
fn an_exact_body_stays_exact_behind_a_plane() {
    let (normal, offset) = upright_at_thirty_degrees();

    let behind = block().behind(normal, offset);

    assert!(
        behind.is_exact(),
        "an exact block cut behind a plane the kernel can cut went to the flats"
    );
}

#[test]
fn an_exact_body_stays_exact_on_both_sides_of_a_plane_it_is_parted_by() {
    let (normal, offset) = upright_at_thirty_degrees();

    let (behind, in_front) = block().parted(normal, offset);

    assert!(
        behind.is_exact() && in_front.is_exact(),
        "an exact block parted by a plane the kernel can cut went to the flats: \
         behind exact {}, in front exact {}",
        behind.is_exact(),
        in_front.is_exact(),
    );
}

#[test]
fn a_block_s_section_is_a_filled_face_on_the_plane() {
    let part = block();
    let tolerance = tolerance(&part);
    let (normal, offset) = upright_at_thirty_degrees();
    let cos = 30f64.to_radians().cos();

    let section = part.behind(normal, offset);

    let faces = faces_on_the_plane(&section, normal, offset, tolerance);
    assert_eq!(
        faces.len(),
        1,
        "one face of the section lies on the cutting plane, facing out of it: {faces:?}"
    );
    let triangles = section.triangles();
    assert_eq!(closed(&triangles), Ok(()), "the section is filled");
    assert_eq!(uncrossed(&triangles), Ok(()));

    let face: Vec<[DVec3; 3]> = section.triangles_of(faces[0]).collect();
    let promised = 20.0 / cos * 10.0;
    assert!(
        ((area(&face) - promised) / promised).abs() <= 1e-9,
        "the face on the plane covers {}, the block's section {promised}",
        area(&face),
    );
    let volume = 200.0 / cos * 10.0;
    assert!(
        ((section.volume() - volume) / volume).abs() <= 1e-9,
        "the section encloses {}, the block behind the plane {volume}",
        section.volume(),
    );
    for (from, to) in outline(&face, tolerance) {
        for point in [from, (from + to) / 2.0, to] {
            assert!(
                on_a_flat_face_of(&part, point, tolerance),
                "{point} on the section's outline stands off every face of the block"
            );
        }
    }
}

fn raised(outline: Contour, frame: Frame, height: f64) -> Body {
    Body::default()
        .tool_raised(&exact(outline), frame, frame.normal() * height)
        .expect("the exact kernel raises it")
}

fn disc(center: DVec2, radius: f64) -> Contour {
    Contour {
        corners: vec![center + DVec2::X * radius],
        runs: vec![Run::Round { center, turn: TAU }],
    }
}

/// A profile turned whole about the sketch's Y axis, `sampled` the drawing
/// the profile was read from.
fn turned(corners: Vec<DVec2>, runs: Vec<Run>, sampled: Vec<DVec2>) -> Body {
    let sampled: &'static [DVec2] = Box::leak(sampled.into_boxed_slice());
    let profile = Profile {
        exact: Some((Contour { corners, runs }, Vec::new())),
        sampled: Loop::straight(sampled),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    let about_y = Axis {
        origin: DVec2::ZERO,
        direction: DVec2::Y,
    };
    on_a_part(
        Body::default()
            .tool_turned(&profile, GROUND, &Turn::of(about_y, TAU, 1e-4, &profile))
            .expect("the exact kernel turns it"),
    )
}

/// The area of the disc of radius `r` beyond a chord `d` from its centre.
fn segment(r: f64, d: f64) -> f64 {
    r * r * (d / r).acos() - d * (r * r - d * d).sqrt()
}

/// The plane `z = 5 + 0.2 x`, everything under it behind.
fn leaning_a_fifth() -> (DVec3, f64) {
    let length = 1.04f64.sqrt();
    (DVec3::new(-0.2, 0.0, 1.0) / length, 5.0 / length)
}

/// How far `point` stands from the cylinder of `radius` about Z.
fn off_the_wall(radius: f64, point: DVec3) -> f64 {
    (point.truncate().length() - radius).abs()
}

/// A cylinder of radius 20 standing from 0 to 10, bored through on its axis
/// with a radius of 5.
fn bored_cylinder() -> Body {
    let below = Frame {
        origin: DVec3::NEG_Z,
        ..GROUND
    };
    on_a_part(raised(disc(DVec2::ZERO, 20.0), GROUND, 10.0))
        .difference(&raised(disc(DVec2::ZERO, 5.0), below, 12.0))
        .expect("the exact kernel bores it")
}

fn off_the_bored_cylinder(point: DVec3) -> f64 {
    off_the_wall(20.0, point)
        .min(off_the_wall(5.0, point))
        .min(point.z.abs())
        .min((point.z - 10.0).abs())
}

/// A cone of radius 10 on the ground and height 20 along Y, its apex up.
fn cone() -> Body {
    let corners = vec![
        DVec2::new(0.0, 0.0),
        DVec2::new(10.0, 0.0),
        DVec2::new(0.0, 20.0),
    ];
    turned(corners.clone(), vec![Run::Straight; 3], corners)
}

/// How far `point` stands from the cone's wall, `r = 10 - y / 2`, or from its
/// base.
fn off_the_cone(point: DVec3) -> f64 {
    let across = DVec2::new(point.x, point.z).length();
    let wall = (across - 10.0 + point.y / 2.0).abs() * 2.0 / 5f64.sqrt();
    wall.min(point.y.abs())
}

/// The cylinder of radius 20 from 0 to 10, everything above the plane
/// `z = 5 + 0.2 x` taken away: its top an ellipse, its rim a conic.
fn shaft_cut_at_a_slant() -> Body {
    let slant = Frame {
        origin: DVec3::new(0.0, 0.0, 5.0),
        u: DVec3::new(1.0, 0.0, 0.2).normalize(),
        v: DVec3::Y,
    };
    on_a_part(raised(disc(DVec2::ZERO, 20.0), GROUND, 10.0))
        .difference(&raised(
            Contour::rectangle(DVec2::splat(-40.0), DVec2::splat(40.0)),
            slant,
            30.0,
        ))
        .expect("the exact kernel cuts it")
}

fn off_the_shaft_cut_at_a_slant(point: DVec3) -> f64 {
    off_the_wall(20.0, point)
        .min(point.z.abs())
        .min((point.z - 5.0 - 0.2 * point.x).abs() / 1.04f64.sqrt())
}

/// An ellipse of semi-axes 15 and 8 raised 10.
fn elliptic_post() -> Body {
    on_a_part(raised(
        Contour {
            corners: vec![DVec2::new(15.0, 0.0)],
            runs: vec![Run::Oval {
                center: DVec2::ZERO,
                first: DVec2::new(15.0, 0.0),
                second: 8.0,
                turn: TAU,
            }],
        },
        GROUND,
        10.0,
    ))
}

/// How far `point` stands from the post's wall, bounded above by how far its
/// reading of the ellipse is from one times the larger semi-axis, or from its
/// ends.
fn off_the_elliptic_post(point: DVec3) -> f64 {
    let read = DVec2::new(point.x / 15.0, point.y / 8.0).length();
    ((read - 1.0).abs() * 15.0)
        .min(point.z.abs())
        .min((point.z - 10.0).abs())
}

/// What is left of `part` behind the plane, held to what the user sees of a
/// section: exact, closed, one face on the plane covering `area`, `volume`
/// enclosed when it is given, and every corner of that face's outline on the
/// part's surfaces.
fn holds_its_section(
    part: &Body,
    (normal, offset): (DVec3, f64),
    area_promised: f64,
    volume_promised: Option<f64>,
    off_the_part: impl Fn(DVec3) -> f64,
) {
    let tolerance = tolerance(part);

    let section = part.behind(normal, offset);

    assert!(section.is_exact(), "the section went to the flats");
    let faces = faces_on_the_plane(&section, normal, offset, tolerance);
    assert_eq!(
        faces.len(),
        1,
        "one face of the section lies on the cutting plane, facing out of it: {faces:?}"
    );
    let triangles = section.triangles();
    assert_eq!(closed(&triangles), Ok(()), "the section is filled");
    assert_eq!(uncrossed(&triangles), Ok(()));
    let face: Vec<[DVec3; 3]> = section.triangles_of(faces[0]).collect();
    let outline = outline(&face, tolerance);
    assert!(!outline.is_empty(), "the face on the plane has no outline");
    let length: f64 = outline.iter().map(|(from, to)| from.distance(*to)).sum();
    let drawn_area = area(&face);
    let sliver = length * DRAWN * reach(part);
    assert!(
        (drawn_area - area_promised).abs() <= sliver,
        "the face on the plane is drawn over {drawn_area}, the part's section is \
         {area_promised}, and chords {length} long leave out no more than {sliver}",
    );
    if let Some(volume) = volume_promised {
        assert!(
            ((section.volume() - volume) / volume).abs() <= 1e-9,
            "the section encloses {}, the part behind the plane {volume}",
            section.volume(),
        );
    }
    for (corner, _) in outline {
        assert!(
            off_the_part(corner) <= tolerance,
            "{corner} on the section's outline stands {} off every surface of the part, \
             past {tolerance}",
            off_the_part(corner),
        );
    }
}

/// How far the drawn triangles may stand from the exact surfaces, as a share
/// of the reach: the kernel's `DRAWN`, which a curved outline's chords stand
/// inside by.
const DRAWN: f64 = 2e-4;

fn reach(body: &Body) -> f64 {
    let (low, high) = body.bounds().expect("the body holds matter");
    low.abs().max(high.abs()).max_element()
}

fn area_of(body: &Body, face: usize) -> f64 {
    area(&body.triangles_of(face).collect::<Vec<_>>())
}

#[test]
fn a_bored_cylinder_s_section_is_a_filled_face_on_the_plane() {
    holds_its_section(
        &bored_cylinder(),
        leaning_a_fifth(),
        375.0 * PI * 1.04f64.sqrt(),
        Some(5.0 * 375.0 * PI),
        off_the_bored_cylinder,
    );
}

#[test]
fn a_cone_s_section_is_a_filled_face_on_the_plane() {
    let hyperbola = 2.0 * (10.0 * 91f64.sqrt() - 9.0 * ((10.0 + 91f64.sqrt()) / 3.0).ln());
    holds_its_section(&cone(), (DVec3::X, 3.0), hyperbola, None, off_the_cone);
}

#[test]
fn a_shaft_cut_at_a_slant_s_section_is_a_filled_face_on_the_plane() {
    let below_the_slant = segment(20.0, 5.0);
    let volume = 2000.0 * PI - 2.0 / 15.0 * 375f64.powf(1.5) + below_the_slant;
    holds_its_section(
        &shaft_cut_at_a_slant(),
        (DVec3::Z, 6.0),
        below_the_slant,
        Some(volume),
        off_the_shaft_cut_at_a_slant,
    );
}

#[test]
fn an_elliptic_wall_s_section_is_a_filled_face_on_the_plane() {
    holds_its_section(
        &elliptic_post(),
        leaning_a_fifth(),
        120.0 * PI * 1.04f64.sqrt(),
        Some(600.0 * PI),
        off_the_elliptic_post,
    );
}

/// The flat faces of `body` facing `-normal`, each with how far along
/// `normal` its plane stands.
fn faces_facing_back(body: &Body, normal: DVec3) -> Vec<(usize, f64)> {
    (0..body.faces_end())
        .chain(body.pieces())
        .filter(|&face| body.has_face(face) && body.is_flat(face))
        .filter_map(|face| Some((face, body.plane_of(face)?)))
        .filter(|(_, plane)| plane.normal.dot(normal) < -1.0 + 1e-9)
        .map(|(face, plane)| (face, plane.corners[0].dot(normal)))
        .collect()
}

#[test]
fn a_block_parted_by_a_plane_is_filled_on_both_sides() {
    let part = block();
    let tolerance = tolerance(&part);
    let (normal, offset) = upright_at_thirty_degrees();
    let reach = reach(&part);

    let (behind, in_front) = part.parted(normal, offset);

    assert_eq!(
        faces_on_the_plane(&behind, normal, offset, tolerance).len(),
        1,
        "what is left behind is closed by one face on the plane"
    );
    assert_eq!(closed(&behind.triangles()), Ok(()), "behind is filled");
    let in_front_triangles = in_front.triangles();
    assert_eq!(closed(&in_front_triangles), Ok(()), "in front is filled");
    assert_eq!(uncrossed(&in_front_triangles), Ok(()));
    assert!(
        in_front_triangles
            .iter()
            .flatten()
            .all(|point| point.dot(normal) - offset > tolerance),
        "some of what stands in front lies on the plane, under the sketch drawn there"
    );
    let caps: Vec<(usize, f64)> = faces_facing_back(&in_front, normal)
        .into_iter()
        .filter(|&(_, at)| at - offset < 1e-3 * reach)
        .collect();
    assert_eq!(
        caps.len(),
        1,
        "what stands in front is closed by one face just ahead of the plane, facing it: {caps:?}"
    );
    let (cap, at) = caps[0];
    let gap = at - offset;
    let between = area_of(&in_front, cap) * gap;
    let whole = part.volume();
    let parted = behind.volume() + between + in_front.volume();
    assert!(
        ((parted - whole) / whole).abs() <= 1e-9,
        "behind, the slab of {gap} between and in front enclose {parted}, the block {whole}",
    );
}

/// A ball of radius 5: a half disc turned about its diameter.
fn ball() -> Body {
    let sampled = (0..=32)
        .map(|step| DVec2::from_angle(-FRAC_PI_2 + PI * f64::from(step) / 32.0) * 5.0)
        .collect();
    turned(
        vec![DVec2::new(0.0, -5.0), DVec2::new(0.0, 5.0)],
        vec![
            Run::Round {
                center: DVec2::ZERO,
                turn: PI,
            },
            Run::Straight,
        ],
        sampled,
    )
}

/// The area of the part of `triangle` with `normal · p ≤ offset`.
fn area_behind([a, b, c]: [DVec3; 3], normal: DVec3, offset: f64) -> f64 {
    let corners = [a, b, c];
    let mut kept = Vec::new();
    for index in 0..3 {
        let (from, to) = (corners[index], corners[(index + 1) % 3]);
        let (here, there) = (from.dot(normal) - offset, to.dot(normal) - offset);
        if here <= 0.0 {
            kept.push(from);
        }
        if (here < 0.0) != (there < 0.0) && here != 0.0 && there != 0.0 {
            kept.push(from + (to - from) * (here / (here - there)));
        }
    }
    (1..kept.len().saturating_sub(1))
        .map(|index| {
            (kept[index] - kept[0])
                .cross(kept[index + 1] - kept[0])
                .length()
                / 2.0
        })
        .sum()
}

/// Whether `point` lies on `triangle`, within `tolerance`.
fn on_the_triangle(point: DVec3, [a, b, c]: [DVec3; 3], tolerance: f64) -> bool {
    let normal = (b - a).cross(c - a);
    let unit = normal.normalize();
    if (point - a).dot(unit).abs() > tolerance {
        return false;
    }
    [(a, b), (b, c), (c, a)].iter().all(|&(from, to)| {
        (to - from).cross(point - from).dot(unit) >= -tolerance * (to - from).length()
    })
}

/// What `body` behind the plane is drawn with, held to its own triangles
/// clipped there: nothing in front, nothing that is not on one of them, all
/// of them behind the plane covered, and the cut left open.
fn is_its_drawing_clipped(body: &Body, normal: DVec3, offset: f64) {
    let tolerance = tolerance(body);
    let drawn = body.triangles();

    let clipped = body.behind(normal, offset).triangles();

    assert!(
        closed(&clipped).is_err(),
        "the cut is closed, which clipping triangles does not do"
    );
    assert!(
        clipped
            .iter()
            .flatten()
            .all(|point| point.dot(normal) - offset <= tolerance),
        "some of what is drawn stands in front of the plane"
    );
    for triangle in &clipped {
        let middle = (triangle[0] + triangle[1] + triangle[2]) / 3.0;
        assert!(
            drawn
                .iter()
                .any(|&original| on_the_triangle(middle, original, tolerance)),
            "{triangle:?} is on none of the triangles the body is drawn with"
        );
    }
    let promised: f64 = drawn
        .iter()
        .map(|&triangle| area_behind(triangle, normal, offset))
        .sum();
    assert!(
        ((area(&clipped) - promised) / promised).abs() <= 1e-9,
        "what is drawn covers {}, the body's own triangles behind the plane {promised}",
        area(&clipped),
    );
}

#[test]
fn a_section_the_kernel_declines_is_clipped_from_the_drawn_triangles() {
    let part = ball();
    let normal = DVec3::new(1.0, 1.0, 0.0).normalize();

    is_its_drawing_clipped(&part, normal, 1.0);
}

/// A disc of radius 20 drawn as 32 flat sides, raised 10 by the flats: the
/// drawing handed no runs.
fn flats_disc() -> Body {
    let corners: &'static [DVec2] = Box::leak(
        (0..32)
            .map(|index| DVec2::from_angle(index as f64 * TAU / 32.0) * 20.0)
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    );
    let triangles: &'static [[DVec2; 3]] = Box::leak(
        (1..corners.len() - 1)
            .map(|index| [corners[0], corners[index], corners[index + 1]])
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    );
    let profile = Profile {
        exact: None,
        sampled: Loop::straight(corners),
        sampled_holes: Vec::new(),
        triangles,
    };
    Body::default()
        .tool_raised(&profile, GROUND, DVec3::Z * 10.0)
        .expect("the flats never decline")
}

#[test]
fn a_part_on_the_flats_is_clipped_as_before() {
    let part = flats_disc();
    assert!(!part.is_exact());

    is_its_drawing_clipped(&part, DVec3::new(1.0, 0.0, 0.5).normalize(), 3.0);

    assert!(!part.behind(DVec3::X, 3.0).is_exact());
}

#[test]
fn a_block_sliced_at_mid_height_leaves_its_footprint_on_the_plane() {
    let part = block();
    let tolerance = tolerance(&part);

    let section = part.behind(DVec3::Z, 5.0);

    let faces = faces_on_the_plane(&section, DVec3::Z, 5.0, tolerance);
    assert_eq!(
        faces.len(),
        1,
        "one face lies on the plane at mid-height: {faces:?}"
    );
    assert_eq!(
        closed(&section.triangles()),
        Ok(()),
        "the section is filled"
    );
    let face: Vec<[DVec3; 3]> = section.triangles_of(faces[0]).collect();
    assert!(
        (area(&face) - 600.0).abs() <= 1e-9 * 600.0,
        "the cut across a block 30 by 20 covers {}, not its footprint, 600",
        area(&face),
    );
    assert!(
        (section.volume() - 3000.0).abs() <= 1e-9 * 3000.0,
        "the lower half of the block encloses {}, not 3000",
        section.volume(),
    );
    assert!(
        section
            .triangles()
            .iter()
            .flatten()
            .all(|at| at.z <= 5.0 + tolerance),
        "something of the block stands over the plane it was sliced at",
    );
}
