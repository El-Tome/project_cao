//! Bodies built by hand whose faces are bounded by the curve two
//! perpendicular cylinders meet along: the stock bored across, the stock
//! crossed by a cylinder lying through it, two equal cylinders crossing, a
//! bore touching the stock's wall from inside.
//!
//! Every cylinder lying across stands on the stock's middle height, and the
//! curves are the kernel's own, from `Meeting::of`.

use std::f64::consts::TAU;

use glam::DVec3;

use super::fixtures::{Build, HEIGHT, STOCK_RADIUS};
use crate::brep::curve::Meet;
use crate::brep::topology::{Body, Coedge, EdgeId};

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
