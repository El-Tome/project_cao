//! What app · screens/viewport/section.rs is held to.
//!
//! Closes #193 — the whole transcription is in
//! `crates/app/src/screens/viewport/render/tests.rs`; these bullets are the
//! ones the panel and the handle answer.
//! - the panel waits for a flat face to be clicked once « Face cliquée » is
//!   chosen, until one is or a plane is chosen instead —
//!   `the_panel_waits_for_a_face_until_one_is_clicked_or_a_plane_chosen`
//! - a handle on the plane is pulled in the view: where a pointer ray lands
//!   along the normal, the closest point between a ray and a line —
//!   `a_ray_across_the_normal_pulls_the_handle_to_the_closest_point`,
//!   `a_ray_along_the_normal_pulls_the_handle_nowhere`; only a ray near the
//!   handle takes hold of it — `a_ray_far_from_the_handle_takes_no_hold_of_it`
//! - the distance typed and the handle each follow the other —
//!   `the_handle_stands_where_the_distance_typed_puts_the_plane`,
//!   `pulling_the_handle_changes_the_distance_the_panel_shows`
//! - each side's state is hidden, see-through or opaque, stepped round on its
//!   own in the panel — `each_side_steps_round_its_three_states_on_its_own`

use cao_render::MatterInFront;
use cao_sketch::WorkPlane;
use cao_solid::Body;
use cao_solid::profile::{Contour, Frame, Profile};
use glam::{DVec2, DVec3};

use super::*;

const TOLERANCE: f64 = 1e-9;

/// A block 10 by 10 from 0 to 4 over XY, raised by the exact kernel: its
/// middle is `(5, 5, 2)`.
fn block() -> Body {
    let corners = [
        DVec2::ZERO,
        DVec2::new(10.0, 0.0),
        DVec2::new(10.0, 10.0),
        DVec2::new(0.0, 10.0),
    ];
    let triangles = [
        [corners[0], corners[1], corners[2]],
        [corners[0], corners[2], corners[3]],
    ];
    let profile = Profile {
        exact: Some((
            Contour::rectangle(DVec2::ZERO, DVec2::new(10.0, 10.0)),
            Vec::new(),
        )),
        sampled: cao_solid::Loop::straight(&corners),
        sampled_holes: Vec::new(),
        triangles: &triangles,
    };
    let frame = Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    };
    Body::default()
        .tool_raised(&profile, frame, DVec3::Z * 4.0)
        .expect("the exact kernel raises a lone profile")
}

fn close_to(found: f64, wanted: f64) -> bool {
    (found - wanted).abs() <= TOLERANCE
}

#[test]
fn a_ray_across_the_normal_pulls_the_handle_to_the_closest_point() {
    let up_through_the_origin = (DVec3::ZERO, DVec3::Z);
    let up_through_the_middle = (DVec3::new(5.0, 5.0, 0.0), DVec3::Z);
    let slanted = DVec3::new(-1.0, 0.0, 1.0).normalize();

    for (line, ray, wanted) in [
        (
            up_through_the_origin,
            (DVec3::new(10.0, 0.0, 3.0), DVec3::NEG_X),
            3.0,
        ),
        (
            up_through_the_origin,
            (DVec3::new(10.0, 5.0, 7.0), DVec3::NEG_X),
            7.0,
        ),
        (
            up_through_the_origin,
            (DVec3::new(10.0, 2.0, 0.0), slanted),
            10.0,
        ),
        (
            up_through_the_origin,
            (DVec3::new(0.0, -20.0, -4.0), DVec3::Y),
            -4.0,
        ),
        (
            up_through_the_middle,
            (DVec3::new(5.0, -20.0, 4.0), DVec3::Y),
            4.0,
        ),
    ] {
        let found = closest_along(line, ray);

        assert!(
            found.is_some_and(|found| close_to(found, wanted)),
            "a ray from {} along {} lands at {found:?} along the line from {} along {}, not {wanted}",
            ray.0,
            ray.1,
            line.0,
            line.1,
        );
    }
}

#[test]
fn a_ray_along_the_normal_pulls_the_handle_nowhere() {
    let line = (DVec3::ZERO, DVec3::Z);

    for direction in [DVec3::Z, DVec3::NEG_Z] {
        assert_eq!(
            closest_along(line, (DVec3::new(3.0, 0.0, 10.0), direction)),
            None,
            "a ray running alongside the normal has no one point closest to it",
        );
    }
}

