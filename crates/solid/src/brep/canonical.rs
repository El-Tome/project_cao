//! What the boolean decides once and reads back as a symbol afterwards:
//! which surfaces of the two operands are one (decision 1 of
//! `docs/exact-kernel.md`), which pairs of them no place lies on both of,
//! which curves are one and the surfaces each lies on (decision 3), which
//! corners are one and the surfaces each lies on (decision 5).

mod apart;
mod arcs;
mod corners;
mod curves;
mod planes;
mod surfaces;

pub(super) use apart::Apart;
pub(super) use arcs::parting;
pub(super) use corners::{Pool, lies_on};
pub(super) use curves::{Registered, Registry, distance, same, within};
pub(super) use planes::Planes;
pub(super) use surfaces::Surfaces;

#[cfg(test)]
mod tests;
