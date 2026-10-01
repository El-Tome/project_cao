//! Drawings made from a seed, weighted towards the drawings whose areas break.
//!
//! A drawing in general position does not break: the walk fails where two
//! sides lie along each other, where a corner lands on a point already drawn,
//! where a curve touches another — and worse, where they very nearly do.
//! Corners are drawn on a lattice of round numbers so that coincidences come
//! up on their own; half the shapes are drawn from one already there so that
//! they come up often; and a coincidence is now and then missed by a hair.

use std::f64::consts::TAU;

use cao_sketch::Sketch;
use glam::DVec2;

use super::random::Random;
use super::{Axis, Gesture};

/// The distance under which the drawing calls two places one, as a share of
/// how far a place stands out: `THE_SAME_PLACE`, `crates/sketch/src/edges.rs`.
const THE_SAME_PLACE: f64 = 1e-7;

/// A shape drawn earlier, as the generator remembers it: what a later shape
/// can be drawn from.
#[derive(Clone, Copy, Debug)]
enum Shape {
    Box { low: DVec2, high: DVec2 },
    Round { centre: DVec2, radius: f64 },
}

struct Hand {
    random: Random,
    /// How big the drawing is drawn: most on a lattice of units, some thirty
    /// times larger, where a tolerance taken in absolute units stops holding.
    scale: f64,
    sketch: Sketch,
    shapes: Vec<Shape>,
}

/// The drawing a seed stands for. The same seed always draws the same
/// drawing, which is what lets a failure be named by its seed alone.
pub fn drawn(seed: u64) -> Vec<super::Gesture> {
    let mut random = Random::seeded(seed);
    let scale = *random.pick(&[1.0, 1.0, 1.0, 5.0, 30.0]);
    let mut hand = Hand {
        random,
        scale,
        sketch: Sketch::new(cao_sketch::WorkPlane::XY),
        shapes: Vec::new(),
    };
    let count = *hand.random.pick(&[1, 2, 2, 3, 3, 4, 4, 5, 6, 7, 8, 10, 12]);
    let mut gestures = Vec::new();
    for _ in 0..count {
        let changing = hand.sketch.drawn_points().next().is_some() && hand.random.chance(0.35);
        let gesture = match changing.then(|| hand.change()).flatten() {
            Some(gesture) => gesture,
            None => hand.draw(),
        };
        gesture.lay(&mut hand.sketch);
        gestures.push(gesture);
    }
    gestures
}

fn pair(at: DVec2) -> [f64; 2] {
    at.to_array()
}

impl Hand {
    /// A coordinate: mostly on the lattice, sometimes on its half.
    fn coordinate(&mut self, low: f64, high: f64) -> f64 {
        let step = if self.random.chance(0.8) { 1.0 } else { 0.5 };
        self.random.on_lattice(low, high, step) * self.scale
    }

    fn length(&mut self, low: f64, high: f64) -> f64 {
        self.random.on_lattice(low, high, 0.5) * self.scale
    }

    fn place(&mut self) -> DVec2 {
        DVec2::new(self.coordinate(0.0, 10.0), self.coordinate(0.0, 10.0))
    }

    /// A miss by a hair: from five times the distance under which the drawing
    /// calls two places one — read forty times the drawing's unit out from the
    /// origin, further than the places a hair is drawn at, once moved or
    /// turned to be laid again — up to a thousand times that.
    fn hair(&mut self) -> f64 {
        let furthest = 40.0 * self.scale;
        let finest = 5.0 * THE_SAME_PLACE * (1.0 + furthest);
        finest * *self.random.pick(&[1.0, 2.0, 10.0, 100.0, 1000.0])
    }

    /// A direction on the lattice: along an axis, or diagonal.
    fn direction(&mut self) -> DVec2 {
        *self.random.pick(&[
            DVec2::X,
            DVec2::Y,
            DVec2::NEG_X,
            DVec2::NEG_Y,
            DVec2::ONE.normalize(),
            DVec2::new(1.0, -1.0).normalize(),
        ])
    }

    fn construction(&mut self) -> bool {
        self.random.chance(0.15)
    }

    fn earlier(&mut self) -> Option<Shape> {
        if self.shapes.is_empty() || self.random.chance(0.5) {
            return None;
        }
        Some(*self.random.pick(&self.shapes))
    }

    fn draw(&mut self) -> Gesture {
        match self.random.below(20) {
            0..=5 => self.rectangle(),
            6..=8 => self.circle(),
            9 | 10 => self.arc(),
            11 => self.ellipse(),
            12 => self.half_ellipse(),
            13..=18 => self.chain(),
            _ => self.point(),
        }
    }

