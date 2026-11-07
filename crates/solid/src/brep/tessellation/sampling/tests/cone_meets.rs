use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use super::super::Samples;
use crate::brep::curve::{Curve, Wall};
use crate::brep::surface::{Cylinder, Surface};
use crate::brep::topology::Body;
use crate::profile::{Contour, Frame, Run};
use crate::turning::{Axis, ON_THE_AXIS, Straight, Turn};

/// A whole circle of `radius` about `center`, one run from one corner.
fn circle(center: DVec2, radius: f64) -> Contour {
    Contour {
        corners: vec![center + DVec2::X * radius],
        runs: vec![Run::Round { center, turn: TAU }],
    }
}

/// The campaign's seed 354000000005870: a shaft of radius 20 along Y, bored
/// at 80° by one of radius 7.5 touching its wall from inside at a point, then
/// cut by a cone leaning 24° whose radius falls from 5 to 2.5 along it.
fn a_shaft_bored_at_80_degrees_then_by_a_cone_leaning_24_degrees() -> Body {
    let shaft = Frame {
        origin: DVec3::new(0.0, -5.0, 0.0),
        u: DVec3::X,
        v: DVec3::Z,
    };
    let body = Body::raised(
        &circle(DVec2::new(10.0, 0.0), 20.0),
        &[],
        shaft,
        DVec3::Y * 42.5,
    )
    .expect("the shaft raises");
    let (sin, cos) = 80f64.to_radians().sin_cos();
    let tilted = Frame {
        origin: DVec3::new(10.0, 110.5501270299569, -16.627656679617015),
        u: DVec3::X,
        v: DVec3::new(0.0, cos, sin),
    };
    let along = DVec3::new(0.0, -sin, cos) * 191.50971698084905;
    let bore = Body::raised(
        &circle(DVec2::new(12.499999999999998, 0.0), 7.5),
        &[],
        tilted,
        along,
    )
    .expect("the bore raises");
    let body = body.cut_by(&bore).expect("the bore cuts the shaft");
    let axis = Axis {
        origin: DVec2::new(0.0, 11.797713146914639),
        direction: DVec2::from_angle(24f64.to_radians()),
    };
    let at = |a: f64, s: f64| axis.origin + axis.direction * a + axis.direction.perp() * s;
    let from = -84.80849570536405;
    let to = from + 191.50971698084905;
    let section = Contour::straight(vec![at(from, 0.0), at(to, 0.0), at(to, 2.5), at(from, 5.0)]);
    let ground = Frame {
        origin: DVec3::new(0.0, 0.0, -1.0000000000000001e-7),
        u: DVec3::X,
        v: DVec3::Y,
    };
    let turn = Turn {
        axis,
        angle: TAU,
        resolution: 0.0,
        on_the_axis: ON_THE_AXIS * 5.0,
    };
    let straight =
        Straight::of(&section, &[], ground, &turn, 0.0).expect("the section lies straight");
    let cone = Body::turned(&straight, ground, &turn).expect("the cone turns");
    body.cut_by(&cone).expect("the cone cuts the shaft")
}

/// Every sample of a meet on a cylinder of `body` that falls strictly
/// between two neighbouring samples of a circle on that cylinder, with how
/// far round the wall it stands from the nearer of the two.
fn off_the_rays(body: &Body, samples: &Samples) -> Vec<(DVec3, f64)> {
    let eps = body.scale().eps();
    let mut cylinders: Vec<Cylinder> = Vec::new();
    for face in body.face_ids() {
        if let Surface::Cylinder(cylinder) = body.surface(body.face(face).surface)
            && !cylinders.contains(cylinder)
        {
            cylinders.push(*cylinder);
        }
    }
    let mut missed = Vec::new();
    for cylinder in cylinders {
        let on_it = |point: DVec3| {
            let from = point - cylinder.origin;
            ((from - cylinder.axis * from.dot(cylinder.axis)).length() - cylinder.radius).abs()
                <= eps
        };
        let angle = |point: DVec3| cylinder.parameters(point).x.rem_euclid(TAU);
        let mut chords = Vec::new();
        let mut curve = Vec::new();
        for edge in body.edge_ids() {
            let points: Vec<DVec3> = samples
                .edge(edge)
                .iter()
                .map(|id| samples.point(*id))
                .collect();
            match body.curve(body.edge(edge).curve) {
                Curve::Circle(circle)
                    if circle.axis.cross(cylinder.axis).length() <= 1e-12
                        && (circle.radius - cylinder.radius).abs() <= eps
                        && points.iter().all(|point| on_it(*point)) =>
                {
                    chords.extend(
                        points
                            .windows(2)
                            .map(|pair| [angle(pair[0]), angle(pair[1])]),
                    );
                }
                Curve::Meet(meet)
                    if [meet.first, meet.second].contains(&Wall::Cylinder(cylinder)) =>
                {
                    curve.extend(points);
                }
                _ => {}
            }
        }
        for point in curve {
            let at = angle(point);
            for [one, other] in &chords {
                let span = (other - one).rem_euclid(TAU);
                let (from, span) = if span <= TAU / 2.0 {
                    (*one, span)
                } else {
                    (*other, TAU - span)
                };
                let into = (at - from).rem_euclid(TAU);
                let gap = into.min(span - into) * cylinder.radius;
                if into < span && gap > eps {
                    missed.push((point, gap));
                    break;
                }
            }
        }
    }
    missed
}

#[test]
fn the_circles_of_a_shaft_a_bore_and_a_cone_cross_are_sampled_through_every_sample_of_their_curves()
{
    let body = a_shaft_bored_at_80_degrees_then_by_a_cone_leaning_24_degrees();
    for tolerance in [1e-3, 0.02, 0.11, 0.5] {
        let samples = Samples::of(&body, tolerance);
        let missed = off_the_rays(&body, &samples);
        assert!(
            missed.is_empty(),
            "within {tolerance}, {} samples of a curve stand between two samples of a circle \
             of their wall, up to {} round it from the nearer: {missed:?}",
            missed.len(),
            missed.iter().map(|(_, gap)| *gap).fold(0.0, f64::max),
        );
    }
}
