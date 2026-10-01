//! The outlines made of straight runs and arcs — a rectangle with rounded
//! corners and a slot — as the one contour both kernels are handed: the exact
//! kernel raises it as it is, and the flats are sampled from it the way the
//! application samples an arc. And the same outlines as the rectangles and
//! discs they are the union of, which is what a line is measured against.

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use cao_solid::profile::{Contour, Run};
use glam::DVec2;

use super::{CIRCLE_STEPS, Outline};

/// Rectangles, each from its low corner to its high one, and discs, each a
/// centre and a radius, whose union is an outline.
pub struct Pieces {
    pub rectangles: Vec<(DVec2, DVec2)>,
    pub discs: Vec<(DVec2, f64)>,
}

impl Outline {
    /// The contour of a rounded rectangle or a slot, anticlockwise, every arc
    /// tangent to the straight runs either side of it: `None` for any other
    /// outline.
    pub fn contour(&self) -> Option<Contour> {
        match *self {
            Outline::Rounded { low, high, radius } => Some(rounded(low, high, radius)),
            Outline::Slot { from, to, radius } => Some(slot(from, to, radius)),
            _ => None,
        }
    }

    /// The pieces of a rounded rectangle or a slot grown by `by` all round,
    /// or shrunk when `by` is negative: `None` for any other outline.
    ///
    /// Both are exact either way. A rounded rectangle grown keeps the centres
    /// of its corners and takes `by` on their radius; shrunk past its radius
    /// it is the rectangle with square corners inside it. A slot keeps its
    /// two centres, and shrunk past its radius is nothing.
    pub fn pieces(&self, by: f64) -> Option<Pieces> {
        match *self {
            Outline::Rounded { low, high, radius } => {
                let (low, high, radius) = (low - by, high + by, (radius + by).max(0.0));
                let (near, far) = (low + radius, high - radius);
                Some(Pieces {
                    rectangles: vec![
                        (near.with_y(low.y), far.with_y(high.y)),
                        (low.with_y(near.y), high.with_y(far.y)),
                    ],
                    discs: [near, far.with_y(near.y), far, near.with_y(far.y)]
                        .map(|center| (center, radius))
                        .to_vec(),
                })
            }
            Outline::Slot { from, to, radius } => {
                let radius = radius + by;
                let across = if from.y == to.y { DVec2::Y } else { DVec2::X } * radius;
                Some(Pieces {
                    rectangles: vec![(from.min(to) - across, from.max(to) + across)],
                    discs: vec![(from, radius), (to, radius)],
                })
            }
            _ => None,
        }
    }
}

/// The four sides from the bottom one round, each a straight run and the
/// quarter circle that turns into the next. A side no longer than the two
/// quarters at its ends takes up has no straight run left.
fn rounded(low: DVec2, high: DVec2, radius: f64) -> Contour {
    let (near, far) = (low + radius, high - radius);
    let size = high - low;
    let sides = [
        (
            DVec2::new(near.x, low.y),
            DVec2::new(far.x, low.y),
            far.with_y(near.y),
            size.x,
        ),
        (
            DVec2::new(high.x, near.y),
            DVec2::new(high.x, far.y),
            far,
            size.y,
        ),
        (
            DVec2::new(far.x, high.y),
            DVec2::new(near.x, high.y),
            near.with_y(far.y),
            size.x,
        ),
        (
            DVec2::new(low.x, far.y),
            DVec2::new(low.x, near.y),
            near,
            size.y,
        ),
    ];
    let mut contour = Contour {
        corners: Vec::new(),
        runs: Vec::new(),
    };
    for (start, end, center, length) in sides {
        if length > 2.0 * radius {
            contour.corners.push(start);
            contour.runs.push(Run::Straight);
        }
        contour.corners.push(end);
        contour.runs.push(Run::Round {
            center,
            turn: FRAC_PI_2,
        });
    }
    contour
}

/// Along the side to the right of the way from `from` to `to`, round the
/// half circle about `to`, back along the other side and round the half
/// circle about `from`.
fn slot(from: DVec2, to: DVec2, radius: f64) -> Contour {
    let side = (to - from).normalize().perp() * radius;
    Contour {
        corners: vec![from - side, to - side, to + side, from + side],
        runs: vec![
            Run::Straight,
            Run::Round {
                center: to,
                turn: PI,
            },
            Run::Straight,
            Run::Round {
                center: from,
                turn: PI,
            },
        ],
    }
}

/// The corners of the flats a contour is sampled into, and the curve each
/// flat from a corner to the next was sampled from: an arc takes its share
/// of the steps a whole circle is cut into, as the application cuts it, and
/// arcs of one circle are one curve.
pub fn sampled(contour: &Contour) -> (Vec<DVec2>, Vec<Option<usize>>) {
    let mut corners = Vec::new();
    let mut curves = Vec::new();
    let mut circles: Vec<(DVec2, f64)> = Vec::new();
    for (&corner, &run) in contour.corners.iter().zip(&contour.runs) {
        match run {
            Run::Straight => {
                corners.push(corner);
                curves.push(None);
            }
            Run::Round { center, turn } => {
                let radius = corner.distance(center);
                let curve = circles
                    .iter()
                    .position(|&circle| circle == (center, radius))
                    .unwrap_or_else(|| {
                        circles.push((center, radius));
                        circles.len() - 1
                    });
                let steps = ((turn.abs() / TAU * CIRCLE_STEPS as f64).ceil() as usize).max(2);
                for step in 0..steps {
                    let turned = DVec2::from_angle(turn * step as f64 / steps as f64);
                    corners.push(if step == 0 {
                        corner
                    } else {
                        center + turned.rotate(corner - center)
                    });
                    curves.push(Some(curve));
                }
            }
        }
    }
    (corners, curves)
}
