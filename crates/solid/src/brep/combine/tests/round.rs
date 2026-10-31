//! A sphere or a torus joined to or cut from what shares its axis (#576):
//! a plane square to the axis or holding it, a coaxial shaft, bore, cone,
//! sphere or torus. Each body is listed sound and holds the arithmetic's
//! volume, by Pappus, to the bit the cone's booleans hold; everything else
//! declines as unsupported, unless the faces stand clear of each other.

use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::*;
use crate::brep::turned::tests::round::{ball, bent, ring, turned};
use crate::brep::{Declined, Face};
use crate::turning::{Corner, Straight};

/// The volume of the ball of radius 5 about the origin, turned about Y.
const BALL: f64 = 500.0 * PI / 3.0;

/// The volume of the ring of tube radius 2 about the circle of radius 6 at
/// `y = 3`.
const RING: f64 = 2.0 * PI * PI * 6.0 * 4.0;

/// A shaft of `radius` about the Y axis, from `from` to `to` along it,
/// `across` from the axis along X.
fn shaft(radius: f64, from: f64, to: f64, across: f64) -> Body {
    let frame = Frame {
        origin: DVec3::Y * from,
        u: DVec3::Z,
        v: DVec3::X,
    };
    let outline = Contour::circle(DVec2::new(0.0, across), radius);
    Body::raised(&outline, &[], frame, DVec3::Y * (to - from)).expect("a shaft raises")
}

/// A profile of straight runs laid in `(h, r)`, turned whole about Y.
fn turned_whole(corners: &[[f64; 2]]) -> Body {
    let count = corners.len();
    let contour = corners
        .iter()
        .enumerate()
        .map(|(run, at)| Corner {
            at: DVec2::from(*at),
            run: run as u32,
            bend: None,
        })
        .collect();
    let last_off_the_axis = (0..count)
        .rev()
        .find(|&run| corners[run][1] != 0.0 || corners[(run + 1) % count][1] != 0.0)
        .map(|run| run as u32);
    let straight = Straight {
        side: -1.0,
        contours: vec![contour],
        runs: count as u32,
        last_off_the_axis,
    };
    turned(&straight, 360.0)
}

/// A ball of `radius` about the point of the Y axis at `h`.
fn ball_of(h: f64, radius: f64) -> Body {
    turned(
        &bent(&[&[
            ([h - radius, 0.0], Some(([h, 0.0], -PI))),
            ([h + radius, 0.0], None),
        ]]),
        360.0,
    )
}

/// A ring torus of tube radius `tube` about the circle of radius `big` at
/// `h`.
fn ring_of(h: f64, big: f64, tube: f64) -> Body {
    turned(
        &bent(&[&[
            ([h, big + tube], Some(([h, big], -PI))),
            ([h, big - tube], Some(([h, big], -PI))),
        ]]),
        360.0,
    )
}

/// The faces of a body lying on a sphere or a torus.
fn rounds(body: &Body) -> Vec<&Face> {
    body.face_ids()
        .filter(|&face| {
            matches!(
                body.surface(body.face(face).surface),
                Surface::Sphere(_) | Surface::Torus(_)
            )
        })
        .map(|face| body.face(face))
        .collect()
}

fn assert_sound(body: &Body, what: &str) {
    let listing = body.listing();
    let faces: Vec<String> = listing
        .faces
        .iter()
        .map(|face| {
            format!(
                "{:?}/{:?}/{:?}/{:?}",
                face.surface, face.outward, face.loops, face.poles
            )
        })
        .collect();
    let edges: Vec<String> = listing
        .edges
        .iter()
        .map(|edge| {
            format!(
                "{:?} {}..{} {:?}",
                edge.curve, edge.from, edge.to, edge.ends
            )
        })
        .collect();
    assert_eq!(
        listed(&listing, body.scale().reach()),
        Ok(()),
        "{what}\nFACES {faces:#?}\nEDGES {edges:#?}\nVERTICES {:?}",
        listing.vertices
    );
}

/// The body listed sound and holding `volume` to the bit.
fn held(body: &Body, volume: f64, what: &str) {
    assert_sound(body, what);
    assert!(
        (body.volume() - volume).abs() <= 1e-9 * volume.abs(),
        "{what}: the body holds {}, the arithmetic {volume}",
        body.volume()
    );
}

