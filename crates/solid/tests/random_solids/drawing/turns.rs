//! Turned leaves drawn from a seed, among prisms of the profiles: straight
//! sections turned about lines of the planes of the origin, half of them
//! fresh and half drawn from a leaf before them — about the axis of a circle
//! raised before, on the edge of a block, along the axis of a turn before
//! it — where the coincidences a kernel breaks on come from.

use glam::{DVec2, DVec3};

use super::super::{Along, Axis, Leaf, Outline, Plane, Section};
use super::{Among, Drawing, shifted};

/// The angles a campaign tries a hair from where a turn changes kind: half a
/// turn, a quarter, a whole turn closing or still open, and a turn too short
/// to make anything to speak of.
const ON_THE_EDGE: [f64; 7] = [
    180.0 + 1e-7,
    180.0 - 1e-7,
    90.0 + 1e-7,
    90.0 - 1e-7,
    359.95,
    359.9,
    0.01,
];

/// A line of the world: a point it runs through, and which world axis it
/// runs along.
#[derive(Clone, Copy)]
struct Line {
    point: DVec3,
    axis: usize,
}

impl Drawing {
    /// Whether the draw goes past the lattice of the gate's: angles a hair
    /// from where a turn changes kind, and axes slanted or leaning a hair.
    fn off_the_lattice(&self) -> bool {
        self.among
            == Among::Turned {
                off_the_lattice: true,
            }
    }

    /// A leaf of a case of turns: a prism or a turned leaf, fresh or drawn
    /// from one before it.
    pub(super) fn turned_leaf(&mut self, before: Option<&Leaf>) -> Leaf {
        match before {
            None if self.random.chance(0.5) => self.fresh_turned(),
            None => Leaf::Prism {
                plane: self.plane(),
                outline: self.outline(),
                height: self.height(),
            },
            Some(before @ Leaf::Prism { .. }) if self.random.chance(0.5) => {
                self.turned_from_prism(before)
            }
            Some(before @ Leaf::Prism { .. }) => self.related(before),
            Some(Leaf::Turned {
                plane,
                axis,
                section,
                degrees,
            }) if axis.lean == 0.0 => {
                let line = Line::of(*plane, *axis);
                if self.random.chance(0.5) {
                    self.turned_along(line, *axis, section, *degrees)
                } else {
                    self.prism_from_turned(line, *axis, section)
                }
            }
            Some(_) => self.fresh_turned(),
        }
    }

    fn fresh_turned(&mut self) -> Leaf {
        let plane = self.plane();
        let axis = self.axis();
        let section = self.section();
        let section = self.aside(section);
        Leaf::Turned {
            plane,
            axis,
            section,
            degrees: self.degrees(),
        }
    }

    /// A line of the plane to turn about: the sketch's own axis half the
    /// time, a line drawn parallel to it on the lattice otherwise, run the
    /// other way a quarter of the time; and in a campaign, now and then,
    /// slanted, or moved and leaning by a hair.
    fn axis(&mut self) -> Axis {
        let along = if self.random.chance(0.5) {
            Along::First
        } else {
            Along::Second
        };
        let across = if self.random.chance(0.5) {
            0.0
        } else {
            self.coordinate(-3.0, 6.0)
        };
        let mut axis = Axis {
            along,
            across,
            backwards: self.random.chance(0.25),
            lean: 0.0,
        };
        if self.off_the_lattice() && self.random.chance(0.15) {
            if self.random.chance(0.5) {
                axis.lean = *self.random.pick(&[30.0, -45.0, 60.0, 15.0]);
            } else {
                axis.lean = *self.random.pick(&[1e-7, -1e-5, 1e-3]);
                axis.across += self.nudge();
            }
        }
        axis
    }

    /// How far a leaf turns: a whole turn half the time, otherwise a quarter
    /// turn's multiple or a step of fifteen degrees, backwards three times
    /// in ten; and in a campaign, now and then, a hair from where a turn
    /// changes kind.
    fn degrees(&mut self) -> f64 {
        let degrees = if self.off_the_lattice() && self.random.chance(0.05) {
            *self.random.pick(&ON_THE_EDGE)
        } else if self.random.chance(0.5) {
            360.0
        } else if self.random.chance(0.6) {
            *self.random.pick(&[90.0, 180.0, 270.0])
        } else {
            self.random.on_lattice(15.0, 345.0, 15.0)
        };
        if self.random.chance(0.3) {
            -degrees
        } else {
            degrees
        }
    }

