//! Every segment and arc still drawn, as the half-edges a face walk can turn
//! at — cut apart wherever two of them cross.
//!
//! The walk reads nothing but the vertices and the angular order of the edges
//! leaving each one, so two edges meeting in space with no vertex of their own
//! are invisible to it. Giving that meeting a vertex here is what lets the walk
//! stay as it is and still find the areas a bowtie bounds.

use glam::DVec2;

use crate::arcing::{ArcDraft, places_along, sweep_of};
use crate::crossing::{where_arcs_cross, where_segment_crosses_arc, where_segments_cross};
use crate::sketch::Sketch;

/// One end of one arc, as a graph half-edge: where it leaves from, the tangent
/// it leaves along — not the straight line to its far end, which is what tells
/// the region walk apart from a plain segment's — and which way round the curve
/// it walks.
pub(crate) struct ArcHalfEdge {
    pub(crate) center: DVec2,
    /// Whether this half leaves the piece's own start, curving the way it was
    /// drawn, or leaves its end and so walks the same curve backwards.
    pub(crate) forward: bool,
}

impl ArcHalfEdge {
    /// The direction it leaves `from` in: perpendicular to the reach from the
    /// centre, turned the way the curve actually bends at that end.
    pub(crate) fn departure(&self, from: DVec2) -> DVec2 {
        let reach = from - self.center;
        match self.forward {
            true => DVec2::new(-reach.y, reach.x),
            false => DVec2::new(reach.y, -reach.x),
        }
    }

    /// The curve this half contributes to an outline: sampled from its own
    /// `from` up to, but not including, `to` — the same convention a segment's
    /// single point already follows, so the next half-edge, or the walk
    /// closing, supplies the rest.
    pub(crate) fn points_along(&self, from: DVec2, to: DVec2) -> Vec<DVec2> {
        let (start, end) = match self.forward {
            true => (from, to),
            false => (to, from),
        };
        let mut sampled = places_along(ArcDraft {
            centre: self.center,
            start,
            end,
        });
        if !self.forward {
            sampled.reverse();
        }
        sampled.pop();
        sampled
    }
}

/// The drawing as a graph with every crossing standing on a vertex of its own.
///
/// `places` is the drawing's own points, then one more for each crossing.
/// `ends` pairs every half-edge with its twin next to it, the straight ones
/// first; `split` is where the curved ones start, and `arcs` holds one entry
/// for each of those.
pub(crate) struct Crossed {
    pub(crate) places: Vec<DVec2>,
    pub(crate) ends: Vec<(usize, usize)>,
    pub(crate) split: usize,
    pub(crate) arcs: Vec<ArcHalfEdge>,
}

enum Curve {
    Straight {
        from: usize,
        to: usize,
    },
    Bent {
        centre: DVec2,
        from: usize,
        to: usize,
    },
}

impl Curve {
    fn ends(&self) -> (usize, usize) {
        match self {
            Curve::Straight { from, to } | Curve::Bent { from, to, .. } => (*from, *to),
        }
    }

    fn draft(&self, places: &[DVec2]) -> Option<ArcDraft> {
        match self {
            Curve::Straight { .. } => None,
            Curve::Bent { centre, from, to } => Some(ArcDraft {
                centre: *centre,
                start: places[*from],
                end: places[*to],
            }),
        }
    }

    fn place_at(&self, places: &[DVec2], fraction: f64) -> DVec2 {
        let (from, to) = self.ends();
        match self.draft(places) {
            None => places[from].lerp(places[to], fraction),
            Some(drawn) => {
                let radius = drawn.centre.distance(drawn.start);
                let angle = (drawn.start - drawn.centre).to_angle() + sweep_of(drawn) * fraction;
                drawn.centre + DVec2::from_angle(angle) * radius
            }
        }
    }
}

/// How far along each of the two a crossing stands, for every crossing they
/// have.
fn between(first: &Curve, second: &Curve, places: &[DVec2]) -> Vec<(f64, f64)> {
    let (this, that) = (first.ends(), second.ends());
    match (first.draft(places), second.draft(places)) {
        (None, None) => where_segments_cross(
            places[this.0],
            places[this.1],
            places[that.0],
            places[that.1],
        )
        .into_iter()
        .collect(),
        (None, Some(curve)) => where_segment_crosses_arc(places[this.0], places[this.1], curve),
        (Some(curve), None) => where_segment_crosses_arc(places[that.0], places[that.1], curve)
            .into_iter()
            .map(|(along, round)| (round, along))
            .collect(),
        (Some(near), Some(far)) => where_arcs_cross(near, far),
    }
}

