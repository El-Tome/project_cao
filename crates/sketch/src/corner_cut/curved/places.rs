//! A fillet beside a curve as the solver reads it: its centre and where it
//! touches each side, each carried with how it moves with the points the cut
//! stands on.
//!
//! The places are the ones the drawing shows, found by the very search that
//! draws the fillet, in plain numbers. How they move is what no formula
//! gives: where the two offsets meet slides as the corner, the sides or the
//! curve do. One more step of Newton's method from where the search ended,
//! taken over jets, carries it — a step of nought, since the offsets meet
//! there, whose slopes are how far the meeting slides as each point moves.

use glam::DVec2;

use super::path::{JetPath, Run};
use super::{Found, found};
use crate::corner_cut::points::{COORDINATES, CutPlaces};
use crate::corner_cut::{CornerCut, CutShape, CutSide};
use crate::jet::{Jet, JetPlace};
use crate::sketch::{PointId, SegmentId, Sketch};

impl Sketch {
    /// The places of a fillet beside a curve, over the coordinates of
    /// `stands_on` in that order: nothing when its corner gives it no place,
    /// or when it fits nowhere and is drawn where the sides' tangents alone
    /// would put it.
    pub(in crate::corner_cut) fn places_beside_a_curve(
        &self,
        cut: &CornerCut,
        stands_on: Vec<PointId>,
    ) -> Option<CutPlaces> {
        let CutShape::Fillet { radius } = cut.shape else {
            return None;
        };
        if 2 * stands_on.len() + 4 > COORDINATES {
            return None;
        }
        let runs = self.runs_away_from(cut.sides, cut.corner)?;
        let Found { off, at, .. } = found(self.point(cut.corner), runs, radius)?;
        let at = at?;
        let jet = |point: PointId| {
            let rank = stands_on.iter().position(|stand| *stand == point)?;
            Some(JetPlace::free(self.point(point), 2 * rank))
        };
        let paths = [
            self.jet_path(cut.sides[0], cut.corner, &jet)?,
            self.jet_path(cut.sides[1], cut.corner, &jet)?,
        ];
        let at = slid(runs, &paths, off, at)?;
        let centre = paths[0].off_by(at[0], off[0]);
        Some(CutPlaces {
            along: (centre - jet(cut.corner)?).normalized(),
            touches: [paths[0].place(at[0]), paths[1].place(at[1])],
            round: Some((centre, radius)),
            stands_on,
        })
    }

    /// The path a side leaving `corner` lies on, read off the points it is
    /// drawn from as `jet` carries them — as `run_away_from` reads it in
    /// plain numbers.
    fn jet_path(
        &self,
        side: CutSide,
        corner: PointId,
        jet: &impl Fn(PointId) -> Option<JetPlace<COORDINATES>>,
    ) -> Option<JetPath> {
        let at = jet(corner)?;
        Some(match side {
            CutSide::Trait(segment) => JetPath::Line {
                through: at,
                along: (jet(self.far_end(segment, corner))? - at).normalized(),
            },
            CutSide::Arc(arc) => {
                let arc = self.arc(arc);
                let centre = jet(arc.center)?;
                JetPath::Circle {
                    centre,
                    radius: (jet(arc.start)? - centre).length(),
                }
            }
            CutSide::ArcOfEllipse(ellipse) => {
                let oval = *self.ellipses().get(ellipse.0)?;
                let reach = |axis: SegmentId| {
                    let held = *self.segments().get(axis.0)?;
                    let share = self.axis_share(oval.center, axis)?;
                    Some((jet(held.end)? - jet(held.start)?).scaled(Jet::constant(share)))
                };
                JetPath::Ellipse {
                    centre: jet(oval.center)?,
                    first: reach(oval.first)?,
                    second: reach(oval.second)?.length(),
                }
            }
        })
    }
}

/// Where on each side's path the fillet touches, `at`, carried with how far
/// that slides as each point moves: the step Newton's method takes from
/// there, over jets — the meeting's own slopes, the implicit function's —
/// standing on `at` itself rather than on the step's value, which is the
/// search's leftover, so the places stay the ones drawn.
fn slid(
    runs: [Run; 2],
    paths: &[JetPath; 2],
    off: [f64; 2],
    at: [f64; 2],
) -> Option<[Jet<COORDINATES>; 2]> {
    let still = at.map(Jet::constant);
    let gap = paths[1].off_by(still[1], off[1]) - paths[0].off_by(still[0], off[0]);
    let one: DVec2 = runs[0].path.off_by(at[0], off[0]).1;
    let other: DVec2 = -runs[1].path.off_by(at[1], off[1]).1;
    let across = one.perp_dot(other);
    if across == 0.0 {
        return None;
    }
    let step = [
        (gap.x * other.y - gap.y * other.x) * (1.0 / across),
        (gap.y * one.x - gap.x * one.y) * (1.0 / across),
    ];
    Some([0, 1].map(|rank| Jet {
        value: at[rank],
        slope: step[rank].slope,
    }))
}