    /// A low edge: on the axis a quarter of the time, off it on the lattice
    /// otherwise.
    fn low(&mut self) -> f64 {
        if self.random.chance(0.25) {
            0.0
        } else {
            self.coordinate(0.5, 4.0)
        }
    }

    /// A section on the left of its axis, or across it one time in eight:
    /// a rectangle, a stepped shaft on the axis, an L, a tube or a band with
    /// a hole; its edges on the axis now and then moved a hair off it.
    fn section(&mut self) -> Section {
        let from = self.coordinate(-3.0, 5.0);
        if self.random.chance(0.125) {
            return self.across(from);
        }
        let section = match self.random.below(20) {
            0..=6 => {
                let low = self.low();
                let band = [self.length(1.0, 6.0), low, low + self.length(1.0, 5.0)];
                Section::bands(from, &[band])
            }
            7..=11 => {
                let count = 2 + self.random.below(3);
                let mut bands: Vec<[f64; 3]> = Vec::new();
                for _ in 0..count {
                    let high = match bands.last() {
                        Some(last) if self.random.chance(0.125) => last[2] + self.nudge(),
                        _ => self.length(1.0, 5.0),
                    };
                    bands.push([self.length(0.5, 4.0), 0.0, high]);
                }
                Section::bands(from, &bands)
            }
            12..=14 => {
                let (low, high) = (self.low(), self.length(1.0, 5.0));
                let other = high + self.length(0.5, 3.0);
                let (first, second) = (self.length(0.5, 4.0), self.length(0.5, 4.0));
                if self.random.chance(0.5) {
                    Section::bands(
                        from,
                        &[[first, low, low + high], [second, low, low + other]],
                    )
                } else {
                    let top = low + other;
                    Section::bands(from, &[[first, low, top], [second, low + high, top]])
                }
            }
            15..=17 => {
                let count = 1 + self.random.below(3);
                let lows: Vec<f64> = (0..count).map(|_| self.coordinate(0.5, 3.0)).collect();
                let deepest = lows.iter().copied().fold(0.0, f64::max);
                let bands: Vec<[f64; 3]> = lows
                    .iter()
                    .map(|&low| [self.length(0.5, 4.0), low, deepest + self.length(0.5, 3.0)])
                    .collect();
                Section::bands(from, &bands)
            }
            _ => {
                let low = self.low();
                let (length, width) = (self.length(3.0, 8.0), self.length(3.0, 6.0));
                let band = [length, low, low + width];
                let hole = self.hole(from, band);
                Section::bands(from, &[band]).with_holes(&[hole])
            }
        };
        if self.random.chance(0.125) {
            self.off_by_a_hair(section)
        } else {
            section
        }
    }

    /// A hole strictly inside a band starting at `from`, on the lattice, and
    /// now and then a hair from the band's high edge.
    fn hole(&mut self, from: f64, [length, low, high]: [f64; 3]) -> ([f64; 2], [f64; 2]) {
        let unit = self.scale;
        let start = from + self.length(0.5, length / unit - 1.5);
        let end = start + self.length(0.5, (from + length - start) / unit - 0.5);
        let bottom = low + self.length(0.5, (high - low) / unit - 1.5);
        let mut top = bottom + self.length(0.5, (high - bottom) / unit - 0.5);
        if self.random.chance(0.2) {
            let hair = self.nudge().abs();
            if hair > 0.0 {
                top = high - hair;
            }
        }
        ([start, bottom], [end, top])
    }

    /// A section across its axis: bands each reaching both sides of it, and
    /// now and then a hole on the left.
    fn across(&mut self, from: f64) -> Section {
        let count = 1 + self.random.below(2);
        let bands: Vec<[f64; 3]> = (0..count)
            .map(|_| {
                [
                    self.length(1.0, 4.0),
                    -self.length(0.5, 4.0),
                    self.length(0.5, 4.0),
                ]
            })
            .collect();
        let section = Section::bands(from, &bands);
        let [length, _, high] = bands[0];
        if high >= 2.5 * self.scale && length >= 2.5 * self.scale && self.random.chance(0.3) {
            let hole = self.hole(from, [length, 0.0, high]);
            section.with_holes(&[hole])
        } else {
            section
        }
    }

