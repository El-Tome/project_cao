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
use crate::brep::meet::Configuration;
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
    stock_bored_across_at(radius, axis, 0.0)
}

/// The same bore, its axis moved `across` from the stock's, square to both
/// axes along `Z × axis`: two windows still, as long as the bore stays
/// inside the stock's wall where it is nearest, however close to it.
pub(crate) fn stock_bored_across_at(radius: f64, axis: DVec3, across: f64) -> Body {
    assert!(
        radius < MIDDLE,
        "the bore stays clear of the top and the bottom"
    );
    let mut build = Build::new();
    let stock = build.cylinder(DVec3::ZERO, DVec3::Z, STOCK_RADIUS);
    let aside = DVec3::Z.cross(axis) * across;
    let bore = build.cylinder(DVec3::Z * MIDDLE + aside, axis, radius);
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

/// The radius of two equal cylinders crossing, and how far each reaches
/// either way from where their axes meet.
pub(crate) const EQUAL: f64 = 5.0;
pub(crate) const EQUAL_REACH: f64 = 15.0;

/// A post along Z and a beam along X, both of radius `EQUAL` about axes
/// meeting at the origin, joined: they meet along two ellipses, in the planes
/// `z = x` and `z = −x`, crossing at two nodes on Y where the walls touch.
/// Each ellipse is two edges between the nodes; the post keeps its wall above
/// and below the ellipses, the beam east and west of them, four faces
/// meeting at each node, and each wall goes all the way round to its end.
pub(crate) fn equal_cylinders_crossed() -> Body {
    let mut build = Build::new();
    let post = build.cylinder(DVec3::ZERO, DVec3::Z, EQUAL);
    let beam = build.cylinder(DVec3::ZERO, DVec3::X, EQUAL);
    let meeting = build.meeting(post, beam, EQUAL_REACH);
    assert_eq!(meeting.configuration, Configuration::TwoEllipses);
    let [south, north] = [0, 1].map(|node| build.vertex(meeting.nodes[node].point));
    let mut between = |component: usize| {
        let meet = meeting.components[component];
        [
            build.meet_edge(meet, Some([south, north]), 0.0, PI),
            build.meet_edge(meet, Some([north, south]), PI, TAU),
        ]
    };
    let [upper_east, lower_west] = between(0);
    let [lower_east, upper_west] = between(1);
    let post_top = build.circle(post, EQUAL_REACH, None);
    let post_bottom = build.circle(post, -EQUAL_REACH, None);
    let beam_east = build.circle(beam, EQUAL_REACH, None);
    let beam_west = build.circle(beam, -EQUAL_REACH, None);

    let lap = |uses: &[(EdgeId, bool)]| uses.iter().map(|&(e, f)| use_of(e, f)).collect();
    build.face(
        post,
        false,
        vec![
            lap(&[(upper_east, true), (upper_west, true)]),
            lap(&[(post_top, false)]),
        ],
    );
    build.face(
        post,
        false,
        vec![
            lap(&[(lower_east, false), (lower_west, false)]),
            lap(&[(post_bottom, true)]),
        ],
    );
    build.face(
        beam,
        false,
        vec![
            lap(&[(lower_east, true), (upper_east, false)]),
            lap(&[(beam_east, false)]),
        ],
    );
    build.face(
        beam,
        false,
        vec![
            lap(&[(lower_west, true), (upper_west, false)]),
            lap(&[(beam_west, true)]),
        ],
    );
    for (ring, way, forward) in [
        (post_top, DVec3::Z, true),
        (post_bottom, DVec3::NEG_Z, false),
        (beam_east, DVec3::X, true),
        (beam_west, DVec3::NEG_X, false),
    ] {
        let (cap, flipped) = build.plane(way * EQUAL_REACH, way);
        build.face(cap, flipped, vec![lap(&[(ring, forward)])]);
    }
    build.finish(EQUAL_REACH)
}

/// The post of `equal_cylinders_crossed` bored through by its beam: the post
/// keeps its wall above and below the ellipses as when joined, and the
/// beam's wall inside the post, whose matter lies outside it, is two faces,
/// above the post's axis and below, each a lens pinched to both nodes. Beside
/// a node the post's wall and the beam's lie over each other, parting only
/// as the square of the distance from it.
pub(crate) fn equal_cylinders_bored() -> Body {
    let mut build = Build::new();
    let post = build.cylinder(DVec3::ZERO, DVec3::Z, EQUAL);
    let beam = build.cylinder(DVec3::ZERO, DVec3::X, EQUAL);
    let meeting = build.meeting(post, beam, EQUAL_REACH);
    assert_eq!(meeting.configuration, Configuration::TwoEllipses);
    let [south, north] = [0, 1].map(|node| build.vertex(meeting.nodes[node].point));
    let mut between = |component: usize| {
        let meet = meeting.components[component];
        [
            build.meet_edge(meet, Some([south, north]), 0.0, PI),
            build.meet_edge(meet, Some([north, south]), PI, TAU),
        ]
    };
    let [upper_east, lower_west] = between(0);
    let [lower_east, upper_west] = between(1);
    let post_top = build.circle(post, EQUAL_REACH, None);
    let post_bottom = build.circle(post, -EQUAL_REACH, None);

    let lap = |uses: &[(EdgeId, bool)]| uses.iter().map(|&(e, f)| use_of(e, f)).collect();
    build.face(
        post,
        false,
        vec![
            lap(&[(upper_east, true), (upper_west, true)]),
            lap(&[(post_top, false)]),
        ],
    );
    build.face(
        post,
        false,
        vec![
            lap(&[(lower_east, false), (lower_west, false)]),
            lap(&[(post_bottom, true)]),
        ],
    );
    build.face(
        beam,
        true,
        vec![lap(&[(upper_east, false), (upper_west, false)])],
    );
    build.face(
        beam,
        true,
        vec![lap(&[(lower_east, true), (lower_west, true)])],
    );
    for (ring, way, forward) in [
        (post_top, DVec3::Z, true),
        (post_bottom, DVec3::NEG_Z, false),
    ] {
        let (cap, flipped) = build.plane(way * EQUAL_REACH, way);
        build.face(cap, flipped, vec![lap(&[(ring, forward)])]);
    }
    build.finish(EQUAL_REACH)
}

/// The radius of the bore whose wall touches the stock's from inside.
pub(crate) const TOUCHING: f64 = 3.0;

/// The stock bored across along X by a cylinder of radius `TOUCHING` whose
/// axis stands `STOCK_RADIUS − TOUCHING` from the stock's, on Y at the middle
/// height: its wall touches the stock's from inside at one point, and the two
/// meet along a figure of eight through a node there. Each lobe is an edge
/// from the node round to it: two windows in the stock's wall touching at the
/// node, and between them the bore's wall, pinched to that point, one face
/// with a single loop that visits the node twice.
pub(crate) fn stock_bored_touching_its_wall() -> Body {
    let mut build = Build::new();
    let stock = build.cylinder(DVec3::ZERO, DVec3::Z, STOCK_RADIUS);
    let axis = DVec3::new(0.0, STOCK_RADIUS - TOUCHING, MIDDLE);
    let bore = build.cylinder(axis, DVec3::X, TOUCHING);
    let meeting = build.meeting(stock, bore, STOCK_RADIUS);
    assert_eq!(meeting.configuration, Configuration::FigureOfEight);
    let meet = meeting.components[0];
    let period = meet.period().expect("a component closes on itself");
    let [(_, first), (_, second)] = [meeting.nodes[0].on[0], meeting.nodes[0].on[1]];
    let node = build.vertex(meeting.nodes[0].point);
    let west = build.meet_edge(meet, Some([node, node]), first, second);
    let east = build.meet_edge(meet, Some([node, node]), second, first + period);
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
            vec![use_of(west, false)],
            vec![use_of(east, true)],
        ],
    );
    build.face(
        bore,
        true,
        vec![vec![use_of(east, false), use_of(west, true)]],
    );
    let (top, top_flipped) = build.plane(DVec3::Z * HEIGHT, DVec3::Z);
    build.face(top, top_flipped, vec![vec![use_of(high, true)]]);
    let (bottom, bottom_flipped) = build.plane(DVec3::ZERO, DVec3::NEG_Z);
    build.face(bottom, bottom_flipped, vec![vec![use_of(low, false)]]);
    build.finish(STOCK_RADIUS)
}

