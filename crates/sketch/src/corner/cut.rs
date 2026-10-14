//! A corner a chamfer or a fillet cut, read back from what the cut left: the
//! corner's own point, held on the line of each side, and the cut running
//! between the ends of the two sides.
//!
//! The pieces left of the sides are what is drawn, but a side of the corner
//! still runs from the corner out to its free end: the end at the cut is where
//! the cut starts, and moves along the side whenever the corner opens or
//! closes. A drag of a free end and an angle typed between the sides turn a
//! side about the corner for that reason (#565).

use crate::arc::ArcId;
use crate::holding::Support;
use crate::sketch::{PointId, SegmentId, Sketch};

/// A corner a cut took away, as the drawing has it now.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct CutCorner {
    /// The point left where the corner was.
    pub(crate) point: PointId,
    pub(crate) sides: [CutSide; 2],
}

/// What is left of one side of a cut corner.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct CutSide {
    /// The trait still drawn.
    pub(crate) piece: SegmentId,
    /// Its end at the cut.
    pub(crate) near: PointId,
    /// Its free end, the one the side runs out to.
    pub(crate) far: PointId,
}

/// What turning one side of a cut corner about it, against the other side,
/// leaves to the drawing and what it does not.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SideTurned {
    pub(crate) corner: CutCorner,
    /// What holds the side that stays: the corner and its free end. Its end at
    /// the cut slides along it as the corner opens or closes.
    pub(crate) stays: [PointId; 2],
    /// What the cut laid between the sides, past the end of the side turned:
    /// the other side's end at the cut and a fillet's centre. Neither side's,
    /// they are mended once the side has turned.
    pub(crate) cut: Vec<PointId>,
    /// The free end of the side turned, which the turn puts where it belongs:
    /// left free, the mending of the cut slides it along the side.
    pub(crate) free_end: PointId,
}

impl Sketch {
    /// Every corner a cut left in the drawing, whole: each side's end at the
    /// cut between the corner and its free end.
    pub(crate) fn cut_corners(&self) -> Vec<CutCorner> {
        let mut holds: Vec<Vec<Support>> = vec![Vec::new(); self.points().len()];
        for (point, support) in self
            .constraints()
            .iter()
            .filter_map(|rule| Support::held_by(*rule))
        {
            if let Some(on) = holds.get_mut(point.0) {
                on.push(support);
            }
        }
        self.live_points()
            .filter_map(|(point, _)| self.cut_corner_held(point, &holds[point.0]))
            .filter(|corner| self.is_whole(corner))
            .collect()
    }

    /// One side of a cut corner turned against the other, `first` staying: the
    /// corner the two are the sides of, each named by its piece or by the
    /// stretch the cut laid back in construction between the corner and that
    /// piece. Nothing when they are not the two sides of one cut corner.
    pub(crate) fn side_turned(&self, first: SegmentId, second: SegmentId) -> Option<SideTurned> {
        self.cut_corners().into_iter().find_map(|corner| {
            let one = self.side_named(&corner, first)?;
            let other = self.side_named(&corner, second)?;
            if one == other {
                return None;
            }
            let mut cut = vec![corner.sides[one].near];
            cut.extend(self.cut_curve(&corner).map(|arc| self.arc(arc).center));
            Some(SideTurned {
                corner,
                stays: [corner.point, corner.sides[one].far],
                cut,
                free_end: corner.sides[other].far,
            })
        })
    }

    /// Lays a corner's cut again for the way its sides run out now, as the cut
    /// itself lays it: a fillet's curve of the radius it has, grazing both
    /// sides, or a chamfer's ends as far back along each side as they stand.
    /// Where the cut no longer fits the sides, nothing is moved.
    ///
    /// What is left to mend is then within the solver's precision. Left to the
    /// solver, a corner opened or closed far in one go comes out with its cut
    /// laid in the quarter across the corner, the sides running past it.
    pub(crate) fn lay_cut_again(&mut self, corner: &CutCorner) {
        let at = self.point(corner.point);
        let [Some(one), Some(other)] = corner
            .sides
            .map(|side| (self.point(side.far) - at).try_normalize())
        else {
            return;
        };
        let reaches = corner.sides.map(|side| self.point(side.far).distance(at));
        let (backs, centre) = match self.cut_curve(corner) {
            Some(arc) => {
                let half = one.angle_to(other).abs() / 2.0;
                let radius = self.arc_radius(arc);
                let Some(inwards) = (one + other).try_normalize() else {
                    return;
                };
                let centre = (self.arc(arc).center, at + inwards * (radius / half.sin()));
                ([radius / half.tan(); 2], Some(centre))
            }
            None => (
                corner.sides.map(|side| self.point(side.near).distance(at)),
                None,
            ),
        };
        if backs
            .iter()
            .zip(reaches)
            .any(|(back, reach)| !back.is_finite() || *back >= reach)
        {
            return;
        }
        if let Some((centre, place)) = centre {
            self.move_point(centre, place);
        }
        for ((side, way), back) in corner.sides.iter().zip([one, other]).zip(backs) {
            self.move_point(side.near, at + way * back);
        }
    }

