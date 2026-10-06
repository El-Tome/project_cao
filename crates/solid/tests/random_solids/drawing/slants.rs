//! Slanted runs drawn into turned sections (#536), and the tools drawn from
//! a slant or onto one: what a turn makes a cone of.
//!
//! Three fresh sections in four take one or two slants, weighted towards
//! what parts are made of — chamfers where a shaft steps, points, chamfered
//! ends and bores, countersinks, tapers, ridges — and towards the shapes a
//! kernel breaks on: a chamfer whose rim lands on the next wall, two slants
//! on one line, a slope a hair from parallel or from square. A slant that
//! leaves the section no solid is dropped, and so is one that comes within
//! half a unit of a hole: a hole a hair from a slant is a profile the reading
//! hands to the flats by design. Tools drawn from what came before take a
//! slant of their own: a countersunk hole in a block, a chamfer cut round a
//! raised circle, a cone standing on it, and after a slant the same cone
//! again, a cylinder at its rim, a cone crossing it at its rim, a slant a
//! hair off it, a circle on its rim.

use cao_solid::soundness::NEAR;
use glam::{DVec2, DVec3};

use super::super::{Axis, Leaf, Outline, Plane, Section};
use super::Drawing;
use super::turns::{Line, square_to, world_axis};

/// A band with its edges at both of its ends: its length, its low and high
/// edge where it starts, and its low and high edge where it ends.
type Band = [f64; 5];

fn banded(section: &Section) -> Vec<Band> {
    (0..section.bands.len())
        .map(|index| {
            let ([low, high], [low_end, high_end]) = section.edges(index);
            [section.bands[index][0], low, high, low_end, high_end]
        })
        .collect()
}

fn is_level(band: &Band) -> bool {
    band[1] == band[3] && band[2] == band[4]
}

/// The section's bands as `bands` from `from`, its holes kept.
fn rebuilt(section: &Section, from: f64, bands: &[Band]) -> Section {
    let starts: Vec<[f64; 3]> = bands
        .iter()
        .map(|band| [band[0], band[1], band[2]])
        .collect();
    let ends: Vec<[f64; 2]> = bands.iter().map(|band| [band[3], band[4]]).collect();
    Section {
        holes: section.holes.clone(),
        ..Section::bands(from, &starts).sloping_to(&ends)
    }
}

impl Drawing {
    /// A fresh section slanted three times in four, once or twice: each
    /// slant drawn again, up to three times, while it leaves the section
    /// no solid.
    pub(super) fn slanted_section(&mut self, section: Section) -> Section {
        if !self.random.chance(0.75) {
            return section;
        }
        let mut section = section;
        for _ in 0..1 + self.random.below(2) {
            for _ in 0..3 {
                let slanted = if section.side().is_some() {
                    self.slant(&section)
                } else {
                    self.slant_across(&section)
                };
                if let Some(slanted) = slanted.filter(|slanted| self.keeps_its_holes_clear(slanted))
                {
                    section = slanted;
                    break;
                }
            }
        }
        section
    }

    /// One slant of a section on one side of its axis, by the weights the
    /// harness's design gives them.
    fn slant(&mut self, section: &Section) -> Option<Section> {
        match self.random.below(20) {
            0..=5 => self.chamfer_at_a_step(section),
            6..=8 => self.pointed(section),
            9 | 10 => self.chamfered_end(section),
            11 | 12 => self.chamfered_bore(section),
            13 | 14 => self.countersink(section),
            15 => self.taper(section),
            16 => self.two_slants_on_one_line(section),
            17 => self.a_hair_from_parallel(section),
            18 => self.a_hair_from_square(section),
            _ => self.ridge(section),
        }
    }

