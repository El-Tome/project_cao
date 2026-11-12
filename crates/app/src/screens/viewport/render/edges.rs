//! The edge of the part under the hand, lit along the points the kernel draws
//! it through.

use cao_render::{Vertex, srgb};
use cao_solid::Body;

use super::super::ViewportState;
use crate::ui::theme::tokens;

const WIDTH: f32 = 3.0;

pub(super) fn push_edge_under_the_hand(
    lines: &mut Vec<Vertex>,
    state: &ViewportState,
    body: &Body,
) {
    let Some(points) = state
        .edge_under_the_hand
        .as_ref()
        .and_then(|name| state.edge_points(body, name))
    else {
        return;
    };
    let [r, g, b, a] = tokens::ACCENT_AMBER.to_normalized_gamma_f32();
    let colour = srgb(r, g, b, a);
    for pair in points.windows(2) {
        lines.push(Vertex::line(pair[0].as_vec3(), colour, WIDTH));
        lines.push(Vertex::line(pair[1].as_vec3(), colour, WIDTH));
    }
}