/// The volume of the matter a disc of the meridian half-plane, about
/// `center` and of `radius`, turns into about the axis on the side of the
/// line through `through` along `along` its normal `normal` points to, by
/// Pappus: the whole disc less the segment beyond the line where the centre
/// stands on that side, the segment alone otherwise.
fn turned_disc_beside(center: DVec2, radius: f64, (through, normal): (DVec2, DVec2)) -> f64 {
    let standing = normal.dot(center - through);
    let distance = standing.abs();
    assert!(distance < radius, "the line cuts the disc");
    let angle = 2.0 * (distance / radius).acos();
    let segment = radius * radius * (angle - angle.sin()) / 2.0;
    let centroid = 4.0 * radius * (angle / 2.0).sin().powi(3) / (3.0 * (angle - angle.sin()));
    let whole = PI * radius * radius;
    let (area, moment) = if standing >= 0.0 {
        (
            whole - segment,
            whole * center.y - segment * (center.y - normal.y * centroid),
        )
    } else {
        (segment, segment * (center.y + normal.y * centroid))
    };
    assert!(area > 0.0);
    TAU * moment
}

/// What a cap of the ball of radius 5 holds above `h`.
fn cap_above(h: f64) -> f64 {
    let height = 5.0 - h;
    PI * height * height * (15.0 - height) / 3.0
}

#[test]
fn a_ball_cut_by_a_plane_square_to_its_axis_keeps_what_lies_below_it_and_pappus_s_volume() {
    for (h, poles) in [(3.0, 1), (0.0, 1), (-2.0, 1)] {
        let slab = block([-10.0, h, -10.0], [10.0, 10.0, 10.0]);
        let cut = turned(&ball(), 360.0)
            .cut_by(&slab)
            .expect("the ball is cut");
        let what = format!("a ball cut above {h}");
        held(&cut, BALL - cap_above(h), &what);
        let [round] = rounds(&cut)[..] else {
            panic!("{what}: one round face");
        };
        assert_eq!(round.poles.len(), poles, "{what}");
        assert!(!round.flipped, "{what}");
        let joined = turned(&ball(), 360.0)
            .joined(&slab)
            .expect("the ball is joined");
        held(
            &joined,
            BALL + 400.0 * (10.0 - h) - cap_above(h),
            &format!("a ball joined to a slab above {h}"),
        );
    }
}

#[test]
fn a_plane_square_at_a_ball_s_pole_touches_it_at_a_point() {
    let eps = Scale::of(10.0).eps();
    for h in [5.0, 5.0 + eps / 2.0, 5.0 - eps / 2.0] {
        let slab = block([-10.0, h, -10.0], [10.0, 10.0, 10.0]);
        let cut = turned(&ball(), 360.0)
            .cut_by(&slab)
            .expect("the ball is cut");
        held(
            &cut,
            BALL,
            &format!("a ball cut by a slab resting on its pole at {h}"),
        );
        let joined = turned(&ball(), 360.0)
            .joined(&slab)
            .expect("the ball is joined");
        held(
            &joined,
            BALL + 400.0 * (10.0 - h),
            &format!("a ball joined to a slab resting on its pole at {h}"),
        );
    }
}

#[test]
fn a_ball_cut_by_a_block_through_its_axis_is_a_half_ball_with_both_poles_as_vertices() {
    let half = block([0.0, -10.0, -10.0], [10.0, 10.0, 10.0]);
    let cut = turned(&ball(), 360.0)
        .cut_by(&half)
        .expect("the ball is cut");
    held(&cut, BALL / 2.0, "a half ball");
    let [round] = rounds(&cut)[..] else {
        panic!("one round face");
    };
    assert!(round.poles.is_empty(), "{:?}", round.poles);
    assert_eq!(round.loops.len(), 1);
    assert_eq!(round.loops[0].len(), 2);
    for pole in [DVec3::Y * 5.0, DVec3::Y * -5.0] {
        assert!(
            cut.vertices
                .iter()
                .any(|vertex| vertex.point.distance(pole) < 1e-12),
            "{pole} among {:?}",
            cut.vertices
        );
    }
    let joined = turned(&ball(), 360.0)
        .joined(&half)
        .expect("the ball is joined");
    held(
        &joined,
        BALL / 2.0 + 4000.0,
        "a half ball joined to a block",
    );
}