    fn rectangle(&mut self) -> Gesture {
        let construction = self.construction();
        let flat = |hand: &mut Self| match hand.random.chance(0.03) {
            true => 0.0,
            false => 1.0,
        };
        let size = DVec2::new(self.length(1.0, 6.0) * flat(self), self.length(1.0, 6.0));
        let (corner, opposite) = match self.earlier() {
            Some(Shape::Box { low, high }) => self.beside(low, high, size),
            _ => {
                let corner = self.place();
                let toward = *self.random.pick(&[
                    DVec2::ONE,
                    DVec2::new(-1.0, 1.0),
                    DVec2::NEG_ONE,
                    DVec2::new(1.0, -1.0),
                ]);
                (corner, corner + size * toward)
            }
        };
        let (corner, opposite) = if self.random.chance(0.1) {
            (corner + self.direction() * self.hair(), opposite)
        } else {
            (corner, opposite)
        };
        self.shapes.push(Shape::Box {
            low: corner.min(opposite),
            high: corner.max(opposite),
        });
        Gesture::Rectangle {
            corner: pair(corner),
            opposite: pair(opposite),
            construction,
        }
    }

    /// A rectangle drawn from one already there: from its corner, along its
    /// side, inside it against its edge, standing on it, sharing a side, or on
    /// its baseline.
    fn beside(&mut self, low: DVec2, high: DVec2, size: DVec2) -> (DVec2, DVec2) {
        let width = high - low;
        let along = |hand: &mut Self, from: f64, span: f64| {
            from + hand.random.on_lattice(0.0, span / hand.scale, 0.5) * hand.scale
        };
        match self.random.below(6) {
            0 => {
                let corner = *self.random.pick(&[
                    low,
                    high,
                    DVec2::new(low.x, high.y),
                    DVec2::new(high.x, low.y),
                ]);
                let toward =
                    *self
                        .random
                        .pick(&[DVec2::ONE, DVec2::NEG_ONE, DVec2::new(1.0, -1.0)]);
                (corner, corner + size * toward)
            }
            1 => {
                let y = along(self, low.y - size.y, width.y + size.y);
                (
                    DVec2::new(high.x, y),
                    DVec2::new(high.x + size.x, y + size.y),
                )
            }
            2 => {
                let x = along(self, low.x, width.x);
                let right = (x + size.x).min(high.x);
                let bottom = (high.y - size.y).max(low.y);
                (
                    DVec2::new(x, high.y),
                    DVec2::new(right.max(x + self.scale * 0.5), bottom),
                )
            }
            3 => {
                let x = along(self, low.x - size.x, width.x + size.x);
                (
                    DVec2::new(x, high.y),
                    DVec2::new(x + size.x, high.y + size.y),
                )
            }
            4 => (
                DVec2::new(high.x, low.y),
                DVec2::new(high.x + size.x, high.y),
            ),
            _ => {
                let x = along(self, low.x, width.x);
                (DVec2::new(x, low.y), DVec2::new(x + size.x, low.y + size.y))
            }
        }
    }

    fn circle(&mut self) -> Gesture {
        let construction = self.construction();
        let (centre, radius) = match self.earlier() {
            Some(Shape::Round { centre, radius }) => {
                let radius_now = self.length(0.5, 4.0);
                match self.random.below(4) {
                    0 => (centre, radius_now),
                    1 => (
                        centre + self.direction() * (radius + radius_now),
                        radius_now,
                    ),
                    2 => {
                        let apart = (radius - radius_now).abs();
                        (centre + self.direction() * apart, radius_now)
                    }
                    _ => (centre + self.direction() * radius, radius_now),
                }
            }
            Some(Shape::Box { low, high }) => {
                let middle = (low + high) / 2.0;
                match self.random.below(3) {
                    0 => (middle, (high - low).min_element() / 2.0),
                    1 => (low, self.length(0.5, 3.0)),
                    _ => (middle, (high.y - middle.y).max(self.scale * 0.5)),
                }
            }
            None => (self.place(), self.length(0.5, 5.0)),
        };
        let radius = if self.random.chance(0.1) {
            radius + self.hair() * *self.random.pick(&[1.0, -1.0])
        } else {
            radius
        };
        self.shapes.push(Shape::Round { centre, radius });
        Gesture::Circle {
            centre: pair(centre),
            radius,
            construction,
        }
    }

