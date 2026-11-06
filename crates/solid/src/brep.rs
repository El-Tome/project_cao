//! The exact kernel of #498: a boundary representation exact on planes and
//! cylinders, behind the vocabulary `cao_solid` speaks — raise a profile,
//! join, cut, and hand back the triangles to draw. Behind [`crate::Body`],
//! beside the flats: nothing above `cao_solid` reaches it (#526).
//! The design is `docs/exact-kernel.md`.

mod assembly;
mod canonical;
mod combine;
mod conic;
mod curve;
mod domain;
mod laying;
mod listing;
mod meet;
mod names;
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

pub use conic::{Arm, Conic, On, Round};
pub use curve::{Circle, Curve, Line, Meet, Wall};
pub use domain::Location;
pub use listing::{ListedEdge, ListedFace, Listing};
pub use meet::{Configuration, Meeting, Node};
pub use names::{EdgeName, Lost, VertexName};
pub use overlay::{Arc, Overlay, Region};
pub use relation::{Crossing, Crossings, Relation, crossings, relation};
pub use scale::Scale;
pub use surface::{Cone, Cylinder, EllipticCylinder, Plane, Sphere, Surface, Torus};
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
    /// for the tolerance to hold its points, a number that is not one. Or,
    /// turned, none the kernel reads: one it cannot lay against its axis, or
    /// one holding an ellipse, which no surface of its own turns into (#632).
    Profile,
    /// The travel does not stand square to the profile's plane, or is shorter
    /// than the tolerance; or a turn moves a corner of its profile, or leaves
    /// the slit of a partial turn, narrower than the tolerance. A turn that
    /// short or that nearly whole is read before the kernel as nothing or as
    /// whole: this is the kernel's own net.
    Travel,
    /// Two surfaces meet along a curve the kernel does not build: a cone
    /// within the tolerance of a plane or a cylinder of its axis all across
    /// the box, or a parallel wall within decision 8's hair of it all round;
    /// two walls nearly parallel, past decision 11, that meet at a sliver of
    /// a lean, stand within the tolerance of each other along a stretch, or
    /// closer than a chord's sag (#577); or a cone against a wall off its
    /// axis whose ruling runs along that wall's axis, a hair off coaxial,
    /// with an apex on the other wall, with a hole in the flank of a face
    /// holding its apex (#711), or closer than a chord's sag without meeting
    /// (#706). A plane meets a cylinder, a cone or an elliptic wall at any
    /// other angle along a conic, a parallel round or elliptic wall meets an
    /// elliptic one along rulings, and any other round, elliptic or conical
    /// wall along the curve the two meet along, which are built (#552,
    /// #571, #577). Or a chamfer (#696) or a fillet (#697) on an edge that is
    /// not straight between two flat faces meeting outwards, that runs on
    /// tangentially into the next edge of a face, or whose cut, carried along
    /// the edge's line, would take matter beyond the edge.
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
    /// A step pointed at an edge its name no longer finds (#696): gone, or
    /// split in pieces, as [`crate::Body::lost`] says.
    Lost(Lost),
    /// A chamfer's distance (#696) or a fillet's radius (#697) that is no
    /// length the kernel can set a cut back by: not a number, nought or less,
    /// or within a few `Scale::HAIR` tolerances of what the cut replaces,
    /// where the kernel would read it back onto that — a chamfer onto its
    /// edge, a fillet's round onto the edge's two faces. A chamfer's distance
    /// is measured as it is asked. A fillet's radius is measured so too, and
    /// by how far its round would stand off the two faces at the middle of
    /// its turn, which grows with the radius: on an edge bent nearly flat a
    /// small radius stands too near them, and a larger one, up to the most
    /// the faces give, rounds the edge. A fillet whose round the kernel read
    /// onto the faces even so, no face of its own, is declined so too.
    Distance,
}

/// A step the kernel built though it reads it unstably (#552): a plane of one
/// operand leans off square to, or along, the axis of a cylinder, a cone or an
/// elliptic wall (#571) of the other — or a wall's axis off square or
/// parallel to another's (#577) — their faces near enough to meet, by more than decision 11 lays
/// exactly and less than the kernel reads without magnifying its rounding.
/// The conic such a plane cuts divides by the lean, so a change too small to
/// see may change what is built. Said, never declined.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Unstable {
    pub lean: Lean,
    /// The least such lean of the step, in degrees.
    pub degrees: f64,
}

/// Which way a plane leans off a wall's axis, or a wall off another's
/// (#577), a hair or a little.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Lean {
    /// Nearly square to the axis: its normal nearly along it.
    Square,
    /// Nearly along the axis: its normal nearly square to it.
    Along,
    /// Two walls whose axes are nearly square to each other.
    Across,
    /// Two walls whose axes are nearly parallel.
    Abreast,
}
