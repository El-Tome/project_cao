//! A profile of straight runs turned about a line of its plane: the line, the
//! section in the line's own coordinates, the corners the kernels are handed
//! and the trapezoids the arithmetic turns.
//!
//! A section is read along its axis and across it — `a` along, `s` across,
//! positive on the left of the way the axis runs — as bands laid end to end
//! along the axis, each from a low edge to a high one, and rectangular holes.
//! A band's edges are level, or slope from where it starts to where it ends
//! (#536): every run is square to the axis, parallel to it or slanted, and
//! turns into a plane, a cylinder or a cone.

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
/// `[length, low edge, high edge]` where it starts, and holes, each the low
/// and the high corner of a rectangle in `(a, s)`. `sloping_to` is empty
/// when every band is level, and otherwise holds, for each band, its low and
/// its high edge where it ends.
#[derive(Clone, Debug, PartialEq)]
pub struct Section {
    pub from: f64,
    pub bands: Vec<[f64; 3]>,
    pub sloping_to: Vec<[f64; 2]>,
    pub holes: Vec<[DVec2; 2]>,
}

/// A piece of a section between two lines square to its axis: from
/// `along[0]` to `along[1]` along it, its low edge from `low[0]` to `low[1]`
/// and its high edge from `high[0]` to `high[1]`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Trapezoid {
    pub along: [f64; 2],
    pub low: [f64; 2],
    pub high: [f64; 2],
}

/// A trapezoid of a section on one side of its axis, turned: from
/// `along[0]` to `along[1]` along the axis, from `away[0]` to `away[1]` away
/// from it where it starts and from `ending[0]` to `ending[1]` where it ends,
/// on the `side` of it `+1.0` or `-1.0` says. A slab of an annulus when the
/// two are one, and of the room between two cones otherwise.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub along: [f64; 2],
    pub away: [f64; 2],
    pub ending: [f64; 2],
    pub side: f64,
}

impl Piece {
    pub fn is_level(&self) -> bool {
        self.away == self.ending
    }

    /// The inner and the outer radius `along` the axis, within the piece.
    pub fn at(&self, along: f64) -> [f64; 2] {
        if self.is_level() {
            return self.away;
        }
        let share = (along - self.along[0]) / (self.along[1] - self.along[0]);
        [0, 1].map(|edge| self.away[edge] + (self.ending[edge] - self.away[edge]) * share)
    }

    /// How fast the inner and the outer radius grow along the axis.
    pub fn slopes(&self) -> [f64; 2] {
        let length = self.along[1] - self.along[0];
        [0, 1].map(|edge| (self.ending[edge] - self.away[edge]) / length)
    }

