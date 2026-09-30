//! Bodies built by hand whose faces are bounded by the curve two
//! perpendicular cylinders meet along: the stock bored across, the stock
//! crossed by a cylinder lying through it, two equal cylinders crossing, a
//! bore touching the stock's wall from inside.
//!
//! Every cylinder lying across stands on the stock's middle height, and the
//! curves are the kernel's own, from `Meeting::of`.

use std::f64::consts::{PI, TAU};

use glam::DVec3;

use super::fixtures::{Build, HEIGHT, STOCK_RADIUS};
use crate::brep::curve::Meet;
use crate::brep::topology::{Body, Coedge, EdgeId, VertexId};

/// The height of the axis of every cylinder lying across the stock.
pub(crate) const MIDDLE: f64 = HEIGHT / 2.0;

fn use_of(edge: EdgeId, forward: bool) -> Coedge {
    Coedge { edge, forward }
}

/// A whole component, round its period, with no vertex.
fn whole(build: &mut Build, meet: Meet) -> EdgeId {
    let period = meet.period().expect("a component closes on itself");
    build.meet_edge(meet, None, 0.0, period)
}

/// The stock bored across by a cylinder of `radius` lying along `axis`, clear
/// of its top and its bottom: two windows in the stock's wall, each a whole
/// loop, and the bore's wall between them, whose matter lies outside it.
///
/// Along either axis, the first component is the window on the side the axis
/// points to, and runs round it clockwise seen from outside the stock, round
/// the bore's axis the way its angle falls.
pub(crate) fn stock_bored_across(radius: f64, axis: DVec3) -> Body {
    assert!(
        radius < MIDDLE,
        "the bore stays clear of the top and the bottom"
    );
    let mut build = Build::new();
    let stock = build.cylinder(DVec3::ZERO, DVec3::Z, STOCK_RADIUS);
    let bore = build.cylinder(DVec3::Z * MIDDLE, axis, radius);
    let meeting = build.meeting(stock, bore, STOCK_RADIUS);
    let [near, far] = [0, 1].map(|component| whole(&mut build, meeting.components[component]));
    let (low, high) = (
        build.circle(stock, 0.0, None),
        build.circle(stock, HEIGHT, None),
    );
    build.face(
        stock,
        false,
        vec![
            vec![use_of(low, true)],
            vec![use_of(high, false)],
            vec![use_of(near, true)],
            vec![use_of(far, false)],
        ],
    );
    build.face(
        bore,
        true,
        vec![vec![use_of(near, false)], vec![use_of(far, true)]],
    );
    let (top, top_flipped) = build.plane(DVec3::Z * HEIGHT, DVec3::Z);
    build.face(top, top_flipped, vec![vec![use_of(high, true)]]);
    let (bottom, bottom_flipped) = build.plane(DVec3::ZERO, DVec3::NEG_Z);
    build.face(bottom, bottom_flipped, vec![vec![use_of(low, false)]]);
    build.finish(STOCK_RADIUS)
}

/// The four corners where a cylinder of radius `MIDDLE` lying across the
/// stock touches its top and its bottom on its wall, at `x = 0`: by the side
/// of the stock, the one `+across` points to first, then by height, the top
/// first.
fn flush_corners(build: &mut Build, across: DVec3) -> [[VertexId; 2]; 2] {
    [1.0, -1.0].map(|side| {
        [HEIGHT, 0.0].map(|height| build.vertex(across * side * STOCK_RADIUS + DVec3::Z * height))
    })
}

/// The two halves of a component touching the top at `t = π/2` and the
/// bottom at `3π/2`: from the top down through `t = π`, where the height
/// across both axes is highest, then from the bottom up through `2π`, where
/// it is lowest.
fn halves(build: &mut Build, meet: Meet, [top, bottom]: [VertexId; 2]) -> [EdgeId; 2] {
    [
        build.meet_edge(meet, Some([top, bottom]), PI / 2.0, 3.0 * PI / 2.0),
        build.meet_edge(meet, Some([bottom, top]), 3.0 * PI / 2.0, 5.0 * PI / 2.0),
    ]
}

/// The stock bored across along Y by a cylinder of radius five through its
/// middle height: the bore's wall touches the top and the bottom along the
/// lines `x = 0` there, each with four uses, and each window touches both
/// rings at a point. The stock's wall, its top, its bottom and the bore's wall
/// are each two faces, one either side of `x = 0`.
pub(crate) fn stock_bored_flush_across() -> Body {
    let mut build = Build::new();
    let stock = build.cylinder(DVec3::ZERO, DVec3::Z, STOCK_RADIUS);
    let bore = build.cylinder(DVec3::Z * MIDDLE, DVec3::Y, MIDDLE);
    let meeting = build.meeting(stock, bore, STOCK_RADIUS);
    let [[near_top, near_bottom], [far_top, far_bottom]] = flush_corners(&mut build, DVec3::Y);
    let [near_west, near_east] = halves(&mut build, meeting.components[0], [near_top, near_bottom]);
    let [far_west, far_east] = halves(&mut build, meeting.components[1], [far_top, far_bottom]);
    let top_line = build.line(far_top, near_top);
    let bottom_line = build.line(far_bottom, near_bottom);
    let top_east = build.arc(stock, HEIGHT, far_top, near_top);
    let top_west = build.arc(stock, HEIGHT, near_top, far_top);
    let bottom_east = build.arc(stock, 0.0, far_bottom, near_bottom);
    let bottom_west = build.arc(stock, 0.0, near_bottom, far_bottom);

    let lap = |uses: &[(EdgeId, bool)]| vec![uses.iter().map(|&(e, f)| use_of(e, f)).collect()];
    build.face(
        stock,
        false,
        lap(&[
            (bottom_east, true),
            (near_east, true),
            (top_east, false),
            (far_east, false),
        ]),
    );
    build.face(
        stock,
        false,
        lap(&[
            (bottom_west, true),
            (far_west, false),
            (top_west, false),
            (near_west, true),
        ]),
    );
    build.face(
        bore,
        true,
        lap(&[
            (near_east, false),
            (bottom_line, false),
            (far_east, true),
            (top_line, true),
        ]),
    );
    build.face(
        bore,
        true,
        lap(&[
            (near_west, false),
            (top_line, false),
            (far_west, true),
            (bottom_line, true),
        ]),
    );
    let (top, top_flipped) = build.plane(DVec3::Z * HEIGHT, DVec3::Z);
    build.face(
        top,
        top_flipped,
        lap(&[(top_east, true), (top_line, false)]),
    );
    build.face(top, top_flipped, lap(&[(top_west, true), (top_line, true)]));
    let (bottom, bottom_flipped) = build.plane(DVec3::ZERO, DVec3::NEG_Z);
    build.face(
        bottom,
        bottom_flipped,
        lap(&[(bottom_east, false), (bottom_line, true)]),
    );
    build.face(
        bottom,
        bottom_flipped,
        lap(&[(bottom_west, false), (bottom_line, false)]),
    );
    build.finish(STOCK_RADIUS)
}

