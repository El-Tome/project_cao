//! The rounds in which the curves two walls meet along and the rays the
//! walls' circles take are drawn from each other: each curve is sampled on
//! its walls' rays, and each wall takes the rays through its curves'
//! samples.

use std::collections::BTreeMap;

use glam::DVec3;

use super::abreast::Abreast;
use super::axis::Axes;
use super::cone::Grid;
use super::{conic, meet, on_meets};
use crate::brep::curve::Curve;
use crate::brep::tessellation::contact::{self, Contact, Zones};
use crate::brep::topology::{Body, SurfaceId};

/// The samples of every curve two walls meet along, the rays and steps each
/// wall's circles take and leave out, and the axes gathered from them. The
/// curves are sampled twice, the second time on the rays the first gave;
/// some curves a cone meets a wall along a third time, on the last rays
/// ([`meet::on_last_rays`]), and the walls then take the rays through
/// their samples once more.
pub(super) fn sampled(
    body: &Body,
    (walls, zones): (&[contact::Wall], &Zones),
    (grids, abreast, axes): (&BTreeMap<SurfaceId, Grid>, &Abreast, Axes),
    beneath: &BTreeMap<SurfaceId, Vec<DVec3>>,
    tolerance: f64,
) -> (Vec<Vec<DVec3>>, BTreeMap<SurfaceId, Contact>, Axes) {
    let mut given = abreast.joined(axes.given());
    let mut meets = on_meets(body, walls, zones, &BTreeMap::new(), grids, tolerance);
    let mut contacts = contact::contacts(body, walls, zones, &meets, beneath, &given, tolerance);
    let meeting = body
        .edge_ids()
        .any(|id| matches!(body.curve(body.edge(id).curve), Curve::Meet(_)));
    if meeting {
        meets = on_meets(body, walls, zones, &contacts, grids, tolerance);
        contacts = contact::contacts(body, walls, zones, &meets, beneath, &given, tolerance);
    }
    let mut axes = axes.gathered(&contacts);
    let gathered = abreast.joined(axes.given());
    if gathered != given {
        given = gathered;
        contacts = contact::contacts(body, walls, zones, &meets, beneath, &given, tolerance);
        axes = axes.gathered(&contacts);
    }
    let along = conic::rays_along(body, walls, zones, grids, &contacts, tolerance);
    if !along.is_empty() {
        given = abreast.joined(axes.given());
        for (wall, rays) in along {
            given.entry(wall).or_default().extend(rays);
        }
        contacts = contact::contacts(body, walls, zones, &meets, beneath, &given, tolerance);
        if meeting {
            meets = on_meets(body, walls, zones, &contacts, grids, tolerance);
            contacts = contact::contacts(body, walls, zones, &meets, beneath, &given, tolerance);
        }
    }
    let last = meet::on_last_rays(body, (walls, zones, &contacts), grids, tolerance, &meets);
    if last != meets {
        meets = last;
        contacts = contact::contacts(body, walls, zones, &meets, beneath, &given, tolerance);
    }
    (meets, contacts, axes)
}
