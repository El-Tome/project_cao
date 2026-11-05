//! Walls at any angle to each other (#577), drawn before the kernel builds
//! them: a shaft, a tube, a cone or an elliptic shaft about a world axis to
//! start with, and tools crossing its wall at an angle — a hole or a pin,
//! round or elliptic, raised on a leaning plane, and a cone turned about a
//! leaning line of a plane of the origin.
//!
//! The angle off the start's axis comes on a step of fifteen degrees, on a
//! degree's step, at a real angle, or a hair from square or from parallel —
//! in the band a lay would take and in the band past it. The tool's axis
//! passes the start's at a distance drawn towards where the curve the two
//! walls meet along changes kind: through it, a hair off it, where the two
//! walls touch inside or outside, a hair from there, and anywhere between;
//! its radius now and then the start's own. A tool's wall and the start's
//! are neither parallel nor square, or a cone's axis is not the other's, so
//! every case asks beyond the conics, and the scope rule excuses a decline
//! as unsupported until the kernel builds the pair.

use glam::{DVec2, DVec3};

use super::super::{Along, Axis, Case, Leaf, Mode, Outline, Oval, Plane, Section, Step};
use super::ovals::{in_plane, plane_axes, spanning, square_to_axis};
use super::turns::Line;
use super::{Among, Drawing, shifted};
use cao_solid::soundness::Random;

/// Hairs off square or off parallel, in degrees, that move a tool as large
/// as the part by under twenty tolerances.
const LAID: [f64; 4] = [2e-8, 4e-8, 1e-7, 3e-7];

/// Hairs off square or off parallel, in degrees, past what a lay takes and
/// under a sine of a thousandth.
const WARNED: [f64; 5] = [1e-5, 1e-4, 1e-3, 1e-2, 5e-2];

/// The start's wall: the world axis it turns about, a point of that axis,
/// the levels its ends stand at along it, and how far its wall reaches from
/// the axis at each of them — or, for an elliptic one, its ellipse about the
/// origin of the plane square to the axis.
#[derive(Clone, Copy, Debug)]
struct Wall {
    axis: usize,
    point: DVec3,
    levels: [f64; 2],
    radii: [f64; 2],
    oval: Option<Oval>,
}

impl Wall {
    fn at(&self, level: f64) -> DVec3 {
        let mut point = self.point;
        point[self.axis] = level;
        point
    }

    /// How far the wall reaches from its axis at `level` along world axis
    /// `toward`, square to its own.
    fn reach(&self, level: f64, toward: usize) -> f64 {
        if let Some(oval) = self.oval {
            return oval.support(in_plane(self.axis, toward));
        }
        let [from, to] = self.levels;
        let share = (level - from) / (to - from);
        self.radii[0] + (self.radii[1] - self.radii[0]) * share
    }
}

/// Where a tool crosses the start's wall: the start's world axis, the
/// world axis square to it the tool passes along, the point of the start's
/// axis it crosses at, how far the start's wall reaches towards it there,
/// and how far the tool runs either side.
#[derive(Clone, Copy, Debug)]
struct Across {
    axis: usize,
    aside: usize,
    crossing: DVec3,
    reach: f64,
    length: f64,
}

/// A tool's section across its axis: a circle, or an ellipse about its
/// centre.
#[derive(Clone, Copy, Debug)]
enum Shape {
    Round(f64),
    Oval(Oval),
}

impl Shape {
    /// How far the tool's wall reaches from its axis along `direction` of
    /// its plane.
    fn support(self, direction: DVec2) -> f64 {
        match self {
            Shape::Round(radius) => radius,
            Shape::Oval(oval) => oval.support(direction),
        }
    }

    fn about(self, center: DVec2) -> Outline {
        match self {
            Shape::Round(radius) => Outline::Circle {
                center,
                radius,
                from: 0.0,
            },
            Shape::Oval(oval) => Outline::Ellipse {
                center,
                first: oval.first,
                second: oval.second,
                from: 0.0,
            },
        }
    }
}