/// The stock bored across along X by a cylinder of radius `TOUCHING` whose
/// axis stands `across` from the stock's, on Y at the middle height, so far
/// out that it breaks through the stock's wall there, however little: the
/// two meet along one loop, a single window in the stock's wall, with a neck
/// where the bore breaks through, and the bore's wall inside the stock no
/// longer goes round its axis.
pub(crate) fn stock_bored_through_its_wall(across: f64) -> Body {
    let mut build = Build::new();
    let stock = build.cylinder(DVec3::ZERO, DVec3::Z, STOCK_RADIUS);
    let axis = DVec3::new(0.0, across, MIDDLE);
    let bore = build.cylinder(axis, DVec3::X, TOUCHING);
    let meeting = build.meeting(stock, bore, STOCK_RADIUS);
    assert_eq!(meeting.configuration, Configuration::OneLoop);
    let window = whole(&mut build, meeting.components[0]);
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
            vec![use_of(window, true)],
        ],
    );
    build.face(bore, true, vec![vec![use_of(window, false)]]);
    let (top, top_flipped) = build.plane(DVec3::Z * HEIGHT, DVec3::Z);
    build.face(top, top_flipped, vec![vec![use_of(high, true)]]);
    let (bottom, bottom_flipped) = build.plane(DVec3::ZERO, DVec3::NEG_Z);
    build.face(bottom, bottom_flipped, vec![vec![use_of(low, false)]]);
    build.finish(STOCK_RADIUS)
}

