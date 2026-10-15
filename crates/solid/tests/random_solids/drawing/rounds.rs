//! Arcs drawn into turned sections (#576), and the tools drawn from an arc
//! or onto one: what a turn makes a sphere or a torus of.
//!
//! Three fresh sections in four take an arc, weighted towards what parts
//! are made of — a ball, a zone or a cap of one, a ring torus and arcs of
//! one, an apple of a spindle torus, a shaft with a rounded end, a
//! round-bottomed groove, a rounded shoulder, a fillet, a barrel bore and a
//! round groove in a bore — and towards the hairs a kernel breaks on: a
//! ball's centre a hair off the axis, an arc a hair from tangent to the wall
//! it continues, a sphere a hair larger than the shaft it ends, a cap whose
//! centre stands a hair off the axis, and in a campaign an arc a hair short
//! of a quarter circle. Every number of an arc comes from the lattice: its
//! centre on a coordinate, its radius on a length, its corners on the
//! lattice points of its circle — the ends of its diameters, and the points
//! a Pythagorean triple puts there. A shape that leaves the section no solid
//! is drawn again, up to three times. Tools drawn from an arc take its
//! numbers: a coaxial cylinder at its equator's radius or a hair off it, a
//! shaft ending square through its centre, at its pole or a hair from
//! either, the same arc again or a hair along the axis, the whole ball or
//! torus it lies on or a hair larger, a cone tangent at a lattice latitude,
//! a sleeve at its equator, a block through the axis, a circle or a ring
//! square to the axis at the centre or the pole; and a ball or a dome on the
//! rim of a circle raised before.

use cao_solid::soundness::in_steps;
use glam::DVec2;

use super::super::{Axis, Bow, Leaf, Outline, Section};
use super::slants::{Band, banded, is_level};
use super::turns::{Line, square_to, world_axis};
use super::{Among, Drawing};

/// The lattice points of a circle of radius five halves, as offsets from
/// its centre in units of the lattice's step, the upper half from left to
/// right: the ends of its diameter, and the points of the triple three,
/// four, five. Scaled by a whole number they are the points of a circle
/// that much larger.
const ON_THE_CIRCLE: [[f64; 2]; 7] = [
    [-2.5, 0.0],
    [-2.0, 1.5],
    [-1.5, 2.0],
    [0.0, 2.5],
    [1.5, 2.0],
    [2.0, 1.5],
    [2.5, 0.0],
];

/// The angles a campaign draws an arc short of a quarter circle by, in
/// radians: the arc then ends a hair off the axis rather than on it.
const SHORT_OF_A_QUARTER: [f64; 2] = [1e-7, 1e-5];

/// The section's bands as `bands` from `from`, each edge bowed as `bows`
/// says, its holes kept.
fn rebuilt(section: &Section, from: f64, bands: &[Band], bows: &[[[f64; 3]; 2]]) -> Section {
    let starts: Vec<[f64; 3]> = bands
        .iter()
        .map(|band| [band[0], band[1], band[2]])
        .collect();
    let ends: Vec<[f64; 2]> = bands.iter().map(|band| [band[3], band[4]]).collect();
    Section {
        holes: section.holes.clone(),
        ..Section::bands(from, &starts).sloping_to(&ends).bowed(bows)
    }
}

/// The bows of a section, one entry per band whether it bows or not.
fn bows_of(section: &Section) -> Vec<[[f64; 3]; 2]> {
    (0..section.bands.len())
        .map(|index| section.bowed.get(index).copied().unwrap_or([[0.0; 3]; 2]))
        .collect()
}

fn bow(center: DVec2, radius: f64) -> [f64; 3] {
    [center.x, center.y, radius]
}

const STRAIGHT: [[f64; 3]; 2] = [[0.0; 3]; 2];

impl Drawing {
    /// Whether the edges of a turned section may bow (#576). Every number
    /// the arcs take is drawn only then, which keeps every case the draws
    /// before them gave.
    pub(super) fn bowing(&self) -> bool {
        matches!(self.among, Among::Turned { rounded: true, .. })
    }

