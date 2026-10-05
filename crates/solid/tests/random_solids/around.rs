//! What a turned leaf holds along one line, the box it spans and the volume
//! it encloses, by arithmetic alone.
//!
//! A rectangle of a section turned about the axis is a slab of an annulus cut
//! by a wedge: a line lies inside it where it lies between the two planes
//! square to the axis at the rectangle's ends, between the two cylinders of
//! its radii, and on the near side of the two planes through the axis the
//! turn starts and ends on. The leaf holds the union of its rectangles.

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use glam::{DVec2, DVec3};

use super::along::{Crossing, Seen, Stretch, far, ring, slab, union};
use super::sections::{Axis, Piece, Section};
use super::{Leaf, Plane};

/// A turned leaf, whichever way the case wrote it.
#[derive(Clone, Debug, PartialEq)]
pub struct Turned {
    pub plane: Plane,
    pub axis: Axis,
    pub section: Section,
    pub degrees: f64,
}

/// How close to a whole turn, in radians, the arithmetic takes a turn as
/// whole: the product's rule, written here again rather than read from it.
const WHOLE: f64 = 1e-3;

impl Leaf {
    /// The leaf as a section turned about an axis: a turned leaf as it is,
    /// a revolution as the rectangle it turns about the plane's second axis,
    /// nothing for a prism.
    pub fn as_turned(&self) -> Option<Turned> {
        match self {
            Leaf::Turned {
                plane,
                axis,
                section,
                degrees,
            } => Some(Turned {
                plane: *plane,
                axis: *axis,
                section: section.clone(),
                degrees: *degrees,
            }),
            Leaf::Revolution {
                plane,
                low,
                high,
                degrees,
            } => Some(Turned {
                plane: *plane,
                axis: Axis::second(0.0),
                section: Section::bands(low.y, &[[high.y - low.y, -high.x, -low.x]]),
                degrees: *degrees,
            }),
            Leaf::Prism { .. } => None,
        }
    }
}

/// One side of a turned leaf in space: a point of the axis and its
/// direction, the way from the axis to the side as drawn, and the way the
/// side turns from there, all of unit length and square to each other.
#[derive(Clone, Copy, Debug)]
pub struct Swept {
    pub origin: DVec3,
    pub along: DVec3,
    pub out: DVec3,
    pub onward: DVec3,
}

/// The cosine and the sine of a turn, exact on every quarter turn.
fn turned_by(radians: f64) -> DVec2 {
    let quarters = radians / FRAC_PI_2;
    if (quarters - quarters.round()).abs() < 1e-12 {
        [DVec2::X, DVec2::Y, DVec2::NEG_X, DVec2::NEG_Y][quarters.round().rem_euclid(4.0) as usize]
    } else {
        DVec2::from_angle(radians)
    }
}

impl Turned {
    /// How far the leaf turns, in radians, either way: a whole turn when it
    /// is within a thousandth of one.
    pub fn angle(&self) -> f64 {
        let angle = self.degrees.abs().to_radians();
        if self.is_whole() { TAU } else { angle }
    }

    pub fn is_whole(&self) -> bool {
        (self.degrees.abs().to_radians() - TAU).abs() < WHOLE
    }

    /// Where the side `side` of the leaf stands in space.
    pub fn swept(&self, side: f64) -> Swept {
        let line = self.axis.line();
        let origin = self.plane.to_world(line.origin);
        let along = (self.plane.to_world(line.origin + line.direction) - origin).normalize();
        let across =
            (self.plane.to_world(line.origin + line.direction.perp()) - origin).normalize();
        let out = across * side;
        Swept {
            origin,
            along,
            out,
            onward: along.cross(out) * self.degrees.signum(),
        }
    }

    /// The point of the side `side` at `along` along the axis and `away`
    /// from it, turned from where it was drawn by the turn whose cosine and
    /// sine `turn` holds.
    pub fn point(&self, side: f64, along: f64, away: f64, turn: DVec2) -> DVec3 {
        let swept = self.swept(side);
        swept.origin + swept.along * along + (swept.out * turn.x + swept.onward * turn.y) * away
    }

    /// The stretches of the line through `origin` along `direction` inside
    /// the leaf grown by `by`, or shrunk when `by` is negative: each
    /// rectangle's ends moved out by `by`, its radii apart by `by` either
    /// way and the planes its turn starts and ends on moved out by `by`.
    pub fn along_grown(&self, origin: DVec3, direction: DVec3, by: f64) -> Vec<Stretch> {
        self.along_pieces(&self.section.pieces(), origin, direction, by)
    }

    /// The same for other pieces than the section's own, turned as the leaf
    /// is.
    pub fn along_pieces(
        &self,
        pieces: &[Piece],
        origin: DVec3,
        direction: DVec3,
        by: f64,
    ) -> Vec<Stretch> {
        let mut stretches = Vec::new();
        for side in [1.0, -1.0] {
            let mine: Vec<&Piece> = pieces.iter().filter(|piece| piece.side == side).collect();
            if mine.is_empty() {
                continue;
            }
            let swept = self.swept(side);
            let from = origin - swept.origin;
            let seen = Seen {
                start: DVec2::new(from.dot(swept.out), from.dot(swept.onward)),
                step: DVec2::new(direction.dot(swept.out), direction.dot(swept.onward)),
                level: from.dot(swept.along),
                rise: direction.dot(swept.along),
                speed: direction.length(),
            };
            let wedge = self.wedge(&seen, by);
            for piece in mine {
                let Some(slab) = slab(&seen, piece.along[0] - by, piece.along[1] + by) else {
                    continue;
                };
                let annulus = ring(&seen, DVec2::ZERO, piece.away[1] + by, piece.away[0] - by);
                stretches.extend(meet(&meet(&[slab], &annulus), &wedge));
            }
        }
        union(stretches)
    }

