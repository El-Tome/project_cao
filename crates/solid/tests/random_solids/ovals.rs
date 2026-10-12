//! The outlines an ellipse bounds (#571) — a whole ellipse, one cut by
//! traits, a slot with elliptic ends, an elliptic ring — as the kernels are
//! handed them, and what arithmetic holds of them along a line.
//!
//! An ellipse is a circle seen through a map that keeps lines straight and
//! keeps how far along a line a place lies: a line crosses it where it
//! crosses the unit circle in the ellipse's own coordinates, one quadratic,
//! and a trait cutting it is a chord there. Grown or shrunk by a distance, an
//! ellipse is no ellipse. The one held for it keeps its axes, moves its minor
//! half-axis by the distance, and moves its major as far as the wall's
//! curvature at the end of the minor axis asks: grown, it holds every place
//! within the distance of the ellipse, and shrunk, none nearer its wall than
//! the distance — larger grown and smaller shrunk, the side a room for a
//! tolerance may err on.

use std::f64::consts::{PI, TAU};

use cao_solid::profile::{Contour, Run};
use glam::{DVec2, DVec3};

use super::Outline;
use super::along::{Crossing, Seen, Stretch, clipped, far, meet, union};

/// An ellipse of a plane: its centre, its first semi-axis as drawn, and the
/// half-length of its second, a quarter turn counter-clockwise from the
/// first.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Oval {
    pub center: DVec2,
    pub first: DVec2,
    pub second: f64,
}

impl Oval {
    pub fn new(center: DVec2, first: DVec2, second: f64) -> Oval {
        Oval {
            center,
            first,
            second,
        }
    }

    /// The second semi-axis.
    pub fn across(&self) -> DVec2 {
        self.first.perp().normalize() * self.second
    }

    /// The place at the eccentric angle `angle`, in radians, from the first
    /// semi-axis towards the second.
    pub fn at(&self, angle: f64) -> DVec2 {
        self.center + self.first * angle.cos() + self.across() * angle.sin()
    }

    pub fn major(&self) -> f64 {
        self.first.length().max(self.second)
    }

    pub fn minor(&self) -> f64 {
        self.first.length().min(self.second)
    }

    /// The direction of the major axis, or of the first when the two are
    /// one.
    pub fn major_direction(&self) -> DVec2 {
        if self.second > self.first.length() {
            self.first.perp().normalize()
        } else {
            self.first.normalize()
        }
    }

    /// How far the ellipse reaches from its centre along `direction`, of
    /// unit length: where a line square to it stands when it touches the
    /// ellipse.
    pub fn support(&self, direction: DVec2) -> f64 {
        self.first
            .dot(direction)
            .hypot(self.across().dot(direction))
    }

    /// Where a place stands against the ellipse: below nought inside it,
    /// nought on it, above outside.
    pub fn level(&self, place: DVec2) -> f64 {
        let away = place - self.center;
        let unit = self.first.normalize();
        let (x, y) = (
            away.dot(unit) / self.first.length(),
            away.dot(unit.perp()) / self.second,
        );
        x * x + y * y - 1.0
    }

    /// The outward normal of unit length at the eccentric angle `angle`.
    pub fn normal_at(&self, angle: f64) -> DVec2 {
        let unit = self.first.normalize();
        (unit * (angle.cos() / self.first.length()) + unit.perp() * (angle.sin() / self.second))
            .normalize()
    }

    /// The ellipse held for this one grown by `by` all round, or shrunk when
    /// `by` is negative: nothing once its minor half-axis is gone.
    ///
    /// Its minor half-axis moves by `by` and its major goes to
    /// `√((m + by)(M² + m·by)/m)`: then its support moves by at least `by`
    /// in every direction grown, and by at most `by` shrunk, which is what
    /// holds or keeps out every place within `by` of the wall. Read off the
    /// support function, square, and bound the concave root by its tangent
    /// at the minor axis's end.
    pub fn grown(&self, by: f64) -> Option<Oval> {
        if by == 0.0 {
            return Some(*self);
        }
        let (major, minor) = (self.major(), self.minor());
        let least = minor + by;
        if least <= 0.0 {
            return None;
        }
        let most = (least * (major * major + minor * by) / minor).sqrt();
        let (first, second) = if self.first.length() < self.second {
            (least, most)
        } else {
            (most, least)
        };
        Some(Oval {
            center: self.center,
            first: self.first.normalize() * first,
            second,
        })
    }