#[test]
fn a_coaxial_bore_through_a_ball_leaves_a_napkin_ring_between_two_rims() {
    let bore = shaft(2.0, -10.0, 10.0, 0.0);
    let cut = turned(&ball(), 360.0)
        .cut_by(&bore)
        .expect("the ball is bored");
    held(&cut, 4.0 * PI / 3.0 * 21f64.powf(1.5), "a bored ball");
    let [round] = rounds(&cut)[..] else {
        panic!("one round face");
    };
    assert_eq!(round.loops.len(), 2);
    assert!(round.poles.is_empty());
    let pin = shaft(2.0, -10.0, 10.0, 0.0);
    let joined = turned(&ball(), 360.0)
        .joined(&pin)
        .expect("the ball is pinned");
    held(
        &joined,
        BALL + PI * 4.0 * 20.0 - 4.0 * PI / 3.0 * (125.0 - 21f64.powf(1.5)),
        "a ball on a pin",
    );
}

#[test]
fn a_shaft_of_a_ball_s_radius_joined_at_its_equator_touches_it_along_one_rim() {
    let joined = turned(&ball(), 360.0)
        .joined(&shaft(5.0, 0.0, 15.0, 0.0))
        .expect("the shaft is joined");
    held(
        &joined,
        BALL / 2.0 + PI * 25.0 * 15.0,
        "a rounded shaft end",
    );
    let [round] = rounds(&joined)[..] else {
        panic!("one round face");
    };
    assert_eq!(round.poles.len(), 1);
    let cut = turned(&ball(), 360.0)
        .cut_by(&shaft(5.0, 0.0, 15.0, 0.0))
        .expect("the ball is cut");
    held(&cut, BALL / 2.0, "a half ball left by a shaft");
}

#[test]
fn a_shaft_a_hair_larger_or_smaller_than_the_ball_joined_at_its_equator_holds_its_volume() {
    let hair = 3.0 * Scale::of(15.0).eps();
    let larger = turned(&ball(), 360.0)
        .joined(&shaft(5.0 + hair, 0.0, 15.0, 0.0))
        .expect("a shaft a hair larger is joined");
    held(
        &larger,
        BALL / 2.0 + PI * (5.0 + hair) * (5.0 + hair) * 15.0,
        "a shaft a hair larger than its ball",
    );
    let radius = 5.0 - hair;
    let rim = (25.0 - radius * radius).sqrt();
    let smaller = turned(&ball(), 360.0)
        .joined(&shaft(radius, 0.0, 15.0, 0.0))
        .expect("a shaft a hair smaller is joined");
    held(
        &smaller,
        BALL / 2.0
            + PI * (25.0 * rim - rim * rim * rim / 3.0)
            + PI * radius * radius * (15.0 - rim),
        "a shaft a hair smaller than its ball",
    );
    let cut = turned(&ball(), 360.0)
        .cut_by(&shaft(radius, 0.0, 15.0, 0.0))
        .expect("the ball is cut by a shaft a hair smaller");
    held(
        &cut,
        BALL / 2.0 + PI * (25.0 * rim - rim * rim * rim / 3.0) - PI * radius * radius * rim,
        "a ball cut by a shaft a hair smaller",
    );
}

#[test]
fn a_ring_cut_by_a_coaxial_cone_keeps_the_segment_of_its_tube_beyond_the_slope() {
    let cone = turned_whole(&[[-5.0, 0.0], [-5.0, 10.5], [10.0, 3.0], [10.0, 0.0]]);
    let cut = turned(&ring(), 360.0)
        .cut_by(&cone)
        .expect("the ring is cut");
    let normal = DVec2::new(0.5, 1.0).normalize();
    let segment = turned_disc_beside(DVec2::new(3.0, 6.0), 2.0, (DVec2::new(0.0, 8.0), normal));
    held(&cut, segment, "a ring cut by a cone");
    let [round] = rounds(&cut)[..] else {
        panic!("one round face");
    };
    assert_eq!(round.loops.len(), 2);
    let joined = turned(&ring(), 360.0)
        .joined(&cone)
        .expect("the ring is joined");
    let cone_volume = PI * (10.5 * 10.5 + 10.5 * 3.0 + 9.0) * 15.0 / 3.0;
    held(&joined, cone_volume + segment, "a ring joined to a cone");
}

