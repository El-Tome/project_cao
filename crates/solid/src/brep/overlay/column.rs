//! Vertical lines across the overlay, each halfway between two abscissae
//! where a piece ends, and what each crosses from the bottom up. Between
//! two such abscissae no piece ends and no two cross, so a line there meets
//! every piece it meets away from its ends, and in an order that holds over
//! the whole gap.

use super::Arc;
use super::piece::Piece;

/// Abscissae closer than this, relative to the largest, are one: what parts
/// them is rounding, and a line between them would meet arcs too close
/// together to be ordered.
const SNAP: f64 = 1e-12;

#[derive(Clone, Copy, Debug)]
pub(super) struct Crossing {
    pub piece: usize,
    pub height: f64,
}

#[derive(Clone, Debug)]
pub(super) struct Column {
    pub x: f64,
    /// The width of the gap the column stands in the middle of.
    pub width: f64,
    pub crossings: Vec<Crossing>,
}

pub(super) struct Columns {
    /// Each run of abscissae taken as one, by its least and its greatest,
    /// in order; on a cylinder, taken modulo the period, the last running
    /// past it when it meets the first.
    pub clusters: Vec<[f64; 2]>,
    /// The column in the gap to the right of each cluster: all of them on a
    /// cylinder, all but the last on a plane.
    pub list: Vec<Column>,
    pub period: Option<f64>,
}

pub(super) fn columns(arcs: &[Arc], pieces: &[Piece], period: Option<f64>) -> Columns {
    let clusters = clusters(pieces, period);
    let count = clusters.len();
    let gaps = match period {
        Some(_) => count,
        None => count.saturating_sub(1),
    };
    let list = (0..gaps)
        .map(|rank| {
            let from = clusters[rank][1];
            let to = if rank + 1 < count {
                clusters[rank + 1][0]
            } else {
                clusters[0][0] + period.unwrap_or(0.0)
            };
            let x = 0.5 * (from + to);
            let mut crossings: Vec<Crossing> = pieces
                .iter()
                .enumerate()
                .filter_map(|(rank, piece)| {
                    let at = reach(piece, x, period)?;
                    Some(Crossing {
                        piece: rank,
                        height: piece.height(&arcs[piece.arc].trace, at),
                    })
                })
                .collect();
            crossings.sort_by(|low, high| low.height.total_cmp(&high.height));
            Column {
                x,
                width: to - from,
                crossings,
            }
        })
        .collect();
    Columns {
        clusters,
        list,
        period,
    }
}

/// Where the vertical line at `x` meets the piece, in the piece's own
/// unwrapped abscissae, when it meets it away from its ends.
pub(super) fn reach(piece: &Piece, x: f64, period: Option<f64>) -> Option<f64> {
    let (low, high) = (piece.left.x, piece.right.x);
    let at = match period {
        Some(period) => low + (x - low).rem_euclid(period),
        None => x,
    };
    (at > low && at < high).then_some(at)
}

fn clusters(pieces: &[Piece], period: Option<f64>) -> Vec<[f64; 2]> {
    let mut abscissae: Vec<f64> = pieces
        .iter()
        .flat_map(|piece| [piece.left.x, piece.right.x])
        .map(|x| match period {
            Some(period) => {
                let turned = x.rem_euclid(period);
                if turned >= period { 0.0 } else { turned }
            }
            None => x,
        })
        .collect();
    abscissae.sort_by(f64::total_cmp);
    let largest = match period {
        Some(period) => period,
        None => abscissae
            .iter()
            .fold(1.0_f64, |largest, x| largest.max(x.abs())),
    };
    let snap = SNAP * largest;
    let mut clusters: Vec<[f64; 2]> = Vec::new();
    for x in abscissae {
        match clusters.last_mut() {
            Some(last) if x - last[1] <= snap => last[1] = x,
            _ => clusters.push([x, x]),
        }
    }
    if let Some(period) = period
        && clusters.len() > 1
        && clusters[0][0] + period - clusters[clusters.len() - 1][1] <= snap
    {
        let first = clusters.remove(0);
        let last = clusters.len() - 1;
        clusters[last][1] = first[1] + period;
    }
    clusters
}

impl Columns {
    /// The cluster an abscissa where a piece ends was taken into.
    pub fn cluster_of(&self, x: f64) -> usize {
        let apart = |cluster: &[f64; 2]| {
            let shifts = match self.period {
                Some(period) => vec![-period, 0.0, period],
                None => vec![0.0],
            };
            shifts
                .into_iter()
                .map(|shift| {
                    let x = x + shift;
                    (cluster[0] - x).max(x - cluster[1]).max(0.0)
                })
                .fold(f64::INFINITY, f64::min)
        };
        (0..self.clusters.len())
            .min_by(|&one, &other| {
                apart(&self.clusters[one]).total_cmp(&apart(&self.clusters[other]))
            })
            .unwrap_or(0)
    }
}
