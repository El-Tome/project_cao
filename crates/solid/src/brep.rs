//! #498's maquette: a boundary representation exact on planes and cylinders,
//! behind the vocabulary `cao_solid` speaks — raise a profile, join, cut, and
//! hand back the triangles to draw. On a branch not meant to be merged; the
//! design is `docs/exact-kernel.md`.

mod assembly;
mod canonical;
mod combine;
mod curve;
mod domain;
mod listing;
mod meet;
mod overlay;
mod prism;
mod ray;
mod relation;
mod scale;
mod selection;
mod surface;
mod tessellation;
mod topology;
mod trace;
mod volume;

pub use curve::{Circle, Curve, Line, Meet};
pub use domain::Location;
pub use listing::{ListedEdge, ListedFace, Listing};
pub use meet::{Configuration, Meeting, Node};
pub use relation::{Crossing, Crossings, Relation, crossings, relation};
pub use scale::Scale;
pub use surface::{Cylinder, Plane, Surface};
pub use topology::{
    Body, Coedge, CurveId, Edge, EdgeId, Face, FaceId, SurfaceId, Vertex, VertexId,
};
pub use trace::Trace;

/// Why the kernel gave no answer. It never hands back a solid it could not
/// build or could not verify.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Declined {
    /// The profile describes no solid: corners on each other, an arc whose
    /// ends are not on its circle, a loop crossing itself.
    Profile,
    /// The travel does not stand square to the profile's plane.
    Travel,
    /// Two surfaces meet in a way the kernel does not build: a plane oblique
    /// to a cylinder's axis, two cylinders at a skew angle.
    Unsupported,
    /// Two curves touch more closely than their curvatures can order.
    Tie,
    /// The result failed the check of its own listing.
    Unverified,
    /// Not written yet.
    Unfinished,
}
