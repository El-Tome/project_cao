//! What the arc tool shows before its curve is drawn: a radius or a distance
//! typed while the second place is picked, and while the third is, the angle a
//! `ByCenter` arc sweeps or the radius a `ByEnds` one is bent to.

use cao_sketch::{ArcMode, Sketch, arc_angle_reference, sweep_of};
use glam::DVec2;

use super::curves::push_arc_at;
use super::{push_point_marker, push_preview_line};
use crate::screens::viewport::input::{arc_aimed, arc_preview};
use crate::screens::viewport::{SketchContext, ViewScale};

pub(crate) fn live_fields(
    context: &SketchContext<'_>,
    cursor: DVec2,
) -> Option<([&'static str; 2], [f64; 2])> {
    let places = context.editor.arc_places();
    match (context.editor.arc_mode, places.len()) {
        (ArcMode::ByCenter, 1) | (ArcMode::ByEnds, 1) => {
            let aimed = arc_aimed(context, places, cursor);
            let distance = places[0].distance(aimed) * context.document.scale();
            Some((["mm", ""], [distance, 0.0]))
        }
        (ArcMode::ByCenter, 2) => {
            let drawn = arc_preview(context, cursor)?;
            Some((["°", ""], [sweep_of(drawn).to_degrees(), 0.0]))
        }
        (ArcMode::ByEnds, 2) => {
            let drawn = arc_preview(context, cursor)?;
            let radius = drawn.centre.distance(drawn.start) * context.document.scale();
            Some((["mm", ""], [radius, 0.0]))
        }
        _ => None,
    }
}

/// The curve a click right now would draw, the places it stands on, and — while
/// a by-centre arc is being swept — the leg its angle opens from.
pub(crate) fn push_preview(
    out: &mut Vec<cao_render::Vertex>,
    context: &SketchContext<'_>,
    sketch: &Sketch,
    cursor: DVec2,
    preview: [f32; 4],
    scale: ViewScale,
) {
    let marker = scale.world_size_of(3.0);
    match arc_preview(context, cursor) {
        Some(drawn) => {
            push_arc_at(
                out,
                sketch,
                drawn,
                preview,
                1.5,
                context.editor.construction,
                scale,
            );
            for place in [drawn.centre, drawn.start, drawn.end] {
                push_point_marker(out, sketch, place, marker, preview, 1.5);
            }
        }
        // A single place given so far is not a curve yet, whichever mode: a
        // centre alone has no radius, and one end alone has no distance to
        // the other. What follows the cursor — aimed, so a typed value bends
        // it exactly as far as the field says rather than wherever the mouse
        // happens to sit — is the reach the next click is about to fix.
        // Past two places, `None` means the places chosen do not bend into an
        // arc at all, which the blocking message already says; nothing here
        // should look like one.
        None => {
            if let [only] = context.editor.arc_places() {
                let reach = arc_aimed(context, context.editor.arc_places(), cursor);
                push_preview_line(out, sketch, *only, reach, preview, false, scale);
                push_point_marker(out, sketch, *only, marker, preview, 1.5);
            }
        }
    }
    if let Some((centre, towards)) =
        arc_angle_reference(context.editor.arc_mode, context.editor.arc_places())
    {
        push_preview_line(out, sketch, centre, towards, preview, true, scale);
    }
}

#[cfg(test)]
mod tests {
    use cao_part::PartDocument;
    use cao_prefs::config::ViewportConfig;
    use cao_render::camera::OrbitCamera;
    use cao_sketch::{ToolState, WorkPlane};
    use chrono::Utc;
    use glam::Vec3;

    use super::*;
    use crate::lang::Catalogue;
    use crate::screens::extrusion::ExtrusionState;
    use crate::screens::sketch::SketchEditor;

    /// How far two places may be apart and still count as one, and how far off
    /// a line one may sit and still count as on it. Well under the few pixels a
    /// point marker's square is wide, so a marker beside a leg is never
    /// mistaken for part of it.
    const TOLERANCE: f64 = 1e-6;

    /// One straight step of what was painted, in sketch coordinates.
    #[derive(Clone, Copy, Debug)]
    struct Step {
        from: DVec2,
        to: DVec2,
    }

    /// Every step a run of vertices holds.
    fn steps(painted: &[cao_render::Vertex], sketch: &Sketch) -> Vec<Step> {
        painted
            .as_chunks::<2>()
            .0
            .iter()
            .map(|[from, to]| Step {
                from: place(*from, sketch),
                to: place(*to, sketch),
            })
            .collect()
    }

    /// The steps that run along the way from one place to another: on that line,
    /// and within its span. A dashed leg comes back as several of them, a plain
    /// one as a single step.
    fn along(steps: &[Step], from: DVec2, to: DVec2) -> Vec<Step> {
        let span = to - from;
        let length = span.length();
        if length < TOLERANCE {
            return Vec::new();
        }
        let direction = span / length;
        steps
            .iter()
            .copied()
            .filter(|step| {
                [step.from, step.to].into_iter().all(|place| {
                    let offset = place - from;
                    let reach = offset.dot(direction);
                    (offset - direction * reach).length() < TOLERANCE
                        && (-TOLERANCE..=length + TOLERANCE).contains(&reach)
                })
            })
            .collect()
    }

    fn place(vertex: cao_render::Vertex, sketch: &Sketch) -> DVec2 {
        sketch
            .plane
            .to_local(Vec3::from(vertex.position).as_dvec3())
    }

    /// What one frame of the canvas paints for an arc part-way through being
    /// placed, read back as the straight steps it is made of.
    fn a_preview(mode: ArcMode, places: Vec<DVec2>, cursor: DVec2) -> Vec<Step> {
        a_preview_locked(mode, places, cursor, None)
    }

    /// The same, with a value typed into the live field as the next click
    /// would find it.
    fn a_preview_locked(
        mode: ArcMode,
        places: Vec<DVec2>,
        cursor: DVec2,
        locked: Option<f64>,
    ) -> Vec<Step> {
        let mut document = PartDocument::new("part", Utc::now());
        let mut editor = SketchEditor::default();
        let mut extrusion = ExtrusionState::default();
        let lang = Catalogue::french();
        editor.arc_mode = mode;
        editor.tool_state = ToolState::Arc {
            places,
            first_typed: false,
        };
        editor.live.field(0).locked = locked;

        let millimetres = document.scale();
        let context = SketchContext {
            document: &mut document,
            editor: &mut editor,
            extrusion: &mut extrusion,
            lang: &lang,
        };
        let scale = ViewScale::of(
            &OrbitCamera::default(),
            egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1280.0, 800.0)),
            1.0,
            &ViewportConfig::default(),
            millimetres,
        );

        let sketch = Sketch::new(WorkPlane::XY);
        let mut out = Vec::new();
        push_preview(&mut out, &context, &sketch, cursor, [1.0; 4], scale);
        steps(&out, &sketch)
    }

    #[test]
    fn a_by_centre_arc_waiting_for_its_sweep_shows_the_leg_that_angle_opens_from() {
        let (centre, start) = (DVec2::ZERO, DVec2::new(100.0, 0.0));

        let painted = a_preview(
            ArcMode::ByCenter,
            vec![centre, start],
            DVec2::new(0.0, 80.0),
        );
        let leg = along(&painted, centre, start);

        assert!(
            !leg.is_empty(),
            "nothing runs from the centre to the end already picked",
        );
        assert!(
            leg.len() > 1,
            "the leg came out whole, so it is drawn solid rather than dashed",
        );
        assert!(
            leg.iter()
                .any(|step| step.from.distance(centre) < TOLERANCE),
            "no dash starts at the centre, so the angle opens from nowhere",
        );
    }

    #[test]
    fn an_arc_bent_by_its_ends_shows_no_such_leg_because_it_measures_no_angle() {
        let (a, b) = (DVec2::new(-50.0, 0.0), DVec2::new(50.0, 0.0));

        let painted = a_preview(ArcMode::ByEnds, vec![a, b], DVec2::new(0.0, 40.0));

        assert!(
            along(&painted, a, b).is_empty(),
            "something was drawn along the chord, and a by-ends arc opens from nothing",
        );
    }

    #[test]
    fn the_curve_shown_before_the_last_click_runs_at_one_reach_from_the_centre() {
        let (centre, start) = (DVec2::ZERO, DVec2::new(100.0, 0.0));
        let radius = centre.distance(start);

        let painted = a_preview(
            ArcMode::ByCenter,
            vec![centre, start],
            DVec2::new(0.0, 80.0),
        );
        let curve: Vec<Step> = painted
            .iter()
            .copied()
            .filter(|step| {
                (step.from.distance(centre) - radius).abs() < TOLERANCE
                    && (step.to.distance(centre) - radius).abs() < TOLERANCE
            })
            .collect();

        assert!(
            curve.len() > 1,
            "the curve came out in {} step(s), which is no curve",
            curve.len(),
        );
        assert!(
            curve
                .iter()
                .any(|step| step.from.distance(start) < TOLERANCE),
            "the curve does not set off from the end already picked",
        );
        assert!(
            curve
                .iter()
                .any(|step| step.to.distance(DVec2::new(0.0, radius)) < TOLERANCE),
            "the curve does not reach round to where the cursor points",
        );
    }

    #[test]
    fn an_arc_given_only_its_centre_shows_the_reach_it_is_about_to_be_drawn_at() {
        let (centre, cursor) = (DVec2::ZERO, DVec2::new(60.0, 0.0));

        let painted = a_preview(ArcMode::ByCenter, vec![centre], cursor);
        let reach = along(&painted, centre, cursor);

        assert_eq!(
            reach.len(),
            1,
            "the reach is not one plain step: a curve is not settled enough to be dashed",
        );
        assert!(
            reach[0].from.distance(centre) < TOLERANCE && reach[0].to.distance(cursor) < TOLERANCE,
            "the reach runs from {:?} to {:?} instead of the centre to the cursor",
            reach[0].from,
            reach[0].to,
        );
    }

    #[test]
    fn an_arc_given_only_its_first_end_shows_the_reach_its_second_end_would_land_at() {
        let (first_end, cursor) = (DVec2::new(-50.0, 0.0), DVec2::new(50.0, 0.0));

        let painted = a_preview(ArcMode::ByEnds, vec![first_end], cursor);
        let reach = along(&painted, first_end, cursor);

        assert!(
            !reach.is_empty(),
            "nothing shows where the second end would land before it is placed",
        );
        assert!(
            painted
                .iter()
                .any(|step| step.from.distance(first_end) < 10.0
                    && step.to.distance(first_end) < 10.0),
            "nothing marks the first end, so the click that placed it leaves no trace",
        );
    }

    #[test]
    fn a_radius_typed_with_only_the_centre_placed_bends_the_reach_to_it_rather_than_the_cursor() {
        let centre = DVec2::ZERO;
        let cursor = DVec2::new(200.0, 0.0);

        let painted = a_preview_locked(ArcMode::ByCenter, vec![centre], cursor, Some(30.0));

        assert!(
            !painted
                .iter()
                .any(|step| step.to.distance(cursor) < TOLERANCE),
            "the reach still reaches all the way to the cursor, ignoring the typed radius",
        );
        assert!(
            !along(&painted, centre, DVec2::new(30.0, 0.0)).is_empty(),
            "the reach does not stop at the 30 mm the field was typed with",
        );
    }
}