    /// A fresh section given an arc three times in four: the shape drawn
    /// again, up to three times, while it leaves the section no solid.
    pub(super) fn rounded_section(&mut self, section: Section) -> Section {
        if !self.random.chance(0.75) {
            return section;
        }
        for _ in 0..3 {
            let rounded = match self.random.below(20) {
                0..=2 => Some(self.ball()),
                3 | 4 => Some(self.zone()),
                5 | 6 => Some(self.ring_torus()),
                7 => Some(self.apple()),
                8..=10 => self.rounded_end(&section, 0.0, 0.0),
                11 | 12 => self.round_bottomed_groove(&section),
                13 | 14 => self.rounded_shoulder(&section),
                15 => self.rounded_bore(&section),
                16 => {
                    let larger = self.hair();
                    self.rounded_end(&section, larger, 0.0)
                }
                17 => {
                    let off_tangent = self.hair();
                    self.rounded_end(&section, 0.0, off_tangent)
                }
                18 => self.cap_a_hair_off_the_axis(&section),
                _ if self.off_the_lattice() => self.short_of_a_quarter(&section),
                _ => {
                    let larger = self.hair();
                    self.rounded_end(&section, larger, 0.0)
                }
            };
            if let Some(rounded) = rounded.filter(Section::is_solid) {
                return rounded;
            }
        }
        section
    }

    /// A hair, or nothing half the time: a hair a tool is given on purpose
    /// beside a tangency, which `nudge` would leave out more often than not.
    fn hair_or_nothing(&mut self) -> f64 {
        if self.random.chance(0.5) {
            0.0
        } else {
            self.hair()
        }
    }

    /// A radius on the lattice, now and then a multiple of five halves,
    /// which has lattice points off its diameters.
    fn arc_radius(&mut self) -> f64 {
        if self.random.chance(0.4) {
            2.5 * self.scale * *self.random.pick(&[1.0, 1.0, 2.0])
        } else {
            self.length(0.5, 4.0)
        }
    }

    /// The radius as a multiple of five halves of the lattice's step, when
    /// it is one: its circle then has lattice points off its diameters.
    fn on_a_triple(&self, radius: f64) -> Option<f64> {
        let halves = in_steps(radius / self.scale, 0.5);
        (halves % 5.0 == 0.0).then_some(radius / 2.5)
    }

    /// Two lattice points of the circle of radius five halves of `k` about
    /// `center`, the first on the left.
    fn lattice_chord(&mut self, center: DVec2, k: f64) -> [DVec2; 2] {
        let from = self.random.below(ON_THE_CIRCLE.len() - 1);
        let to = from + 1 + self.random.below(ON_THE_CIRCLE.len() - from - 1);
        [from, to].map(|index| center + DVec2::from(ON_THE_CIRCLE[index]) * k)
    }

    /// A ball: a half circle whose both ends stand on the axis, now and
    /// then with its centre a hair off it, either side.
    fn ball(&mut self) -> Section {
        let radius = self.arc_radius();
        let across = if self.random.chance(0.3) {
            self.hair()
        } else {
            0.0
        };
        let center = DVec2::new(self.coordinate(-3.0, 5.0), across);
        Section::bands(center.x - radius, &[[2.0 * radius, across, across]])
            .bowed(&[[[0.0; 3], bow(center, radius)]])
    }

    /// A zone of a sphere between two lattice points of a circle of radius
    /// five halves, or a cap of one reaching the axis at one end, its low
    /// edge on the axis.
    fn zone(&mut self) -> Section {
        let k = self.scale * *self.random.pick(&[1.0, 1.0, 2.0]);
        let center = DVec2::new(self.coordinate(-3.0, 5.0), 0.0);
        let [start, end] = self.lattice_chord(center, k);
        Section::bands(start.x, &[[end.x - start.x, 0.0, start.y]])
            .sloping_to(&[[0.0, end.y]])
            .bowed(&[[[0.0; 3], bow(center, 2.5 * k)]])
    }