    /// Whether the section is solid and no hole of it comes within half a
    /// unit of a slanted run.
    fn keeps_its_holes_clear(&self, section: &Section) -> bool {
        let half = 0.5 * self.scale;
        let (outline, holes) = section.corners();
        section.is_solid()
            && holes.iter().flatten().all(|corner| {
                (0..outline.len()).all(|index| {
                    let (from, to) = (outline[index], outline[(index + 1) % outline.len()]);
                    let run = to - from;
                    let share = ((*corner - from).dot(run) / run.length_squared()).clamp(0.0, 1.0);
                    run.x == 0.0 || run.y == 0.0 || corner.distance(from + run * share) >= half
                })
            })
    }

    /// A hair a slant is drawn with, never nought: the hairs `nudge` draws.
    fn hair(&mut self) -> f64 {
        let reach = 20.0 * self.scale;
        let hair = *self.random.pick(&[
            5e-10 * reach,
            1e-9 * reach,
            3e-9 * reach,
            1e-8 * reach,
            1e-7,
            1e-5,
        ]);
        let hair = hair.max(5.0 * NEAR * reach);
        if self.random.chance(0.5) { hair } else { -hair }
    }

    /// A length on the half lattice from half a unit up to `most`, or
    /// nothing when `most` is shorter than half a unit.
    fn up_to(&mut self, most: f64) -> Option<f64> {
        let value = self.random.on_lattice(0.5, most / self.scale, 0.5) * self.scale;
        (value <= most).then_some(value)
    }

    /// How long along the axis a slant runs to change its edge by `rise`:
    /// as long, at forty-five degrees, and in a campaign now and then at
    /// thirty or sixty, whose tangent no lattice holds.
    fn run_for(&mut self, rise: f64) -> f64 {
        if self.off_the_lattice() && self.random.chance(0.2) {
            rise / self.random.pick(&[30.0f64, 60.0]).to_radians().tan()
        } else {
            rise
        }
    }

    /// A chamfer where the high edge steps between two level bands: the
    /// convex corner cut, or the concave one filled, by the step itself —
    /// the slant then lands on the other band's wall — by the step less a
    /// hair, or by less on the lattice.
    fn chamfer_at_a_step(&mut self, section: &Section) -> Option<Section> {
        let mut bands = banded(section);
        let steps: Vec<usize> = (0..bands.len().saturating_sub(1))
            .filter(|&index| {
                is_level(&bands[index])
                    && is_level(&bands[index + 1])
                    && bands[index][2] != bands[index + 1][2]
            })
            .collect();
        if steps.is_empty() {
            return None;
        }
        let at = *self.random.pick(&steps);
        let (high, next) = (bands[at][2], bands[at + 1][2]);
        let step = (next - high).abs();
        let rise = match self.random.below(10) {
            0..=3 => step,
            4 | 5 => step - self.hair().abs(),
            _ => self.up_to(step)?,
        };
        let run = self.run_for(rise);
        let cut = self.random.chance(0.5);
        let after = (next < high) == cut;
        let given = if after { at } else { at + 1 };
        let [length, low, high, ..] = bands[given];
        if run >= length {
            return None;
        }
        let other = if cut { high - rise } else { high + rise };
        bands[given][0] -= run;
        if after {
            bands.insert(given + 1, [run, low, high, low, other]);
        } else {
            bands.insert(given, [run, low, other, low, high]);
        }
        Some(rebuilt(section, section.from, &bands))
    }

    /// An end band on the axis brought to a point: a cone added past it, the
    /// whole band sloped down to the axis, or a cone cut short a hair or a
    /// lattice step from its tip.
    fn pointed(&mut self, section: &Section) -> Option<Section> {
        let mut bands = banded(section);
        let last = self.random.chance(0.5);
        let index = if last { bands.len() - 1 } else { 0 };
        let band = bands[index];
        if !is_level(&band) || band[1] != 0.0 {
            return None;
        }
        let [length, _, high, ..] = band;
        let tip = match self.random.below(3) {
            0 => 0.0,
            1 => {
                bands[index] = if last {
                    [length, 0.0, high, 0.0, 0.0]
                } else {
                    [length, 0.0, 0.0, 0.0, high]
                };
                return Some(rebuilt(section, section.from, &bands));
            }
            _ if self.random.chance(0.3) => self.hair().abs(),
            _ => self.up_to(high - 0.5 * self.scale)?,
        };
        let added = self.length(0.5, 4.0);
        if last {
            bands.push([added, 0.0, high, 0.0, tip]);
            Some(rebuilt(section, section.from, &bands))
        } else {
            bands.insert(0, [added, 0.0, tip, 0.0, high]);
            Some(rebuilt(section, section.from - added, &bands))
        }
    }

