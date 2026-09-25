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
    ArcId, Area, Became, Chamfer, CircleId, Corner, CurveId, DimensionTarget, EllipseId, PointId,
    SegmentId, Sketch, Standing,
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
    ) -> Option<Outcome> {
        let Some(mode) = asked.worked_out(&self.values) else {
            self.broke(Broken::Operation(self.replaying));
            return None;
        };
        let notes = asked.as_laid().map(Formula::note);
        self.every_corner(sketch, corners, &notes, |drawing, scale, first, second| {
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
        asked: &Formula,
    ) -> Option<Outcome> {
        let radius = self.size(asked, |_| true)?;
        self.every_corner(
            sketch,
            corners,
            &[asked.note()],
            |drawing, scale, first, second| {
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
            },
        )
    }

    /// The two traits a corner stands between, as the drawing has them now and
    /// in the order the gesture named them.
    ///
    /// A corner named by its two traits keeps the first one, because the first
    /// distance is measured along it — and that trait is followed through
    /// every cut since, rather than trusted as a number. Two corners of one
    /// shape share a trait: cutting the first replaces it, and the second would
    /// otherwise measure from a curve that is gone.
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
        notes: &[Option<String>],
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
            refused: 0,
        };
        let refused = Outcome::Cut {
            rules: 0,
            values: 0,
            refused: 1,
        };
        for corner in corners {
            let Some((first, second)) = self.sides_now(sketch, *corner) else {
                total = total.and(refused);
                continue;
            };
            let mut typed = Vec::new();
            let Some(made) = self.cutting(sketch, |drawing, scale| {
                let (made, wanted) = cut(drawing, scale, first, second)?;
                typed = wanted;
                Some(made)
            }) else {
                total = total.and(refused);
                continue;
            };
            self.write_down(sketch, typed, notes);
            total = total.and(made);
        }
        if matches!(total, Outcome::Cut { refused, .. } if refused > 0) {
            self.broke(Broken::Operation(self.replaying));
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
    ///
    /// Each carries what it was written from, in the order the cut laid them.
    fn write_down(
        &mut self,
        sketch: usize,
        typed: Vec<(DimensionTarget, f64)>,
        notes: &[Option<String>],
    ) {
        for (rank, (target, value)) in typed.into_iter().enumerate() {
            let target = target.normalised();
            let outcome = self.apply_dimension(sketch, target, value);
            self.remember_as(
                sketch,
                target,
                outcome,
                notes.get(rank).cloned().flatten(),
                None,
            );
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
