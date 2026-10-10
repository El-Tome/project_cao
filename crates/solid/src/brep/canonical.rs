//! What the boolean decides once and reads back as a symbol afterwards: how
//! the second operand is turned to stand exactly square to, or along, the
//! first's walls (decision 11 of `docs/exact-kernel.md`), which surfaces of
//! the two operands are one (decisions 1, 8 and 10), which pairs of them no
//! place lies on both of, which curves are one and the surfaces each lies on
//! (decision 3), which corners are one and the surfaces each lies on
//! (decision 5).

mod apart;
mod arcs;
mod carried;
mod corners;
mod crescent;
mod curves;
mod flush;
mod planes;
mod squared;
mod surfaces;
mod touches;

pub(super) use apart::Apart;
pub(super) use arcs::parting;
pub(super) use carried::carried_along;
pub(super) use corners::{Pool, lies_on};
pub(super) use crescent::closed;
pub(super) use curves::{Registered, Registry, distance, same, within};
pub(super) use flush::flush;
pub(super) use planes::{Place, Planes};
pub(super) use squared::{laid_exactly, leaning, squared};
pub(super) use surfaces::Surfaces;

#[cfg(test)]
mod tests;