    /// The section with its edges on the axis moved a hair off it, either
    /// side, by a share of its reach no kernel may take for more than the
    /// solver's noise.
    fn off_by_a_hair(&mut self, mut section: Section) -> Section {
        let hair = *self.random.pick(&[1e-9, 1e-7, 1e-6]) * section.reach();
        let hair = if self.random.chance(0.5) { hair } else { -hair };
        for band in &mut section.bands {
            if band[1] == 0.0 {
                band[1] = hair;
            }
        }
        section
    }

    /// The section as drawn, or on the other side of its axis a quarter of
    /// the time.
    fn aside(&mut self, section: Section) -> Section {
        if !self.random.chance(0.25) {
            return section;
        }
        Section {
            from: section.from,
            bands: section
                .bands
                .iter()
                .map(|&[length, low, high]| [length, -high, -low])
                .collect(),
            holes: section
                .holes
                .iter()
                .map(|[low, high]| [DVec2::new(low.x, -high.y), DVec2::new(high.x, -low.y)])
                .collect(),
        }
    }

    /// A plane of the origin holding a world line, and the line as an axis
    /// of it, run either way.
    fn holding(&mut self, line: Line) -> (Plane, Axis) {
        let point = line.point;
        let choices = match line.axis {
            0 => [
                (Plane::Xy(point.z), Axis::first(point.y)),
                (Plane::Xz(point.y), Axis::first(point.z)),
            ],
            1 => [
                (Plane::Xy(point.z), Axis::second(point.x)),
                (Plane::Yz(point.x), Axis::first(point.z)),
            ],
            _ => [
                (Plane::Xz(point.y), Axis::second(point.x)),
                (Plane::Yz(point.x), Axis::second(point.y)),
            ],
        };
        let (plane, axis) = *self.random.pick(&choices);
        let axis = if self.random.chance(0.25) {
            axis.backwards()
        } else {
            axis
        };
        (plane, axis)
    }

    /// A turned leaf drawn from a prism before it: about the axis of a
    /// circle, an arc or a ring of it, a wall flush with the prism's or
    /// inside it, a shoulder flush with one of its ends; or about a line
    /// square to it, on an edge of a block, tangent to one of its faces, or
    /// tangent to its wall.
    fn turned_from_prism(&mut self, before: &Leaf) -> Leaf {
        let Leaf::Prism {
            plane,
            outline,
            height,
        } = before
        else {
            return self.fresh_turned();
        };
        let (_, u, v) = plane.frame();
        let normal = plane.normal();
        let round = match outline {
            Outline::Circle { center, radius, .. } => Some((*center, *radius)),
            Outline::Ring {
                center,
                outer,
                inner,
            } => Some((*center, *self.random.pick(&[*outer, *inner]))),
            Outline::Slot { from, to, radius } => Some((*self.random.pick(&[*from, *to]), *radius)),
            Outline::Rounded { low, radius, .. } => Some((*low + *radius, *radius)),
            Outline::Rectangle { .. } | Outline::Star { .. } => None,
        };
        let ends = [
            plane.to_world(DVec2::ZERO),
            plane.to_world(DVec2::ZERO) + normal * *height,
        ];
        if let Some((center, radius)) = round.filter(|_| self.random.chance(0.6)) {
            let line = Line {
                point: plane.to_world(center),
                axis: world_axis(normal),
            };
            let (plane, axis) = self.holding(line);
            let levels = ends.map(|end| line.along(axis, end));
            let section = self.coaxial(levels, radius);
            let section = self.aside(section);
            return Leaf::Turned {
                plane,
                axis,
                section,
                degrees: self.degrees(),
            };
        }
        let (low, high) = outline_box(outline);
        let (side, across_side, running) = if self.random.chance(0.5) {
            (v, u, 1)
        } else {
            (u, v, 0)
        };
        let radius = self.length(0.5, 3.0);
        let point = match round {
            Some((center, wall)) => {
                plane.to_world(center)
                    + across_side * (wall + radius + self.nudge())
                    + normal * (height / 2.0)
            }
            None => {
                let on_top = self.random.chance(0.5);
                let edge = ends[usize::from(on_top)] + across_side * high[1 - running];
                if self.random.chance(0.5) {
                    edge
                } else {
                    let outward = normal * height.signum() * if on_top { 1.0 } else { -1.0 };
                    edge + outward * (radius + self.nudge())
                }
            }
        };
        let line = Line {
            point,
            axis: world_axis(side),
        };
        let (plane, axis) = self.holding(line);
        let read = |level: f64| if axis.backwards { -level } else { level };
        let (start, end) = (read(low[running]), read(high[running]));
        let margin = self.length(0.0, 2.0);
        let section = Section::bands(
            start.min(end) - margin,
            &[[(end - start).abs() + 2.0 * margin, 0.0, radius]],
        );
        Leaf::Turned {
            plane,
            axis,
            section,
            degrees: self.degrees(),
        }
    }

