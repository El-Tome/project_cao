//! What the boolean decides once and reads back as a symbol afterwards: how
//! the second operand is turned to stand exactly square to, or along, the
//! first's walls (decision 11 of `docs/exact-kernel.md`), which surfaces of
//! the two operands are one (decisions 1, 8 and 10), how it is moved onto a
//! touch with a round a hair off (decision 12), how one of its planes is
//! moved onto an edge of the first it stands a hair from (decision 13),
//! which pairs of them no
//! place lies on both of, which curves are one and the surfaces each lies on
//! (decision 3), which corners are one and the surfaces each lies on
//! (decision 5).

mod apart;
mod arcs;
mod carried;
mod corners;
mod crescent;
mod curves;
mod edged;
mod flush;
mod placed;
mod planes;
mod squared;
mod surfaces;
mod tangent;
mod touches;

pub(super) use apart::Apart;
pub(super) use arcs::parting;
pub(super) use carried::carried_along;
pub(super) use corners::{Pool, lies_on};
pub(super) use crescent::closed;
pub(super) use curves::{Registered, Registry, distance, same, within};
pub(super) use edged::edged;
pub(super) use flush::flush;
pub(super) use planes::{Place, Planes};
pub(super) use squared::{Off, laid_exactly, leaning, squared};
pub(super) use surfaces::Surfaces;
pub(super) use tangent::tangent;

#[cfg(test)]
mod tests;
