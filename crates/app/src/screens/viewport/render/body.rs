//! The part's matter in a frame: what the sketch's cut or the section leaves
//! of it, each face drawn opaque, see-through or not at all as the volume
//! that made it says.

use cao_part::PartDocument;
use cao_render::MatterInFront;
use cao_solid::Body;
use glam::{DVec3, Vec3};

use super::{tint, tint_at};
use crate::screens::viewport::ViewportState;
use crate::screens::viewport::matter::Drawn;

const SEE_THROUGH_OPACITY: f32 = 0.25;

pub(super) fn push_matter(
    solids: &mut Vec<cao_render::Vertex>,
    surfaces: &mut Vec<cao_render::Vertex>,
    state: &ViewportState,
    document: &PartDocument,
    drawn: Option<&Drawn<'_>>,
) {
    let (theme, camera) = (&state.theme, &state.camera);
    let Some(drawn) = drawn else {
        return;
    };
    let faces = state.face_modes(document);
    let (mut opaque, mut veiled) = (Vec::new(), Vec::new());
    for (triangle, face) in drawn.opaque.triangles_by_face() {
        match faded_most(state.body_mode(), faces.of(face)) {
            MatterInFront::Opaque => opaque.push(triangle),
            MatterInFront::SeeThrough => veiled.push(triangle),
            MatterInFront::Hidden => {}
        }
    }
    for (triangle, face) in drawn.see_through.iter().flat_map(Body::triangles_by_face) {
        if faces.of(face) != MatterInFront::Hidden {
            veiled.push(triangle);
        }
    }
    push_body(solids, opaque, tint(theme.solid), camera.forward());
    let eye = camera.eye().as_dvec3();
    // The surface pass culls nothing: a far side left in would be stacked
    // behind the near one.
    veiled.retain(|[a, b, c]| (*b - *a).cross(*c - *a).dot(eye - *a) > 0.0);
    let color = tint_at(theme.solid, SEE_THROUGH_OPACITY);
    push_body(surfaces, veiled, color, camera.forward());
}

/// The fainter of two ways of drawing matter: what the whole body is set to,
/// and what its own face is.
fn faded_most(body: MatterInFront, face: MatterInFront) -> MatterInFront {
    use MatterInFront::{Hidden, Opaque, SeeThrough};
    match (body, face) {
        (Hidden, _) | (_, Hidden) => Hidden,
        (SeeThrough, _) | (_, SeeThrough) => SeeThrough,
        (Opaque, Opaque) => Opaque,
    }
}

fn push_body(
    out: &mut Vec<cao_render::Vertex>,
    triangles: Vec<[DVec3; 3]>,
    color: [f32; 4],
    towards: Vec3,
) {
    let triangles: Vec<_> = triangles
        .iter()
        .map(|corners| corners.map(|corner| corner.as_vec3()))
        .collect();
    cao_render::push_solid(out, &triangles, color, towards);
}