    /// Where a line crosses the ellipse: the unit circle's quadratic in the
    /// ellipse's own coordinates, its roots worked out as `disc` works a
    /// circle's, each crossing told how squarely the line meets the wall
    /// there.
    pub(super) fn stretches(&self, seen: &Seen) -> Vec<Stretch> {
        let unit = self.first.normalize();
        let side = unit.perp();
        let (first, second) = (self.first.length(), self.second);
        if first <= 0.0 || second <= 0.0 {
            return Vec::new();
        }
        let away = seen.start - self.center;
        let place = DVec2::new(away.dot(unit) / first, away.dot(side) / second);
        let step = DVec2::new(seen.step.dot(unit) / first, seen.step.dot(side) / second);
        let (a, b, c) = (
            step.length_squared(),
            2.0 * step.dot(place),
            place.length_squared() - 1.0,
        );
        if a == 0.0 {
            return if c < 0.0 {
                vec![everywhere()]
            } else {
                Vec::new()
            };
        }
        let discriminant = b * b - 4.0 * a * c;
        if discriminant <= 0.0 {
            return Vec::new();
        }
        let root = discriminant.sqrt();
        let q = -(b + b.signum() * root) / 2.0;
        let (one, other) = if q == 0.0 {
            (-root / (2.0 * a), root / (2.0 * a))
        } else {
            (q / a, c / q)
        };
        let crossing = |at: f64| {
            let there = place + step * at;
            let normal = DVec2::new(there.x / first, there.y / second).length();
            Crossing {
                at,
                cosine: root / (2.0 * normal * seen.speed),
                curved: true,
            }
        };
        vec![Stretch {
            from: crossing(one.min(other)),
            to: crossing(one.max(other)),
        }]
    }

    /// The trait from the place at eccentric angle `from` to the one at `to`,
    /// both in radians, the arc between them going counter-clockwise left
    /// out: the unit normal pointing away from what the trait keeps, and how
    /// far along it from the centre the trait stands.
    fn chord(&self, from: f64, to: f64) -> (DVec2, f64) {
        let (middle, half) = ((from + to) / 2.0, (to - from) / 2.0);
        let unit = self.first.normalize();
        let toward = unit * (middle.cos() / self.first.length())
            + unit.perp() * (middle.sin() / self.second);
        (toward.normalize(), half.cos() / toward.length())
    }

    /// The places of the world furthest along each world axis, either way,
    /// of the ellipse drawn on a plane through `base` with axes `u` and `v`,
    /// with the eccentric angle each lies at.
    fn extremes(&self, base: DVec3, u: DVec3, v: DVec3) -> Vec<(f64, DVec3)> {
        let across = self.across();
        let middle = base + u * self.center.x + v * self.center.y;
        let (first, second) = (
            u * self.first.x + v * self.first.y,
            u * across.x + v * across.y,
        );
        (0..3)
            .flat_map(|axis| {
                let angle = second[axis].atan2(first[axis]);
                [angle, angle + PI]
            })
            .map(|angle| (angle, middle + first * angle.cos() + second * angle.sin()))
            .collect()
    }
}

fn everywhere() -> Stretch {
    Stretch {
        from: far(f64::NEG_INFINITY),
        to: far(f64::INFINITY),
    }
}

/// Where the line lies on the side of a trait through the plane that its
/// normal points away from: the trait `offset` along `normal`, of unit
/// length, from `center`.
fn kept_by(seen: &Seen, center: DVec2, normal: DVec2, offset: f64) -> Vec<Stretch> {
    let (level, rise) = (
        (seen.start - center).dot(normal) - offset,
        seen.step.dot(normal),
    );
    if rise == 0.0 {
        return if level <= 0.0 {
            vec![everywhere()]
        } else {
            Vec::new()
        };
    }
    let crossing = Crossing {
        at: -level / rise,
        cosine: rise.abs() / seen.speed,
        curved: false,
    };
    vec![if rise > 0.0 {
        Stretch {
            from: far(f64::NEG_INFINITY),
            to: crossing,
        }
    } else {
        Stretch {
            from: crossing,
            to: far(f64::INFINITY),
        }
    }]
}