#[test]
fn a_plane_square_through_a_ring_s_tube_cuts_two_rims_and_touching_its_top_one() {
    let cut = turned(&ring(), 360.0)
        .cut_by(&block([-10.0, 4.0, -10.0], [10.0, 10.0, 10.0]))
        .expect("the ring is cut");
    let kept = turned_disc_beside(DVec2::new(3.0, 6.0), 2.0, (DVec2::new(4.0, 0.0), -DVec2::X));
    held(&cut, kept, "a ring cut above its tube's centre");
    let eps = Scale::of(10.0).eps();
    for h in [5.0, 5.0 + eps / 2.0, 5.0 - eps / 2.0] {
        let slab = block([-10.0, h, -10.0], [10.0, 10.0, 10.0]);
        let touched = turned(&ring(), 360.0)
            .cut_by(&slab)
            .expect("the ring is cut");
        held(
            &touched,
            RING,
            &format!("a ring under a slab resting on it at {h}"),
        );
        let joined = turned(&ring(), 360.0)
            .joined(&slab)
            .expect("the ring is joined");
        held(
            &joined,
            RING + 400.0 * (10.0 - h),
            &format!("a ring joined to a slab resting on it at {h}"),
        );
    }
}

#[test]
fn a_bore_of_the_root_s_radius_through_a_grooved_shaft_touches_the_groove_along_its_root() {
    let grooved = shaft(10.0, 0.0, 10.0, 0.0)
        .cut_by(&ring_of(5.0, 12.0, 4.0))
        .expect("the shaft is grooved");
    let removed = turned_disc_beside(
        DVec2::new(5.0, 12.0),
        4.0,
        (DVec2::new(5.0, 10.0), -DVec2::Y),
    );
    held(&grooved, PI * 100.0 * 10.0 - removed, "a grooved shaft");
    let bored = grooved
        .cut_by(&shaft(8.0, -5.0, 15.0, 0.0))
        .expect("the grooved shaft is bored");
    held(
        &bored,
        PI * 100.0 * 10.0 - removed - PI * 64.0 * 10.0,
        "a grooved shaft bored to the groove's root",
    );
}

#[test]
fn two_balls_of_one_centre_and_radius_are_one_surface_and_a_hair_apart_in_radius_two() {
    let eps = Scale::of(5.0).eps();
    for radius in [5.0, 5.0 + eps / 2.0] {
        let joined = turned(&ball(), 360.0)
            .joined(&ball_of(0.0, radius))
            .expect("the balls are joined");
        held(
            &joined,
            BALL,
            &format!("a ball joined to one of radius {radius}"),
        );
        assert_eq!(rounds(&joined).len(), 1);
    }
    let radius = 5.0 + 3.0 * eps;
    let joined = turned(&ball(), 360.0)
        .joined(&ball_of(0.0, radius))
        .expect("the balls are joined");
    held(
        &joined,
        4.0 * PI / 3.0 * radius * radius * radius,
        "a ball joined to one a hair larger",
    );
}

#[test]
fn two_balls_on_one_axis_touching_meet_at_a_point() {
    let eps = Scale::of(11.0).eps();
    for h in [8.0, 8.0 + eps / 2.0] {
        let joined = turned(&ball(), 360.0)
            .joined(&ball_of(h, 3.0))
            .expect("the balls are joined");
        held(
            &joined,
            BALL + 36.0 * PI,
            &format!("two balls touching at {h}"),
        );
        assert_eq!(rounds(&joined).len(), 2);
        assert!(
            joined
                .vertices
                .iter()
                .any(|vertex| vertex.point.distance(DVec3::Y * 5.0) <= eps),
            "{:?}",
            joined.vertices
        );
    }
    let apart = turned(&ball(), 360.0)
        .joined(&ball_of(8.0 + 2.0 * eps, 3.0))
        .expect("the balls are joined");
    held(&apart, BALL + 36.0 * PI, "two balls a hair apart");
}

#[test]
fn a_torus_sharing_the_tube_circle_is_one_surface() {
    let joined = turned(&ring(), 360.0)
        .joined(&ring_of(3.0, 6.0, 2.0))
        .expect("the rings are joined");
    held(&joined, RING, "a ring joined to itself");
    assert_eq!(rounds(&joined).len(), 1);
}

#[test]
fn a_half_ball_cut_out_of_a_block_is_a_flipped_round_face_through_a_pole() {
    let hollowed = block([-10.0, 0.0, -10.0], [10.0, 10.0, 10.0])
        .cut_by(&turned(&ball(), 360.0))
        .expect("the block is hollowed");
    held(
        &hollowed,
        4000.0 - BALL / 2.0,
        "a block with a half ball cut out",
    );
    let [round] = rounds(&hollowed)[..] else {
        panic!("one round face");
    };
    assert!(round.flipped);
    assert_eq!(round.poles.len(), 1);
    let [pole] = round.poles[..] else {
        unreachable!()
    };
    assert!(hollowed.vertex(pole).point.distance(DVec3::Y * 5.0) < 1e-12);
}

