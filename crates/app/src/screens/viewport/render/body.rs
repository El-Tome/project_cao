//! The part's matter in a frame: what the sketch's cut or the section leaves
//! of it, drawn opaque, see-through or not at all as the body's mode says.

use cao_render::MatterInFront;
use glam::{DVec3, Vec3};

use super::{tint, tint_at};
use crate::screens::SketchContext;
use crate::screens::viewport::ViewportState;

const SEE_THROUGH_OPACITY: f32 = 0.25;

pub(super) fn push_matter(
    solids: &mut Vec<cao_render::Vertex>,
    surfaces: &mut Vec<cao_render::Vertex>,
    state: &ViewportState,
    context: &SketchContext<'_>,
) {
    let (theme, camera) = (&state.theme, &state.camera);
    if state.body_mode() == MatterInFront::Hidden {
        return;
    }
    let drawn = super::super::matter::shown(state, context.editor, context.document.body());
    let opaque_veiled = match state.body_mode() {
        MatterInFront::SeeThrough => Some(&*drawn.opaque),
        _ => {
            let solid = tint(theme.solid);
            push_body(solids, drawn.opaque.triangles(), solid, camera.forward());
            None
        }
    };
    let eye = camera.eye().as_dvec3();
    for veiled in opaque_veiled.into_iter().chain(drawn.see_through.as_ref()) {
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