    /// A ring torus, a whole circle off the axis, or an arc of one: the
    /// upper arc between two lattice points of the circle raised on a
    /// cylinder, or the lower one bored through a tube.
    fn ring_torus(&mut self) -> Section {
        let radius = self.arc_radius();
        let center = DVec2::new(self.coordinate(-3.0, 5.0), radius + self.length(0.5, 3.0));
        let whole = Section::bands(center.x - radius, &[[2.0 * radius, center.y, center.y]])
            .bowed(&[[bow(center, -radius), bow(center, radius)]]);
        let Some(k) = self.on_a_triple(radius) else {
            return whole;
        };
        if self.random.chance(0.5) {
            return whole;
        }
        let [start, end] = self.lattice_chord(center, k);
        if self.random.chance(0.5) {
            let low = self.low_below(start.y.min(end.y));
            Section::bands(start.x, &[[end.x - start.x, low, start.y]])
                .sloping_to(&[[low, end.y]])
                .bowed(&[[[0.0; 3], bow(center, radius)]])
        } else {
            let high = center.y + radius + self.length(0.5, 2.0);
            Section::bands(
                start.x,
                &[[end.x - start.x, 2.0 * center.y - start.y, high]],
            )
            .sloping_to(&[[2.0 * center.y - end.y, high]])
            .bowed(&[[bow(center, -radius), [0.0; 3]]])
        }
    }

    /// A low edge below `radius`: on the axis, or on the lattice under it.
    fn low_below(&mut self, radius: f64) -> f64 {
        if self.random.chance(0.5) {
            0.0
        } else {
            self.random
                .on_lattice(0.0, (radius - 0.5 * self.scale) / self.scale, 0.5)
                * self.scale
        }
    }

    /// An apple: the outer lobe of a spindle torus, an arc reaching the
    /// axis at both ends from a centre across it.
    fn apple(&mut self) -> Section {
        let k = self.scale * *self.random.pick(&[1.0, 1.0, 2.0]);
        let (across, half) = *self.random.pick(&[(-1.5, 2.0), (-2.0, 1.5)]);
        let center = DVec2::new(self.coordinate(-3.0, 5.0), across * k);
        Section::bands(center.x - half * k, &[[2.0 * half * k, 0.0, 0.0]])
            .bowed(&[[[0.0; 3], bow(center, 2.5 * k)]])
    }

    /// An end band on the axis given a rounded end: a quarter circle from
    /// its wall to the axis, tangent to both; of a radius `larger` than the
    /// wall's by a hair, which steps there; or meeting a wall sloping by
    /// `off_tangent` over its length, a hair from tangent.
    fn rounded_end(&mut self, section: &Section, larger: f64, off_tangent: f64) -> Option<Section> {
        let mut bands = banded(section);
        let mut bows = bows_of(section);
        let last = self.random.chance(0.5);
        let index = if last { bands.len() - 1 } else { 0 };
        let [length, low, high, ..] = bands[index];
        if !is_level(&bands[index]) || low != 0.0 || bows[index] != STRAIGHT {
            return None;
        }
        let radius = high + larger;
        if length <= radius + 0.5 * self.scale {
            return None;
        }
        bands[index][0] -= radius;
        let ends = section.ends();
        if last {
            let at = ends[index + 1] - radius;
            bands[index][2] = high + off_tangent;
            bands.push([radius, 0.0, radius, 0.0, 0.0]);
            bows.push([[0.0; 3], bow(DVec2::new(at, 0.0), radius)]);
        } else {
            let at = ends[index] + radius;
            bands[index][4] = high + off_tangent;
            bands.insert(0, [radius, 0.0, 0.0, 0.0, radius]);
            bows.insert(0, [[0.0; 3], bow(DVec2::new(at, 0.0), radius)]);
        }
        Some(rebuilt(section, section.from, &bands, &bows))
    }

    /// A shaft's last band ended by a quarter circle whose centre stands a
    /// hair above the axis: the arc ends a hair above it too, where the
    /// kernel decides whether a pole stands.
    fn cap_a_hair_off_the_axis(&mut self, section: &Section) -> Option<Section> {
        let mut bands = banded(section);
        let mut bows = bows_of(section);
        let index = bands.len() - 1;
        let [length, low, high, ..] = bands[index];
        if !is_level(&bands[index]) || low != 0.0 || bows[index] != STRAIGHT {
            return None;
        }
        let hair = self.hair().abs();
        let radius = high - hair;
        if length <= radius + 0.5 * self.scale || radius <= 0.5 * self.scale {
            return None;
        }
        let at = section.ends()[index + 1] - radius;
        bands[index][0] -= radius;
        bands.push([radius, 0.0, high, 0.0, hair]);
        bows.push([[0.0; 3], bow(DVec2::new(at, hair), radius)]);
        Some(rebuilt(section, section.from, &bands, &bows))
    }

