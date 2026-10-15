//! An arc an edge of a section bows along, and what it turns into: a sphere
//! where its centre stands on the axis, a torus elsewhere. The arc's own
//! arithmetic — its height along the axis, the turn from one of its corners
//! to the other, what it sweeps — and what a line holds through a ball or a
//! torus, by arithmetic alone.
//!
//! A line through a sphere is one quadratic, as through a disc. A line
//! through a torus is a quartic in how far along the line it stands: solved
//! by bracketing between the critical points of the quartic, which its
//! derivative's roots give the same way, and bisecting to the last bit, each
//! root then polished on the distance to the tube's own circle, which also
//! tells the torus's own lobe from its mirror image across the axis.

use glam::{DVec2, DVec3};

use super::along::{Crossing, Seen, Stretch};

/// An arc an edge of a band bows along, from the corner where the band
/// starts to the one where it ends: the centre of its circle in the
/// section's `(a, s)`, and its radius, signed towards `+s` when the arc
/// bulges that way between its two corners and towards `-s` otherwise. A bow
/// holds the arc that stays a function of `a` — the short way round, or a
/// half circle — so its corners are the furthest it reaches along the axis,
/// and it lies on the branch of its circle its sign names.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bow {
    pub center: DVec2,
    pub radius: f64,
}

/// How far a corner may stand off its circle, as a share of the radius, and
/// still be read as on it: a few roundings of a corner moved by a hair.
const ON_THE_CIRCLE: f64 = 1e-9;

impl Bow {
    /// Which way the arc bulges: `+1.0` towards `+s`, `-1.0` towards `-s`.
    pub fn bulge(&self) -> f64 {
        self.radius.signum()
    }

    /// The arc's height at `along`, on its branch of the circle; the
    /// centre's height past the circle's reach.
    pub fn at(&self, along: f64) -> f64 {
        let r = self.radius.abs();
        let x = along - self.center.x;
        self.center.y + self.bulge() * (r * r - x * x).max(0.0).sqrt()
    }

    /// Whether both `corners` stand on the arc's branch of the circle, each
    /// within its reach along the axis.
    pub fn holds(&self, corners: [DVec2; 2]) -> bool {
        let r = self.radius.abs();
        r > 0.0
            && corners.iter().all(|corner| {
                let x = (corner.x - self.center.x).abs();
                x <= r * (1.0 + ON_THE_CIRCLE)
                    && (self.at(corner.x) - corner.y).abs() <= ON_THE_CIRCLE * r
            })
    }

    /// The arc's furthest point in the way it bulges: the circle's own, when
    /// it stands between the two corners along the axis, and the corner that
    /// reaches furthest otherwise.
    pub fn extreme(&self, corners: [DVec2; 2]) -> DVec2 {
        let bulged = self.center + DVec2::Y * self.radius;
        if corners[0].x <= bulged.x && bulged.x <= corners[1].x {
            bulged
        } else if corners[0].y * self.bulge() >= corners[1].y * self.bulge() {
            corners[0]
        } else {
            corners[1]
        }
    }

    /// The signed turn in radians, anticlockwise positive, from the first
    /// corner to the second along the arc.
    pub fn turn(&self, corners: [DVec2; 2]) -> f64 {
        let (u, v) = (corners[0] - self.center, corners[1] - self.center);
        -self.bulge() * u.perp_dot(v).abs().atan2(u.dot(v))
    }

    /// The same arc read from the other side of the axis.
    pub fn mirrored(&self) -> Bow {
        Bow {
            center: DVec2::new(self.center.x, -self.center.y),
            radius: -self.radius,
        }
    }

    /// The integral of the square of the arc's height from `along[0]` to
    /// `along[1]`: what the edge sweeps per radian, twice over, less the
    /// other edge's.
    pub fn squares(&self, along: [f64; 2]) -> f64 {
        let (cy, r) = (self.center.y, self.radius.abs());
        let primitive = |along: f64| {
            let x = along - self.center.x;
            let root = (r * r - x * x).max(0.0).sqrt();
            (cy * cy + r * r) * x - x * x * x / 3.0
                + cy * self.bulge() * (x * root + r * r * (x / r).clamp(-1.0, 1.0).asin())
        };
        primitive(along[1]) - primitive(along[0])
    }

