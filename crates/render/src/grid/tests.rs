//! What the grid of a work plane is held to.
//!
//! Closes #318.
//! - a sketch on an offset or inclined face shows its grid on that face —
//!   `a_grid_lies_on_the_plane_it_was_given`
//! - its lines are counted from the sketch's own origin —
//!   `lines_are_counted_from_the_plane_origin`
//! - the three base planes look exactly as today — their origin is the world
//!   origin, which is what every test here already draws on:
//!   `grid_skips_the_lines_the_axes_already_draw`

use super::*;

/// The XY plane of the world, which is the only one the grid was ever drawn on
/// before a face of the part could carry one.
fn world_xy() -> GridPlane {
    GridPlane {
        origin: Vec3::ZERO,
        u: Vec3::X,
        v: Vec3::Y,
    }
}

fn on_the_step(distance: f32, step: f32) -> bool {
    (distance / step - (distance / step).round()).abs() < 1e-3
}

#[test]
fn a_grid_lies_on_the_plane_it_was_given() {
    let normal = Vec3::new(1.0, 1.0, 1.0).normalize();
    let u = Vec3::new(1.0, -1.0, 0.0).normalize();
    let plane = GridPlane {
        origin: normal * 12.0,
        u,
        v: normal.cross(u),
    };
    let mut vertices = Vec::new();

    push_grid(
        &mut vertices,
        plane,
        plane.origin + plane.u * 37.0 - plane.v * 14.0,
        10.0,
        100.0,
        &GridStyle::default(),
    );

    assert!(!vertices.is_empty());
    for vertex in &vertices {
        let height = (Vec3::from_array(vertex.position) - plane.origin).dot(normal);
        assert!(
            height.abs() < 1e-2,
            "a grid line stands {height} away from the plane it is drawn for",
        );
    }
}

#[test]
fn lines_are_counted_from_the_plane_origin() {
    let step = 10.0;
    let plane = GridPlane {
        origin: Vec3::new(3.0, -7.0, 12.0),
        u: Vec3::X,
        v: Vec3::Y,
    };
    let mut vertices = Vec::new();

    push_grid(
        &mut vertices,
        plane,
        plane.origin + Vec3::new(41.0, 23.0, 0.0),
        step,
        100.0,
        &GridStyle::default(),
    );

    assert!(!vertices.is_empty());
    for vertex in &vertices {
        let offset = Vec3::from_array(vertex.position) - plane.origin;
        assert!(
            on_the_step(offset.dot(plane.u), step) || on_the_step(offset.dot(plane.v), step),
            "a grid line runs {offset:?} from the sketch's origin, off its {step} steps",
        );
    }
}

/// The step must always keep lines at least `target` pixels apart, and
/// only ever be a 1, 2 or 5 times a power of ten.
#[test]
fn adaptive_step_follows_the_1_2_5_sequence() {
    let target = 48.0;
    let mut units_per_pixel = 1e-4;

    while units_per_pixel < 1e4 {
        let step = adaptive_step(units_per_pixel, target);
        assert!(
            step >= units_per_pixel * target,
            "step {step} too small for {units_per_pixel}"
        );

        let mantissa = step / 10f32.powf(step.log10().floor());
        assert!(
            [1.0, 2.0, 5.0]
                .iter()
                .any(|value| (mantissa - value).abs() < 1e-3),
            "step {step} is not a 1/2/5 multiple"
        );

        units_per_pixel *= 1.3;
    }
}

/// Zooming in never coarsens the grid, zooming out never refines it.
#[test]
fn adaptive_step_grows_with_distance() {
    let mut previous = 0.0;
    for exponent in -4..4 {
        let step = adaptive_step(10f32.powi(exponent), 48.0);
        assert!(step >= previous);
        previous = step;
    }
}

/// Heavy lines must sit on world multiples of the major step whatever the
/// grid is centred on, otherwise they drift away from the axes on a pan.
#[test]
fn major_lines_stay_anchored_to_the_origin_when_panning() {
    let style = GridStyle::default();
    let step = 10.0;
    let major_step = step * style.major_every as f32;

    for center in [
        Vec3::ZERO,
        Vec3::new(37.0, -114.0, 0.0),
        Vec3::new(-950.0, 620.0, 0.0),
    ] {
        let mut vertices = Vec::new();
        push_grid(&mut vertices, world_xy(), center, step, 300.0, &style);

        let major_lines: Vec<_> = vertices
            .iter()
            .filter(|vertex| vertex.width == style.major_width)
            .collect();
        assert!(
            !major_lines.is_empty(),
            "no major line for centre {center:?}"
        );

        for vertex in major_lines {
            // A heavy line runs along one axis, so exactly one of its two
            // in-plane coordinates is the constant that must land on the
            // major step.
            let [x, y, _] = vertex.position;
            let on_x = (x / major_step - (x / major_step).round()).abs() < 1e-3;
            let on_y = (y / major_step - (y / major_step).round()).abs() < 1e-3;
            assert!(
                on_x || on_y,
                "major line vertex at ({x}, {y}) is off the {major_step} grid"
            );
        }
    }
}

#[test]
fn grid_is_opaque_around_its_centre() {
    let mut vertices = Vec::new();
    push_grid(
        &mut vertices,
        world_xy(),
        Vec3::ZERO,
        10.0,
        300.0,
        &GridStyle::default(),
    );

    let near_center = vertices
        .iter()
        .filter(|vertex| Vec3::from_array(vertex.position).length() < 100.0)
        .count();
    assert!(near_center > 0);
    assert!(
        vertices
            .iter()
            .filter(|vertex| Vec3::from_array(vertex.position).length() < 100.0)
            .all(|vertex| vertex.color[3] > 0.2),
        "the grid must not fade out right next to its centre"
    );
}

#[test]
fn grid_skips_the_lines_the_axes_already_draw() {
    let mut vertices = Vec::new();
    push_grid(
        &mut vertices,
        world_xy(),
        Vec3::ZERO,
        10.0,
        50.0,
        &GridStyle::default(),
    );
    assert!(!vertices.is_empty());

    // A segment lying flat on an axis would double up the coloured axis
    // line; individual vertices may still touch an axis when a line
    // crosses it.
    for segment in vertices.as_chunks::<2>().0 {
        let [start, end] = [segment[0].position, segment[1].position];
        assert!(
            !(start[0] == 0.0 && end[0] == 0.0) && !(start[1] == 0.0 && end[1] == 0.0),
            "grid segment {start:?}..{end:?} lies on an axis"
        );
    }
}