    /// A shaft's last band ended by an arc a hair short of a quarter
    /// circle: it ends a hair off the axis, where the kernel decides
    /// whether a pole stands.
    fn short_of_a_quarter(&mut self, section: &Section) -> Option<Section> {
        let mut bands = banded(section);
        let mut bows = bows_of(section);
        let index = bands.len() - 1;
        let [length, low, high, ..] = bands[index];
        if !is_level(&bands[index]) || low != 0.0 || bows[index] != STRAIGHT {
            return None;
        }
        let radius = high;
        if length <= radius + 0.5 * self.scale {
            return None;
        }
        let short = *self.random.pick(&SHORT_OF_A_QUARTER);
        let (sine, cosine) = short.sin_cos();
        let at = section.ends()[index + 1] - radius;
        bands[index][0] -= radius;
        bands.push([radius * cosine, 0.0, high, 0.0, radius * sine]);
        bows.push([[0.0; 3], bow(DVec2::new(at, 0.0), radius)]);
        Some(rebuilt(section, section.from, &bands, &bows))
    }

    /// A level band's high edge given a round-bottomed groove: a half
    /// circle cut into it, straight down from the edge or below walls of
    /// some depth, its ends tangent to them.
    fn round_bottomed_groove(&mut self, section: &Section) -> Option<Section> {
        let mut bands = banded(section);
        let mut bows = bows_of(section);
        let level: Vec<usize> = (0..bands.len())
            .filter(|&index| {
                is_level(&bands[index])
                    && bows[index] == STRAIGHT
                    && bands[index][0] >= 2.0 * self.scale
                    && bands[index][2] - bands[index][1] >= 1.5 * self.scale
            })
            .collect();
        if level.is_empty() {
            return None;
        }
        let index = *self.random.pick(&level);
        let [length, low, high, ..] = bands[index];
        let unit = self.scale;
        let radius = self.up_to(((length - unit) / 2.0).min(high - low - unit))?;
        let depth = if self.random.chance(0.5) {
            0.0
        } else {
            self.up_to(high - low - radius - 0.5 * unit).unwrap_or(0.0)
        };
        let before = self.up_to(length - 2.0 * radius - 0.5 * unit)?;
        let after = length - 2.0 * radius - before;
        let start = section.ends()[index] + before;
        let bottom = high - depth;
        bands[index] = [before, low, high, low, high];
        bands.insert(index + 1, [2.0 * radius, low, bottom, low, bottom]);
        bands.insert(index + 2, [after, low, high, low, high]);
        bows.insert(
            index + 1,
            [[0.0; 3], bow(DVec2::new(start + radius, bottom), -radius)],
        );
        bows.insert(index + 2, STRAIGHT);
        Some(rebuilt(section, section.from, &bands, &bows))
    }