    /// The curve a fillet laid across a cut corner, running from the end of one
    /// side to the end of the other. Nothing for a chamfer, whose cut is a
    /// trait.
    pub(crate) fn cut_curve(&self, corner: &CutCorner) -> Option<ArcId> {
        let ends = corner.sides.map(|side| side.near);
        self.live_arcs()
            .find(|(_, arc)| ends.contains(&arc.start) && ends.contains(&arc.end))
            .map(|(id, _)| id)
    }

    /// Whether every one of `corners` is still whole. A settling that brings a
    /// side out past its corner has turned the cut inside out.
    pub(crate) fn still_cut(&self, corners: &[CutCorner]) -> bool {
        corners.iter().all(|corner| self.is_whole(corner))
    }

    /// Whether each side's end at the cut lies between the corner and the
    /// side's free end.
    fn is_whole(&self, corner: &CutCorner) -> bool {
        let at = self.point(corner.point);
        corner.sides.iter().all(|side| {
            let (near, far) = (self.point(side.near) - at, self.point(side.far) - at);
            near.dot(far) > 0.0 && near.length() < far.length()
        })
    }

    /// Which side of a corner a trait names: the piece itself, or the stretch
    /// between the corner and the piece's end at the cut.
    fn side_named(&self, corner: &CutCorner, segment: SegmentId) -> Option<usize> {
        let line = self.segments().get(segment.0)?;
        let ends = [line.start, line.end];
        corner.sides.iter().position(|side| {
            side.piece == segment || (ends.contains(&corner.point) && ends.contains(&side.near))
        })
    }

    /// The corner a point stands for, held as `holds` say: on two traits and
    /// on nothing else, an end of neither, with a cut — a curve, or a trait —
    /// running from an end of one to an end of the other. Read from what is
    /// joined to what and not from where it lies, so that a corner a gesture
    /// turned inside out is still recognised as the one it was.
    ///
    /// A point laid where two lines meet, with nothing cut between them, is no
    /// corner a cut left.
    fn cut_corner_held(&self, point: PointId, holds: &[Support]) -> Option<CutCorner> {
        let [Support::Segment(one), Support::Segment(other)] = holds[..] else {
            return None;
        };
        if one == other || self.is_erased_segment(one) || self.is_erased_segment(other) {
            return None;
        }
        let (first, second) = (*self.segments().get(one.0)?, *self.segments().get(other.0)?);
        if [first.start, first.end, second.start, second.end].contains(&point) {
            return None;
        }
        for (near_one, far_one) in [(first.start, first.end), (first.end, first.start)] {
            for (near_other, far_other) in [(second.start, second.end), (second.end, second.start)]
            {
                if self.cut_runs_between(near_one, near_other) {
                    return Some(CutCorner {
                        point,
                        sides: [
                            CutSide {
                                piece: one,
                                near: near_one,
                                far: far_one,
                            },
                            CutSide {
                                piece: other,
                                near: near_other,
                                far: far_other,
                            },
                        ],
                    });
                }
            }
        }
        None
    }

    /// Whether a curve or a trait drawn runs between two points: what a fillet
    /// or a chamfer lays across a corner.
    fn cut_runs_between(&self, one: PointId, other: PointId) -> bool {
        let between = |start: PointId, end: PointId| {
            (start == one && end == other) || (start == other && end == one)
        };
        self.live_arcs().any(|(_, arc)| between(arc.start, arc.end))
            || self
                .live_segments()
                .any(|(_, line)| !line.construction && between(line.start, line.end))
    }
}

#[cfg(test)]
mod tests;