/// Nearer than this to an end, a crossing is that end: the sliver it would
/// otherwise cut off is shorter than the arithmetic that found it.
const CLOSE_TO_AN_END: f64 = 1e-9;

/// Two crossings this near each other are the same one, as they are where
/// three curves run through a single place. Relative to how far out the place
/// stands, so the drawing can be measured in anything.
const THE_SAME_PLACE: f64 = 1e-9;

fn vertex_for(places: &mut Vec<DVec2>, place: DVec2) -> usize {
    let tolerance = THE_SAME_PLACE * (1.0 + place.abs().max_element());
    match places
        .iter()
        .position(|known| known.distance(place) < tolerance)
    {
        Some(known) => known,
        None => {
            places.push(place);
            places.len() - 1
        }
    }
}

/// The curve cut into the runs between its crossings, each as the two vertices
/// it joins.
fn pieces(curve: &Curve, cuts: &[(f64, usize)]) -> Vec<(usize, usize)> {
    let (from, to) = curve.ends();
    let mut sorted = cuts.to_vec();
    sorted.sort_by(|left, right| left.0.total_cmp(&right.0));
    let mut chain = vec![from];
    chain.extend(sorted.iter().map(|(_, vertex)| *vertex));
    chain.push(to);
    chain.dedup();
    chain.windows(2).map(|pair| (pair[0], pair[1])).collect()
}

impl Sketch {
    pub(crate) fn crossed(&self) -> Crossed {
        let mut places = self.points().to_vec();
        let curves: Vec<Curve> = self
            .live_segments()
            .filter(|(_, segment)| !segment.construction)
            .map(|(_, segment)| Curve::Straight {
                from: segment.start.0,
                to: segment.end.0,
            })
            .chain(
                self.live_arcs()
                    .filter(|(_, arc)| !arc.construction)
                    .map(|(_, arc)| Curve::Bent {
                        centre: self.point(arc.center),
                        from: arc.start.0,
                        to: arc.end.0,
                    }),
            )
            .collect();

        let held = |fraction: f64| fraction > CLOSE_TO_AN_END && fraction < 1.0 - CLOSE_TO_AN_END;
        let mut cuts: Vec<Vec<(f64, usize)>> = vec![Vec::new(); curves.len()];
        for first in 0..curves.len() {
            for second in (first + 1)..curves.len() {
                for (along_first, along_second) in between(&curves[first], &curves[second], &places)
                {
                    if !held(along_first) || !held(along_second) {
                        continue;
                    }
                    let place = curves[first].place_at(&places, along_first);
                    let vertex = vertex_for(&mut places, place);
                    cuts[first].push((along_first, vertex));
                    cuts[second].push((along_second, vertex));
                }
            }
        }

        let mut ends = Vec::new();
        let straight = curves
            .iter()
            .zip(&cuts)
            .filter(|(curve, _)| matches!(curve, Curve::Straight { .. }));
        for (curve, cut) in straight {
            for (from, to) in pieces(curve, cut) {
                ends.push((from, to));
                ends.push((to, from));
            }
        }

        let split = ends.len();
        let mut arcs = Vec::new();
        let bent = curves
            .iter()
            .zip(&cuts)
            .filter_map(|(curve, cut)| match curve {
                Curve::Straight { .. } => None,
                Curve::Bent { centre, .. } => Some((*centre, curve, cut)),
            });
        for (centre, curve, cut) in bent {
            for (from, to) in pieces(curve, cut) {
                ends.push((from, to));
                ends.push((to, from));
                arcs.push(ArcHalfEdge {
                    center: centre,
                    forward: true,
                });
                arcs.push(ArcHalfEdge {
                    center: centre,
                    forward: false,
                });
            }
        }

        Crossed {
            places,
            ends,
            split,
            arcs,
        }
    }
}