#[test]
fn the_handle_stands_where_the_distance_typed_puts_the_plane() {
    let body = block();
    let top = WorkPlane::from_normal(DVec3::Z * 4.0, DVec3::Z);

    for (plane, offset, wanted) in [
        (WorkPlane::XY, 3.0, DVec3::new(5.0, 5.0, 3.0)),
        (WorkPlane::YZ, 7.0, DVec3::new(7.0, 5.0, 2.0)),
        (top, -1.0, DVec3::new(5.0, 5.0, 3.0)),
    ] {
        let mut section = Section::new(plane);
        section.offset = offset;

        let handle = section.handle(&body);

        assert!(
            (handle - wanted).length() <= TOLERANCE,
            "a distance of {offset} from {plane:?} puts the handle at {handle}, \
             not on the cutting plane over the middle of the part, {wanted}",
        );
    }
}

#[test]
fn pulling_the_handle_changes_the_distance_the_panel_shows() {
    let body = block();
    let mut section = Section::new(WorkPlane::XY);
    section.offset = 2.0;

    assert!(
        section.grab(&body, DVec3::new(20.0, 5.0, 2.2), DVec3::NEG_X, 0.5),
        "a ray passing a fifth of a unit from the handle does not take hold of it",
    );
    section.pull(DVec3::new(20.0, 5.0, 3.7), DVec3::NEG_X);
    let pulled_up = section.offset;
    section.pull(DVec3::new(20.0, 5.0, 0.7), DVec3::NEG_X);
    let pulled_down = section.offset;
    section.let_go();
    section.pull(DVec3::new(20.0, 5.0, 3.0), DVec3::NEG_X);

    assert!(
        close_to(pulled_up, 3.5),
        "pulled 1.5 up from where it was taken, the distance reads {pulled_up}, not 3.5: \
         the handle jumps to the pointer, or does not follow it",
    );
    assert!(
        close_to(pulled_down, 0.5),
        "pulled back down, the distance reads {pulled_down}, not 0.5",
    );
    assert!(
        close_to(section.offset, 0.5),
        "let go, the handle still follows the pointer to {}",
        section.offset,
    );
}

#[test]
fn a_ray_far_from_the_handle_takes_no_hold_of_it() {
    let body = block();
    let mut section = Section::new(WorkPlane::XY);
    section.offset = 2.0;

    let above = section.grab(&body, DVec3::new(20.0, 5.0, 3.0), DVec3::NEG_X, 0.5);
    let aside = section.grab(&body, DVec3::new(20.0, 9.0, 2.0), DVec3::NEG_X, 0.5);
    section.pull(DVec3::new(20.0, 5.0, 3.5), DVec3::NEG_X);

    assert!(
        !above,
        "a ray passing a unit over the handle takes hold of it"
    );
    assert!(
        !aside,
        "a ray passing four units beside the handle takes hold of it"
    );
    assert!(
        close_to(section.offset, 2.0),
        "with no hold taken, pulling moved the distance to {}",
        section.offset,
    );
}

#[test]
fn each_side_steps_round_its_three_states_on_its_own() {
    let mut section = Section::new(WorkPlane::XY);
    let mut in_front = vec![section.in_front];
    for _ in 0..3 {
        section.next_in_front();
        in_front.push(section.in_front);
    }
    let behind_while_in_front_moved = section.behind;
    section.next_behind();

    assert_eq!(
        in_front,
        [
            MatterInFront::Hidden,
            MatterInFront::SeeThrough,
            MatterInFront::Opaque,
            MatterInFront::Hidden,
        ],
        "a click on the side in front does not step it hidden, see-through, opaque",
    );
    assert_eq!(
        behind_while_in_front_moved,
        MatterInFront::Opaque,
        "stepping the side in front moved the side behind",
    );
    assert_eq!(
        (section.behind, section.in_front),
        (MatterInFront::Hidden, MatterInFront::Hidden),
        "a click on the side behind does not step it from opaque to hidden alone",
    );
}

#[test]
fn the_panel_waits_for_a_face_until_one_is_clicked_or_a_plane_chosen() {
    let mut section = Section::new(WorkPlane::XY);
    let at_first = section.is_waiting_for_a_face();

    section.wait_for_a_face();
    let asked = section.is_waiting_for_a_face();
    section.choose_plane(WorkPlane::YZ);

    assert!(
        !at_first,
        "a section just opened waits for a face nobody asked for"
    );
    assert!(
        asked,
        "« Face cliquée » chosen, the panel does not wait for a face"
    );
    assert!(
        !section.is_waiting_for_a_face(),
        "a plane chosen in the panel leaves it still waiting for a face",
    );
    assert_eq!(
        section.plane,
        WorkPlane::YZ,
        "the plane chosen is not the one cut along"
    );
}
