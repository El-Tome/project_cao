//! The geometry cache writes the part's body as JSON. An exact body read back
//! must be the very body written, so that the next step cut into it gives what
//! a replay would.

use std::f64::consts::TAU;

use cao_solid::Body;
use cao_solid::profile::{Contour, Frame, Profile, Run};
use glam::{DVec2, DVec3};

/// A frame turned off every axis, so that no coordinate of the body is one
/// a short decimal writes exactly.
fn turned() -> Frame {
    let (sin, cos) = 0.3f64.sin_cos();
    Frame {
        origin: DVec3::new(0.1, 0.2, 0.3),
        u: DVec3::new(cos, sin, 0.0),
        v: DVec3::new(-sin, cos, 0.0),
    }
}

fn raised(part: &Body, outline: Contour, height: f64) -> Body {
    let profile = Profile {
        exact: Some((outline, Vec::new())),
        sampled: cao_solid::Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    part.tool_raised(&profile, turned(), DVec3::Z * height)
        .expect("the kernel raises it")
}

fn disc(center: DVec2, radius: f64) -> Contour {
    Contour {
        corners: vec![center + DVec2::X * radius],
        runs: vec![Run::Round { center, turn: TAU }],
    }
}

#[test]
fn an_exact_body_comes_back_from_json_and_cuts_as_before() {
    let block = raised(
        &Body::default(),
        Contour::rectangle(DVec2::ZERO, DVec2::new(30.0, 20.0)),
        10.0,
    );
    let first = raised(&block, disc(DVec2::new(10.0 / 3.0, 7.1), 2.7), 10.0);
    let part = block.difference(&first).expect("the kernel bores it");

    let written = serde_json::to_string(&part).expect("a body writes");
    let read: Body = serde_json::from_str(&written).expect("and reads back");

    assert_eq!(read, part);
    let second = raised(&part, disc(DVec2::new(20.0, 13.0 / 7.0 + 8.0), 1.3), 10.0);
    assert_eq!(read.difference(&second), part.difference(&second));
    assert_eq!(read.triangles(), part.triangles());
}