    /// The outer corner at an end of a shaft chamfered.
    fn chamfered_end(&mut self, section: &Section) -> Option<Section> {
        let mut bands = banded(section);
        let last = self.random.chance(0.5);
        let index = if last { bands.len() - 1 } else { 0 };
        let [length, low, high, ..] = bands[index];
        if !is_level(&bands[index]) {
            return None;
        }
        let rise = self.up_to((high - low).min(2.0 * self.scale))?;
        let run = self.run_for(rise);
        if run >= length {
            return None;
        }
        bands[index][0] -= run;
        if last {
            bands.push([run, low, high, low, high - rise]);
        } else {
            bands.insert(0, [run, low, high - rise, low, high]);
        }
        Some(rebuilt(section, section.from, &bands))
    }

    /// The inner corner at an end of a bore chamfered, a fifth of the time
    /// through the whole wall, to a knife's edge.
    fn chamfered_bore(&mut self, section: &Section) -> Option<Section> {
        let mut bands = banded(section);
        let last = self.random.chance(0.5);
        let index = if last { bands.len() - 1 } else { 0 };
        let [length, low, high, ..] = bands[index];
        if !is_level(&bands[index]) || low <= 0.0 {
            return None;
        }
        let rise = if self.random.chance(0.2) {
            high - low
        } else {
            let rise = self.up_to(high - low)?;
            if rise >= high - low {
                return None;
            }
            rise
        };
        let run = self.run_for(rise);
        if run >= length {
            return None;
        }
        bands[index][0] -= run;
        if last {
            bands.push([run, low, high, low + rise, high]);
        } else {
            bands.insert(0, [run, low + rise, high, low, high]);
        }
        Some(rebuilt(section, section.from, &bands))
    }

    /// A cone widening past an end band on the axis: what a countersunk
    /// hole is drawn with.
    fn countersink(&mut self, section: &Section) -> Option<Section> {
        let mut bands = banded(section);
        let last = self.random.chance(0.5);
        let index = if last { bands.len() - 1 } else { 0 };
        let [_, low, high, ..] = bands[index];
        if !is_level(&bands[index]) || low != 0.0 {
            return None;
        }
        let rise = self.length(0.5, 3.0);
        let run = self.run_for(rise);
        if last {
            bands.push([run, 0.0, high, 0.0, high + rise]);
            Some(rebuilt(section, section.from, &bands))
        } else {
            bands.insert(0, [run, 0.0, high + rise, 0.0, high]);
            Some(rebuilt(section, section.from - run, &bands))
        }
    }

    /// A level band whose high edge, or the low edge of a tube, ends
    /// elsewhere on the lattice.
    fn taper(&mut self, section: &Section) -> Option<Section> {
        let mut bands = banded(section);
        let level: Vec<usize> = (0..bands.len())
            .filter(|&index| is_level(&bands[index]))
            .collect();
        if level.is_empty() {
            return None;
        }
        let index = *self.random.pick(&level);
        let [_, low, high, ..] = bands[index];
        if low > 0.0 && self.random.chance(0.3) {
            bands[index][3] = self.up_to(high - 0.5 * self.scale)?;
        } else {
            bands[index][4] = low + self.length(0.5, (high - low) / self.scale + 3.0);
        }
        Some(rebuilt(section, section.from, &bands))
    }