    /// What the piece sweeps per radian, twice over: its length times the
    /// difference of the squares of its radii, for a frustum the mean of
    /// those squares over its length.
    pub fn swept(&self) -> f64 {
        let length = self.along[1] - self.along[0];
        if self.is_level() {
            return (self.away[1].powi(2) - self.away[0].powi(2)) * length;
        }
        let squares = |edge: usize| {
            let (one, other) = (self.away[edge], self.ending[edge]);
            one * one + one * other + other * other
        };
        length * (squares(1) - squares(0)) / 3.0
    }
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
            sloping_to: Vec::new(),
            holes: Vec::new(),
        }
    }

    /// The section with each band's low and high edge where it ends: level
    /// again, and written so, when every band ends where it starts.
    pub fn sloping_to(self, ends: &[[f64; 2]]) -> Section {
        Section {
            sloping_to: ends.to_vec(),
            ..self
        }
        .canonical()
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

    /// The one way of writing the section: no ends at all when every band
    /// is level, which keeps the shrinker from going round two spellings of
    /// one section.
    pub fn canonical(self) -> Section {
        let level = self.sloping_to.len() == self.bands.len()
            && self
                .bands
                .iter()
                .zip(&self.sloping_to)
                .all(|(band, end)| *end == [band[1], band[2]]);
        if level {
            Section {
                sloping_to: Vec::new(),
                ..self
            }
        } else {
            self
        }
    }

    /// Whether some band's edges slope.
    pub fn slopes(&self) -> bool {
        !self.sloping_to.is_empty()
    }

    /// The low and the high edge of band `index` where it starts and where
    /// it ends.
    pub fn edges(&self, index: usize) -> ([f64; 2], [f64; 2]) {
        let [_, low, high] = self.bands[index];
        let end = self.sloping_to.get(index).copied().unwrap_or([low, high]);
        ([low, high], end)
    }

    /// Whether band `index` slopes.
    pub fn slopes_at(&self, index: usize) -> bool {
        let (start, end) = self.edges(index);
        start != end
    }

    /// The low and the high edge of band `index` at `along` the axis, within
    /// the band: what each is where the band starts, for a level one.
    pub fn at(&self, index: usize, along: f64) -> [f64; 2] {
        let (start, end) = self.edges(index);
        if start == end {
            return start;
        }
        let from = self.ends()[index];
        let share = (along - from) / self.bands[index][0];
        [0, 1].map(|edge| start[edge] + (end[edge] - start[edge]) * share)
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
            .chain(self.sloping_to.iter().flatten().copied())
            .map(f64::abs);
        ends.iter()
            .map(|end| end.abs())
            .chain(across)
            .fold(0.0, f64::max)
    }

    /// The outline anticlockwise, along the low edges and back along the high
    /// ones, a corner kept wherever two bands meet even when their edges run
    /// on — two runs on one line — and each hole clockwise, all in `(a, s)`.
    /// A band whose two edges meet at an end of the section closes there at
    /// a point.
    pub fn corners(&self) -> (Vec<DVec2>, Vec<Vec<DVec2>>) {
        let ends = self.ends();
        let mut outline: Vec<DVec2> = Vec::new();
        let mut push = |corner: DVec2| {
            if outline.last() != Some(&corner) {
                outline.push(corner);
            }
        };
        for index in 0..self.bands.len() {
            let ([low, _], [low_end, _]) = self.edges(index);
            push(DVec2::new(ends[index], low));
            push(DVec2::new(ends[index + 1], low_end));
        }
        for index in (0..self.bands.len()).rev() {
            let ([_, high], [_, high_end]) = self.edges(index);
            push(DVec2::new(ends[index + 1], high_end));
            push(DVec2::new(ends[index], high));
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

    /// Whether every band lies on one side of the axis, at both of its ends,
    /// an edge a hair across it taken as on it: the side, or nothing for a
    /// section across it.
    pub fn side(&self) -> Option<f64> {
        let snap = self.snap();
        let edges = |edge: usize| {
            (0..self.bands.len()).flat_map(move |index| {
                let (start, end) = self.edges(index);
                [start[edge], end[edge]]
            })
        };
        if edges(0).all(|low| low >= -snap) {
            Some(1.0)
        } else if edges(1).all(|high| high <= snap) {
            Some(-1.0)
        } else {
            None
        }
    }

    /// Whether the section is one area: every band of some length, its low
    /// edge below its high one wherever two bands meet — the two may meet
    /// only at the section's own ends, at a point or a knife's edge — each
    /// band overlapping the next by more than the rules can tell from a
    /// pinch, at most one hole to a band, strictly inside its edges, either
    /// every band on one side of the axis or every band across it at both of
    /// its ends, a hole then on one side, and no corner on the axis with
    /// both of its runs leaving it, which pinches the turn there.
    pub fn is_solid(&self) -> bool {
        let last = self.bands.len().saturating_sub(1);
        let banded = !self.bands.is_empty()
            && (self.sloping_to.is_empty() || self.sloping_to.len() == self.bands.len())
            && (0..self.bands.len()).all(|index| {
                let ([low, high], [low_end, high_end]) = self.edges(index);
                self.bands[index][0] > 0.0
                    && low <= high
                    && low_end <= high_end
                    && (low < high || low_end < high_end)
                    && (low < high || index == 0)
                    && (low_end < high_end || index == last)
            });
        if !banded {
            return false;
        }
        let reach = self.reach();
        let ends = self.ends();
        let overlapping = (1..self.bands.len()).all(|index| {
            let ((_, [low, high]), ([next_low, next_high], _)) =
                (self.edges(index - 1), self.edges(index));
            high.min(next_high) - low.max(next_low) > 5.0 * NEAR * reach
        });
        let snap = self.snap();
        let across = (0..self.bands.len()).all(|index| {
            let (start, end) = self.edges(index);
            [start, end]
                .iter()
                .all(|[low, high]| *low < -snap && *high > snap)
        });
        let holed = self.holes.iter().all(|[low, high]| {
            let inside = (0..self.bands.len()).filter(|&index| {
                ends[index] < low.x
                    && high.x < ends[index + 1]
                    && [low.x, high.x].iter().all(|&along| {
                        let [floor, ceiling] = self.at(index, along);
                        floor < low.y && high.y < ceiling
                    })
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
        overlapping
            && holed
            && one_to_a_band
            && (self.side().is_some() || across)
            && !self.is_pinched()
    }

    /// Whether a corner of the outline stands on the axis with neither of
    /// its runs lying on it.
    fn is_pinched(&self) -> bool {
        let snap = self.snap();
        let (outline, _) = self.corners();
        let on = |corner: DVec2| corner.y.abs() <= snap;
        (0..outline.len()).any(|index| {
            let before = outline[(index + outline.len() - 1) % outline.len()];
            let after = outline[(index + 1) % outline.len()];
            on(outline[index]) && !on(before) && !on(after)
        })
    }

    /// The section as the one-sided sections the kernels are handed: itself
    /// when it lies on one side of its axis, its two sides cut along the axis
    /// when it lies across, the left one first.
    pub fn sides(&self) -> Vec<Section> {
        if self.side().is_some() {
            return vec![self.clone()];
        }
        let side = |left: bool| {
            let cut = |[low, high]: [f64; 2]| if left { [0.0, high] } else { [low, 0.0] };
            Section {
                from: self.from,
                bands: self
                    .bands
                    .iter()
                    .map(|&[length, low, high]| {
                        let [low, high] = cut([low, high]);
                        [length, low, high]
                    })
                    .collect(),
                sloping_to: self.sloping_to.iter().map(|end| cut(*end)).collect(),
                holes: self
                    .holes
                    .iter()
                    .filter(|[low, _]| (low.y > 0.0) == left)
                    .copied()
                    .collect(),
            }
            .canonical()
        };
        vec![side(true), side(false)]
    }

    /// The section cut into trapezoids that overlap nowhere: a band is one,
    /// a band with a hole the four around the hole, its edges read where the
    /// hole starts and ends.
    pub fn trapezoids(&self) -> Vec<Trapezoid> {
        let ends = self.ends();
        let mut trapezoids = Vec::new();
        for index in 0..self.bands.len() {
            let (start, end) = (ends[index], ends[index + 1]);
            let between = |from: f64, to: f64| {
                let ([low, high], [low_end, high_end]) = (self.at(index, from), self.at(index, to));
                Trapezoid {
                    along: [from, to],
                    low: [low, low_end],
                    high: [high, high_end],
                }
            };
            match self
                .holes
                .iter()
                .find(|[hole, _]| start < hole.x && hole.x < end)
            {
                Some([hole_low, hole_high]) => {
                    let around = between(hole_low.x, hole_high.x);
                    trapezoids.extend([
                        between(start, hole_low.x),
                        between(hole_high.x, end),
                        Trapezoid {
                            high: [hole_low.y; 2],
                            ..around
                        },
                        Trapezoid {
                            low: [hole_high.y; 2],
                            ..around
                        },
                    ]);
                }
                None => trapezoids.push(between(start, end)),
            }
        }
        trapezoids
    }

    /// The section's trapezoids as the arithmetic turns them: each edge
    /// within a hair of the axis laid on it, and a trapezoid across the axis
    /// cut in two along it.
    pub fn pieces(&self) -> Vec<Piece> {
        let snap = self.snap();
        let snapped = |s: f64| if s.abs() <= snap { 0.0 } else { s };
        let mut pieces = Vec::new();
        for Trapezoid { along, low, high } in self.trapezoids() {
            let (low, high) = (low.map(snapped), high.map(snapped));
            if high[0] > 0.0 || high[1] > 0.0 {
                pieces.push(Piece {
                    along,
                    away: [low[0].max(0.0), high[0]],
                    ending: [low[1].max(0.0), high[1]],
                    side: 1.0,
                });
            }
            if low[0] < 0.0 || low[1] < 0.0 {
                pieces.push(Piece {
                    along,
                    away: [(-high[0]).max(0.0), -low[0]],
                    ending: [(-high[1]).max(0.0), -low[1]],
                    side: -1.0,
                });
            }
        }
        pieces
    }
}
