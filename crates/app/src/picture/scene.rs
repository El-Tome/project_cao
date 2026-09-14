use cao_part::PartDocument;
use cao_render::{CubeFace, CubeZone, OrbitCamera, SceneFrame, Vertex, ViewportRect};
use cao_sketch::{Sketch, sweep_of};
use glam::{DVec2, DVec3, Vec3};

/// What a picture of a part is made of: its matter and its drawings, and
/// nothing else.
///
/// No grid, no axes, no orientation cube. At this size they are noise in front
/// of the one thing the picture exists to show, and a folder of them would
/// read as a folder of grids.
pub fn of(document: &PartDocument, side: u32) -> SceneFrame {
    let camera = looking_at(document);
    let mut solids = Vec::new();
    cao_render::push_solid(
        &mut solids,
        &document
            .body()
            .triangles()
            .iter()
            .map(|corners| corners.map(|corner| corner.as_vec3()))
            .collect::<Vec<_>>(),
        MATTER,
        camera.forward(),
    );

    let mut lines = Vec::new();
    for sketch in document.sketches() {
        push_drawing(&mut lines, sketch);
    }

    let whole = ViewportRect {
        x: 0.0,
        y: 0.0,
        width: side as f32,
        height: side as f32,
    };
    SceneFrame {
        scene_view_projection: camera.view_projection(1.0),
        scene_viewport: whole,
        scene_background: Vec::new(),
        scene_world_lines: Vec::new(),
        scene_surfaces: Vec::new(),
        scene_solids: solids,
        scene_lines: lines,
        cube_view_projection: glam::Mat4::IDENTITY,
        cube_triangles: Vec::new(),
        cube_edges: Vec::new(),
        cube_viewport: whole,
    }
}

/// The colour of matter and of a trait in a picture. Not the theme's: a
/// picture outlives the setting it was taken under, and a folder whose rows
/// were taken under four themes reads as four folders.
const MATTER: [f32; 4] = [0.78, 0.80, 0.84, 1.0];
const TRAIT: [f32; 4] = [0.55, 0.75, 1.0, 1.0];
const TRAIT_WIDTH: f32 = 1.5;

/// How many straight steps a whole turn is drawn with. A curve at this size is
/// a few pixels across, and a step finer than a pixel is bytes nobody sees.
const STEPS_OF_A_TURN: usize = 48;

/// The default 3D view, framed on whatever the part has: its matter, or the
/// drawings when nothing has been extruded yet, or the origin when it is
/// empty.
fn looking_at(document: &PartDocument) -> OrbitCamera {
    let mut camera = OrbitCamera::default();
    let corner = CubeZone::corner(CubeFace::PlusX, CubeFace::MinusY, CubeFace::PlusZ);
    let (yaw, pitch) = cao_render::camera::view_angles_towards(corner.direction());
    camera.set_view_angles(yaw, pitch);

    let (centre, radius) = extent(document);
    camera.focus_on(centre, radius, 1.0);
    camera
}

/// The middle of the part and how far it reaches, in world units.
fn extent(document: &PartDocument) -> (Vec3, f32) {
    let mut low = DVec3::splat(f64::INFINITY);
    let mut high = DVec3::splat(f64::NEG_INFINITY);

    if let Some((min, max)) = document.body().bounds() {
        low = low.min(min);
        high = high.max(max);
    }
    for sketch in document.sketches() {
        if let Some((min, max)) = sketch.bounds() {
            low = low.min(sketch.plane.to_world(min));
            high = high.max(sketch.plane.to_world(max));
        }
    }

    if low.x > high.x {
        return (Vec3::ZERO, 1.0);
    }
    let reach = ((high - low).length() * 0.5) as f32;
    // A part of no size at all — one point, one circle of nothing — still has
    // to be looked at from somewhere.
    (((low + high) * 0.5).as_vec3(), reach.max(f32::EPSILON))
}

fn push_drawing(out: &mut Vec<Vertex>, sketch: &Sketch) {
    let at = |point: DVec2| sketch.plane.to_world(point).as_vec3();

    for (id, _) in sketch.live_segments() {
        let (start, end) = sketch.endpoints(id);
        push_step(out, at(start), at(end));
    }
    for (_, circle) in sketch.live_circles() {
        let centre = sketch.point(circle.center);
        push_curve(out, centre, circle.radius, 0.0, std::f64::consts::TAU, &at);
    }
    for (id, _) in sketch.live_arcs() {
        let drawn = sketch.arc_draft(id);
        let radius = (drawn.start - drawn.centre).length();
        let from = (drawn.start - drawn.centre).to_angle();
        push_curve(out, drawn.centre, radius, from, sweep_of(drawn), &at);
    }
}

/// A curve as the straight steps a line renderer can draw, laid on the
/// sketch's own plane before being sent to the world.
fn push_curve(
    out: &mut Vec<Vertex>,
    centre: DVec2,
    radius: f64,
    from: f64,
    sweep: f64,
    at: &impl Fn(DVec2) -> Vec3,
) {
    if radius <= 0.0 || sweep <= 0.0 {
        return;
    }
    let steps = ((sweep / std::f64::consts::TAU) * STEPS_OF_A_TURN as f64).ceil() as usize;
    let steps = steps.max(2);
    let round = |step: usize| {
        let angle = from + sweep * step as f64 / steps as f64;
        at(centre + DVec2::new(angle.cos(), angle.sin()) * radius)
    };
    for step in 0..steps {
        push_step(out, round(step), round(step + 1));
    }
}

