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

use crate::circle_edges::Round;
use crate::ellipse_edges::Oval;
use crate::naming::CurveId;
use crate::sketch::Sketch;

mod border;
mod curve;
pub(crate) mod half_edge;

use border::{Borders, Loops, also_named};
use curve::{Curve, between, pieces};
pub(crate) use half_edge::{Bend, CurvedHalfEdge};

/// The drawing cut apart, before the half-edges are read off it.
struct Cut {
    curves: Vec<Curve>,
    /// Which curve of the drawing each of those was cut out of, in the same
    /// order. A curve crossed twice appears here twice.
    drawn: Vec<CurveId>,
    places: Vec<DVec2>,
    cuts: Vec<Vec<(f64, usize)>>,
    whole: Vec<(Vec<CurveId>, Vec<DVec2>)>,
}

/// The drawing as a graph with every crossing standing on a vertex of its own.
///
/// `places` is the drawing's own points, then one more for each crossing.
/// `ends` pairs every half-edge with its twin next to it, the straight ones
/// first; `split` is where the curved ones start, and `curves` holds one entry
/// for each of those. A stretch two curves lie along is in it once, laid by
/// the curve first in this order and named by both.
pub(crate) struct Crossed {
    pub(crate) places: Vec<DVec2>,
    pub(crate) ends: Vec<(usize, usize)>,
    pub(crate) split: usize,
    pub(crate) curves: Vec<CurvedHalfEdge>,
    /// Which curves of the drawing each edge was cut out of, read for either
    /// of its halves at half the half-edge's number: one, or every curve lying
    /// along that stretch, in order. It is what lets a face walk say which
    /// curves bound it.
    pub(crate) from: Vec<Vec<CurveId>>,
    /// The circles and ellipses nothing cut, each sampled as the closed loop
    /// it still is, and the same curve drawn twice only once, named by both.
    /// They never enter the graph: a curve with no end has no vertex, and the
    /// walk turns at vertices.
    pub(crate) whole: Vec<(Vec<CurveId>, Vec<DVec2>)>,
}

/// Nearer than this to an end, a crossing is that end: the sliver it would
/// otherwise cut off is shorter than the arithmetic that found it.
const CLOSE_TO_AN_END: f64 = 1e-9;

/// Nearer than this and two places are one: where three curves run through a
/// single point, and where a drawn point sits on a curve rather than beside it.
///
/// Wide enough to hold a touch. Where two curves graze rather than cross, the
/// place they meet cannot be found nearer than the square root of what a
/// number can hold — the two are flat against each other there — so the same
/// touch, hunted along one curve and along the other, comes back twice a few
/// ten-millionths apart. Left as two vertices it leaves a sliver of an edge
/// between them, and the walk comes back with one face where the drawing shows
/// two. A ten-millionth of the drawing is a hundredth of a micron on a part a
/// metre across: far below anything drawn, and far below what the solver holds
/// a dimension to.
const THE_SAME_PLACE: f64 = 1e-7;

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

/// The runs an ellipse becomes, once every turn something runs through it
/// stands on a vertex — and nothing when fewer than two of them are distinct,
/// which leaves the loop whole.
fn broken_oval(oval: &Oval, places: &mut Vec<DVec2>) -> Vec<Curve> {
    let Some((from, sweep)) = oval.run else {
        let vertices = vertices_round(oval.turns.iter().map(|turn| oval.place_at(*turn)), places);
        return (0..vertices.len())
            .map(|step| {
                let (starts, ends) = (vertices[step], vertices[(step + 1) % vertices.len()]);
                let turn_of =
                    |vertex: usize| oval.drawn.turn_on(places[vertex]) / std::f64::consts::TAU;
                let (opens, closes) = (turn_of(starts), turn_of(ends));
                piece_of(oval, opens, (closes - opens).rem_euclid(1.0), starts, ends)
            })
            .collect();
    };

    // A stretch a cut left: it runs between its own two ends, and is broken
    // again at everything that runs through it between them.
    let mut along: Vec<f64> = oval
        .turns
        .iter()
        .map(|turn| (turn - from).rem_euclid(1.0))
        .filter(|at| *at > 0.0 && *at < sweep)
        .collect();
    along.sort_by(f64::total_cmp);
    let mut chain = vec![0.0];
    chain.extend(along);
    chain.push(sweep);

    let mut vertices: Vec<(f64, usize)> = chain
        .into_iter()
        .map(|at| (at, vertex_for(places, oval.place_at(from + at))))
        .collect();
    vertices.dedup_by(|near, far| near.1 == far.1);
    vertices
        .windows(2)
        .map(|pair| {
            let (opens, closes) = (pair[0], pair[1]);
            piece_of(oval, from + opens.0, closes.0 - opens.0, opens.1, closes.1)
        })
        .collect()
}