/// The matter a cylinder of radius `standing` along Z and a cylinder of
/// `radius` lying across it, `across` from its axis, have in common within
/// their heights and lengths: over the heights `y` across both axes they
/// share, the chord the standing one has along the lying one's axis times
/// the height of the lying one's disc, integrated numerically.
///
/// Both are square roots, each vanishing at an end of the heights shared or
/// beyond it. Run as `y = lo + (hi − lo)(1 − cos u)/2`, each root that
/// vanishes at an end becomes a sine or a cosine of `u/2`, so the integrand
/// times `|sin u|` is smooth and periodic over a whole turn, which runs the
/// heights twice: the rule of the midpoints converges faster than any power
/// of the step, but where both vanish at one end, at a touch.
pub(crate) fn common(standing: f64, radius: f64, across: f64) -> f64 {
    const PLACES: usize = 4096;
    let (low, high) = (
        (across - radius).max(-standing),
        (across + radius).min(standing),
    );
    if high <= low {
        return 0.0;
    }
    let root = |one: f64, other: f64| (one * other).max(0.0).sqrt();
    let step = TAU / PLACES as f64;
    let mut total = 0.0;
    for place in 0..PLACES {
        let (sin, cos) = ((place as f64 + 0.5) * step).sin_cos();
        let y = low + (high - low) * (1.0 - cos) / 2.0;
        let chord = 2.0 * root(standing - y, standing + y);
        let height = 2.0 * root(radius - (y - across), radius + (y - across));
        total += chord * height * sin.abs() * step;
    }
    total * (high - low) / 4.0
}

/// What a cylinder of `radius` lying across the stock, `across` from its axis,
/// has in common with it.
pub(crate) fn common_across(radius: f64, across: f64) -> f64 {
    common(STOCK_RADIUS, radius, across)
}