fn push_step(out: &mut Vec<Vertex>, from: Vec3, to: Vec3) {
    out.push(Vertex::line(from, TRAIT, TRAIT_WIDTH));
    out.push(Vertex::line(to, TRAIT, TRAIT_WIDTH));
}

#[cfg(test)]
mod tests {
    use super::*;

    use cao_part::{Operation, PointRef};
    use cao_sketch::WorkPlane;

    fn at(text: &str) -> chrono::DateTime<chrono::Utc> {
        text.parse().expect("a date")
    }

    fn part_with(operations: Vec<Operation>) -> PartDocument {
        let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
        for operation in operations {
            document.apply(operation);
        }
        document
    }

    fn a_square() -> Vec<Operation> {
        let corners = [
            (DVec2::ZERO, DVec2::new(10.0, 0.0)),
            (DVec2::new(10.0, 0.0), DVec2::new(10.0, 10.0)),
            (DVec2::new(10.0, 10.0), DVec2::new(0.0, 10.0)),
            (DVec2::new(0.0, 10.0), DVec2::ZERO),
        ];
        let mut drawn = vec![Operation::CreateSketch {
            plane: WorkPlane::XY,
        }];
        drawn.extend(corners.map(|(start, end)| Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(start),
            end: PointRef::New(end),
            construction: false,
        }));
        drawn
    }

    #[test]
    fn a_drawn_trait_reaches_the_picture_as_a_pair_of_ends() {
        let frame = of(&part_with(a_square()), 128);

        assert_eq!(
            frame.scene_lines.len(),
            8,
            "four traits, two ends each, and nothing else drawn",
        );
    }

    #[test]
    fn a_picture_carries_no_grid_no_axes_and_no_cube() {
        let frame = of(&part_with(a_square()), 128);

        assert!(frame.scene_world_lines.is_empty(), "the grid and the axes");
        assert!(frame.cube_triangles.is_empty());
        assert!(frame.cube_edges.is_empty());
        assert!(frame.scene_background.is_empty());
    }

    #[test]
    fn a_circle_reaches_the_picture_as_straight_steps_that_close_on_themselves() {
        let drawn = vec![
            Operation::CreateSketch {
                plane: WorkPlane::XY,
            },
            Operation::AddCircle {
                sketch: 0,
                center: PointRef::New(DVec2::ZERO),
                radius: 5.0,
                rim: Vec::new(),
                construction: false,
            },
        ];

        let frame = of(&part_with(drawn), 128);

        const TOLERANCE: f32 = 1e-5;
        assert_eq!(frame.scene_lines.len(), STEPS_OF_A_TURN * 2);
        let first = Vec3::from(frame.scene_lines[0].position);
        let last = Vec3::from(frame.scene_lines[frame.scene_lines.len() - 1].position);
        assert!(
            first.distance(last) < TOLERANCE,
            "the last step lands at {last} where the first one started at {first}",
        );
    }

    #[test]
    fn an_empty_part_is_looked_at_from_somewhere_rather_than_from_nowhere() {
        let frame = of(&PartDocument::new("Test", at("2026-01-02T09:00:00Z")), 128);

        assert!(frame.scene_lines.is_empty());
        assert!(
            frame.scene_view_projection.is_finite(),
            "a camera framed on nothing divides by nothing and the picture is blank",
        );
    }

    #[test]
    fn a_drawing_is_framed_so_that_every_corner_of_it_lands_inside_the_picture() {
        let frame = of(&part_with(a_square()), 128);

        for corner in [
            DVec3::ZERO,
            DVec3::new(10.0, 0.0, 0.0),
            DVec3::new(10.0, 10.0, 0.0),
            DVec3::new(0.0, 10.0, 0.0),
        ] {
            let clipped = frame.scene_view_projection * corner.as_vec3().extend(1.0);
            let ndc = clipped.truncate() / clipped.w;
            assert!(
                ndc.x.abs() <= 1.0 && ndc.y.abs() <= 1.0,
                "the corner at {corner} falls outside the picture, at {ndc}",
            );
        }
    }

    #[test]
    fn a_part_that_has_been_extruded_shows_its_matter() {
        let mut drawn = vec![
            Operation::CreateSketch {
                plane: WorkPlane::XY,
            },
            Operation::AddRectangle {
                sketch: 0,
                corner: PointRef::New(DVec2::ZERO),
                opposite: PointRef::New(DVec2::new(10.0, 10.0)),
                construction: false,
            },
        ];
        drawn.push(Operation::Extrude {
            sketch: 0,
            picks: vec![DVec2::new(5.0, 5.0)],
            distance: 4.0,
            mode: cao_part::ExtrusionMode::Add,
        });

        let frame = of(&part_with(drawn), 128);

        assert!(
            !frame.scene_solids.is_empty(),
            "the matter the extrusion made is what the picture is mostly for",
        );
    }
}