    fn arc(&mut self) -> Gesture {
        let construction = self.construction();
        let degrees = *self
            .random
            .pick(&[45.0, 90.0, 90.0, 135.0, 180.0, 180.0, 270.0, 60.0, 300.0]);
        let quarter = *self
            .random
            .pick(&[DVec2::X, DVec2::Y, DVec2::NEG_X, DVec2::NEG_Y]);
        let (centre, start) = match self.earlier() {
            Some(Shape::Round { centre, radius }) => (centre, centre + quarter * radius),
            Some(Shape::Box { low, high }) => {
                let corner = *self.random.pick(&[low, high]);
                (corner, corner + quarter * self.length(0.5, 3.0))
            }
            None => {
                let centre = self.place();
                (centre, centre + quarter * self.length(0.5, 5.0))
            }
        };
        Gesture::Arc {
            centre: pair(centre),
            start: pair(start),
            degrees,
            construction,
        }
    }

    fn ellipse(&mut self) -> Gesture {
        let construction = self.construction();
        let (centre, reach, across) = match self.earlier() {
            Some(Shape::Box { low, high }) => {
                let middle = (low + high) / 2.0;
                let half = (high - low) / 2.0;
                (middle, middle + DVec2::X * half.x, half.y)
            }
            Some(Shape::Round { centre, radius }) => {
                (centre, centre + DVec2::X * radius * 2.0, radius)
            }
            None => {
                let centre = self.place();
                let first = self.direction() * self.length(1.0, 5.0);
                (centre, centre + first, self.length(0.5, 3.0))
            }
        };
        Gesture::Ellipse {
            centre: pair(centre),
            reach: pair(reach),
            across,
            construction,
        }
    }

    fn half_ellipse(&mut self) -> Gesture {
        let construction = self.construction();
        let (from, to) = match self.earlier() {
            Some(Shape::Box { low, high }) => (DVec2::new(low.x, high.y), high),
            _ => {
                let from = self.place();
                (from, from + self.direction() * self.length(1.0, 6.0))
            }
        };
        let rise = self.length(0.5, 3.0) * *self.random.pick(&[1.0, -1.0]);
        Gesture::HalfEllipse {
            from: pair(from),
            to: pair(to),
            rise,
            construction,
        }
    }

    /// Traits from place to place, some of the places taken from points
    /// already drawn: a shape closed or left open, a side drawn again, a trait
    /// laid along a side, a shape a hair short of closing.
    fn chain(&mut self) -> Gesture {
        let construction = self.construction();
        let points: Vec<DVec2> = self.sketch.live_points().map(|(_, at)| at).collect();
        let count = 2 + self.random.below(5);
        let mut through: Vec<DVec2> = Vec::new();
        for _ in 0..count {
            let at = if !points.is_empty() && self.random.chance(0.4) {
                *self.random.pick(&points)
            } else if let Some(last) = through.last().copied().filter(|_| self.random.chance(0.6)) {
                last + self.direction().round() * self.length(1.0, 5.0)
            } else {
                self.place()
            };
            if through.last() != Some(&at) {
                through.push(at);
            }
        }
        if through.len() < 2 {
            through.push(through[0] + DVec2::X * self.scale);
        }
        let closed = through.len() > 2 && self.random.chance(0.4);
        if !closed && self.random.chance(0.2) {
            let missed = match self.random.chance(0.5) {
                true => (through.len() > 2).then_some(through[0]),
                false => {
                    let places = self.places_on_curves();
                    (!places.is_empty()).then(|| *self.random.pick(&places))
                }
            };
            if let Some(missed) = missed {
                through.push(missed + self.direction() * self.hair());
            }
        }
        Gesture::Chain {
            through: through.into_iter().map(pair).collect(),
            closed,
            construction,
        }
    }

    fn point(&mut self) -> Gesture {
        let at = match self.random.below(3) {
            0 => self.place(),
            1 => match self.places_on_curves().as_slice() {
                [] => self.place(),
                places => *self.random.pick(places),
            },
            _ => {
                let points: Vec<DVec2> = self.sketch.live_points().map(|(_, at)| at).collect();
                *self.random.pick(&points)
            }
        };
        Gesture::Point { at: pair(at) }
    }

    /// A change to what is drawn, or nothing when there is nothing it could
    /// act on.
    fn change(&mut self) -> Option<Gesture> {
        match self.random.below(12) {
            0 | 1 => self.cornered(true),
            2 | 3 => self.cornered(false),
            4 => self.mirror(),
            5 => self.pattern_around(),
            6 => self.pattern_along(),
            7 | 8 => self.divide(),
            9 | 10 => self.on_a_curve().map(|at| Gesture::Trim { at }),
            _ => self.on_a_curve().map(|at| Gesture::Erase { at }),
        }
    }