    /// The first moment about the axis of the circular segment between the
    /// arc and its chord, positive, by another road than `squares`: the
    /// sector less the triangle to the centre.
    pub fn segment_moment(&self, corners: [DVec2; 2]) -> f64 {
        let (cy, r) = (self.center.y, self.radius.abs());
        let (u, v) = (corners[0] - self.center, corners[1] - self.center);
        let sweep = self.turn(corners).abs();
        let (from, to) = if self.turn(corners) < 0.0 {
            (v.to_angle(), v.to_angle() + sweep)
        } else {
            (u.to_angle(), u.to_angle() + sweep)
        };
        let sector = cy * r * r * sweep / 2.0 + r * r * r / 3.0 * (from.cos() - to.cos());
        let triangle = u.perp_dot(v).abs() / 2.0 * (cy + corners[0].y + corners[1].y) / 3.0;
        sector - triangle
    }
}

/// How many times a root is bisected at most: enough to reach the last bit
/// of a double from any bracket of the bounds.
const BISECTIONS: usize = 200;

/// How many Newton steps polish a root on the distance to the circle.
const POLISHING: usize = 8;

/// How near nought the distance to the tube's own circle must be, as a
/// share of the reach, for a root of the quartic to lie on the torus's own
/// lobe rather than on its mirror image across the axis.
const ON_THE_TUBE: f64 = 1e-9;

/// The real roots of the polynomial whose coefficients are given from the
/// constant term up, ascending: between consecutive critical points, where
/// the polynomial is monotone, a sign change holds one root, found by
/// bisection.
fn real_roots(coefficients: &[f64]) -> Vec<f64> {
    let degree = coefficients.len() - 1;
    let lead = coefficients[degree];
    if degree == 0 || lead == 0.0 {
        return if degree == 0 {
            Vec::new()
        } else {
            real_roots(&coefficients[..degree])
        };
    }
    if degree == 1 {
        return vec![-coefficients[0] / lead];
    }
    let bound = 1.0
        + coefficients[..degree]
            .iter()
            .map(|coefficient| (coefficient / lead).abs())
            .fold(0.0, f64::max);
    let derivative: Vec<f64> = (1..=degree)
        .map(|power| coefficients[power] * power as f64)
        .collect();
    let mut stops = vec![-bound];
    stops.extend(
        real_roots(&derivative)
            .into_iter()
            .filter(|critical| critical.abs() < bound),
    );
    stops.push(bound);
    stops.sort_by(f64::total_cmp);
    let at = |t: f64| {
        coefficients
            .iter()
            .rev()
            .fold(0.0, |value, coefficient| value * t + coefficient)
    };
    let mut roots = Vec::new();
    for pair in stops.windows(2) {
        let (mut low, mut high) = (pair[0], pair[1]);
        let (mut at_low, at_high) = (at(low), at(high));
        if at_low == 0.0 {
            roots.push(low);
            continue;
        }
        if at_low.signum() == at_high.signum() {
            continue;
        }
        for _ in 0..BISECTIONS {
            let middle = (low + high) / 2.0;
            if middle <= low || middle >= high {
                break;
            }
            let at_middle = at(middle);
            if at_middle == 0.0 {
                low = middle;
                high = middle;
                break;
            }
            if at_middle.signum() == at_low.signum() {
                low = middle;
                at_low = at_middle;
            } else {
                high = middle;
            }
        }
        roots.push((low + high) / 2.0);
    }
    if at(bound) == 0.0 {
        roots.push(bound);
    }
    roots
}

/// A sphere or a torus about the axis a line is seen across: the tube's
/// centre `center` along the axis and away from it, the latter nought for a
/// sphere, and the tube's radius.
struct Round {
    center: DVec2,
    radius: f64,
}

impl Round {
    /// How far the line's point `at` along it stands from the axis, and
    /// where it stands along the axis.
    fn seen(&self, seen: &Seen, at: f64) -> (f64, f64) {
        (
            (seen.start + seen.step * at).length(),
            seen.level + seen.rise * at - self.center.x,
        )
    }

    /// The signed distance from the line's point `at` to the tube's circle.
    fn off(&self, seen: &Seen, at: f64) -> f64 {
        let (away, along) = self.seen(seen, at);
        (away - self.center.y).hypot(along) - self.radius
    }

