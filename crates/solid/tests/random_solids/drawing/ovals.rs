//! Prisms of ellipses drawn from a seed (#571): the outlines an ellipse
//! bounds — whole, cut by one trait or two, a slot with elliptic ends, a ring
//! — fresh or about a given ellipse, and the prisms and plates the draws of
//! ellipses raise them among. The three draws are in `ovals/`: raised along
//! one normal (`square.rs`), crossed from the side (`crossed.rs`) and cut by
//! leaning planes (`tilted.rs`).

mod crossed;
mod square;
mod tilted;

use std::f64::consts::TAU;

use glam::DVec2;

use super::super::{Leaf, Outline, Oval, Plane, Step};
use super::{Drawing, in_steps};

/// The hairs a tool drawn from an ellipse misses it by: moved or grown, in
/// units of the case's lattice, or turned, in radians.
const HAIRS: [f64; 4] = [1e-7, 1e-6, 1e-5, 1e-4];

/// How far apart, as a share of their length, the half-axes of an ellipse
/// drawn a hair from a circle stand: under the kernel's tolerance, about it
/// and over it.
const NEAR_A_CIRCLE: [f64; 5] = [1e-12, 1e-10, 1e-8, 1e-6, 1e-4];

/// The plane of the origin square to world axis `axis`, at `level` along
/// it.
pub(super) fn square_to_axis(axis: usize, level: f64) -> Plane {
    match axis {
        0 => Plane::Yz(level),
        1 => Plane::Xz(level),
        _ => Plane::Xy(level),
    }
}

/// The prism square to world axis `axis` reaching from `from` to `to` along
/// it: its plane, and its height signed as the plane's normal runs.
pub(super) fn spanning(axis: usize, from: f64, to: f64) -> (Plane, f64) {
    let plane = square_to_axis(axis, from);
    (plane, (to - from) * plane.normal()[axis])
}

/// The world axes a plane square to world axis `axis` reads as its first and
/// its second.
pub(super) fn plane_axes(axis: usize) -> [usize; 2] {
    match axis {
        0 => [1, 2],
        1 => [0, 2],
        _ => [0, 1],
    }
}

/// The direction of a plane square to world axis `axis` that runs along
/// world axis `world`.
pub(super) fn in_plane(axis: usize, world: usize) -> DVec2 {
    let [first, _] = plane_axes(axis);
    DVec2::AXES[usize::from(world != first)]
}

/// The world levels a prism on a plane of the origin reaches along its
/// normal, from the plane to its far end, and the world axis of that normal.
pub(super) fn levels(leaf: &Leaf) -> Option<(usize, [f64; 2])> {
    let Leaf::Prism { plane, height, .. } = leaf else {
        return None;
    };
    let normal = plane.normal();
    let axis = normal.abs().max_position();
    if normal[axis].abs() != 1.0 {
        return None;
    }
    let from = plane.frame().0[axis];
    Some((axis, [from, from + height * normal[axis]]))
}

/// The ellipses a leaf is drawn on, as `Outline::ovals` gives them.
pub(super) fn ovals_of(leaf: &Leaf) -> Vec<Oval> {
    match leaf {
        Leaf::Prism { outline, .. } => outline.ovals(),
        _ => Vec::new(),
    }
}

impl Drawing {
    /// The same number, or its negative, as a coin falls.
    pub(super) fn either_way(&mut self, value: f64) -> f64 {
        if self.random.chance(0.5) {
            value
        } else {
            -value
        }
    }

    /// A hair of `HAIRS`, either way, in units of the case's lattice.
    pub(super) fn oval_hair(&mut self) -> f64 {
        let hair = *self.random.pick(&HAIRS) * self.scale;
        self.either_way(hair)
    }

    /// The direction of an ellipse's first semi-axis: along one of the
    /// plane's axes half the time, on a step of fifteen degrees a quarter of
    /// it, at any angle otherwise.
    pub(super) fn oval_direction(&mut self) -> DVec2 {
        match self.random.below(4) {
            0 | 1 => *self
                .random
                .pick(&[DVec2::X, DVec2::Y, DVec2::NEG_X, DVec2::NEG_Y]),
            2 => {
                let turned =
                    DVec2::from_angle(self.random.on_lattice(15.0, 345.0, 15.0).to_radians());
                let rounding = |value: f64| if value.abs() < 1e-12 { 0.0 } else { value };
                DVec2::new(rounding(turned.x), rounding(turned.y))
            }
            _ => DVec2::from_angle(self.random.between(0.0, TAU)),
        }
    }

