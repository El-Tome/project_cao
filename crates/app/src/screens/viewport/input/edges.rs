//! The edge of the part under the hand while the edge panel is open: lit as
//! the hand passes, taken by a click.

use cao_solid::{Body, DrawnEdge, EdgeName};
use glam::DVec3;

use crate::screens::blend::BlendState;

use super::super::{EDGE_REACH, ViewportState, to_ndc};

/// How much nearer the eye than a point of an edge, as a share of how far it
/// stands, the matter has to be met to hide it: the faces along the edge are
/// met at the edge itself, give or take their triangles.
const IN_FRONT: f64 = 1e-3;

/// Lights the nearest edge in sight within reach of the hand, and hands it to
/// the panel on a click; a click reaching none leaves the panel's edge as it
/// was.
pub(crate) fn pick_an_edge(
    state: &mut ViewportState,
    response: &egui::Response,
    rect: egui::Rect,
    body: &Body,
    blend: &mut BlendState,
) {
    let under = response
        .hover_pos()
        .and_then(|at| edge_under(state, body, at, rect));
    if response.clicked()
        && let Some(name) = &under
    {
        blend.edge = Some(name.clone());
    }
    state.edge_under_the_hand = under;
}

fn edge_under(
    state: &ViewportState,
    body: &Body,
    at: egui::Pos2,
    rect: egui::Rect,
) -> Option<EdgeName> {
    let edges = state.edges.of(body);
    let mut within: Vec<(f32, &DrawnEdge, DVec3)> = edges
        .iter()
        .filter_map(|edge| {
            let (distance, point) = nearest_on_screen(state, &edge.points, at)?;
            (distance <= EDGE_REACH).then_some((distance, edge, point))
        })
        .collect();
    within.sort_by(|one, other| one.0.total_cmp(&other.0));
    within
        .into_iter()
        .find(|(_, _, point)| in_sight(state, body, *point, rect))
        .map(|(_, edge, _)| edge.name.clone())
}

/// How far from `at` the edge through these points passes on screen, and the
/// point of it that comes nearest.
fn nearest_on_screen(
    state: &ViewportState,
    points: &[DVec3],
    at: egui::Pos2,
) -> Option<(f32, DVec3)> {
    points
        .windows(2)
        .filter_map(|pair| {
            let start = state.where_it_shows(pair[0])?;
            let end = state.where_it_shows(pair[1])?;
            let along = end - start;
            let length = along.length_sq();
            let t = match length > 0.0 {
                true => ((at - start).dot(along) / length).clamp(0.0, 1.0),
                false => 0.0,
            };
            let distance = (start + along * t).distance(at);
            Some((distance, pair[0].lerp(pair[1], f64::from(t))))
        })
        .min_by(|one, other| one.0.total_cmp(&other.0))
}

/// Whether nothing of the part stands between the eye and this point.
fn in_sight(state: &ViewportState, body: &Body, point: DVec3, rect: egui::Rect) -> bool {
    let Some(on_screen) = state.where_it_shows(point) else {
        return false;
    };
    let (origin, direction) = state
        .camera
        .ray(to_ndc(on_screen, rect), rect.aspect_ratio());
    let (origin, direction) = (origin.as_dvec3(), direction.as_dvec3().normalize());
    let along = (point - origin).dot(direction);
    body.ray_hit(origin, direction)
        .is_none_or(|hit| hit.distance >= along - IN_FRONT * along.abs().max(1.0))
}