    /// A step of the high edge between two level bands rounded: its convex
    /// corner by a quarter circle tangent to the higher wall, or its
    /// concave one filleted tangent to the lower, either spanning the whole
    /// step and so tangent to both walls, or less on the lattice.
    fn rounded_shoulder(&mut self, section: &Section) -> Option<Section> {
        let mut bands = banded(section);
        let mut bows = bows_of(section);
        let steps: Vec<usize> = (0..bands.len().saturating_sub(1))
            .filter(|&index| {
                is_level(&bands[index])
                    && is_level(&bands[index + 1])
                    && bows[index] == STRAIGHT
                    && bows[index + 1] == STRAIGHT
                    && bands[index][1] == bands[index + 1][1]
                    && bands[index][2] != bands[index + 1][2]
            })
            .collect();
        if steps.is_empty() {
            return None;
        }
        let at = *self.random.pick(&steps);
        let (high, next) = (bands[at][2], bands[at + 1][2]);
        let step = (next - high).abs();
        let convex = self.random.chance(0.6);
        let carved = if (next > high) == convex { at + 1 } else { at };
        let [length, low, top, ..] = bands[carved];
        let radius = if self.random.chance(0.4) {
            step
        } else {
            self.up_to(step)?
        };
        if radius >= length {
            return None;
        }
        let edge = section.ends()[at + 1];
        let out = if convex { 1.0 } else { -1.0 };
        let by_the_wall = top - out * radius;
        bands[carved][0] -= radius;
        if carved == at {
            let center = DVec2::new(edge - radius, by_the_wall);
            bands.insert(at + 1, [radius, low, top, low, by_the_wall]);
            bows.insert(at + 1, [[0.0; 3], bow(center, out * radius)]);
        } else {
            let center = DVec2::new(edge + radius, by_the_wall);
            bands.insert(at + 1, [radius, low, by_the_wall, low, top]);
            bows.insert(at + 1, [[0.0; 3], bow(center, out * radius)]);
        }
        Some(rebuilt(section, section.from, &bands, &bows))
    }

    /// A tube's low edge bowed between two lattice points of a circle of
    /// radius five halves: towards the axis, a barrel, or into the matter,
    /// a round groove in the bore, whose circle may reach the axis or
    /// cross it.
    fn rounded_bore(&mut self, section: &Section) -> Option<Section> {
        let mut bands = banded(section);
        let mut bows = bows_of(section);
        let tubes: Vec<usize> = (0..bands.len())
            .filter(|&index| {
                is_level(&bands[index])
                    && bows[index] == STRAIGHT
                    && bands[index][1] > 0.0
                    && bands[index][0] >= 4.0 * self.scale
            })
            .collect();
        if tubes.is_empty() {
            return None;
        }
        let index = *self.random.pick(&tubes);
        let [length, low, high, ..] = bands[index];
        let unit = self.scale;
        let (half, rise) = *self.random.pick(&[(1.5, 2.0), (2.0, 1.5)]);
        let (half, rise, radius) = (half * unit, rise * unit, 2.5 * unit);
        let inwards = self.random.chance(0.5);
        let before = self.up_to(length - 2.0 * half - 0.5 * unit)?;
        let after = length - 2.0 * half - before;
        let start = section.ends()[index] + before;
        let center = DVec2::new(start + half, if inwards { low + rise } else { low - rise });
        bands[index] = [before, low, high, low, high];
        bands.insert(index + 1, [2.0 * half, low, high, low, high]);
        bands.insert(index + 2, [after, low, high, low, high]);
        let out = if inwards { -radius } else { radius };
        bows.insert(index + 1, [bow(center, out), [0.0; 3]]);
        bows.insert(index + 2, STRAIGHT);
        Some(rebuilt(section, section.from, &bands, &bows))
    }

    /// A rounded turn drawn from a circle raised before it, three times in
    /// ten: a ball centred on its end whose equator the circle is tangent
    /// to, or larger, or a hair larger; or a dome on its end, a quarter
    /// circle from its rim to the axis. Nothing from any other outline.
    pub(super) fn rounded_from_prism(&mut self, before: &Leaf) -> Option<Leaf> {
        let Leaf::Prism {
            plane,
            outline,
            height,
        } = before
        else {
            return None;
        };
        let (center, radius) = match outline {
            Outline::Circle { center, radius, .. }
            | Outline::Ring {
                center,
                outer: radius,
                ..
            }
            | Outline::Slot {
                from: center,
                radius,
                ..
            } => (*center, *radius),
            Outline::Rounded { low, radius, .. } => (*low + *radius, *radius),
            Outline::Rectangle { .. }
            | Outline::Star { .. }
            | Outline::Ellipse { .. }
            | Outline::Chorded { .. }
            | Outline::OvalSlot { .. }
            | Outline::OvalRing { .. } => return None,
        };
        if !self.random.chance(0.3) {
            return None;
        }
        let normal = plane.normal();
        let floor = plane.to_world(DVec2::ZERO);
        let line = Line {
            point: plane.to_world(center),
            axis: world_axis(normal),
        };
        let (plane, mut axis, _, cap) = self.upright(line, [floor, floor + normal * *height]);
        if self.off_the_lattice() && self.random.chance(0.15) {
            axis.across += self.hair();
        }
        let ball = |radius: f64| {
            Section::bands(cap - radius, &[[2.0 * radius, 0.0, 0.0]])
                .bowed(&[[[0.0; 3], bow(DVec2::new(cap, 0.0), radius)]])
        };
        let section = match self.random.below(4) {
            0 => {
                let larger = self.length(0.5, 2.0);
                let hair = self.hair_or_nothing();
                ball(radius + larger + hair)
            }
            1 => Section::bands(cap, &[[radius, 0.0, radius]])
                .sloping_to(&[[0.0, 0.0]])
                .bowed(&[[[0.0; 3], bow(DVec2::new(cap, 0.0), radius)]]),
            _ => {
                let hair = self.hair_or_nothing();
                ball(radius + hair)
            }
        };
        let leaf = Leaf::Turned {
            plane,
            axis,
            section: self.aside(section),
            degrees: self.degrees(),
        };
        Some(self.or_fresh(leaf))
    }

