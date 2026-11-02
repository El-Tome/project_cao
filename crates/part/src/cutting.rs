//! The six ways a curve of a drawing is replaced by other curves: trimmed,
//! trimmed as an arc, trimmed as a circle, divided at a crossing, chamfered or
//! rounded.
//!
//! They differ only in which cut is made. Each answers with how many rules
//! and how many values the cut took away with it, and with what became of
//! every curve it replaced — which is what carries the name of an area past
//! the cut, so that rounding a corner does not lose what was raised from the
//! shape.

use cao_sketch::{
    ArcId, Area, Became, Chamfer, CircleId, Corner, CurveId, CutShape, DimensionTarget, EllipseId,
    PointId, SegmentId, Sketch, Standing,
};
use glam::DVec2;

use crate::broken::Broken;
use crate::formula::{Formula, Operator};
use crate::history::ChamferAsked;
use crate::outcome::Outcome;
use crate::state::PartState;

use cao_sketch::Region;

/// What one cut left behind.
struct Cut {
    rules: usize,
    values: usize,
    became: Became,
}

impl PartState {
    /// The name as the drawing holds it now, once every cut made since it was
    /// written has been followed.
    ///
    /// Nothing when every curve along one of its borders was cut away
    /// altogether: the area has lost that border, and whatever stood on it
    /// stands on nothing.
    pub(crate) fn standing(&self, sketch: usize, area: &Area) -> Option<Standing> {
        match self.descent.get(&sketch) {
            Some(descent) => descent.follow(area),
            None => Some(area.uncut()),
        }
    }

    /// Which of a drawing's areas a name answers to now: the name followed
    /// through every cut since it was written, then asked of the areas the
    /// drawing encloses.
    ///
    /// The one place that answer is worked out. A panel that asks it a second
    /// way of its own is a panel that reads "fine" while the replay raises
    /// nothing.
    pub(crate) fn area_rank(
        &self,
        sketch: usize,
        area: &Area,
        regions: &[Region],
    ) -> Option<usize> {
        self.standing(sketch, area)?.found_in(regions)
    }

    pub(crate) fn trim(
        &mut self,
        sketch: usize,
        segment: SegmentId,
        from: PointId,
        to: PointId,
    ) -> Option<Outcome> {
        self.cutting(sketch, |drawing, _| {
            let trimmed = drawing.trim(segment, from, to)?;
            Some(Cut {
                rules: trimmed.rules_dropped,
                values: trimmed.values_dropped,
                became: vec![(
                    CurveId::Segment(segment),
                    trimmed.pieces.into_iter().map(CurveId::Segment).collect(),
                )],
            })
        })
    }

    pub(crate) fn trim_arc(
        &mut self,
        sketch: usize,
        arc: ArcId,
        from: PointId,
        to: PointId,
    ) -> Option<Outcome> {
        self.cutting(sketch, |drawing, _| {
            let trimmed = drawing.trim_arc(arc, from, to)?;
            Some(Cut {
                rules: trimmed.rules_dropped,
                values: trimmed.values_dropped,
                became: vec![(
                    CurveId::Arc(arc),
                    trimmed.pieces.into_iter().map(CurveId::Arc).collect(),
                )],
            })
        })
    }

    pub(crate) fn trim_circle(
        &mut self,
        sketch: usize,
        circle: CircleId,
        between: Option<(PointId, PointId)>,
    ) -> Option<Outcome> {
        let diameter = self
            .sketches
            .get(sketch)?
            .dimension_of(DimensionTarget::Diameter(circle))
            .and_then(|value| value.written.as_deref())
            .and_then(Formula::from_stored);
        let mut left = None;
        let outcome = self.cutting(sketch, |drawing, _| {
            let trimmed = drawing.trim_circle(circle, between)?;
            left = trimmed.arc;
            Some(Cut {
                rules: trimmed.rules_dropped,
                values: trimmed.values_dropped,
                became: vec![(
                    CurveId::Circle(circle),
                    trimmed.arc.map(CurveId::Arc).into_iter().collect(),
                )],
            })
        });
        // A diameter goes over as the radius of the arc left, half the number
        // it was. The drawing carries the number and cannot halve a formula it
        // never reads, so what it was written from is halved here.
        if let (Some(arc), Some(diameter), Some(drawing)) =
            (left, diameter, self.sketches.get_mut(sketch))
        {
            let halved = Formula::Combined(
                Operator::Divide,
                Box::new(diameter),
                Box::new(Formula::Number(2.0)),
            );
            drawing.write_dimension_as(DimensionTarget::ArcRadius(arc), halved.note());
        }
        outcome
    }

    pub(crate) fn trim_ellipse(
        &mut self,
        sketch: usize,
        ellipse: EllipseId,
        between: Option<(PointId, PointId)>,
    ) -> Option<Outcome> {
        self.cutting(sketch, |drawing, _| {
            let trimmed = drawing.trim_ellipse(ellipse, between)?;
            Some(Cut {
                // Nothing is handed over and nothing is dropped: what a cut
                // leaves of an ellipse is the very ellipse, so every rule and
                // every value it carried still speaks of it.
                rules: 0,
                values: 0,
                became: vec![(
                    CurveId::Ellipse(ellipse),
                    trimmed.pieces.into_iter().map(CurveId::Ellipse).collect(),
                )],
            })
        })
    }