    /// A sloped band split on the lattice into two that run on one line, a
    /// level one tapered first when none slopes.
    fn two_slants_on_one_line(&mut self, section: &Section) -> Option<Section> {
        let section = if section.slopes() {
            section.clone()
        } else {
            self.taper(section)?
        };
        let mut bands = banded(&section);
        let sloped: Vec<usize> = (0..bands.len())
            .filter(|&index| !is_level(&bands[index]))
            .collect();
        if sloped.is_empty() {
            return None;
        }
        let index = *self.random.pick(&sloped);
        let band = bands[index];
        let split = self.up_to(band[0] - 0.5 * self.scale)?;
        let share = split / band[0];
        let [low, high] = [1, 2].map(|edge| band[edge] + (band[edge + 2] - band[edge]) * share);
        bands[index] = [split, band[1], band[2], low, high];
        bands.insert(index + 1, [band[0] - split, low, high, band[3], band[4]]);
        Some(rebuilt(&section, section.from, &bands))
    }

    /// A level band whose high edge ends a hair off where it starts.
    fn a_hair_from_parallel(&mut self, section: &Section) -> Option<Section> {
        let mut bands = banded(section);
        let level: Vec<usize> = (0..bands.len())
            .filter(|&index| is_level(&bands[index]))
            .collect();
        if level.is_empty() {
            return None;
        }
        let index = *self.random.pick(&level);
        bands[index][4] += self.hair();
        Some(rebuilt(section, section.from, &bands))
    }

    /// A step of the high edge, or a shaft's end, taken down a hair's
    /// length along the axis rather than square to it.
    fn a_hair_from_square(&mut self, section: &Section) -> Option<Section> {
        let mut bands = banded(section);
        let run = self.hair().abs();
        let steps: Vec<usize> = (0..bands.len().saturating_sub(1))
            .filter(|&index| {
                is_level(&bands[index])
                    && is_level(&bands[index + 1])
                    && bands[index][2] != bands[index + 1][2]
            })
            .collect();
        let (index, slant) = if steps.is_empty() {
            let index = bands.len() - 1;
            let [_, low, high, ..] = bands[index];
            if !is_level(&bands[index]) {
                return None;
            }
            let rise = self.up_to(high - low)?;
            (index, [run, low, high, low, high - rise])
        } else {
            let index = *self.random.pick(&steps);
            let [_, low, high, ..] = bands[index];
            (index, [run, low, high, low, bands[index + 1][2]])
        };
        if run >= bands[index][0] {
            return None;
        }
        bands[index][0] -= run;
        bands.insert(index + 1, slant);
        Some(rebuilt(section, section.from, &bands))
    }

    /// A level band made two cones meeting at a rim, now and then at the
    /// radius of a band beside it.
    fn ridge(&mut self, section: &Section) -> Option<Section> {
        let mut bands = banded(section);
        let level: Vec<usize> = (0..bands.len())
            .filter(|&index| is_level(&bands[index]) && bands[index][0] >= self.scale)
            .collect();
        if level.is_empty() {
            return None;
        }
        let index = *self.random.pick(&level);
        let [length, low, high, ..] = bands[index];
        let first = self.up_to(length - 0.5 * self.scale)?;
        let beside: Vec<f64> = [index.checked_sub(1), Some(index + 1)]
            .into_iter()
            .flatten()
            .filter_map(|other| bands.get(other).map(|band| band[2]))
            .filter(|other| *other != high && *other > low)
            .collect();
        let rim = if !beside.is_empty() && self.random.chance(0.3) {
            *self.random.pick(&beside)
        } else {
            low + self.length(0.5, (high - low) / self.scale + 3.0)
        };
        if rim == high {
            return None;
        }
        bands[index] = [first, low, high, low, rim];
        bands.insert(index + 1, [length - first, low, rim, low, high]);
        Some(rebuilt(section, section.from, &bands))
    }