/// How far a tool's axis turns off the start's, in degrees: off its
/// direction, or off square to it, each written so that a hair keeps its
/// digits.
#[derive(Clone, Copy, Debug)]
enum Angle {
    Off(f64),
    OffSquare(f64),
}

impl Angle {
    fn off(self) -> f64 {
        match self {
            Angle::Off(degrees) => degrees,
            Angle::OffSquare(degrees) => 90.0 - degrees,
        }
    }

    fn off_square(self) -> f64 {
        match self {
            Angle::Off(degrees) => 90.0 - degrees,
            Angle::OffSquare(degrees) => degrees,
        }
    }
}

/// The world axis the normal of a plane leaning about `holds` starts along,
/// as `Plane::leaning` turns it.
fn starting_normal(holds: usize) -> usize {
    if holds == 2 { 1 } else { 2 }
}

impl Case {
    /// The case a seed stands for among walls at any angle to each other
    /// (#577), by the weights the module's head gives. A seed space of its
    /// own: none of the cases another draw gives the same seed.
    pub fn drawn_walls_at_any_angle(seed: u64) -> Case {
        let mut random = Random::seeded(seed);
        for _ in 0..8 {
            random.number();
        }
        let scale = *random.pick(&[1.0, 1.0, 1.0, 5.0, 30.0]);
        let mut drawing = Drawing {
            random,
            scale,
            among: Among::Turned {
                off_the_lattice: true,
                slanted: true,
                tilted: true,
                rounded: false,
            },
        };
        drawing.walls_at_any_angle()
    }
}

impl Drawing {
    fn walls_at_any_angle(&mut self) -> Case {
        let axis = *self.random.pick(&[2, 2, 1, 0]);
        let (start, wall) = match self.random.below(20) {
            0..=8 => self.shaft(axis),
            9..=14 => self.cone_standing_on(axis),
            _ => self.elliptic_shaft(axis),
        };
        let count = *self.random.pick(&[1, 1, 2, 2, 3]);
        let mut steps = Vec::new();
        for _ in 0..count {
            if let Some(step) = (0..4).find_map(|_| self.crossing_the_wall(&start, wall)) {
                steps.push(step);
            }
        }
        Case::new(start, steps)
    }

    /// Two levels along an axis, the second above the first.
    fn levels(&mut self) -> [f64; 2] {
        let from = self.coordinate(-4.0, 0.0);
        [from, from + self.length(4.0, 10.0)]
    }

    /// A place of a plane of the origin near its origin.
    fn near_the_origin(&mut self) -> DVec2 {
        DVec2::new(self.coordinate(-2.0, 2.0), self.coordinate(-2.0, 2.0))
    }

    /// A round shaft about world axis `axis`, a tube one time in five.
    fn shaft(&mut self, axis: usize) -> (Leaf, Wall) {
        let [from, to] = self.levels();
        let (plane, height) = spanning(axis, from, to);
        let center = self.near_the_origin();
        let radius = self.length(2.0, 5.0);
        let outline = match self.up_to(radius - 0.5 * self.scale) {
            Some(inner) if self.random.chance(0.2) => Outline::Ring {
                center,
                outer: radius,
                inner,
            },
            _ => Outline::Circle {
                center,
                radius,
                from: 0.0,
            },
        };
        let wall = Wall {
            axis,
            point: plane.to_world(center),
            levels: [from, to],
            radii: [radius; 2],
            oval: None,
        };
        (Leaf::prism(plane, outline, height), wall)
    }