    pub(crate) fn split(
        &mut self,
        sketch: usize,
        segments: &[SegmentId],
        arcs: &[ArcId],
        at: DVec2,
    ) -> Option<Outcome> {
        self.cutting(sketch, |drawing, _| {
            let split = drawing.split(segments, arcs, at)?;
            Some(Cut {
                rules: split.rules_dropped,
                values: split.values_dropped,
                became: split.became,
            })
        })
    }

    pub(crate) fn chamfer(
        &mut self,
        sketch: usize,
        corners: &[Corner],
        asked: &ChamferAsked,
        construction: bool,
    ) -> Option<Outcome> {
        let Some(mode) = asked.worked_out(&self.values) else {
            self.broke(Broken::Operation(self.replaying));
            return None;
        };
        self.lay_cuts(sketch, corners, construction, |scale| {
            CutShape::Chamfer(in_units(mode, scale))
        })
    }

    pub(crate) fn fillet(
        &mut self,
        sketch: usize,
        corners: &[Corner],
        asked: &Formula,
        construction: bool,
    ) -> Option<Outcome> {
        let radius = self.size(asked, |_| true)?;
        self.lay_cuts(sketch, corners, construction, |scale| CutShape::Fillet {
            radius: radius / scale,
        })
    }

    /// The two traits a corner stands between, as the drawing has them now and
    /// in the order the gesture named them.
    ///
    /// A corner named by its two traits keeps the first one, because the first
    /// distance is measured along it — and that trait is followed through
    /// every trim or division since, rather than trusted as a number.
    fn sides_now(&self, sketch: usize, corner: Corner) -> Option<(SegmentId, SegmentId)> {
        let drawing = self.sketches.get(sketch)?;
        let Corner::Between(first, second) = corner else {
            return drawing.sides_of(corner);
        };
        for a in self.now(sketch, first) {
            for b in self.now(sketch, second) {
                if drawing.shared_point(a, b).is_some() {
                    return Some((a, b));
                }
            }
        }
        None
    }

    /// What a trait has become, as the drawing holds it now. Itself when no cut
    /// has touched it.
    fn now(&self, sketch: usize, side: SegmentId) -> Vec<SegmentId> {
        let Some(descent) = self.descent.get(&sketch) else {
            return vec![side];
        };
        descent
            .descendants(CurveId::Segment(side))
            .into_iter()
            .filter_map(|curve| match curve {
                CurveId::Segment(piece) => Some(piece),
                _ => None,
            })
            .collect()
    }

    /// Cuts every corner the gesture named, with the same values, as one group.
    ///
    /// Nothing of the drawing is trimmed: each corner only says what is drawn
    /// in its place, so a value too big for a corner is laid all the same and
    /// drawn as one that does not fit. A corner refused is one that is not
    /// there to cut, or one already cut, and refusing it breaks the step.
    ///
    /// The first value a part is ever given is what fixes what its drawing is
    /// worth in millimetres, and a cut is as good a first value as any: its
    /// values were turned into the drawing's units at one millimetre a unit,
    /// so that is the scale it fixes.
    fn lay_cuts(
        &mut self,
        sketch: usize,
        corners: &[Corner],
        construction: bool,
        shape: impl Fn(f64) -> CutShape,
    ) -> Option<Outcome> {
        let scale = self.scale();
        let sides: Vec<Option<(SegmentId, SegmentId)>> = corners
            .iter()
            .map(|corner| self.sides_now(sketch, *corner))
            .collect();
        let drawing = self.sketches.get_mut(sketch)?;
        let group = drawing.fresh_cut_group();
        let mut refused = 0;
        for named in sides {
            let laid = named.and_then(|(first, second)| {
                drawing
                    .cut_corner(first, second, shape(scale), construction, group)
                    .ok()
            });
            if laid.is_none() {
                refused += 1;
            }
        }
        if !self.has_scale() && refused < corners.len() {
            self.millimeters_per_unit = Some(scale);
        }
        if refused > 0 {
            self.broke(Broken::Operation(self.replaying));
        }
        Some(Outcome::Cut {
            rules: 0,
            values: 0,
            refused,
        })
    }

    fn cutting(
        &mut self,
        sketch: usize,
        cut: impl FnOnce(&mut Sketch, f64) -> Option<Cut>,
    ) -> Option<Outcome> {
        let scale = self.scale();
        let drawing = self.sketches.get_mut(sketch)?;
        let made = cut(drawing, scale)?;
        drawing.resolve(scale);
        let descent = self.descent.entry(sketch).or_default();
        for (was, now) in made.became {
            descent.record(was, now);
        }
        Some(Outcome::Cut {
            rules: made.rules,
            values: made.values,
            refused: 0,
        })
    }
}

/// A chamfer as the drawing measures it. The history records millimetres, the
/// way every other length the user types is recorded; the sketch works in its
/// own units, and only the distances convert.
fn in_units(mode: Chamfer, millimeters_per_unit: f64) -> Chamfer {
    match mode {
        Chamfer::Equal(reach) => Chamfer::Equal(reach / millimeters_per_unit),
        Chamfer::Sided { first, second } => Chamfer::Sided {
            first: first / millimeters_per_unit,
            second: second / millimeters_per_unit,
        },
        Chamfer::Angled { along, degrees } => Chamfer::Angled {
            along: along / millimeters_per_unit,
            degrees,
        },
    }
}
