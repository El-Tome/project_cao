//! The traits, the cuts and the arcs of a drawing as they are drawn: each
//! trait and arc from where the cuts on its ends leave it, and the curve of
//! every cut.

use glam::DVec2;

use super::curve::Curve;
use super::vertex_for;
use crate::corner_cut::{CutCurve, PulledEnds};
use crate::naming::CurveId;
use crate::regions::Region;
use crate::sketch::{PointId, Sketch};

/// Lays every straight trait drawn as the outline, then the curve of every cut
/// that fits and is drawn as the outline, then every arc drawn as the
/// outline. A cut that does not fit, or one in construction, leaves its corner
/// sharp: its sides are laid whole for it.
///
/// `vertex` is where each of the drawing's own points stands. An end a cut
/// pulled back stands on a place of its own, found or added in `places`.
/// Hands back where the cuts pull the ends of each side to, which the
/// stretches of ellipses are laid from in their turn.
pub(super) fn traits_and_cuts(
    sketch: &Sketch,
    vertex: &[usize],
    places: &mut Vec<DVec2>,
    curves: &mut Vec<Curve>,
    names: &mut Vec<CurveId>,
) -> PulledEnds {
    let drawn = sketch.drawn_cuts();
    let pulled = sketch.pulled_ends(&drawn);
    for (id, segment) in sketch.live_segments().filter(|(_, it)| !it.construction) {
        let Some(ends) = sketch.visible_ends(id, &pulled) else {
            continue;
        };
        let from = end_at(vertex, places, segment.start, ends[0]);
        let to = end_at(vertex, places, segment.end, ends[1]);
        curves.push(Curve::Straight { from, to });
        names.push(CurveId::Segment(id));
    }
    for cut in drawn.iter().filter(|cut| cut.fits && !cut.construction) {
        curves.push(match cut.curve {
            CutCurve::Arc(arc) => Curve::Bent {
                centre: arc.centre,
                from: vertex_for(places, arc.start),
                to: vertex_for(places, arc.end),
            },
            CutCurve::Straight { from, to } => Curve::Straight {
                from: vertex_for(places, from),
                to: vertex_for(places, to),
            },
        });
        names.push(CurveId::Corner(cut.id));
    }
    for (id, arc) in sketch.live_arcs().filter(|(_, it)| !it.construction) {
        if sketch.arc_stretches(id, &pulled).shows.is_none() {
            continue;
        }
        let ends = pulled.of(id.into());
        let from = end_at(vertex, places, arc.start, ends[0]);
        let to = end_at(vertex, places, arc.end, ends[1]);
        curves.push(Curve::Bent {
            centre: sketch.point(arc.center),
            from,
            to,
        });
        names.push(CurveId::Arc(id));
    }
    pulled
}

/// The vertex an end of a side stands on: its own point's, or the place a cut
/// pulled it back to.
fn end_at(
    vertex: &[usize],
    places: &mut Vec<DVec2>,
    point: PointId,
    pulled: Option<DVec2>,
) -> usize {
    match pulled {
        Some(place) => vertex_for(places, place),
        None => vertex[point.0],
    }
}

impl Sketch {
    /// Names a side two cuts left no straight piece of along the curves of
    /// those cuts, in every area they bound: the side is still a border of
    /// the area, run along by them in its place. A rectangle rounded into an
    /// obround keeps the name its short sides gave it.
    pub(crate) fn name_the_sides_cuts_took(&self, regions: &mut [Region]) {
        let drawn = self.drawn_cuts();
        let pulled = self.pulled_ends(&drawn);
        let taken: Vec<(CurveId, CurveId)> = drawn
            .iter()
            .filter(|cut| cut.fits && !cut.construction)
            .flat_map(|cut| {
                cut.sides
                    .iter()
                    .filter(|side| self.what_shows(**side, &pulled).is_none())
                    .map(move |side| (CurveId::Corner(cut.id), CurveId::from(*side)))
            })
            .collect();
        if taken.is_empty() {
            return;
        }
        for region in regions {
            let outlines = std::iter::once(&mut region.outline).chain(region.holes.iter_mut());
            for border in outlines.flat_map(|outline| outline.bounds.iter_mut()) {
                for (cut, side) in &taken {
                    if border.contains(cut) && !border.contains(side) {
                        border.push(*side);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests;