    /// Where the line lies between the plane the turn starts on and the one
    /// it ends on, each moved out by `by`: both, up to half a turn, either
    /// past it, and anywhere for a whole turn.
    fn wedge(&self, seen: &Seen, by: f64) -> Vec<Stretch> {
        if self.is_whole() {
            return vec![Stretch {
                from: far(f64::NEG_INFINITY),
                to: far(f64::INFINITY),
            }];
        }
        let end = turned_by(self.angle());
        let opening = beyond(seen, DVec2::Y, by);
        let closing = beyond(seen, DVec2::new(end.y, -end.x), by);
        if self.degrees.abs() <= 180.0 {
            meet(&opening, &closing)
        } else {
            union([opening, closing].concat())
        }
    }

    /// The lowest and the highest corner of the box the leaf spans: each
    /// rectangle's outer radius at both ends of the turn and wherever in
    /// between it goes furthest along a world axis, and its inner radius at
    /// both ends.
    pub fn bounds(&self) -> Option<(DVec3, DVec3)> {
        let mut points = Vec::new();
        for piece in self.section.pieces() {
            let swept = self.swept(piece.side);
            let mut turns: Vec<(DVec2, bool)> = Vec::new();
            if !self.is_whole() {
                turns.push((DVec2::X, true));
                turns.push((turned_by(self.angle()), true));
            }
            for axis in 0..3 {
                let (cosine, sine) = (swept.out[axis], swept.onward[axis]);
                if cosine == 0.0 && sine == 0.0 {
                    continue;
                }
                let furthest = sine.atan2(cosine).rem_euclid(TAU);
                for turn in [furthest, (furthest + PI).rem_euclid(TAU)] {
                    if self.is_whole() || turn <= self.angle() {
                        turns.push((turned_by(turn), false));
                    }
                }
            }
            for along in piece.along {
                for &(turn, end) in &turns {
                    points.push(self.point(piece.side, along, piece.away[1], turn));
                    if end {
                        points.push(self.point(piece.side, along, piece.away[0], turn));
                    }
                }
            }
        }
        points
            .into_iter()
            .map(|point| (point, point))
            .reduce(|(low, high), (other, _)| (low.min(other), high.max(other)))
    }

    /// The volume the leaf encloses: half the turn times what each rectangle
    /// sweeps per radian, less, past half a turn, what the two sides of a
    /// section across its axis sweep twice.
    pub fn volume(&self) -> f64 {
        let pieces = self.section.pieces();
        let swept: f64 = pieces
            .iter()
            .map(|piece| {
                (piece.away[1].powi(2) - piece.away[0].powi(2)) * (piece.along[1] - piece.along[0])
            })
            .sum();
        let twice: f64 = pieces
            .iter()
            .filter(|piece| piece.side > 0.0)
            .flat_map(|left| {
                pieces
                    .iter()
                    .filter(|piece| piece.side < 0.0)
                    .map(move |right| overlap(left, right))
            })
            .sum();
        self.angle() / 2.0 * swept - (self.angle() - PI).max(0.0) * twice
    }
}

/// What two rectangles of either side of the axis sweep per radian in
/// common, halved twice over: the length they share times the difference of
/// the squares of the radii they share.
fn overlap(one: &Piece, other: &Piece) -> f64 {
    let length = one.along[1].min(other.along[1]) - one.along[0].max(other.along[0]);
    let (inner, outer) = (
        one.away[0].max(other.away[0]),
        one.away[1].min(other.away[1]),
    );
    if length <= 0.0 || outer <= inner {
        return 0.0;
    }
    length * (outer * outer - inner * inner)
}

/// Where the line, seen across the axis, lies on the side of the line
/// through the axis that `normal` points into, that line moved out by `by`.
fn beyond(seen: &Seen, normal: DVec2, by: f64) -> Vec<Stretch> {
    let (level, rise) = (seen.start.dot(normal) + by, seen.step.dot(normal));
    if rise == 0.0 {
        return if level >= 0.0 {
            vec![Stretch {
                from: far(f64::NEG_INFINITY),
                to: far(f64::INFINITY),
            }]
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
            from: crossing,
            to: far(f64::INFINITY),
        }
    } else {
        Stretch {
            from: far(f64::NEG_INFINITY),
            to: crossing,
        }
    }]
}

/// Where the line lies inside both of two sets of stretches.
fn meet(one: &[Stretch], other: &[Stretch]) -> Vec<Stretch> {
    let mut met = Vec::new();
    for first in one {
        for second in other {
            let from = if second.from.at > first.from.at {
                second.from
            } else {
                first.from
            };
            let to = if second.to.at < first.to.at {
                second.to
            } else {
                first.to
            };
            if from.at < to.at {
                met.push(Stretch { from, to });
            }
        }
    }
    met
}
