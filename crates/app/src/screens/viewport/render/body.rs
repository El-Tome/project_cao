//! The part's matter in a frame: what the sketch's cut or the section leaves
//! of the part shown, drawn opaque, and of the ghosts of the volumes shown
//! see-through, drawn see-through.

use glam::{DVec3, Vec3};

use super::{tint, tint_at};
use crate::screens::viewport::ViewportState;
use crate::screens::viewport::matter::Drawn;

const SEE_THROUGH_OPACITY: f32 = 0.25;

pub(super) fn push_matter(
    solids: &mut Vec<cao_render::Vertex>,
    surfaces: &mut Vec<cao_render::Vertex>,
    state: &ViewportState,
    drawn: Option<&Drawn<'_>>,
) {
    let (theme, camera) = (&state.theme, &state.camera);
    let Some(drawn) = drawn else {
        return;
    };
    let solid = tint(theme.solid);
    push_body(solids, drawn.opaque.triangles(), solid, camera.forward());
    let eye = camera.eye().as_dvec3();
    for veiled in drawn.see_through.iter().chain(&drawn.ghosts) {
        // The surface pass culls nothing: a far side left in would be stacked
        // behind the near one.
        let facing = veiled
            .triangles()
            .into_iter()
            .filter(|[a, b, c]| (*b - *a).cross(*c - *a).dot(eye - *a) > 0.0)
            .collect();
        let color = tint_at(theme.solid, SEE_THROUGH_OPACITY);
        push_body(surfaces, facing, color, camera.forward());
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