    /// The arcs of a section read along the line its leaf turns about as
    /// `axis` reads it: each as its corners, along the axis and away from
    /// it, the left one first, and its arc, the radius positive where it
    /// bulges away from the axis. An arc a hair wide is left out: a tool
    /// drawn to it would be a sliver.
    fn arcs_along(
        &self,
        line: Line,
        before: Axis,
        axis: Axis,
        section: &Section,
    ) -> Vec<([DVec2; 2], Bow)> {
        let level = |at: f64| line.along(axis, line.at(before, at));
        section
            .pieces()
            .iter()
            .flat_map(|piece| {
                [0, 1].into_iter().filter_map(move |edge| {
                    let bow = piece.bows[edge]?;
                    let chord = piece.chord(edge);
                    let read = |corner: DVec2| DVec2::new(level(corner.x), corner.y);
                    let (one, other) = (read(chord[0]), read(chord[1]));
                    let corners = if one.x <= other.x {
                        [one, other]
                    } else {
                        [other, one]
                    };
                    Some((
                        corners,
                        Bow {
                            center: read(bow.center),
                            radius: bow.radius,
                        },
                    ))
                })
            })
            .filter(|(_, bow)| bow.radius.abs() > self.scale / 4.0)
            .collect()
    }

    /// A turn about the line of a rounded turn before it, drawn from one of
    /// its arcs read along the line: a coaxial cylinder at the equator's
    /// radius, or a hair off it; the same arc again, or a hair along the
    /// axis; the whole ball or torus the arc lies on, or a hair larger; a
    /// cone tangent to the arc's circle at a lattice point of it; a coaxial
    /// shaft ending square through the centre, at a pole, or a hair from
    /// either; or a sleeve at the equator. In a campaign now and then a hair
    /// off its axis. With no arc to draw from, a fresh turn.
    pub(super) fn along_an_arc(
        &mut self,
        line: Line,
        before: Axis,
        section: &Section,
        degrees: f64,
    ) -> Leaf {
        let (plane, mut axis) = self.holding(line);
        let arcs = self.arcs_along(line, before, axis, section);
        if arcs.is_empty() {
            return self.fresh_turned();
        }
        let (corners, arc) = *self.random.pick(&arcs);
        if self.off_the_lattice() && self.random.chance(0.15) {
            axis.across += self.hair();
        }
        let (center, radius) = (arc.center, arc.radius.abs());
        let equator = arc.extreme(corners).y.abs();
        let unit = self.scale;
        let kind = self.random.below(12);
        let section = if kind == 7
            && let Some(k) = self.on_a_triple(radius)
        {
            let touch = center + DVec2::new(1.5, 2.0) * k;
            Section::bands(touch.x - 2.0 * k, &[[4.0 * k, 0.0, touch.y + 1.5 * k]])
                .sloping_to(&[[0.0, touch.y - 1.5 * k]])
        } else {
            match kind {
                0..=2 => {
                    let margin = self.length(0.0, 2.0);
                    let hair = self.hair_or_nothing();
                    Section::bands(
                        center.x - radius - margin,
                        &[[2.0 * (radius + margin), 0.0, equator + hair]],
                    )
                }
                3 | 4 => {
                    let shift = self.hair_or_nothing();
                    let [start, end] = corners.map(|corner| corner + DVec2::X * shift);
                    let low = self.low_below(start.y.min(end.y).max(0.5 * unit));
                    Section::bands(start.x, &[[end.x - start.x, low, start.y.abs()]])
                        .sloping_to(&[[low, end.y.abs()]])
                        .bowed(&[[[0.0; 3], bow(center + DVec2::X * shift, radius)]])
                }
                5 | 6 => {
                    let radius = radius + self.hair_or_nothing();
                    if center.y.abs() <= unit / 4.0 {
                        Section::bands(center.x - radius, &[[2.0 * radius, 0.0, 0.0]])
                            .bowed(&[[[0.0; 3], bow(center.with_y(0.0), radius)]])
                    } else {
                        Section::bands(center.x - radius, &[[2.0 * radius, center.y, center.y]])
                            .bowed(&[[bow(center, -radius), bow(center, radius)]])
                    }
                }
                7 | 8 => {
                    let hair = self.hair();
                    let level = *self.random.pick(&[
                        center.x,
                        center.x + radius,
                        center.x - radius,
                        center.x + hair,
                        center.x + radius + hair,
                    ]);
                    let length = self.length(0.5, 3.0);
                    let from = if self.random.chance(0.5) {
                        level - length
                    } else {
                        level
                    };
                    let wall = if self.random.chance(0.5) {
                        equator
                    } else {
                        self.length(0.5, 3.0)
                    };
                    Section::bands(from, &[[length, 0.0, wall]])
                }
                _ => {
                    let sleeve = self.length(0.5, 2.0);
                    Section::bands(
                        center.x - radius,
                        &[[2.0 * radius, equator, equator + sleeve]],
                    )
                }
            }
        };
        let leaf = Leaf::Turned {
            plane,
            axis,
            section: self.aside(section),
            degrees: self.turned_again(degrees),
        };
        self.or_fresh(leaf)
    }

