//! The five ways a curve of a drawing is replaced by other curves: trimmed,
//! trimmed as an arc, divided at a crossing, chamfered or rounded.
//!
//! They differ only in which cut is made. Each answers with how many rules
//! and how many values the cut took away with it, and with what became of
//! every curve it replaced — which is what carries the name of an area past
//! the cut, so that rounding a corner does not lose what was raised from the
//! shape.

use cao_sketch::{
    ArcId, Area, Became, Chamfer, Corner, CurveId, DimensionTarget, PointId, SegmentId, Sketch,
    Standing,
};
use glam::DVec2;

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
    /// Nothing when one of the curves it names was cut away altogether: the
    /// area has lost a border, and whatever stood on it stands on nothing.
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
        mode: Chamfer,
    ) -> Option<Outcome> {
        self.every_corner(sketch, corners, |drawing, scale, first, second| {
            let chamfered = drawing.chamfer(first, second, in_units(mode, scale))?;
            let typed = chamfered.typed(mode);
            Some((
                Cut {
                    rules: chamfered.rules_dropped,
                    values: chamfered.values_dropped,
                    became: chamfered.became,
                },
                typed,
            ))
        })
    }

    pub(crate) fn fillet(
        &mut self,
        sketch: usize,
        corners: &[Corner],
        radius: f64,
    ) -> Option<Outcome> {
        self.every_corner(sketch, corners, |drawing, scale, first, second| {
            let rounded = drawing.fillet(first, second, radius / scale)?;
            let typed = rounded.typed(radius);
            Some((
                Cut {
                    rules: rounded.rules_dropped,
                    values: rounded.values_dropped,
                    became: rounded.became,
                },
                typed,
            ))
        })
    }

    /// Cuts every corner the gesture named, with the same values, as one
    /// operation.
    ///
    /// Each corner is read against the drawing as it stands when its own turn
    /// comes, not as it stood when the gesture was made: cutting one corner of
    /// a shape trims the traits its neighbours lean on, and the four corners of
    /// a plate all share theirs. That is what [`Corner`] records a point for.
    fn every_corner(
        &mut self,
        sketch: usize,
        corners: &[Corner],
        cut: impl Fn(
            &mut Sketch,
            f64,
            SegmentId,
            SegmentId,
        ) -> Option<(Cut, Vec<(DimensionTarget, f64)>)>,
    ) -> Option<Outcome> {
        let mut total = Outcome::Cut {
            rules: 0,
            values: 0,
        };
        for corner in corners {
            let Some((first, second)) = self.sketches.get(sketch)?.sides_of(*corner) else {
                continue;
            };
            let mut typed = Vec::new();
            let Some(made) = self.cutting(sketch, |drawing, scale| {
                let (made, wanted) = cut(drawing, scale, first, second)?;
                typed = wanted;
                Some(made)
            }) else {
                continue;
            };
            self.write_down(sketch, typed);
            total = total.and(made);
        }
        Some(total)
    }

    /// Records the values a cut was given, the same way any other typed length
    /// is recorded.
    ///
    /// Through `apply_dimension` rather than straight onto the drawing: the
    /// first value a part is ever given is what fixes what its drawing is worth
    /// in millimetres, and a chamfer is as good a first value as any. Writing
    /// it directly left a part whose scale was still unset while the dimension
    /// sat there — and compaction, which replays every dimension as an
    /// operation, then rebuilt a part that measured differently.
    fn write_down(&mut self, sketch: usize, typed: Vec<(DimensionTarget, f64)>) {
        for (target, value) in typed {
            self.apply_dimension(sketch, target.normalised(), value);
        }
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