    /// The half-lengths of an ellipse on the half lattice: one now and then,
    /// a hair apart from time to time.
    pub(super) fn half_axes(&mut self) -> (f64, f64) {
        let first = self.length(0.5, 6.0);
        let second = match self.random.below(20) {
            0 | 1 => first,
            2 => {
                let hair = *self.random.pick(&NEAR_A_CIRCLE);
                first * (1.0 + self.either_way(hair))
            }
            _ => self.length(0.5, 6.0),
        };
        (first, second)
    }

    /// A fresh ellipse about a place of the lattice.
    pub(super) fn fresh_ellipse(&mut self) -> Oval {
        let center = self.point();
        let direction = self.oval_direction();
        let (first, second) = self.half_axes();
        Oval::new(center, direction * first, second)
    }

    /// A fresh outline an ellipse bounds: the ellipse whole half the time,
    /// cut by traits a fifth, a slot with elliptic ends or an elliptic ring
    /// otherwise.
    pub(super) fn fresh_oval(&mut self) -> Outline {
        let oval = self.fresh_ellipse();
        self.oval_outline(oval)
    }

    /// An outline drawn on an ellipse, by the weights `fresh_oval` gives.
    pub(super) fn oval_outline(&mut self, oval: Oval) -> Outline {
        match self.random.below(20) {
            0..=9 => self.whole(oval),
            10..=13 => self.chorded(oval),
            14..=16 => {
                let way = oval.first.normalize();
                let length = self.length(0.5, 6.0);
                Outline::OvalSlot {
                    from: oval.center,
                    to: oval.center + way * length,
                    across: oval.second,
                    along: oval.first.length(),
                }
            }
            _ => self.oval_ring(oval),
        }
    }

    /// The ellipse whole, its corner at the end of its first semi-axis, or
    /// anywhere round it a fifth of the time.
    pub(super) fn whole(&mut self, oval: Oval) -> Outline {
        let from = if self.random.chance(0.2) {
            self.random.between(0.0, 360.0)
        } else {
            0.0
        };
        Outline::Ellipse {
            center: oval.center,
            first: oval.first,
            second: oval.second,
            from,
        }
    }

    /// The ellipse whole seven times in ten, an outline drawn on it by the
    /// weights `fresh_oval` gives otherwise: a tool drawn to touch another
    /// leaf keeps its wall where the touch is drawn most of the time.
    pub(super) fn dressed(&mut self, oval: Oval) -> Outline {
        if self.random.chance(0.7) {
            self.whole(oval)
        } else {
            self.oval_outline(oval)
        }
    }

    /// The ellipse cut by one trait, or, three times in ten, by two square
    /// to one direction, which leave a band of it between them and two arcs
    /// of one ellipse.
    fn chorded(&mut self, oval: Oval) -> Outline {
        let arcs = if self.random.chance(0.3) {
            self.band(oval)
        } else {
            self.one_trait(oval)
        };
        Outline::Chorded {
            center: oval.center,
            first: oval.first,
            second: oval.second,
            arcs,
        }
    }

    /// A direction a trait through an ellipse stands square to: one of its
    /// axes, one of the plane's, either way.
    fn trait_direction(&mut self, oval: Oval) -> DVec2 {
        let direction = match self.random.below(3) {
            0 => oval.first.normalize(),
            1 => oval.first.normalize().perp(),
            _ => *self.random.pick(&[DVec2::X, DVec2::Y]),
        };
        self.either_way(1.0) * direction
    }

    /// Where a trait square to `toward` cuts an ellipse, from its centre:
    /// through its middle, on the half lattice inside its reach, or a hair
    /// from touching it.
    fn trait_offset(&mut self, oval: Oval, toward: DVec2) -> f64 {
        let reach = oval.support(toward);
        let unit = 0.5 * self.scale;
        match self.random.below(10) {
            0 | 1 => 0.0,
            2 => reach - self.oval_hair().abs().max(1e-6 * reach),
            _ => {
                let most = in_steps(reach - unit / 2.0, unit).floor().max(0.0) as i64;
                unit * (self.random.below(2 * most as usize + 1) as i64 - most) as f64
            }
        }
    }