/// One run of an ellipse, from where it opens over how far it goes.
fn piece_of(oval: &Oval, starts: f64, sweep: f64, from: usize, to: usize) -> Curve {
    Curve::Oval {
        drawn: oval.drawn,
        starts,
        // The two ends of a run that is the whole curve fall on the same
        // place, and a sweep of nothing is no run at all.
        sweep: match sweep <= 0.0 {
            true => 1.0,
            false => sweep,
        },
        from,
        to,
    }
}

/// The vertices a closed curve is broken at, in order round it, and none at
/// all when fewer than two of them are distinct.
fn vertices_round(round: impl Iterator<Item = DVec2>, places: &mut Vec<DVec2>) -> Vec<usize> {
    let mut vertices: Vec<usize> = round.map(|place| vertex_for(places, place)).collect();
    vertices.dedup();
    if vertices.len() > 1 && vertices.first() == vertices.last() {
        vertices.pop();
    }
    match vertices.len() < 2 {
        true => Vec::new(),
        false => vertices,
    }
}

/// The arcs a circle becomes, once every turn something runs through it stands
/// on a vertex — and nothing when fewer than two of them are distinct, which
/// leaves the loop whole. The vertices are made either way: a circle a single
/// run enters is still one loop, and that place is still a crossing to catch.
fn broken(round: &Round, places: &mut Vec<DVec2>) -> Vec<Curve> {
    let vertices = vertices_round(round.turns.iter().map(|turn| round.place_at(*turn)), places);
    (0..vertices.len())
        .map(|step| Curve::Bent {
            centre: round.centre,
            from: vertices[step],
            to: vertices[(step + 1) % vertices.len()],
        })
        .collect()
}

/// Whether a place that far along a curve falls inside it, rather than on one
/// of its ends.
fn held(fraction: f64) -> bool {
    fraction > CLOSE_TO_AN_END && fraction < 1.0 - CLOSE_TO_AN_END
}