    /// One slant of a section across its axis, every edge kept on its own
    /// side of it: a band tapered, or an outer corner at an end chamfered.
    fn slant_across(&mut self, section: &Section) -> Option<Section> {
        let mut bands = banded(section);
        let high_edge = self.random.chance(0.5);
        if self.random.chance(0.5) {
            let index = self.random.below(bands.len());
            if high_edge {
                bands[index][4] = self.length(0.5, 4.0);
            } else {
                bands[index][3] = -self.length(0.5, 4.0);
            }
            return Some(rebuilt(section, section.from, &bands));
        }
        let last = self.random.chance(0.5);
        let index = if last { bands.len() - 1 } else { 0 };
        let [length, low, high, ..] = bands[index];
        if !is_level(&bands[index]) {
            return None;
        }
        let edge = if high_edge { high } else { -low };
        let rise = self.up_to(edge - 0.5 * self.scale)?;
        let run = self.run_for(rise);
        if run >= length {
            return None;
        }
        let (low_end, high_end) = if high_edge {
            (low, high - rise)
        } else {
            (low + rise, high)
        };
        bands[index][0] -= run;
        if last {
            bands.push([run, low, high, low_end, high_end]);
        } else {
            bands.insert(0, [run, low_end, high_end, low, high]);
        }
        Some(rebuilt(section, section.from, &bands))
    }

    /// A line of the plane of the origin holding `line`, run so that the
    /// end `ends[1]` of a prism reads above its end `ends[0]`, and where the
    /// two stand along it.
    fn upright(&mut self, line: Line, ends: [DVec3; 2]) -> (Plane, Axis, f64, f64) {
        let (plane, mut axis) = self.holding(line);
        if line.along(axis, ends[1]) < line.along(axis, ends[0]) {
            axis.backwards = !axis.backwards;
        }
        (
            plane,
            axis,
            line.along(axis, ends[0]),
            line.along(axis, ends[1]),
        )
    }