#[test]
fn a_plane_at_a_slant_through_a_ball_is_declined_as_unsupported() {
    let normal = DVec3::new(1.0, 1.0, 0.0).normalize();
    let u = normal.any_orthonormal_vector();
    let frame = Frame {
        origin: DVec3::ZERO,
        u,
        v: normal.cross(u),
    };
    let slab = Body::raised(
        &Contour::rectangle(DVec2::splat(-20.0), DVec2::splat(20.0)),
        &[],
        frame,
        normal * 10.0,
    )
    .expect("a slab raises");
    assert_eq!(
        turned(&ball(), 360.0).cut_by(&slab).err(),
        Some(Declined::Unsupported)
    );
}

#[test]
fn a_block_beside_a_ball_knob_is_passed_over_by_clear() {
    let knob = turned(&ball(), 360.0);
    let joined = knob
        .joined(&block([4.0, -1.0, 4.0], [8.0, 1.0, 8.0]))
        .expect("a block beside the knob joins it");
    held(&joined, BALL + 32.0, "a block beside a ball knob");
}

#[test]
fn a_hole_drilled_off_axis_through_a_ball_is_declined_as_unsupported() {
    let hole = shaft(1.0, -10.0, 10.0, 2.0);
    assert_eq!(
        turned(&ball(), 360.0).cut_by(&hole).err(),
        Some(Declined::Unsupported)
    );
}

/// Lane T's campaign found a quarter ball or a ring joined with a body
/// standing apart from it whose rays found nothing, or the wrong sign, while
/// the leaf alone passed every rule (seeds 5576500038, 5576400016).
#[test]
fn a_rounded_leaf_joined_with_a_block_apart_from_it_holds_both_volumes() {
    let far = block([20.0, 20.0, 20.0], [30.0, 30.0, 30.0]);
    for (what, leaf, volume) in [
        ("a quarter ball", turned(&ball(), 90.0), BALL / 4.0),
        ("a half ball", turned(&ball(), 180.0), BALL / 2.0),
        ("a ball", turned(&ball(), 360.0), BALL),
        ("a ring", turned(&ring(), 360.0), RING),
        ("a quarter ring", turned(&ring(), 90.0), RING / 4.0),
    ] {
        let joined = leaf.joined(&far).expect(what);
        held(
            &joined,
            volume + 1000.0,
            &format!("{what} joined with a block apart"),
        );
        let joined = far.joined(&leaf).expect(what);
        held(
            &joined,
            volume + 1000.0,
            &format!("a block joined with {what} apart"),
        );
        let cut = leaf.cut_by(&far).expect(what);
        held(&cut, volume, &format!("{what} cut by a block apart"));
        let cut = far.cut_by(&leaf).expect(what);
        held(&cut, 1000.0, &format!("a block cut by {what} apart"));
    }
}

/// Lane T's campaign found a plane crossing a ball, or standing a hair inside
/// its crest, that `clear` passed as apart, so that the body's faces crossed
/// (seeds 5576502870, 5576400540): a whole ball's face has no edge to read
/// its reach off.
#[test]
fn a_block_whose_side_crosses_a_ball_off_its_axis_is_declined_as_unsupported() {
    let eps = Scale::of(10.0).eps();
    for low in [2.0, 4.9, -3.0, 5.0 - eps] {
        let slab = block([low, -10.0, -10.0], [10.0, 10.0, 10.0]);
        assert_eq!(
            turned(&ball(), 360.0).cut_by(&slab).err(),
            Some(Declined::Unsupported),
            "{low}"
        );
        assert_eq!(
            turned(&ball(), 360.0).joined(&slab).err(),
            Some(Declined::Unsupported),
            "{low}"
        );
    }
}

#[test]
fn a_block_in_a_ring_s_hole_is_passed_over_by_clear() {
    let post = block([-2.0, 0.0, -2.0], [2.0, 6.0, 2.0]);
    let joined = turned(&ring(), 360.0)
        .joined(&post)
        .expect("a post in the ring's hole joins it");
    held(&joined, RING + 96.0, "a post in a ring's hole");
    let cut = turned(&ring(), 360.0)
        .cut_by(&post)
        .expect("a post in the ring's hole cuts nothing");
    held(&cut, RING, "a ring cut by a post in its hole");
}