/// Cuts every curve each of these vertices stands on, wherever it is not
/// already an end of that curve or a cut in it.
///
/// Every vertex is held against every curve, so a curve whose box the vertex
/// stands clear of is passed over before anything costlier is asked of it.
fn cut_through(
    vertices: impl Iterator<Item = usize>,
    curves: &[Curve],
    places: &[DVec2],
    cuts: &mut [Vec<(f64, usize)>],
) {
    let reaches: Vec<(DVec2, DVec2)> = curves.iter().map(|curve| curve.reach(places)).collect();
    for vertex in vertices {
        let (place, off) = (places[vertex], off_by(places[vertex]));
        for ((curve, cut), (low, high)) in curves.iter().zip(cuts.iter_mut()).zip(&reaches) {
            let (from, to) = curve.ends();
            let clear = place.cmplt(*low - off).any() || place.cmpgt(*high + off).any();
            if clear || vertex == from || vertex == to {
                continue;
            }
            let Some(fraction) = curve.fraction_at(places, place).filter(|at| held(*at)) else {
                continue;
            };
            if cut.iter().all(|(_, at)| *at != vertex) {
                cut.push((fraction, vertex));
            }
        }
    }
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
        let mut curves: Vec<Curve> = Vec::new();
        let mut names: Vec<CurveId> = Vec::new();
        for (id, segment) in self.live_segments().filter(|(_, it)| !it.construction) {
            curves.push(Curve::Straight {
                from: segment.start.0,
                to: segment.end.0,
            });
            names.push(CurveId::Segment(id));
        }
        for (id, arc) in self.live_arcs().filter(|(_, it)| !it.construction) {
            curves.push(Curve::Bent {
                centre: self.point(arc.center),
                from: arc.start.0,
                to: arc.end.0,
            });
            names.push(CurveId::Arc(id));
        }

        let mut loops = Loops::default();
        for round in self.rounds() {
            let pieces = broken(&round, &mut places);
            let id = CurveId::Circle(round.id);
            match pieces.is_empty() {
                true => loops.lay((Bend::Round(round.centre), round.place_at(0.0)), id, || {
                    round.sampled()
                }),
                false => {
                    names.extend(std::iter::repeat_n(id, pieces.len()));
                    curves.extend(pieces);
                }
            }
        }
        for oval in self.ovals() {
            let pieces = broken_oval(&oval, &mut places);
            let id = CurveId::Ellipse(oval.id);
            match pieces.is_empty() {
                true => loops.lay((Bend::Oval(oval.drawn), oval.place_at(0.0)), id, || {
                    oval.sampled()
                }),
                false => {
                    names.extend(std::iter::repeat_n(id, pieces.len()));
                    curves.extend(pieces);
                }
            }
        }

        let mut cuts: Vec<Vec<(f64, usize)>> = vec![Vec::new(); curves.len()];
        let cutting = self
            .live_points()
            .map(|(point, _)| point.0)
            .chain(drawn..places.len());
        cut_through(cutting, &curves, &places, &mut cuts);

        let crossed_from = places.len();
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
        // A crossing found between two curves is a place every curve running
        // through it is cut at, not those two alone. Where two curves lie
        // along each other and a third grazes them, whether the graze is seen
        // is a matter of rounding, and it is often seen on one of the two
        // only: cut there and not on the other, they would end on different
        // vertices and stay two borders along one stretch.
        cut_through(crossed_from..places.len(), &curves, &places, &mut cuts);

        Cut {
            curves,
            drawn: names,
            places,
            cuts,
            whole: loops.into_whole(),
        }
    }

    pub(crate) fn crossed(&self) -> Crossed {
        let Cut {
            curves,
            drawn,
            places,
            cuts,
            whole,
        } = self.cut();

        let mut ends = Vec::new();
        let mut from = Vec::new();
        let mut borders = Borders::default();
        let named = || drawn.iter().zip(&curves).zip(&cuts);
        for ((id, curve), cut) in named().filter(|((_, it), _)| it.is_straight()) {
            for (start, end) in pieces(curve, cut) {
                if let Some(edge) = borders.along_straight(start, end, from.len()) {
                    also_named(&mut from[edge], *id);
                    continue;
                }
                ends.push((start, end));
                ends.push((end, start));
                from.push(vec![*id]);
            }
        }

        let split = ends.len();
        let mut bent = Vec::new();
        let curved = named().filter_map(|((id, curve), cut)| match curve {
            Curve::Straight { .. } => None,
            Curve::Bent { centre, .. } => Some((*id, Bend::Round(*centre), curve, cut)),
            Curve::Oval { drawn, .. } => Some((*id, Bend::Oval(*drawn), curve, cut)),
        });
        for (id, bend, curve, cut) in curved {
            for (start, end) in pieces(curve, cut) {
                if let Some(edge) = borders.along_curved(bend, (start, end), &places, from.len()) {
                    also_named(&mut from[edge], id);
                    continue;
                }
                ends.push((start, end));
                ends.push((end, start));
                from.push(vec![id]);
                for forward in [true, false] {
                    bent.push(CurvedHalfEdge { bend, forward });
                }
            }
        }

        Crossed {
            places,
            ends,
            split,
            curves: bent,
            from,
            whole,
        }
    }
}