    /// An elliptic shaft about world axis `axis`, its axes on the plane's or
    /// turned.
    fn elliptic_shaft(&mut self, axis: usize) -> (Leaf, Wall) {
        let [from, to] = self.levels();
        let (plane, height) = spanning(axis, from, to);
        let center = self.near_the_origin();
        let first = self.oval_direction() * self.length(2.0, 5.0);
        let second = self.length(2.0, 5.0);
        let outline = Outline::Ellipse {
            center,
            first,
            second,
            from: 0.0,
        };
        let wall = Wall {
            axis,
            point: plane.to_world(center),
            levels: [from, to],
            radii: [0.0; 2],
            oval: Some(Oval::new(DVec2::ZERO, first, second)),
        };
        (Leaf::prism(plane, outline, height), wall)
    }

    /// A cone about world axis `axis`: a point, or a frustum either way up.
    fn cone_standing_on(&mut self, axis: usize) -> (Leaf, Wall) {
        let levels = self.levels();
        let center = self.near_the_origin();
        let [first, second] = plane_axes(axis);
        let mut point = DVec3::ZERO;
        point[first] = center.x;
        point[second] = center.y;
        let wide = self.length(2.0, 5.0);
        let narrow = if self.random.chance(0.35) {
            0.0
        } else {
            self.up_to(wide - 0.5 * self.scale).unwrap_or(0.0)
        };
        let radii = if self.random.chance(0.5) {
            [wide, narrow]
        } else {
            [narrow, wide]
        };
        let wall = Wall {
            axis,
            point,
            levels,
            radii,
            oval: None,
        };
        let line = Line { point, axis };
        let ends = levels.map(|level| wall.at(level));
        let (plane, turned, low, high) = self.upright(line, ends);
        let section =
            Section::bands(low, &[[high - low, 0.0, radii[0]]]).sloping_to(&[[0.0, radii[1]]]);
        (Leaf::turned(plane, turned, section, 360.0), wall)
    }

    /// How far a tool turns off the start's axis.
    fn angle(&mut self) -> Angle {
        match self.random.below(20) {
            0..=5 => Angle::Off(self.random.on_lattice(15.0, 75.0, 15.0)),
            6..=8 => Angle::Off(self.random.on_lattice(1.0, 89.0, 1.0)),
            9..=11 => Angle::Off(self.random.between(1.0, 89.0)),
            12 | 13 => Angle::OffSquare(*self.random.pick(&LAID)),
            14 | 15 => Angle::OffSquare(*self.random.pick(&WARNED)),
            16 | 17 => Angle::Off(*self.random.pick(&LAID)),
            _ => Angle::Off(*self.random.pick(&WARNED)),
        }
    }

    /// How far a tool's axis passes from the start's, either way: through
    /// it or a hair off, where a tool's wall reaching `width` towards the
    /// start's, which reaches `reach` towards it, touches it inside or
    /// outside or a hair from there, or anywhere inside.
    fn passing(&mut self, reach: f64, width: f64) -> f64 {
        let inside = (reach - width).abs();
        let distance = match self.random.below(20) {
            0..=5 => 0.0,
            6 | 7 => self.hair().abs(),
            8..=10 => inside,
            11 => inside + self.hair(),
            12 | 13 => reach + width,
            14 => reach + width + self.hair(),
            _ => self.random.between(0.0, reach),
        };
        self.either_way(distance)
    }

    /// A tool's section against a wall reaching `reach`: a circle of the
    /// wall's own radius a time in five, a hair off it now and then, on the
    /// lattice otherwise; an ellipse a time in seven.
    fn shape(&mut self, reach: f64) -> Shape {
        if self.random.below(7) == 0 {
            let first = self.oval_direction() * self.length(0.5, 2.0);
            return Shape::Oval(Oval::new(DVec2::ZERO, first, self.length(0.5, 2.0)));
        }
        Shape::Round(match self.random.below(10) {
            0 | 1 => reach,
            2 => reach + self.hair(),
            _ => self.length(0.5, 2.0),
        })
    }