    /// The root moved onto the circle by Newton's method on `off`.
    fn polished(&self, seen: &Seen, mut at: f64) -> f64 {
        for _ in 0..POLISHING {
            let off = self.off(seen, at);
            if off == 0.0 {
                break;
            }
            let (away, along) = self.seen(seen, at);
            let place = seen.start + seen.step * at;
            let away_rate = if away > 0.0 {
                place.dot(seen.step) / away
            } else {
                seen.step.length()
            };
            let rate = ((away - self.center.y) * away_rate + along * seen.rise)
                / (away - self.center.y).hypot(along);
            if rate == 0.0 || !rate.is_finite() {
                break;
            }
            let next = at - off / rate;
            if !next.is_finite() || next == at {
                break;
            }
            at = next;
        }
        at
    }

    /// The crossing at `at`: how squarely the line meets the tube there,
    /// read from the tube's normal, nought on the axis, where a sphere's
    /// pole and a horn's point have none the chart can tell.
    fn crossing(&self, seen: &Seen, at: f64) -> Crossing {
        let (away, along) = self.seen(seen, at);
        let place = seen.start + seen.step * at;
        let cosine = if away <= 1e-9 * (seen.start.length() + (seen.step * at).length()) {
            0.0
        } else {
            let normal = DVec3::new(
                (away - self.center.y) / self.radius * place.x / away,
                (away - self.center.y) / self.radius * place.y / away,
                along / self.radius,
            );
            normal
                .dot(DVec3::new(seen.step.x, seen.step.y, seen.rise))
                .abs()
                / seen.speed
        };
        Crossing {
            at,
            cosine,
            curved: true,
        }
    }

    /// Where the line crosses the tube, in order along it: the roots of the
    /// quartic on the tube's own lobe, or of the quadratic of a sphere.
    fn crossings(&self, seen: &Seen, reach: f64) -> Vec<f64> {
        let (a, b, c) = (
            seen.step.length_squared(),
            2.0 * seen.start.dot(seen.step),
            seen.start.length_squared(),
        );
        let (d, e) = (seen.rise, seen.level - self.center.x);
        let (cy, r) = (self.center.y, self.radius);
        let mut roots = if cy == 0.0 {
            real_roots(&[c + e * e - r * r, b + 2.0 * d * e, a + d * d])
        } else {
            let (alpha, beta, gamma) = (a + d * d, b + 2.0 * d * e, c + cy * cy + e * e - r * r);
            real_roots(&[
                gamma * gamma - 4.0 * cy * cy * c,
                2.0 * beta * gamma - 4.0 * cy * cy * b,
                beta * beta + 2.0 * alpha * gamma - 4.0 * cy * cy * a,
                2.0 * alpha * beta,
                alpha * alpha,
            ])
        };
        for root in &mut roots {
            *root = self.polished(seen, *root);
        }
        roots.retain(|root| root.is_finite() && self.off(seen, *root).abs() <= ON_THE_TUBE * reach);
        roots.sort_by(f64::total_cmp);
        roots.dedup_by(|one, other| (*one - *other).abs() <= ON_THE_TUBE * reach);
        roots
    }

    /// Where the line lies inside the tube, in order along it.
    fn inside(&self, seen: &Seen, reach: f64) -> Vec<Stretch> {
        if self.radius <= 0.0 {
            return Vec::new();
        }
        let crossings = self.crossings(seen, reach);
        let (pairs, _) = crossings.as_chunks::<2>();
        pairs
            .iter()
            .filter(|[from, to]| from < to)
            .map(|&[from, to]| Stretch {
                from: self.crossing(seen, from),
                to: self.crossing(seen, to),
            })
            .collect()
    }
}

/// Where the line lies inside the sphere or the torus of `tube` radius
/// about the axis the line is seen across, whose centre stands `center`
/// along the axis and away from it: a sphere when the latter is nought, and
/// otherwise the torus's own lobe, the one on the near side of the axis.
pub(super) fn tube(seen: &Seen, center: DVec2, radius: f64) -> Vec<Stretch> {
    let reach = seen.start.length().max(seen.level.abs()).max(radius) + center.abs().max_element();
    Round { center, radius }.inside(seen, reach.max(1.0))
}