    /// A section coaxial with a wall of `radius` between the levels the
    /// prism's ends stand at along the axis: its outer edge flush with the
    /// wall, its inner edge flush with it, or inside it; or a shoulder flush
    /// with one of the ends.
    fn coaxial(&mut self, levels: [f64; 2], radius: f64) -> Section {
        let (bottom, top) = (levels[0].min(levels[1]), levels[0].max(levels[1]));
        let span = top - bottom;
        let start = bottom - self.length(0.0, 2.0);
        let length = span + self.length(0.5, 3.0);
        match self.random.below(4) {
            0 => Section::bands(
                start,
                &[[length, self.low_under(radius), radius + self.nudge()]],
            ),
            1 => Section::bands(start, &[[length, radius, radius + self.length(0.5, 2.0)]]),
            2 => {
                let under = (radius - self.length(0.5, 2.0)).max(radius / 2.0);
                Section::bands(start, &[[length, 0.0, under]])
            }
            _ => {
                let shoulder = *self.random.pick(&levels);
                let first = self.length(0.5, 3.0);
                Section::bands(
                    shoulder - first,
                    &[
                        [first, 0.0, radius + self.length(0.5, 2.0)],
                        [self.length(0.5, 3.0), 0.0, radius / 2.0],
                    ],
                )
            }
        }
    }

    /// A low edge below `radius`: on the axis, or half way to it.
    fn low_under(&mut self, radius: f64) -> f64 {
        if self.random.chance(0.5) {
            0.0
        } else {
            radius / 2.0
        }
    }

    /// A turned leaf about the very line a turn before it turned about,
    /// drawn on the same plane or on the other plane of the origin holding
    /// it: a wall of one of its radii, a sleeve on its outer wall, a groove
    /// across its outer wall, or ends on its shoulders; turned as far, the
    /// other way, or as the draw goes, from its side of the axis or the
    /// other.
    fn turned_along(&mut self, line: Line, before: Axis, section: &Section, degrees: f64) -> Leaf {
        let (plane, axis) = self.holding(line);
        let ends: Vec<f64> = section
            .ends()
            .iter()
            .map(|&end| line.along(axis, line.at(before, end)))
            .collect();
        let radii = radii_of(section, self.scale);
        let outer = radii.iter().copied().fold(0.0, f64::max);
        let (bottom, top) = (
            ends.iter().copied().fold(f64::MAX, f64::min),
            ends.iter().copied().fold(f64::MIN, f64::max),
        );
        let section = match self.random.below(4) {
            0 => {
                let radius = *self.random.pick(&radii);
                Section::bands(
                    bottom - self.length(0.0, 1.0),
                    &[[top - bottom + self.length(0.5, 2.0), 0.0, radius]],
                )
            }
            1 => Section::bands(
                bottom,
                &[[top - bottom, outer, outer + self.length(0.5, 2.0)]],
            ),
            2 => {
                let start = bottom + self.length(0.0, (top - bottom) / self.scale / 2.0);
                let room = top - start;
                Section::bands(
                    start,
                    &[[
                        (room / 2.0).max(0.5 * self.scale),
                        outer - self.length(0.5, 1.0).min(outer / 2.0),
                        outer + self.length(0.5, 2.0),
                    ]],
                )
            }
            _ => {
                let first = *self.random.pick(&ends);
                let last = *self.random.pick(&ends);
                let (start, end) = (first.min(last), first.max(last));
                let length = if end > start { end - start } else { self.scale };
                Section::bands(start, &[[length, 0.0, outer + self.length(0.5, 2.0)]])
            }
        };
        let section = self.aside(section);
        let degrees = match self.random.below(3) {
            0 => degrees,
            1 => -degrees,
            _ => self.degrees(),
        };
        Leaf::Turned {
            plane,
            axis,
            section,
            degrees,
        }
    }

