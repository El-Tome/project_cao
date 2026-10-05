//! A profile of straight runs turned about a line of its plane: the line, the
//! section in the line's own coordinates, the corners the kernels are handed
//! and the rectangles the arithmetic turns.
//!
//! A section is read along its axis and across it — `a` along, `s` across,
//! positive on the left of the way the axis runs — as bands laid end to end
//! along the axis, each from a low edge to a high one, and rectangular holes.
//! Every run is square or parallel to the axis, which is what a profile the
//! exact kernel turns is made of.

use cao_solid::soundness::NEAR;
use cao_solid::turning;
use glam::DVec2;

/// Which axis of its plane a turned leaf's axis runs along.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Along {
    First,
    Second,
}

/// The line a section turns about: along the plane's first or second axis,
/// `across` from it — nought is the sketch's own axis, anything else a line
/// drawn parallel to it — the other way round when `backwards`, and leaning
/// off it by `lean` degrees: a slanted line of the plane.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Axis {
    pub along: Along,
    pub across: f64,
    pub backwards: bool,
    pub lean: f64,
}

impl Axis {
    pub fn first(across: f64) -> Axis {
        Axis {
            along: Along::First,
            across,
            backwards: false,
            lean: 0.0,
        }
    }

    pub fn second(across: f64) -> Axis {
        Axis {
            along: Along::Second,
            across,
            backwards: false,
            lean: 0.0,
        }
    }

    pub fn backwards(self) -> Axis {
        Axis {
            backwards: true,
            ..self
        }
    }

    pub fn leaning(self, lean: f64) -> Axis {
        Axis { lean, ..self }
    }

    /// The axis as the kernels read it, in the plane's own coordinates: a
    /// point it runs through and its direction, of unit length.
    pub fn line(&self) -> turning::Axis {
        let (origin, base) = match self.along {
            Along::First => (DVec2::new(0.0, self.across), DVec2::X),
            Along::Second => (DVec2::new(self.across, 0.0), DVec2::Y),
        };
        let base = if self.backwards { -base } else { base };
        let direction = if self.lean == 0.0 {
            base
        } else {
            DVec2::from_angle(self.lean.to_radians()).rotate(base)
        };
        turning::Axis { origin, direction }
    }

    /// A place of a section, `a` along the axis and `s` across it, in the
    /// plane's own coordinates.
    pub fn at(&self, place: DVec2) -> DVec2 {
        let line = self.line();
        line.origin + line.direction * place.x + line.direction.perp() * place.y
    }
}

/// A profile of bands laid end to end along the axis from `from`, each
/// `[length, low edge, high edge]`, and holes, each the low and the high
/// corner of a rectangle in `(a, s)`.
#[derive(Clone, Debug, PartialEq)]
pub struct Section {
    pub from: f64,
    pub bands: Vec<[f64; 3]>,
    pub holes: Vec<[DVec2; 2]>,
}

/// A rectangle of a section on one side of its axis, turned: from `along[0]`
/// to `along[1]` along the axis, from `away[0]` to `away[1]` away from it,
/// on the `side` of it `+1.0` or `-1.0` says.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub along: [f64; 2],
    pub away: [f64; 2],
    pub side: f64,
}

/// How close to its axis, as a share of its reach, the arithmetic takes a
/// section's edge as on the axis: ten times the finest hair it is drawn a
/// hair off with, and far below half a unit of its lattice. The kernels'
/// own band is theirs to choose in between.
pub const ON_THE_AXIS: f64 = 1e-5;

impl Section {
    pub fn bands(from: f64, bands: &[[f64; 3]]) -> Section {
        Section {
            from,
            bands: bands.to_vec(),
            holes: Vec::new(),
        }
    }

    pub fn with_holes(self, holes: &[([f64; 2], [f64; 2])]) -> Section {
        Section {
            holes: holes
                .iter()
                .map(|(low, high)| [DVec2::from(*low), DVec2::from(*high)])
                .collect(),
            ..self
        }
    }

    /// Where each band starts and ends along the axis.
    pub fn ends(&self) -> Vec<f64> {
        let mut at = self.from;
        std::iter::once(at)
            .chain(self.bands.iter().map(|band| {
                at += band[0];
                at
            }))
            .collect()
    }

    /// The furthest any corner of the section stands from the axis's origin
    /// along it or across it.
    pub fn reach(&self) -> f64 {
        let ends = self.ends();
        let across = self
            .bands
            .iter()
            .flat_map(|band| [band[1], band[2]])
            .map(f64::abs);
        ends.iter()
            .map(|end| end.abs())
            .chain(across)
            .fold(0.0, f64::max)
    }

    /// The outline anticlockwise, along the low edges and back along the high
    /// ones, a corner kept wherever two bands meet even when their edges are
    /// level — two runs on one line — and each hole clockwise, all in
    /// `(a, s)`.
    pub fn corners(&self) -> (Vec<DVec2>, Vec<Vec<DVec2>>) {
        let ends = self.ends();
        let mut outline: Vec<DVec2> = Vec::new();
        let mut push = |corner: DVec2| {
            if outline.last() != Some(&corner) {
                outline.push(corner);
            }
        };
        for (index, band) in self.bands.iter().enumerate() {
            push(DVec2::new(ends[index], band[1]));
            push(DVec2::new(ends[index + 1], band[1]));
        }
        for (index, band) in self.bands.iter().enumerate().rev() {
            push(DVec2::new(ends[index + 1], band[2]));
            push(DVec2::new(ends[index], band[2]));
        }
        if outline.len() > 1 && outline.first() == outline.last() {
            outline.pop();
        }
        let holes = self
            .holes
            .iter()
            .map(|[low, high]| {
                vec![
                    *low,
                    DVec2::new(low.x, high.y),
                    *high,
                    DVec2::new(high.x, low.y),
                ]
            })
            .collect();
        (outline, holes)
    }

