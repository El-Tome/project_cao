//! The volume a round leaf keeps on one side of a plane, by slices across its
//! axis rather than by lines: an oracle that owes nothing to the line check,
//! for the cuts at a slant #552 brings.
//!
//! A slice of a circle or a ring prism, or of a whole turn, square to its
//! axis is a disc or a set of annuli, and a plane meets it along a chord
//! whose distance from the axis moves linearly along it: the slice keeps the
//! segments of its discs on one side of that chord. The area is smooth but
//! where a band starts or ends, where two edges cross, and where the chord
//! comes onto or leaves an edge, a three-halves power of the distance; the
//! integral is cut at every one of those places, all of them found in closed
//! form, and each piece is summed by tanh–sinh quadrature, which holds such
//! ends to the rounding.

use std::f64::consts::{FRAC_PI_2, PI};

use glam::DVec3;

use super::{Leaf, Outline};

/// One annulus of a round leaf along its axis: from `along[0]` to `along[1]`,
/// its inner and outer radius at each of those two places, linear between.
#[derive(Clone, Copy, Debug)]
struct Ring {
    along: [f64; 2],
    inner: [f64; 2],
    outer: [f64; 2],
}

impl Ring {
    fn edge(&self, radii: [f64; 2]) -> Edge {
        let slope = (radii[1] - radii[0]) / (self.along[1] - self.along[0]);
        Edge {
            at: self.along[0],
            radius: radii[0],
            slope,
        }
    }

    fn radii(&self, along: f64) -> [f64; 2] {
        [self.edge(self.inner), self.edge(self.outer)].map(|edge| edge.radius_at(along))
    }
}

/// A radius moving linearly along the axis.
#[derive(Clone, Copy, Debug)]
struct Edge {
    at: f64,
    radius: f64,
    slope: f64,
}

impl Edge {
    fn radius_at(&self, along: f64) -> f64 {
        (self.radius + self.slope * (along - self.at)).max(0.0)
    }

    /// Where along the axis `radius + slope·(s − at)` equals `base + rate·s`.
    fn meets(&self, base: f64, rate: f64) -> Option<f64> {
        let gap = self.slope - rate;
        (gap != 0.0).then(|| (base - self.radius + self.slope * self.at) / gap)
    }
}

/// A round leaf: a point of its axis, its direction of unit length, and its
/// annuli.
struct Round {
    origin: DVec3,
    axis: DVec3,
    rings: Vec<Ring>,
}

impl Leaf {
    /// The volume of the leaf on the side `normal · x ≤ offset` of a plane,
    /// to a relative 1e-12: nothing for a leaf that is not a circle or a ring
    /// prism or a whole turn.
    pub fn kept_by_a_half_space(&self, normal: DVec3, offset: f64) -> Option<f64> {
        let round = self.round()?;
        let along = normal.dot(round.axis);
        let across = (normal - round.axis * along).length();
        let level = offset - normal.dot(round.origin);
        let (low, high) = round
            .rings
            .iter()
            .fold((f64::MAX, f64::MIN), |(low, high), ring| {
                (low.min(ring.along[0]), high.max(ring.along[1]))
            });
        let square = across <= 1e-15 * normal.length();
        let (base, rate) = if square {
            (0.0, 0.0)
        } else {
            (level / across, -along / across)
        };
        let mut breaks = vec![low, high];
        for ring in &round.rings {
            breaks.extend(ring.along);
        }
        if square {
            if along != 0.0 {
                breaks.push(level / along);
            }
        } else {
            let edges: Vec<Edge> = round
                .rings
                .iter()
                .flat_map(|ring| [ring.edge(ring.inner), ring.edge(ring.outer)])
                .collect();
            for (index, edge) in edges.iter().enumerate() {
                breaks.extend(edge.meets(base, rate));
                breaks.extend(edge.meets(-base, -rate));
                for other in &edges[index + 1..] {
                    breaks.extend(edge.meets(other.radius - other.slope * other.at, other.slope));
                }
            }
        }
        breaks.retain(|at| at.is_finite() && (low..=high).contains(at));
        breaks.sort_by(f64::total_cmp);
        breaks.dedup();
        let area = |s: f64| {
            let annuli = round.annuli(s);
            if square {
                let kept = along * s <= level;
                return if kept {
                    annuli
                        .iter()
                        .map(|[inner, outer]| PI * (outer * outer - inner * inner))
                        .sum()
                } else {
                    0.0
                };
            }
            let chord = base + rate * s;
            annuli
                .iter()
                .map(|[inner, outer]| segment(*outer, chord) - segment(*inner, chord))
                .sum()
        };
        Some(
            breaks
                .windows(2)
                .map(|pair| tanh_sinh(&area, pair[0], pair[1]))
                .sum(),
        )
    }