    /// The arc of an ellipse one trait leaves, in degrees: what stands on
    /// the side of the trait its direction points away from.
    fn one_trait(&mut self, oval: Oval) -> Vec<[f64; 2]> {
        if self.random.chance(0.15) {
            let from = self.random.between(-180.0, 180.0);
            return vec![[from, from + self.random.between(40.0, 320.0)]];
        }
        let toward = self.trait_direction(oval);
        let offset = self.trait_offset(oval, toward);
        let (furthest, half) = facing(oval, toward, offset);
        vec![[furthest + half, furthest + 360.0 - half]]
    }

    /// The two arcs of an ellipse two traits square to one direction leave
    /// between them, in degrees, each trait on a quarter of the lattice
    /// inside its reach; one trait through its middle when it has no room
    /// for two.
    fn band(&mut self, oval: Oval) -> Vec<[f64; 2]> {
        let toward = self.trait_direction(oval);
        let reach = oval.support(toward);
        let step = 0.25 * self.scale;
        let most = in_steps(reach - step, step).floor() as i64;
        if most < 1 {
            let (furthest, half) = facing(oval, toward, 0.0);
            return vec![[furthest + half, furthest + 360.0 - half]];
        }
        let low = self.random.below(2 * most as usize) as i64 - most;
        let high = low + 1 + self.random.below((most - low) as usize) as i64;
        let (furthest, wide) = facing(oval, toward, step * low as f64);
        let (_, narrow) = facing(oval, toward, step * high as f64);
        vec![
            [furthest - wide, furthest - narrow],
            [furthest + narrow, furthest + wide],
        ]
    }

    /// The ellipse with a hole about its centre, half a unit inside it at
    /// least: round, of its own shape, or of its own axes and other lengths.
    fn oval_ring(&mut self, oval: Oval) -> Outline {
        let unit = 0.5 * self.scale;
        let (first, second) = (
            oval.first.length().max(4.0 * unit),
            oval.second.max(4.0 * unit),
        );
        let hole = match self.random.below(3) {
            0 => DVec2::splat(self.up_to(first.min(second) - unit).unwrap_or(unit)),
            1 => DVec2::new(first, second) / 2.0,
            _ => DVec2::new(
                self.up_to(first - unit).unwrap_or(unit),
                self.up_to(second - unit).unwrap_or(unit),
            ),
        };
        Outline::OvalRing {
            center: oval.center,
            first: oval.first.normalize() * first,
            second,
            hole,
        }
    }

    /// A step of a drawn tool, cutting six times in ten, adding otherwise.
    pub(super) fn stepped(&mut self, tool: Leaf) -> Step {
        if self.random.chance(0.6) {
            Step::cut(tool)
        } else {
            Step::add(tool)
        }
    }

    /// A fresh prism of an outline an ellipse bounds, square to world axis
    /// `axis`.
    pub(super) fn oval_prism(&mut self, axis: usize) -> Leaf {
        let level = self.coordinate(-2.0, 8.0);
        Leaf::Prism {
            plane: square_to_axis(axis, level),
            outline: self.fresh_oval(),
            height: self.height(),
        }
    }

    /// A fresh prism of a rectangle, a circle, a rounded rectangle, a slot
    /// or a ring, square to world axis `axis`.
    pub(super) fn profile_prism(&mut self, axis: usize) -> Leaf {
        let level = self.coordinate(-2.0, 8.0);
        Leaf::Prism {
            plane: square_to_axis(axis, level),
            outline: self.outline(),
            height: self.height(),
        }
    }

    /// A plate square to world axis `axis`, wide enough to raise an ellipse
    /// on or bore one through.
    pub(super) fn plate(&mut self, axis: usize) -> Leaf {
        let level = self.coordinate(-2.0, 4.0);
        let low = self.point() - DVec2::splat(3.0 * self.scale);
        let size = DVec2::new(self.length(4.0, 14.0), self.length(4.0, 14.0));
        let thickness = self.length(1.0, 4.0);
        Leaf::Prism {
            plane: square_to_axis(axis, level),
            outline: Outline::Rectangle {
                low,
                high: low + size,
            },
            height: self.either_way(thickness),
        }
    }
}

/// Where a trait square to `toward`, `offset` from an ellipse's centre,
/// cuts it, as eccentric angles in degrees: the angle the ellipse reaches
/// furthest along `toward` at, and how far either side of it the trait
/// crosses the wall.
fn facing(oval: Oval, toward: DVec2, offset: f64) -> (f64, f64) {
    let along = oval.first.dot(toward);
    let across = oval.across().dot(toward);
    let reach = along.hypot(across);
    (
        across.atan2(along).to_degrees(),
        (offset / reach).clamp(-1.0, 1.0).acos().to_degrees(),
    )
}
