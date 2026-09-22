use cao_part::PartDocument;
use cao_render::{CubeFace, CubeZone, OrbitCamera, SceneFrame, Vertex, ViewportRect};
use cao_sketch::{Sketch, sweep_of};
use glam::{DVec2, Vec3};

/// What a picture of a part is made of: its matter and its drawings, and
/// nothing else.
///
/// No grid, no axes, no orientation cube. At this size they are noise in front
/// of the one thing the picture exists to show, and a folder of them would
/// read as a folder of grids.
pub fn of(document: &PartDocument, side: u32) -> SceneFrame {
    let matter: Vec<[Vec3; 3]> = document
        .body()
        .triangles()
        .iter()
        .map(|corners| corners.map(|corner| corner.as_vec3()))
        .collect();

    let mut lines = Vec::new();
    for sketch in document.sketches() {
        push_drawing(&mut lines, sketch);
    }

    // Framed on what is drawn, and only on that. A sketch holds points nothing
    // shows — the centre an arc turns about, the corner a rectangle was pulled
    // from — and framing on those leaves the drawing a speck in the middle of
    // a box it never filled.
    let camera = looking_at(extent(&lines, &matter));
    let mut solids = Vec::new();
    cao_render::push_solid(&mut solids, &matter, MATTER, camera.forward());

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

/// The default 3D view, looking at the part from the corner the application
/// opens on.
fn looking_at((centre, radius): (Vec3, f32)) -> OrbitCamera {
    let mut camera = OrbitCamera::default();
    let corner = CubeZone::corner(CubeFace::PlusX, CubeFace::MinusY, CubeFace::PlusZ);
    let (yaw, pitch) = cao_render::camera::view_angles_towards(corner.direction());
    camera.set_view_angles(yaw, pitch);
    camera.focus_on(centre, radius, 1.0);
    camera
}

/// The middle of what is drawn and how far it reaches, in world units.
fn extent(lines: &[Vertex], matter: &[[Vec3; 3]]) -> (Vec3, f32) {
    let mut low = Vec3::splat(f32::INFINITY);
    let mut high = Vec3::splat(f32::NEG_INFINITY);
    let mut seen = |point: Vec3| {
        low = low.min(point);
        high = high.max(point);
    };

    for vertex in lines {
        seen(Vec3::from(vertex.position));
    }
    for corners in matter {
        for corner in corners {
            seen(*corner);
        }
    }

    if low.x > high.x {
        return (Vec3::ZERO, 1.0);
    }
    // A part of no size at all — a single point, a circle of nothing — still
    // has to be looked at from somewhere.
    let reach = ((high - low).length() * 0.5).max(f32::EPSILON);
    ((low + high) * 0.5, reach)
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
    for (id, _) in sketch.live_ellipses() {
        for pair in sketch.ellipse_polyline(id).windows(2) {
            push_step(out, at(pair[0]), at(pair[1]));
        }
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
mod tests;