    fn round(&self) -> Option<Round> {
        match self {
            Leaf::Prism {
                plane,
                outline,
                height,
            } => {
                let (center, inner, outer) = match outline {
                    Outline::Circle { center, radius, .. } => (*center, 0.0, *radius),
                    Outline::Ring {
                        center,
                        outer,
                        inner,
                    } => (*center, *inner, *outer),
                    _ => return None,
                };
                Some(Round {
                    origin: plane.to_world(center),
                    axis: plane.normal().normalize(),
                    rings: vec![Ring {
                        along: [height.min(0.0), height.max(0.0)],
                        inner: [inner; 2],
                        outer: [outer; 2],
                    }],
                })
            }
            Leaf::Revolution { .. } | Leaf::Turned { .. } => {
                let turned = self.as_turned().filter(|turned| turned.is_whole())?;
                let swept = turned.swept(1.0);
                Some(Round {
                    origin: swept.origin,
                    axis: swept.along,
                    rings: turned
                        .section
                        .pieces()
                        .iter()
                        .map(|piece| Ring {
                            along: piece.along,
                            inner: [piece.away[0], piece.ending[0]],
                            outer: [piece.away[1], piece.ending[1]],
                        })
                        .collect(),
                })
            }
        }
    }
}

impl Round {
    /// The annuli of the slice at `along` the axis, those of rings that
    /// overlap merged into one.
    fn annuli(&self, along: f64) -> Vec<[f64; 2]> {
        let mut spans: Vec<[f64; 2]> = self
            .rings
            .iter()
            .filter(|ring| ring.along[0] <= along && along <= ring.along[1])
            .map(|ring| ring.radii(along))
            .filter(|[inner, outer]| inner < outer)
            .collect();
        spans.sort_by(|one, other| one[0].total_cmp(&other[0]));
        let mut merged: Vec<[f64; 2]> = Vec::new();
        for span in spans {
            match merged.last_mut() {
                Some(last) if span[0] <= last[1] => last[1] = last[1].max(span[1]),
                _ => merged.push(span),
            }
        }
        merged
    }
}

/// The area of a disc of `radius` on the side of a chord `chord` from its
/// centre where the chord's own coordinate is smaller.
fn segment(radius: f64, chord: f64) -> f64 {
    if radius <= 0.0 || chord <= -radius {
        return 0.0;
    }
    if chord >= radius {
        return PI * radius * radius;
    }
    radius * radius * (-chord / radius).acos() + chord * (radius * radius - chord * chord).sqrt()
}

/// The integral of `f` from `from` to `to` by tanh–sinh quadrature, a step of
/// a sixty-fourth and two hundred steps either way, the nodes near each end
/// placed by their distance from it so that none lands on it.
fn tanh_sinh(f: &impl Fn(f64) -> f64, from: f64, to: f64) -> f64 {
    let half = (to - from) / 2.0;
    if half <= 0.0 {
        return 0.0;
    }
    let step = 1.0 / 64.0;
    let mut sum = 0.0;
    for k in -200..=200 {
        let t = f64::from(k) * step;
        let u = FRAC_PI_2 * t.sinh();
        let near = 1.0 / ((2.0 * u.abs()).exp() + 1.0);
        let place = if u >= 0.0 {
            to - (to - from) * near
        } else {
            from + (to - from) * near
        };
        let weight = half * FRAC_PI_2 * t.cosh() / u.cosh().powi(2);
        if weight > 0.0 && place > from && place < to {
            sum += weight * f(place);
        }
    }
    sum * step
}