    /// An edge this close to the axis is on it.
    fn snap(&self) -> f64 {
        ON_THE_AXIS * self.reach()
    }

    /// Whether every band lies on one side of the axis, an edge a hair across
    /// it taken as on it: the side, or nothing for a section across it.
    pub fn side(&self) -> Option<f64> {
        let snap = self.snap();
        if self.bands.iter().all(|band| band[1] >= -snap) {
            Some(1.0)
        } else if self.bands.iter().all(|band| band[2] <= snap) {
            Some(-1.0)
        } else {
            None
        }
    }

    /// Whether the section is one area: every band of some length and some
    /// width, each overlapping the next by more than the rules can tell from
    /// a pinch, at most one hole to a band, strictly inside it, and either
    /// every band on one side of the axis or every band across it, a hole
    /// then on one side.
    pub fn is_solid(&self) -> bool {
        let reach = self.reach();
        let ends = self.ends();
        let banded = !self.bands.is_empty()
            && self
                .bands
                .iter()
                .all(|[length, low, high]| *length > 0.0 && low < high);
        let overlapping = self.bands.windows(2).all(|pair| {
            pair[0][2].min(pair[1][2]) - pair[0][1].max(pair[1][1]) > 5.0 * NEAR * reach
        });
        let snap = self.snap();
        let across = self
            .bands
            .iter()
            .all(|band| band[1] < -snap && band[2] > snap);
        let holed = self.holes.iter().all(|[low, high]| {
            let inside = (0..self.bands.len()).filter(|&index| {
                let band = self.bands[index];
                ends[index] < low.x
                    && high.x < ends[index + 1]
                    && band[1] < low.y
                    && high.y < band[2]
            });
            low.x < high.x
                && low.y < high.y
                && inside.count() == 1
                && (!across || low.y > 0.0 || high.y < 0.0)
        });
        let one_to_a_band = (0..self.bands.len()).all(|index| {
            self.holes
                .iter()
                .filter(|[low, _]| ends[index] < low.x && low.x < ends[index + 1])
                .count()
                <= 1
        });
        banded && overlapping && holed && one_to_a_band && (self.side().is_some() || across)
    }

    /// The section as the one-sided sections the kernels are handed: itself
    /// when it lies on one side of its axis, its two sides cut along the axis
    /// when it lies across, the left one first.
    pub fn sides(&self) -> Vec<Section> {
        if self.side().is_some() {
            return vec![self.clone()];
        }
        let side = |left: bool| Section {
            from: self.from,
            bands: self
                .bands
                .iter()
                .map(|&[length, low, high]| {
                    if left {
                        [length, 0.0, high]
                    } else {
                        [length, low, 0.0]
                    }
                })
                .collect(),
            holes: self
                .holes
                .iter()
                .filter(|[low, _]| (low.y > 0.0) == left)
                .copied()
                .collect(),
        };
        vec![side(true), side(false)]
    }

    /// The section cut into rectangles that overlap nowhere, each
    /// `[from, to, low, high]`: a band is one, a band with a hole the four
    /// around the hole.
    pub fn rectangles(&self) -> Vec<[f64; 4]> {
        let ends = self.ends();
        let mut rectangles = Vec::new();
        for (index, &[_, low, high]) in self.bands.iter().enumerate() {
            let (start, end) = (ends[index], ends[index + 1]);
            match self
                .holes
                .iter()
                .find(|[hole, _]| start < hole.x && hole.x < end)
            {
                Some([hole_low, hole_high]) => rectangles.extend([
                    [start, hole_low.x, low, high],
                    [hole_high.x, end, low, high],
                    [hole_low.x, hole_high.x, low, hole_low.y],
                    [hole_low.x, hole_high.x, hole_high.y, high],
                ]),
                None => rectangles.push([start, end, low, high]),
            }
        }
        rectangles
    }

    /// The section's rectangles as the arithmetic turns them: each edge
    /// within a hair of the axis laid on it, and a rectangle across the axis
    /// cut in two along it.
    pub fn pieces(&self) -> Vec<Piece> {
        let snap = self.snap();
        let snapped = |s: f64| if s.abs() <= snap { 0.0 } else { s };
        let mut pieces = Vec::new();
        for [from, to, low, high] in self.rectangles() {
            let (low, high) = (snapped(low), snapped(high));
            if high > 0.0 {
                pieces.push(Piece {
                    along: [from, to],
                    away: [low.max(0.0), high],
                    side: 1.0,
                });
            }
            if low < 0.0 {
                pieces.push(Piece {
                    along: [from, to],
                    away: [(-high).max(0.0), -low],
                    side: -1.0,
                });
            }
        }
        pieces
    }
}
