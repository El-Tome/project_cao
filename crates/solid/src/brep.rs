//! The exact kernel of #498: a boundary representation exact on planes and
//! cylinders, behind the vocabulary `cao_solid` speaks — raise a profile,
//! join, cut, and hand back the triangles to draw. Behind [`crate::Body`],
//! beside the flats: nothing above `cao_solid` reaches it (#526).
//! The design is `docs/exact-kernel.md`.

mod assembly;
mod canonical;
mod combine;
mod curve;
mod domain;
mod laying;
mod listing;
mod meet;
mod overlay;
mod piece;
mod prism;
mod ray;
mod relation;
mod scale;
mod selection;
mod surface;
mod tessellation;
mod topology;
mod trace;
mod turned;
mod volume;

use serde::{Deserialize, Serialize};

pub use curve::{Circle, Curve, Line, Meet};
pub use domain::Location;
pub use listing::{ListedEdge, ListedFace, Listing};
pub use meet::{Configuration, Meeting, Node};
pub use overlay::{Arc, Overlay, Region};
pub use relation::{Crossing, Crossings, Relation, crossings, relation};
pub use scale::Scale;
pub use surface::{Cylinder, Plane, Surface};
pub use topology::{
    Body, Coedge, CurveId, Edge, EdgeId, Face, FaceId, SurfaceId, Vertex, VertexId,
};
pub use trace::Trace;

/// Why the kernel gave no answer. It never hands back a solid it could not
/// build or could not verify.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Declined {
    /// The profile describes no solid: corners on each other within a loop
    /// or across loops, an arc whose ends are not on its circle or too large
    /// for the tolerance to hold its points, a number that is not one.
    Profile,
    /// The travel does not stand square to the profile's plane, or is shorter
    /// than the tolerance; or a turn moves a corner of its profile, or leaves
    /// the slit of a partial turn, narrower than the tolerance. A turn that
    /// short or that nearly whole is read before the kernel as nothing or as
    /// whole: this is the kernel's own net.
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
    /// The body was built, but a face of it could not be cut into triangles:
    /// drawn, it would show a hole where there is matter. Never said by the
    /// kernel itself: [`crate::Body`] draws what it builds, and declines what
    /// it cannot draw whole.
    Undrawn,
    /// The kernel stopped on a bug of its own. Never said by the kernel
    /// itself: [`crate::Body`] catches the stop, so that a shape nobody tried
    /// costs the step, not the user's work.
    Panicked,
}