/// The arcs of a chorded ellipse in radians, each from its start to its end,
/// and the trait from its end to the next one's start, both read
/// counter-clockwise.
fn gaps(arcs: &[[f64; 2]]) -> Vec<[f64; 2]> {
    (0..arcs.len())
        .map(|index| {
            let next = arcs[(index + 1) % arcs.len()][0];
            let next = if index + 1 == arcs.len() {
                next + 360.0
            } else {
                next
            };
            [arcs[index][1].to_radians(), next.to_radians()]
        })
        .collect()
}

/// How far the slot's ends reach past its centres, and how far its sides
/// stand from the way between them, as the ellipse of either end.
fn caps(from: DVec2, to: DVec2, across: f64, along: f64) -> [Oval; 2] {
    let way = (to - from).normalize();
    [from, to].map(|center| Oval::new(center, way * along, across))
}

impl Outline {
    /// The ellipse an outline is drawn from: the one it is, the one its arcs
    /// follow, the outer one of a ring; for a slot, the one at its first
    /// end; nothing for an outline no ellipse bounds.
    pub fn oval(&self) -> Option<Oval> {
        match *self {
            Outline::Ellipse {
                center,
                first,
                second,
                ..
            }
            | Outline::Chorded {
                center,
                first,
                second,
                ..
            }
            | Outline::OvalRing {
                center,
                first,
                second,
                ..
            } => Some(Oval::new(center, first, second)),
            Outline::OvalSlot {
                from,
                to,
                across,
                along,
            } => Some(caps(from, to, across, along)[0]),
            _ => None,
        }
    }

    /// The hole of an elliptic ring as an ellipse about its centre.
    pub fn oval_hole(&self) -> Option<Oval> {
        let Outline::OvalRing {
            center,
            first,
            hole,
            ..
        } = *self
        else {
            return None;
        };
        Some(Oval::new(center, first.normalize() * hole.x, hole.y))
    }

    /// Every ellipse whose arcs bound the outline: a slot's two ends, a
    /// ring's outer ellipse and its hole.
    pub fn ovals(&self) -> Vec<Oval> {
        match *self {
            Outline::OvalSlot {
                from,
                to,
                across,
                along,
            } => caps(from, to, across, along).to_vec(),
            _ => self.oval().into_iter().chain(self.oval_hole()).collect(),
        }
    }

    /// The outline and the holes an outline an ellipse bounds is handed to
    /// the exact kernel as, every run anticlockwise: a whole ellipse as one
    /// turn from its corner, a chorded one as its arcs and the traits
    /// between them, a slot with elliptic ends as its two straight runs and
    /// two half ellipses, a ring with the hole a whole circle or a whole
    /// ellipse from the end of its first semi-axis. Nothing for any other
    /// outline.
    pub fn oval_contours(&self) -> Option<(Contour, Vec<Contour>)> {
        let oval = self.oval()?;
        let whole = |oval: Oval, from: f64| Contour {
            corners: vec![oval.at(from)],
            runs: vec![run(oval, TAU)],
        };
        Some(match self {
            Outline::Ellipse { from, .. } => (whole(oval, from.to_radians()), Vec::new()),
            Outline::Chorded { arcs, .. } => {
                let mut contour = Contour {
                    corners: Vec::new(),
                    runs: Vec::new(),
                };
                for [from, to] in arcs {
                    contour.corners.push(oval.at(from.to_radians()));
                    contour.runs.push(run(oval, (to - from).to_radians()));
                    contour.corners.push(oval.at(to.to_radians()));
                    contour.runs.push(Run::Straight);
                }
                (contour, Vec::new())
            }
            Outline::OvalSlot { .. } => {
                let [start, end] = self.ovals()[..] else {
                    unreachable!("a slot has two ends")
                };
                let side = start.across();
                (
                    Contour {
                        corners: vec![
                            start.center - side,
                            end.center - side,
                            end.center + side,
                            start.center + side,
                        ],
                        runs: vec![Run::Straight, run(end, PI), Run::Straight, run(start, PI)],
                    },
                    Vec::new(),
                )
            }
            Outline::OvalRing { hole, .. } => {
                let hole = if hole.x == hole.y {
                    super::whole_circle(oval.center, hole.x, oval.first.to_angle().to_degrees())
                } else {
                    whole(self.oval_hole()?, 0.0)
                };
                (whole(oval, 0.0), vec![hole])
            }
            _ => return None,
        })
    }

