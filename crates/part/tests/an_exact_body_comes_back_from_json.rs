//! The geometry cache writes the part's body as JSON. An exact body read back
//! must be the very body written, so that the next step cut into it gives what
//! a replay would.

use std::f64::consts::TAU;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

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
    raised_on(part, outline, turned(), height)
}

/// An outline raised `height` from `frame`, square to it.
fn raised_on(part: &Body, outline: Contour, frame: Frame, height: f64) -> Body {
    let profile = Profile {
        exact: Some((outline, Vec::new())),
        sampled: cao_solid::Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    part.tool_raised(&profile, frame, frame.u.cross(frame.v) * height)
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

/// The top of a post cut away above a plane leaning off it: the post's wall
/// meets that plane along an ellipse, an edge the cache must write as the
/// kernel built it.
#[test]
fn a_body_cut_at_a_slant_comes_back_from_json_and_cuts_as_before() {
    let post = raised(
        &Body::default(),
        disc(DVec2::new(10.0 / 3.0, 7.1), 6.1),
        10.0,
    );
    let level = turned();
    let (sin, cos) = 0.4f64.sin_cos();
    let leaning = Frame {
        origin: level.origin + DVec3::Z * 5.0,
        u: level.u,
        v: level.v * cos + DVec3::Z * sin,
    };
    let above = raised_on(
        &post,
        Contour::rectangle(DVec2::splat(-30.0), DVec2::splat(30.0)),
        leaning,
        10.0,
    );
    let part = post
        .difference(&above)
        .expect("the kernel cuts it at a slant");

    let written = serde_json::to_string(&part).expect("a body writes");
    let read: Body = serde_json::from_str(&written).expect("and reads back");

    assert_eq!(read, part);
    let bore = raised(&part, disc(DVec2::new(2.0, 13.0 / 7.0 + 4.0), 1.3), 10.0);
    assert_eq!(read.difference(&bore), part.difference(&bore));
    assert_eq!(read.triangles(), part.triangles());
    assert!(part.is_exact(), "the post stays exact");
}

/// The plane of the ground, which the slotted block is drawn on.
fn ground() -> Frame {
    Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    }
}

/// The first value in the text equal to `wanted`, wherever it is written.
fn written_as<'a>(
    value: &'a mut serde_json::Value,
    wanted: &serde_json::Value,
) -> Option<&'a mut serde_json::Value> {
    if value == wanted {
        return Some(value);
    }
    match value {
        serde_json::Value::Object(fields) => fields
            .values_mut()
            .find_map(|field| written_as(field, wanted)),
        serde_json::Value::Array(items) => {
            items.iter_mut().find_map(|item| written_as(item, wanted))
        }
        _ => None,
    }
}

#[test]
fn an_exact_body_read_back_with_a_cycle_in_its_roots_still_answers_for_a_name() {
    let block = raised_on(
        &Body::default(),
        Contour::rectangle(DVec2::ZERO, DVec2::new(30.0, 10.0)),
        ground(),
        10.0,
    );
    let slot = raised_on(
        &block,
        Contour::rectangle(DVec2::new(10.0, -1.0), DVec2::new(20.0, 11.0)),
        Frame {
            origin: DVec3::Z * 5.0,
            ..ground()
        },
        6.0,
    );
    let part = block.difference(&slot).expect("the slot cuts the block");
    let name = part
        .edge_names()
        .into_iter()
        .find(|name| name.between == [vec![1], vec![2]])
        .expect("an edge between the top and the front");
    let mut written = serde_json::to_value(&part).expect("a body writes");
    let roots = written_as(&mut written, &serde_json::json!({ "12": 1 })).expect(
        "the body writes that the top's second piece, numbered twelve, was \
         parted from the top",
    );
    *roots = serde_json::json!({ "12": 13, "13": 12 });

    let (answer, answered) = mpsc::channel();
    thread::spawn(move || {
        let read: Body = serde_json::from_value(written).expect("and reads back");
        let _ = answer.send(read.lost(&name));
    });

    let answer = answered.recv_timeout(Duration::from_secs(10));
    assert!(
        !matches!(answer, Err(mpsc::RecvTimeoutError::Timeout)),
        "a body whose roots were edited by hand into a cycle still answers \
         for a name, rather than following its roots round for ever",
    );
    assert!(
        answer.is_ok(),
        "the body reads back and answers: {answer:?}"
    );
}