    /// A prism drawn from a rounded turn before it: a block whose floor
    /// holds the axis, through an arc; or a circle or a ring on the plane
    /// square to the axis through the arc's centre, at a pole or a hair
    /// from either, of the equator's radius or the lattice's. With no arc to
    /// draw from, a fresh prism.
    pub(super) fn prism_at_an_arc(&mut self, line: Line, before: Axis, section: &Section) -> Leaf {
        let (plane, axis) = self.holding(line);
        let arcs = self.arcs_along(line, before, axis, section);
        if arcs.is_empty() {
            return Leaf::Prism {
                plane: self.plane(),
                outline: self.outline(),
                height: self.height(),
            };
        }
        let (corners, arc) = *self.random.pick(&arcs);
        let (center, radius) = (arc.center, arc.radius.abs());
        let equator = arc.extreme(corners).y.abs();
        if self.random.chance(0.25) {
            let half = self.length(0.5, 3.0);
            let (start, end) = (
                axis.at(DVec2::new(center.x - radius, -half)),
                axis.at(DVec2::new(center.x + radius, half)),
            );
            return Leaf::Prism {
                plane,
                outline: Outline::Rectangle {
                    low: start.min(end),
                    high: start.max(end),
                },
                height: self.height(),
            };
        }
        let hair = self.hair();
        let level = *self.random.pick(&[
            center.x,
            center.x + radius,
            center.x - radius,
            center.x + hair,
            center.x + radius + hair,
        ]);
        let (plane, middle) = square_to(line, line.at(axis, level));
        let radius = if self.random.chance(0.5) {
            equator
        } else {
            self.length(0.5, 3.0)
        };
        let outline = match self.random.below(3) {
            0 | 1 => Outline::Circle {
                center: middle,
                radius,
                from: 0.0,
            },
            _ => Outline::Ring {
                center: middle,
                outer: radius + self.length(0.5, 2.0),
                inner: radius,
            },
        };
        Leaf::Prism {
            plane,
            outline,
            height: self.height(),
        }
    }
}