    /// A prism drawn from a turn before it: on the plane square to its axis
    /// at one of its shoulders, a circle or a ring of one of its radii, or a
    /// block whose side is tangent to its outer wall; or a hole across it, a
    /// circle square to its axis with its centre on it.
    fn prism_from_turned(&mut self, line: Line, before: Axis, section: &Section) -> Leaf {
        let radii = radii_of(section, self.scale);
        let radius = *self.random.pick(&radii);
        let outer = radii.iter().copied().fold(0.0, f64::max);
        let shoulder = line.at(before, *self.random.pick(&section.ends()));
        if self.random.chance(0.3) {
            let (plane, axis) = self.holding(line);
            let ends = section.ends();
            let middle = (ends[0] + ends[ends.len() - 1]) / 2.0;
            let center = axis.at(DVec2::new(line.along(axis, line.at(before, middle)), 0.0));
            let depth = outer + self.length(1.0, 2.0);
            let radius = if self.random.chance(0.4) {
                outer
            } else {
                self.length(0.5, 2.0)
            };
            return Leaf::Prism {
                plane: shifted(plane, -depth),
                outline: Outline::Circle {
                    center,
                    radius,
                    from: 0.0,
                },
                height: 2.0 * depth,
            };
        }
        let (plane, center) = square_to(line, shoulder);
        let outline = match self.random.below(3) {
            0 => Outline::Circle {
                center,
                radius,
                from: 0.0,
            },
            1 => Outline::Ring {
                center,
                outer: radius + self.length(0.5, 2.0),
                inner: radius,
            },
            _ => {
                let width = self.length(0.5, 2.0);
                Outline::Rectangle {
                    low: center + DVec2::new(outer + self.nudge(), -width),
                    high: center + DVec2::new(outer + self.length(0.5, 2.0), width),
                }
            }
        };
        Leaf::Prism {
            plane,
            outline,
            height: self.height(),
        }
    }
}

impl Line {
    /// The line a leaf on `plane` turns about, along a world axis.
    fn of(plane: Plane, axis: Axis) -> Line {
        let line = axis.line();
        let point = plane.to_world(line.origin);
        let direction = plane.to_world(line.origin + line.direction) - point;
        Line {
            point,
            axis: world_axis(direction),
        }
    }

    /// The point `along` along the line read as `axis` reads it.
    fn at(&self, axis: Axis, along: f64) -> DVec3 {
        let mut point = self.point;
        point[self.axis] = if axis.backwards { -along } else { along };
        point
    }

    /// How far along the line, read as `axis` reads it, `point` stands.
    fn along(&self, axis: Axis, point: DVec3) -> f64 {
        if axis.backwards {
            -point[self.axis]
        } else {
            point[self.axis]
        }
    }
}

/// The radii a section's edges stand at, but for the hairs a section a hair
/// off its axis has: a tool drawn to one would be a sliver.
fn radii_of(section: &Section, unit: f64) -> Vec<f64> {
    section
        .bands
        .iter()
        .flat_map(|band| [band[1].abs(), band[2].abs()])
        .filter(|radius| *radius > unit / 4.0)
        .collect()
}

/// Which world axis a direction runs along.
fn world_axis(direction: DVec3) -> usize {
    direction.abs().max_position()
}

/// The plane of the origin square to a line through `point`, and where the
/// line pierces it in the plane's own coordinates.
fn square_to(line: Line, point: DVec3) -> (Plane, DVec2) {
    match line.axis {
        0 => (Plane::Yz(point.x), DVec2::new(point.y, point.z)),
        1 => (Plane::Xz(point.y), DVec2::new(point.x, point.z)),
        _ => (Plane::Xy(point.z), DVec2::new(point.x, point.y)),
    }
}

fn outline_box(outline: &Outline) -> (DVec2, DVec2) {
    match outline {
        Outline::Rectangle { low, high } | Outline::Rounded { low, high, .. } => (*low, *high),
        Outline::Circle { center, radius, .. }
        | Outline::Ring {
            center,
            outer: radius,
            ..
        } => (*center - *radius, *center + *radius),
        Outline::Slot { from, to, radius } => (from.min(*to) - *radius, from.max(*to) + *radius),
        Outline::Star { corners, .. } => corners.iter().fold(
            (DVec2::splat(f64::MAX), DVec2::splat(f64::MIN)),
            |(low, high), corner| (low.min(*corner), high.max(*corner)),
        ),
    }
}