    /// A slanted turn drawn from a prism before it: a countersunk hole into
    /// a block, or about the axis of a circle, an arc or a ring of it a
    /// chamfer round its rim or a cone standing on it.
    pub(super) fn slanted_from_prism(&mut self, before: &Leaf) -> Leaf {
        let Leaf::Prism {
            plane,
            outline,
            height,
        } = before
        else {
            return self.fresh_turned();
        };
        let normal = plane.normal();
        let floor = plane.to_world(DVec2::ZERO);
        let ends = [floor, floor + normal * *height];
        let leaf = match outline {
            Outline::Rectangle { low, high } => self.countersunk(*plane, *low, *high, ends),
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
            } => self.on_a_rim(plane.to_world(*center), normal, *radius, ends),
            Outline::Rounded { low, radius, .. } => {
                self.on_a_rim(plane.to_world(*low + *radius), normal, *radius, ends)
            }
            Outline::Star { .. } => return self.fresh_turned(),
        };
        self.or_fresh(leaf)
    }

    /// The leaf drawn, or a fresh turn when it came out no solid.
    fn or_fresh(&mut self, leaf: Leaf) -> Leaf {
        if leaf.is_solid() {
            leaf
        } else {
            self.fresh_turned()
        }
    }

    /// A countersunk hole into a block from its end `ends[1]`, its axis
    /// along the block's normal through a place of the rectangle, on one of
    /// its edges or one of its corners: a bore from below the block, then a
    /// cone whose rim is flush with that end, a hair off it, or carried past
    /// it by a wider bore.
    fn countersunk(&mut self, plane: Plane, low: DVec2, high: DVec2, ends: [DVec3; 2]) -> Leaf {
        let unit = self.scale;
        let mut across =
            |from: f64, to: f64| self.random.on_lattice(from / unit, to / unit, 0.5) * unit;
        let inside = DVec2::new(across(low.x, high.x), across(low.y, high.y));
        let place = match self.random.below(10) {
            0 => DVec2::new(*self.random.pick(&[low.x, high.x]), inside.y),
            1 => DVec2::new(
                *self.random.pick(&[low.x, high.x]),
                *self.random.pick(&[low.y, high.y]),
            ),
            _ => inside,
        };
        let line = Line {
            point: plane.to_world(place),
            axis: world_axis(plane.normal()),
        };
        let (plane, axis, floor, cap) = self.upright(line, ends);
        let (bore, sink) = (self.length(0.5, 2.0), self.length(0.5, 2.0));
        let kind = self.random.below(10);
        let rim = if (5..7).contains(&kind) {
            cap + self.hair()
        } else {
            cap
        };
        let run = self.run_for(sink);
        let start = rim - run;
        let below = floor.min(start) - self.length(0.5, 1.5);
        let mut bands = vec![[start - below, 0.0, bore], [run, 0.0, bore]];
        let mut sloping = vec![[0.0, bore], [0.0, bore + sink]];
        if kind >= 7 {
            bands.push([self.length(0.5, 2.0), 0.0, bore + sink]);
            sloping.push([0.0, bore + sink]);
        }
        let section = Section::bands(below, &bands).sloping_to(&sloping);
        Leaf::Turned {
            plane,
            axis,
            section: self.aside(section),
            degrees: self.degrees(),
        }
    }

    /// A turn about the axis of a circle of `radius` raised from `ends[0]`
    /// to `ends[1]`: a ring whose inner edge chamfers the circle's rim at
    /// `ends[1]`, or a cone standing on that end whose rim is the circle's;
    /// in a campaign now and then a hair off the circle's axis.
    fn on_a_rim(&mut self, center: DVec3, normal: DVec3, radius: f64, ends: [DVec3; 2]) -> Leaf {
        let line = Line {
            point: center,
            axis: world_axis(normal),
        };
        let (plane, mut axis, _, cap) = self.upright(line, ends);
        if self.off_the_lattice() && self.random.chance(0.15) {
            axis.across += self.hair();
        }
        let section = if self.random.chance(0.5) {
            let rise = self.up_to(radius.min(2.0 * self.scale)).unwrap_or(radius);
            let run = self.run_for(rise);
            let past = self.length(0.5, 2.0);
            let outer = radius + past;
            Section::bands(
                cap - run,
                &[[run, radius, outer], [past, radius - rise, outer]],
            )
            .sloping_to(&[[radius - rise, outer], [radius - rise, outer]])
        } else {
            let tip = if self.random.chance(0.5) {
                0.0
            } else {
                self.up_to(radius - 0.5 * self.scale).unwrap_or(0.0)
            };
            Section::bands(cap, &[[self.length(0.5, 4.0), 0.0, radius]]).sloping_to(&[[0.0, tip]])
        };
        Leaf::Turned {
            plane,
            axis,
            section: self.aside(section),
            degrees: self.degrees(),
        }
    }

    /// A slanted turn about the line of a slanted turn before it, drawn
    /// from one of its slanted edges read along the line: the same cone, a
    /// part of it or carried on, filled to the axis or as a sleeve on it; a
    /// cylinder at one of its rims, ending there or a hair from there; a
    /// cone of another slope crossing it at a rim; or the same slant a hair
    /// off it. In a campaign now and then a hair off its axis.
    pub(super) fn along_a_slant(
        &mut self,
        line: Line,
        before: Axis,
        section: &Section,
        degrees: f64,
    ) -> Leaf {
        let ends = section.ends();
        let slants: Vec<([f64; 2], [f64; 2])> = (0..section.bands.len())
            .flat_map(|index| {
                let (start, end) = section.edges(index);
                let along = [ends[index], ends[index + 1]];
                (0..2)
                    .filter(move |&edge| start[edge] != end[edge])
                    .map(move |edge| (along, [start[edge].abs(), end[edge].abs()]))
            })
            .collect();
        let (along, radii) = *self.random.pick(&slants);
        let (plane, mut axis) = self.holding(line);
        if self.off_the_lattice() && self.random.chance(0.15) {
            axis.across += self.hair();
        }
        let levels = along.map(|at| line.along(axis, line.at(before, at)));
        let ([bottom, top], [low, high]) = if levels[0] <= levels[1] {
            (levels, radii)
        } else {
            ([levels[1], levels[0]], [radii[1], radii[0]])
        };
        let radius_at = |level: f64| low + (high - low) * (level - bottom) / (top - bottom);
        let unit = self.scale;
        let section = match self.random.below(10) {
            0..=2 => {
                let (from, to) = if self.random.chance(0.5) {
                    (bottom, top)
                } else if high >= low {
                    (bottom, top + self.length(0.5, 2.0))
                } else {
                    (bottom - self.length(0.5, 2.0), top)
                };
                let (start, end) = (radius_at(from).max(0.0), radius_at(to).max(0.0));
                if self.random.chance(0.5) {
                    Section::bands(from, &[[to - from, 0.0, start]]).sloping_to(&[[0.0, end]])
                } else {
                    let outer = start.max(end) + self.length(0.5, 2.0);
                    Section::bands(from, &[[to - from, start, outer]]).sloping_to(&[[end, outer]])
                }
            }
            3..=5 => {
                let rims: Vec<(f64, f64)> = [(bottom, low), (top, high)]
                    .into_iter()
                    .filter(|(_, radius)| *radius > unit / 4.0)
                    .collect();
                if rims.is_empty() {
                    return self.fresh_turned();
                }
                let (level, radius) = *self.random.pick(&rims);
                let level = if self.random.chance(0.3) {
                    level + self.hair()
                } else {
                    level
                };
                let length = self.length(0.5, 3.0);
                let from = if self.random.chance(0.5) {
                    level - length
                } else {
                    level
                };
                Section::bands(from, &[[length, 0.0, radius]])
            }
            6 | 7 => {
                let (level, radius) = if high >= low {
                    (top, high)
                } else {
                    (bottom, low)
                };
                let run = self.length(0.5, 2.0);
                let rise = (self.length(0.5, 2.0)).min(radius / 2.0);
                let slope = (high - low) / (top - bottom);
                let sign = if self.random.chance(0.5) { 1.0 } else { -1.0 };
                let sign = if -sign * rise / run == slope {
                    -sign
                } else {
                    sign
                };
                Section::bands(level - run, &[[2.0 * run, 0.0, radius + sign * rise]])
                    .sloping_to(&[[0.0, radius - sign * rise]])
            }
            _ => {
                let hair = if low == 0.0 || high == 0.0 {
                    self.hair().abs()
                } else {
                    self.hair()
                };
                Section::bands(bottom, &[[top - bottom, 0.0, low + hair]])
                    .sloping_to(&[[0.0, high + hair]])
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

    /// A prism drawn from a slanted turn before it: a circle or a ring on
    /// the plane square to its axis at one of its rims, of that rim's
    /// radius.
    pub(super) fn prism_at_a_rim(&mut self, line: Line, before: Axis, section: &Section) -> Leaf {
        let ends = section.ends();
        let rims: Vec<(f64, f64)> = (0..section.bands.len())
            .filter(|&index| section.slopes_at(index))
            .flat_map(|index| {
                let (start, end) = section.edges(index);
                [(ends[index], start), (ends[index + 1], end)]
            })
            .flat_map(|(along, edges)| edges.map(|edge| (along, edge.abs())))
            .filter(|(_, radius)| *radius > self.scale / 4.0)
            .collect();
        if rims.is_empty() {
            return Leaf::Prism {
                plane: self.plane(),
                outline: self.outline(),
                height: self.height(),
            };
        }
        let (along, radius) = *self.random.pick(&rims);
        let (plane, center) = square_to(line, line.at(before, along));
        let outline = match self.random.below(4) {
            0 | 1 => Outline::Circle {
                center,
                radius,
                from: 0.0,
            },
            2 => Outline::Ring {
                center,
                outer: radius + self.length(0.5, 2.0),
                inner: radius,
            },
            _ => Outline::Ring {
                center,
                outer: radius,
                inner: (radius - self.length(0.5, 1.0)).max(radius / 2.0),
            },
        };
        Leaf::Prism {
            plane,
            outline,
            height: self.height(),
        }
    }
}
