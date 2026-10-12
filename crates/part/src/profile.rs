//! An area of a drawing as the solid takes it: its runs, for the exact kernel,
//! and the steps and triangles it was sampled into, for the flats.

use cao_sketch::{Leg, Outline, Region};
use cao_solid::Loop;
use cao_solid::profile::{Contour, Profile, Run};
use glam::DVec2;

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

/// The outline and the holes as runs.
fn exact(region: &Region) -> (Contour, Vec<Contour>) {
    (
        contour(&region.outline),
        region.holes.iter().map(contour).collect(),
    )
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