/// How far either way from the stock's axis the cylinder crossing it reaches.
pub(crate) const ARM: f64 = 30.0;

/// The stock joined with a cylinder of radius five lying along X through its
/// middle height, from `-ARM` to `ARM`: a cross. The cylinder touches the
/// planes of the top and the bottom along its highest and lowest lines, but
/// only inside the stock, where it is no face; each window it leaves in the
/// stock's wall touches both rings at a point, and parts the wall into a
/// face to the north and one to the south. Out of the stock, the cylinder's
/// wall goes all the way round from the window to its end.
pub(crate) fn stock_crossed() -> Body {
    let mut build = Build::new();
    let stock = build.cylinder(DVec3::ZERO, DVec3::Z, STOCK_RADIUS);
    let arm = build.cylinder(DVec3::Z * MIDDLE, DVec3::X, MIDDLE);
    let meeting = build.meeting(stock, arm, ARM);
    let [[east_top, east_bottom], [west_top, west_bottom]] = flush_corners(&mut build, DVec3::X);
    let [east_north, east_south] =
        halves(&mut build, meeting.components[0], [east_top, east_bottom]);
    let [west_north, west_south] =
        halves(&mut build, meeting.components[1], [west_top, west_bottom]);
    let top_north = build.arc(stock, HEIGHT, east_top, west_top);
    let top_south = build.arc(stock, HEIGHT, west_top, east_top);
    let bottom_north = build.arc(stock, 0.0, east_bottom, west_bottom);
    let bottom_south = build.arc(stock, 0.0, west_bottom, east_bottom);
    let (east_end, west_end) = (build.circle(arm, ARM, None), build.circle(arm, -ARM, None));

    let lap = |uses: &[(EdgeId, bool)]| uses.iter().map(|&(e, f)| use_of(e, f)).collect();
    build.face(
        stock,
        false,
        vec![lap(&[
            (bottom_north, true),
            (west_north, false),
            (top_north, false),
            (east_north, true),
        ])],
    );
    build.face(
        stock,
        false,
        vec![lap(&[
            (bottom_south, true),
            (east_south, true),
            (top_south, false),
            (west_south, false),
        ])],
    );
    build.face(
        arm,
        false,
        vec![
            lap(&[(east_north, false), (east_south, false)]),
            lap(&[(east_end, false)]),
        ],
    );
    build.face(
        arm,
        false,
        vec![
            lap(&[(west_north, true), (west_south, true)]),
            lap(&[(west_end, true)]),
        ],
    );
    let (top, top_flipped) = build.plane(DVec3::Z * HEIGHT, DVec3::Z);
    build.face(
        top,
        top_flipped,
        vec![lap(&[(top_north, true), (top_south, true)])],
    );
    let (bottom, bottom_flipped) = build.plane(DVec3::ZERO, DVec3::NEG_Z);
    build.face(
        bottom,
        bottom_flipped,
        vec![lap(&[(bottom_south, false), (bottom_north, false)])],
    );
    let (east, east_flipped) = build.plane(DVec3::X * ARM, DVec3::X);
    build.face(east, east_flipped, vec![lap(&[(east_end, true)])]);
    let (west, west_flipped) = build.plane(DVec3::X * -ARM, DVec3::NEG_X);
    build.face(west, west_flipped, vec![lap(&[(west_end, false)])]);
    build.finish(ARM)
}

/// The matter the stock and a cylinder of `radius` lying across it through
/// `(0, across, MIDDLE)` have in common, the cylinder passing through it
/// whole: over the cylinder's disc, the chord the stock has along the
/// cylinder's axis, integrated numerically.
///
/// Across the disc at `s = r sin φ` from its centre the disc is `2r cos φ`
/// high, and `ds = r cos φ dφ`: the integrand `2r² cos² φ · chord` is smooth
/// and periodic in `φ`, so the rule of the midpoints over a whole turn, which
/// runs the disc twice, converges faster than any power of the step.
pub(crate) fn common_across(radius: f64, across: f64) -> f64 {
    const PLACES: usize = 4096;
    let step = TAU / PLACES as f64;
    let mut total = 0.0;
    for place in 0..PLACES {
        let (sin, cos) = ((place as f64 + 0.5) * step).sin_cos();
        let y = across + radius * sin;
        let chord = 2.0 * (STOCK_RADIUS * STOCK_RADIUS - y * y).max(0.0).sqrt();
        total += radius * radius * cos * cos * chord * step;
    }
    total
}