    /// The stretches of a line inside an outline an ellipse bounds, grown by
    /// `by` all round or shrunk when `by` is negative: a chorded ellipse
    /// held by the ellipse and every trait, each moved out by `by`; a slot
    /// as the band between its centres and its two ends; a ring as its
    /// outer ellipse less its hole, which shrinks as the ring grows. Nothing
    /// for any other outline.
    pub(super) fn oval_stretches(&self, seen: &Seen, by: f64) -> Option<Vec<Stretch>> {
        let oval = self.oval()?;
        let crossed = |oval: Oval, by: f64| {
            oval.grown(by)
                .map_or_else(Vec::new, |grown| grown.stretches(seen))
        };
        Some(match self {
            Outline::Ellipse { .. } => crossed(oval, by),
            Outline::Chorded { arcs, .. } => {
                gaps(arcs)
                    .into_iter()
                    .fold(crossed(oval, by), |kept, [from, to]| {
                        let (normal, offset) = oval.chord(from, to);
                        meet(&kept, &kept_by(seen, oval.center, normal, offset + by))
                    })
            }
            Outline::OvalSlot {
                from, to, across, ..
            } => {
                let way = (*to - *from).normalize();
                let along = |point: DVec2| DVec2::new(point.dot(way), point.dot(way.perp()));
                let turned = Seen {
                    start: along(seen.start - *from),
                    step: along(seen.step),
                    level: seen.level,
                    rise: seen.rise,
                    speed: seen.speed,
                };
                let band = clipped(
                    &turned,
                    DVec2::new(0.0, -across - by),
                    DVec2::new(from.distance(*to), across + by),
                    Some(1),
                );
                let ends = self.ovals().into_iter().flat_map(|end| crossed(end, by));
                union(band.into_iter().chain(ends).collect())
            }
            Outline::OvalRing { .. } => {
                let Some(&rim) = crossed(oval, by).first() else {
                    return Some(Vec::new());
                };
                let Some(&bore) = crossed(self.oval_hole()?, -by).first() else {
                    return Some(vec![rim]);
                };
                [
                    Stretch {
                        from: rim.from,
                        to: bore.from,
                    },
                    Stretch {
                        from: bore.to,
                        to: rim.to,
                    },
                ]
                .into_iter()
                .filter(|stretch| stretch.from.at < stretch.to.at)
                .collect()
            }
            _ => return None,
        })
    }

    /// The box in the plane of the whole ellipses an outline is drawn
    /// from: nothing for an outline no ellipse bounds.
    pub fn oval_box(&self) -> Option<(DVec2, DVec2)> {
        self.ovals()
            .iter()
            .map(|oval| {
                let reach = DVec2::new(oval.support(DVec2::X), oval.support(DVec2::Y));
                (oval.center - reach, oval.center + reach)
            })
            .reduce(|(low, high), (other_low, other_high)| {
                (low.min(other_low), high.max(other_high))
            })
    }

    /// The places of the world an outline an ellipse bounds reaches
    /// furthest along each world axis, drawn on a plane through `base` with
    /// axes `u` and `v`: the true curves' rather than their flats'. Nothing
    /// for any other outline.
    pub(super) fn oval_extremes(&self, base: DVec3, u: DVec3, v: DVec3) -> Option<Vec<DVec3>> {
        let oval = self.oval()?;
        Some(match self {
            Outline::Chorded { arcs, .. } => {
                let within = |angle: f64| {
                    arcs.iter().any(|[from, to]| {
                        let (from, to) = (from.to_radians(), to.to_radians());
                        let turned = (angle - from).rem_euclid(TAU);
                        turned <= to - from
                    })
                };
                oval.extremes(base, u, v)
                    .into_iter()
                    .filter(|(angle, _)| within(*angle))
                    .map(|(_, place)| place)
                    .chain(arcs.iter().flatten().map(|angle| {
                        let place = oval.at(angle.to_radians());
                        base + u * place.x + v * place.y
                    }))
                    .collect()
            }
            _ => self
                .ovals()
                .iter()
                .take(if matches!(self, Outline::OvalRing { .. }) {
                    1
                } else {
                    2
                })
                .flat_map(|oval| oval.extremes(base, u, v))
                .map(|(_, place)| place)
                .collect(),
        })
    }
}

/// An arc of `oval` turning `turn` radians from where it starts.
fn run(oval: Oval, turn: f64) -> Run {
    Run::Oval {
        center: oval.center,
        first: oval.first,
        second: oval.second,
        turn,
    }
}
