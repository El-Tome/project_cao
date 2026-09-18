//! Every segment and arc still drawn, as the half-edges a face walk can turn
//! at — cut apart wherever two of them cross, and wherever a drawn point sits
//! on one without being an end of it.
//!
//! The walk reads nothing but the vertices and the angular order of the edges
//! leaving each one, so two edges meeting in space with no vertex of their own
//! are invisible to it. Giving that meeting a vertex here is what lets the walk
//! stay as it is and still find the areas a bowtie bounds.
//!
//! A crossing has no vertex until one is invented for it; a point landing in
//! the middle of a curve already is one, and what it lacks is the cut. Both
//! end as an entry in the same table of cuts.

use glam::DVec2;

use crate::arcing::{ArcDraft, places_along};
use crate::circle_edges::Round;
use crate::sketch::Sketch;

mod curve;

use curve::{Curve, between, pieces};

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

/// The drawing cut apart, before the half-edges are read off it.
struct Cut {
    curves: Vec<Curve>,
    places: Vec<DVec2>,
    cuts: Vec<Vec<(f64, usize)>>,
    whole: Vec<Vec<DVec2>>,
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
    /// The circles nothing cut, each sampled as the closed loop it still is.
    /// They never enter the graph: a curve with no end has no vertex, and the
    /// walk turns at vertices.
    pub(crate) whole: Vec<Vec<DVec2>>,
}

/// Nearer than this to an end, a crossing is that end: the sliver it would
/// otherwise cut off is shorter than the arithmetic that found it.
const CLOSE_TO_AN_END: f64 = 1e-9;

/// Nearer than this and two places are one: where three curves run through a
/// single point, and where a drawn point sits on a curve rather than beside it.
const THE_SAME_PLACE: f64 = 1e-9;

/// Read against how far out the place stands, so the drawing can be measured
/// in anything.
pub(crate) fn off_by(place: DVec2) -> f64 {
    THE_SAME_PLACE * (1.0 + place.abs().max_element())
}

fn vertex_for(places: &mut Vec<DVec2>, place: DVec2) -> usize {
    let tolerance = off_by(place);
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

/// The arcs a circle becomes, once every turn something runs through it stands
/// on a vertex — and nothing when fewer than two of them are distinct, which
/// leaves the loop whole. The vertices are made either way: a circle a single
/// run enters is still one loop, and that place is still a crossing to catch.
fn broken(round: &Round, places: &mut Vec<DVec2>) -> Vec<Curve> {
    let mut vertices: Vec<usize> = round
        .turns
        .iter()
        .map(|turn| vertex_for(places, round.place_at(*turn)))
        .collect();
    vertices.dedup();
    if vertices.len() > 1 && vertices.first() == vertices.last() {
        vertices.pop();
    }
    if vertices.len() < 2 {
        return Vec::new();
    }
    (0..vertices.len())
        .map(|step| Curve::Bent {
            centre: round.centre,
            from: vertices[step],
            to: vertices[(step + 1) % vertices.len()],
        })
        .collect()
}

impl Sketch {
    /// Every place two curves of the drawing run through without a point of
    /// the drawing's own standing there.
    ///
    /// These are the vertices `crossed` invents, and nothing else: a crossing
    /// a point already occupies is that point, and is not reported twice.
    pub fn crossings(&self) -> Vec<DVec2> {
        let drawn = self.points().len();
        self.cut().places.split_off(drawn)
    }

    /// Which curves are drawn, the places they run through — the drawing's own
    /// points first, then one for each crossing — and where each of them has
    /// to be cut apart.
    fn cut(&self) -> Cut {
        let drawn = self.points().len();
        let mut places = self.points().to_vec();
        let mut curves: Vec<Curve> = self
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

        let mut whole = Vec::new();
        for round in self.rounds() {
            let pieces = broken(&round, &mut places);
            match pieces.is_empty() {
                true => whole.push(round.sampled()),
                false => curves.extend(pieces),
            }
        }

        let held = |fraction: f64| fraction > CLOSE_TO_AN_END && fraction < 1.0 - CLOSE_TO_AN_END;
        let mut cuts: Vec<Vec<(f64, usize)>> = vec![Vec::new(); curves.len()];

        let cutting: Vec<usize> = self
            .live_points()
            .map(|(point, _)| point.0)
            .chain(drawn..places.len())
            .collect();
        for point in cutting {
            let place = places[point];
            for (index, curve) in curves.iter().enumerate() {
                let (from, to) = curve.ends();
                if point == from || point == to {
                    continue;
                }
                if let Some(fraction) = curve.fraction_at(&places, place).filter(|at| held(*at)) {
                    cuts[index].push((fraction, point));
                }
            }
        }

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

        Cut {
            curves,
            places,
            cuts,
            whole,
        }
    }

    pub(crate) fn crossed(&self) -> Crossed {
        let Cut {
            curves,
            places,
            cuts,
            whole,
        } = self.cut();

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
            whole,
        }
    }
}
