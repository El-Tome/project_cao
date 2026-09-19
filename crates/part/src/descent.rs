//! What a drawing's curves became, as the replay cuts them.
//!
//! An extrusion names the area it stands on by the curves that bounded it
//! when it was clicked. Five operations replace a curve with other curves —
//! trimming, trimming an arc, dividing at a crossing, chamfering, rounding —
//! and a name written before one of them speaks of a curve the drawing no
//! longer has.
//!
//! Without this, chamfering one corner of an extruded rectangle would lose
//! the extrusion, though nothing meaningful moved. A warning that fires when
//! nothing is wrong is a warning that gets ignored, and the tree that follows
//! needs this one to be believed.
//!
//! It is kept for the length of a replay and never written down: the history
//! says what was cut, so replaying it says the same thing again.

use std::collections::BTreeMap;

use cao_sketch::{Area, CurveId, Standing};

/// Which curves stand where each curve the replay has cut used to.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Descent {
    became: BTreeMap<CurveId, Vec<CurveId>>,
}

impl Descent {
    /// Records that a cut took `was` out and left `now` in its place.
    ///
    /// Anything that already descended to `was` descends past it, so a trait
    /// cut twice is followed in one step rather than chased down a chain.
    ///
    /// That rewrite only catches because cuts are recorded in the order they
    /// are replayed: a piece can only be cut after the curve it came out of
    /// was. Recorded out of order, a stale number would be left in the name.
    pub(crate) fn record(&mut self, was: CurveId, now: Vec<CurveId>) {
        for descendants in self.became.values_mut() {
            if descendants.contains(&was) {
                *descendants = descendants
                    .iter()
                    .flat_map(|curve| match *curve == was {
                        true => now.clone(),
                        false => vec![*curve],
                    })
                    .collect();
            }
        }
        self.became.insert(was, now);
    }

    /// The name as the drawing holds it now, and nothing when one of the
    /// curves it is bounded by was cut away altogether.
    ///
    /// A curve replaced by pieces of itself is the same border under other
    /// numbers, and stays **one** border: the area has to be bounded by one
    /// of those pieces, not by all of them, since dividing a trait two areas
    /// share puts one piece on each side. One replaced by nothing is a border
    /// the drawing has lost, and dropping it from the name would quietly hand
    /// the area over to whatever larger area swallowed it.
    pub(crate) fn follow(&self, area: &Area) -> Option<Standing> {
        let mut borders: Vec<Vec<CurveId>> = Vec::with_capacity(area.bounds.len());
        for curve in &area.bounds {
            match self.became.get(curve) {
                None => borders.push(vec![*curve]),
                Some(now) if now.is_empty() => return None,
                Some(now) => borders.push(now.clone()),
            }
        }
        Some(Standing::new(borders, area.inside))
    }
}

#[cfg(test)]
mod tests;
