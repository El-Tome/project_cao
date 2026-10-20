//! An area of a drawing as the solid takes it: its runs, for the exact kernel,
//! and the steps and triangles it was sampled into, for the flats.

use cao_sketch::{Leg, Outline, Region};
use cao_solid::Loop;
use cao_solid::profile::{Contour, Profile, Run};
use glam::DVec2;

/// Two places closer than this, relative to the drawing's reach, are one: the
/// relative tolerance the exact kernel decides with, stated here because the
/// kernel's own is not part of what it exports.
const ONE_PLACE: f64 = 1e-9;

/// The area handed both ways. `triangles` are the region's own, worked out by
/// the caller, since the profile only borrows them.
pub(crate) fn profile<'a>(region: &'a Region, triangles: &'a [[DVec2; 3]]) -> Profile<'a> {
    let (sampled, sampled_holes) = loops(region);
    Profile {
        exact: Some(exact(region)),
        sampled,
        sampled_holes,
        triangles,
    }
}

/// The loops a region hands the flats: its outline and what it leaves hollow,
/// each carrying the curve every segment was sampled from so that a wall
/// raised from one curve comes out as one face.
fn loops(region: &Region) -> (Loop<'_>, Vec<Loop<'_>>) {
    fn borrow(outline: &Outline) -> Loop<'_> {
        Loop {
            points: &outline.points,
            curves: &outline.curves,
        }
    }
    (
        borrow(&region.outline),
        region.holes.iter().map(borrow).collect(),
    )
}

/// The outline and the holes as runs, a pinched outline parted into them.
fn exact(region: &Region) -> (Contour, Vec<Contour>) {
    let (outline, parted) = pinched_apart(contour(&region.outline));
    let mut holes: Vec<Contour> = region.holes.iter().map(contour).collect();
    holes.extend(parted);
    (outline, holes)
}

/// The loops a pinched outline welds, apart again: the walk hands an area
/// whose hole touches its outline as one loop through the touch twice, and
/// the kernel raises that same place only as two loops meeting at one corner
/// — a whole circle sharing a loop with other runs it declines. Parted at
/// every corner the loop stands on twice, within the kernel's own net; the
/// loop that measures the most is the outline, every other is a hole of it.
fn pinched_apart(whole: Contour) -> (Contour, Vec<Contour>) {
    let reach = whole
        .corners
        .iter()
        .map(|corner| corner.abs().max_element())
        .fold(1.0, f64::max);
    let eps = ONE_PLACE * reach;
    let mut corners: Vec<DVec2> = Vec::new();
    let mut runs: Vec<Run> = Vec::new();
    let mut parted: Vec<Contour> = Vec::new();
    for (corner, run) in whole.corners.into_iter().zip(whole.runs) {
        if let Some(at) = corners.iter().position(|seen| seen.distance(corner) <= eps) {
            parted.push(Contour {
                corners: corners.split_off(at),
                runs: runs.split_off(at),
            });
        }
        corners.push(corner);
        runs.push(run);
    }
    let mut outline = Contour { corners, runs };
    for hole in parted.iter_mut() {
        if measure(hole) > measure(&outline) {
            std::mem::swap(&mut outline, hole);
        }
    }
    (outline, parted)
}

/// What a closed contour encloses, whichever way it turns.
fn measure(contour: &Contour) -> f64 {
    let count = contour.corners.len();
    let doubled: f64 = contour
        .runs
        .iter()
        .enumerate()
        .map(|(at, run)| {
            let from = contour.corners[at];
            let to = contour.corners[(at + 1) % count];
            from.perp_dot(to) + 2.0 * bulge(from, *run)
        })
        .sum();
    (doubled / 2.0).abs()
}

/// The area between a curved run and its chord, signed the way the run turns.
fn bulge(from: DVec2, run: Run) -> f64 {
    match run {
        Run::Straight => 0.0,
        Run::Round { center, turn } => from.distance_squared(center) * (turn - turn.sin()) / 2.0,
        Run::Oval {
            first,
            second,
            turn,
            ..
        } => first.length() * second * (turn - turn.sin()) / 2.0,
    }
}

fn contour(outline: &Outline) -> Contour {
    let (corners, runs) = outline
        .runs()
        .into_iter()
        .map(|(corner, leg)| {
            let run = match leg {
                Leg::Straight => Run::Straight,
                Leg::Round { centre, turned } => Run::Round {
                    center: centre,
                    turn: turned,
                },
                Leg::Oval {
                    centre,
                    first,
                    second,
                    turned,
                } => Run::Oval {
                    center: centre,
                    first,
                    second,
                    turn: turned,
                },
            };
            (corner, run)
        })
        .unzip();
    Contour { corners, runs }
}

#[cfg(test)]
mod tests;
