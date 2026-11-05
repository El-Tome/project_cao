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
//!   `an_elliptic_wall_s_section_is_a_filled_face_on_the_plane`; the matter
//!   strictly in front, which `Body::parted` hands the see-through layer,
//!   is filled the same way — `a_block_parted_by_a_plane_is_filled_on_both_sides`
//! - a section the kernel declines falls back to clipping the drawn
//!   triangles, with no flats involved —
//!   `a_section_the_kernel_declines_is_clipped_from_the_drawn_triangles`
//! - the matter in front of the plane is still hidden while a sketch is
//!   edited — no test in this file: `cao_app`'s, held by
//!   `the_matter_between_the_eye_and_the_sketch_plane_is_not_drawn` in
//!   `crates/app/src/screens/viewport/matter/tests.rs`, kept green
//! - must not break: the section is never the document's, the `.caopart` is
//!   unchanged and leaving the view puts the part back whole — no test in
//!   this file: `behind` and `parted` borrow the body and hand back another,
//!   and leaving the view is `cao_app`'s, held by
//!   `closing_the_sketch_puts_the_whole_body_back` in the same matter tests
//! - must not break: a part still on the flats is clipped as today —
//!   `a_part_on_the_flats_is_clipped_as_before`
//! - from What it does: the cut is computed once each time the plane changes,
//!   never every frame — no test in this file: it is the view's, in
//!   `cao_app`, held by `the_section_is_cut_once_while_the_plane_stays_put`
//!   and `the_section_is_cut_again_when_the_plane_moves` in the same matter
//!   tests, the seam they count through still to be named

use cao_solid::brep::Scale;
use cao_solid::profile::{Contour, Frame, Profile};
use cao_solid::soundness::{closed, uncrossed};
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
    let (low, high) = body.bounds().expect("the body holds matter");
    let reach = low.abs().max(high.abs()).max_element();
    Scale::HAIR * Scale::of(reach).eps()
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