    fn corners(&self) -> Vec<DVec2> {
        self.sketch
            .live_points()
            .filter(|(point, _)| self.sketch.corner_at(*point).is_some())
            .map(|(_, at)| at)
            .collect()
    }

    fn cornered(&mut self, rounded: bool) -> Option<Gesture> {
        let corners = self.corners();
        if corners.is_empty() {
            return None;
        }
        let at = pair(*self.random.pick(&corners));
        let size = *self.random.pick(&[0.25, 0.5, 1.0, 2.0]) * self.scale;
        Some(match rounded {
            true => Gesture::Fillet { at, radius: size },
            false => Gesture::Chamfer { at, length: size },
        })
    }

    /// Places lying on the curves still drawn: the middles and quarters of
    /// traits, a few places round circles, arcs and ellipses.
    fn places_on_curves(&self) -> Vec<DVec2> {
        let sketch = &self.sketch;
        let mut places = Vec::new();
        for (id, _) in sketch.live_segments() {
            let (from, to) = sketch.endpoints(id);
            places.extend([0.5, 0.25].map(|along| from.lerp(to, along)));
        }
        for (_, circle) in sketch.live_circles() {
            let centre = sketch.point(circle.center);
            places.extend((0..4).map(|quarter| {
                centre + DVec2::from_angle(0.3 + quarter as f64 * TAU / 4.0) * circle.radius
            }));
        }
        for (id, _) in sketch.live_arcs() {
            places.push(sketch.arc_midpoint(id));
        }
        for (id, _) in sketch.live_ellipses() {
            let (from, sweep) = sketch.ellipse_run(id);
            let drawn = sketch.ellipse_draft(id);
            places.extend([0.3, 0.6].map(|share| drawn.at(from + sweep * share)));
        }
        places
    }

    fn on_a_curve(&mut self) -> Option<[f64; 2]> {
        let places = self.places_on_curves();
        (!places.is_empty()).then(|| pair(*self.random.pick(&places)))
    }

    /// What a copy is made of: a whole shape drawn earlier when there is one,
    /// otherwise a few curves.
    fn selection(&mut self) -> Option<Vec<[f64; 2]>> {
        let places = self.places_on_curves();
        if places.is_empty() {
            return None;
        }
        if let Some(Shape::Box { low, high }) = self.earlier() {
            let middle = (low + high) / 2.0;
            return Some(
                [
                    DVec2::new(middle.x, low.y),
                    DVec2::new(high.x, middle.y),
                    DVec2::new(middle.x, high.y),
                    DVec2::new(low.x, middle.y),
                ]
                .map(pair)
                .to_vec(),
            );
        }
        let count = 1 + self.random.below(3);
        Some(
            (0..count)
                .map(|_| pair(*self.random.pick(&places)))
                .collect(),
        )
    }

    fn axis(&mut self) -> Axis {
        let traits: Vec<DVec2> = self
            .sketch
            .live_segments()
            .map(|(id, _)| {
                let (from, to) = self.sketch.endpoints(id);
                from.lerp(to, 0.5)
            })
            .collect();
        match self.random.below(3) {
            0 => Axis::U,
            1 => Axis::V,
            _ if !traits.is_empty() => Axis::Trait(pair(*self.random.pick(&traits))),
            _ => Axis::V,
        }
    }

    fn mirror(&mut self) -> Option<Gesture> {
        let of = self.selection()?;
        let axis = self.axis();
        Some(Gesture::Mirror { of, axis })
    }

    fn pattern_around(&mut self) -> Option<Gesture> {
        let of = self.selection()?;
        let points: Vec<DVec2> = self.sketch.live_points().map(|(_, at)| at).collect();
        let centre = pair(*self.random.pick(&points));
        Some(Gesture::PatternAround {
            of,
            centre,
            degrees: *self.random.pick(&[90.0, 120.0, 180.0, 60.0, 45.0, 72.0]),
            count: 2 + self.random.below(4),
        })
    }

    fn pattern_along(&mut self) -> Option<Gesture> {
        let of = self.selection()?;
        let axis = self.axis();
        let step = self.length(1.0, 5.0);
        let step = match self.random.below(4) {
            0 => step + self.hair(),
            _ => step,
        };
        let across = match self.random.chance(0.3) {
            true => (self.length(1.0, 5.0), 2),
            false => (0.0, 1),
        };
        Some(Gesture::PatternAlong {
            of,
            axis,
            along: (step, 2 + self.random.below(3)),
            across,
        })
    }

    fn divide(&mut self) -> Option<Gesture> {
        let crossings = self.sketch.crossings();
        (!crossings.is_empty()).then(|| Gesture::Divide {
            at: pair(*self.random.pick(&crossings)),
        })
    }
}
