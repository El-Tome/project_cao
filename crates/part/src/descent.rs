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

use cao_sketch::{Area, CurveId};
use serde::{Deserialize, Serialize};

/// Which curves stand where each curve the replay has cut used to.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Descent {
    became: BTreeMap<CurveId, Vec<CurveId>>,
}

impl Descent {
    /// Records that a cut took `was` out and left `now` in its place.
    ///
    /// Anything that already descended to `was` descends past it, so a trait
    /// cut twice is followed in one step rather than chased down a chain.
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
    /// A curve replaced by pieces of itself is the same border under another
    /// number. One replaced by nothing is a border the drawing has lost, and
    /// dropping it from the name would quietly hand the area over to whatever
    /// larger area swallowed it.
    pub(crate) fn follow(&self, area: &Area) -> Option<Area> {
        let mut bounds: Vec<CurveId> = Vec::with_capacity(area.bounds.len());
        for curve in &area.bounds {
            match self.became.get(curve) {
                None => bounds.push(*curve),
                Some(now) if now.is_empty() => return None,
                Some(now) => bounds.extend(now.iter().copied()),
            }
        }
        bounds.sort_unstable();
        bounds.dedup();
        Some(Area {
            bounds,
            inside: area.inside,
        })
    }
}

#[cfg(test)]
mod tests;