    /// A tool crossing the start's wall at an angle, cutting it or added to
    /// it; nothing when the plane it was to be raised on stood square to
    /// the lines of measure.
    fn crossing_the_wall(&mut self, start: &Leaf, wall: Wall) -> Option<Step> {
        let (low, high) = start.bounds()?;
        let [from, to] = wall.levels;
        let level = if self.random.chance(0.6) {
            (from + to) / 2.0
        } else {
            self.random.between(from, to)
        };
        let aside = self.square_to(wall.axis);
        let reach = wall.reach(level, aside);
        let across = Across {
            axis: wall.axis,
            aside,
            crossing: wall.at(level),
            reach,
            length: (high - low).length() + reach + self.scale,
        };
        let angle = self.angle();
        let tool = if self.random.chance(0.7) {
            let shape = self.shape(reach);
            self.raised_across(across, angle, shape)?
        } else {
            self.turned_across(across, angle)
        };
        let mode = if self.random.chance(0.75) {
            Mode::Cut
        } else {
            Mode::Add
        };
        Some(Step { mode, tool })
    }

    /// A prism of `shape` raised on a plane holding the world axis the
    /// tool passes along, whose normal turns off the start's axis by
    /// `angle`, its axis passing the crossing by what `passing` draws.
    fn raised_across(&mut self, across: Across, angle: Angle, shape: Shape) -> Option<Leaf> {
        let Across {
            axis,
            aside,
            crossing,
            reach,
            length,
        } = across;
        let degrees = if starting_normal(aside) == axis {
            angle.off()
        } else {
            angle.off_square()
        };
        let first = if self.random.chance(0.5) { 1.0 } else { -1.0 };
        let plane = [first, -first]
            .into_iter()
            .map(|sign| Plane::leaning(aside, sign * degrees, crossing))
            .find(|plane| super::clear_of_the_lines(*plane))?;
        let (_, u, v) = plane.frame();
        let rounding = |value: f64| if value.abs() < 1e-12 { 0.0 } else { value };
        let toward = DVec2::new(
            rounding(DVec3::AXES[aside].dot(u)),
            rounding(DVec3::AXES[aside].dot(v)),
        );
        let passing = self.passing(reach, shape.support(toward));
        Some(Leaf::prism(
            shifted(plane, -length),
            shape.about(toward * passing),
            2.0 * length,
        ))
    }

    /// A cone turned about a line of the plane of the origin square to the
    /// world axis the tool passes along, the line turning off the start's
    /// axis by `angle` and passing the crossing by what `passing` draws:
    /// from a radius to another, or to a point.
    fn turned_across(&mut self, across: Across, angle: Angle) -> Leaf {
        let Across {
            axis,
            aside,
            crossing,
            reach,
            length,
        } = across;
        let toward = 3 - axis - aside;
        let [first, second] = plane_axes(aside);
        let lean = if angle.off() <= 45.0 {
            angle.off()
        } else {
            angle.off_square()
        };
        let base = if angle.off() <= 45.0 { axis } else { toward };
        let along = if base == first {
            Along::First
        } else {
            Along::Second
        };
        let turned = Axis {
            along,
            across: 0.0,
            backwards: false,
            lean: self.either_way(lean),
        };
        let direction = turned.line().direction;
        let at = DVec2::new(crossing[first], crossing[second]);
        let (offset, middle) = match along {
            Along::First => (at.y - at.x * direction.y / direction.x, at.x / direction.x),
            Along::Second => (at.x - at.y * direction.x / direction.y, at.y / direction.y),
        };
        let wide = self.length(0.5, 2.5);
        let narrow = if self.random.chance(0.4) {
            0.0
        } else {
            self.length(0.5, 2.5)
        };
        let [near, far] = if self.random.chance(0.5) {
            [wide, narrow]
        } else {
            [narrow, wide]
        };
        let passing = self.passing(reach, (near + far) / 2.0);
        let plane = square_to_axis(aside, crossing[aside] + passing);
        let section =
            Section::bands(middle - length, &[[2.0 * length, 0.0, near]]).sloping_to(&[[0.0, far]]);
        Leaf::turned(
            plane,
            Axis {
                across: offset,
                ..turned
            },
            section,
            360.0,
        )
    }
}
