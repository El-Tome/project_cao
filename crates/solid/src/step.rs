//! A body as the clear text of an ISO 10303-21 file under AP214, the exchange
//! other CAD programs read: one manifold solid, each face on its exact
//! surface and each edge on its exact curve, in millimetres.

mod geometry;
mod instances;
mod product;
mod shell;

use crate::Body;
use crate::brep::Scale;
use instances::Instances;

/// Why a body is not written: it is on the flats, or one of its curves no
/// STEP entity holds would take a B-spline of more pieces than the cap to
/// stand within the tolerance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unwritten {
    Faceted,
    Unfitted,
}

pub use Unwritten::{Faceted, Unfitted};

/// The body's exact solid in millimetres, declaring the widest the kernel
/// moves a surface by at the body's reach.
pub fn of(body: &Body, millimetres_per_unit: f64) -> Result<String, Unwritten> {
    let brep = body.brep().ok_or(Faceted)?;
    let mut instances = Instances::new(millimetres_per_unit);
    let solid = shell::written(&mut instances, &brep.listing())?;
    let context = product::context(&mut instances, Scale::HAIR * Scale::of(brep.reach()).eps());
    let representation = instances.add(format!(
        "ADVANCED_BREP_SHAPE_REPRESENTATION('',(#{solid}),#{context})"
    ));
    product::shaped_by(&mut instances, representation);
    Ok(instances.file())
}
