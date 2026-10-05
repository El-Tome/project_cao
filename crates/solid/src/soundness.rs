//! What every solid a kernel hands back must keep, whatever it is.
//!
//! Nobody can write down the faces two tilted cylinders should come out as, so
//! a boolean has no expected answer to be held against. What can be written
//! down is what any answer must satisfy: its surface is closed, it encloses
//! what the operation promised, no face of it crosses another, and the same
//! input gives the same answer twice. The fifth rule — an operation and its
//! undo give back the part they started from — needs a history, and is held in
//! `cao_part`'s tests with the same pieces.
//!
//! Everything here reads triangles and nothing else. That is what lets it
//! hold any kernel to the same account: the one written in this crate, or one
//! adopted behind it (#447).
//!
//! Built for the tests alone, behind `test-support`: the application never
//! checks its own solids while it runs.

use glam::DVec3;

mod campaign;
mod closed;
mod crossing;
mod listed;
mod measure;
mod random;
mod shrinking;

pub use campaign::{Check, Finding, Report, answer, campaign};
pub use closed::closed;
pub use crossing::uncrossed;
pub use listed::{Mislisted, listed};
pub use measure::{Along, Lines, Spans, enclosed};
pub use random::Random;
pub use shrinking::shrink;

pub type Triangle = [DVec3; 3];

/// Two points closer than this share a place, as a fraction of how far the
/// solid reaches.
///
/// Relative rather than absolute for the reason `boolean.rs` gives: the noise
/// of a coordinate grows with its size. A ten-billionth is a million times the
/// noise a cut leaves on a corner written twice, and ten times finer than the
/// billionth the kernel written here judges two faces one plane by — so a
/// crack that tolerance leaves is seen, rather than welded shut by the rules
/// looking for it. Two faces closer than this cannot be told from one face
/// laid twice, which is why the cases drawn never ask for a gap that fine.
pub const NEAR: f64 = 1e-10;

/// Which rule a solid broke: what a shrunk case has to go on breaking to still
/// be the same failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Rule {
    Closed,
    Uncrossed,
    Volume,
    Repeatable,
    Undone,
    /// The kernel came back at all, rather than ending the program.
    Answers,
    /// What a body lists itself as made of stands on its geometry.
    Listed,
    /// The exact body, before any triangle, encloses along every line what
    /// arithmetic promised.
    Spans,
}

/// A rule a solid broke, and where — enough for somebody to go and look.
#[derive(Clone, Debug, PartialEq)]
pub enum Flaw {
    /// A stretch of edge with more faces running one way along it than the
    /// other: the surface has a hole there, or a face laid twice.
    Open {
        from: DVec3,
        to: DVec3,
        one_way: usize,
        other_way: usize,
    },
    /// Two faces passing through each other, or lying on each other. Boxed,
    /// so that a flaw stays small enough to travel in a `Result`.
    Crossing {
        first: Box<Triangle>,
        second: Box<Triangle>,
    },
    /// The matter enclosed is not what the operation promised. `worst` is the
    /// line of measure the two disagree most along, when they were measured
    /// that way.
    Volume {
        promised: f64,
        enclosed: f64,
        worst: Option<Along>,
    },
    /// The same input answered twice, differently. `at` is the first triangle
    /// whose bits differ, or the shorter length when one answer has more.
    Unrepeatable { at: usize },
    /// An undo gave back a part other than the one it started from.
    NotUndone { at: usize },
    /// The kernel gave no answer at all.
    NoAnswer(Silence),
    /// A body's listing of its faces, edges and vertices does not hold.
    Mislisted(Mislisted),
    /// The exact body encloses along a line other than what arithmetic
    /// promised: the kernel's own answer is wrong, whatever its triangles.
    Spans(Along),
}

/// How a kernel failed to answer.
#[derive(Clone, Debug, PartialEq)]
pub enum Silence {
    /// It panicked, with the message the panic carried if it carried one.
    Panicked(Option<String>),
    /// It was still working when the patience given ran out.
    Late(std::time::Duration),
    /// It declined: no solid for an input that describes one.
    Refused,
}

impl Flaw {
    /// Whether two flaws are the same failure: the same rule broken, and for
    /// a kernel that gave no answer, the same kind of silence. A panic that
    /// shrinks into a case that never comes back is two bugs, and would be
    /// reported as neither.
    pub fn is_like(&self, other: &Flaw) -> bool {
        match (self, other) {
            (Flaw::NoAnswer(one), Flaw::NoAnswer(other)) => {
                std::mem::discriminant(one) == std::mem::discriminant(other)
            }
            _ => self.rule() == other.rule(),
        }
    }

    pub fn rule(&self) -> Rule {
        match self {
            Flaw::Open { .. } => Rule::Closed,
            Flaw::Crossing { .. } => Rule::Uncrossed,
            Flaw::Volume { .. } => Rule::Volume,
            Flaw::Unrepeatable { .. } => Rule::Repeatable,
            Flaw::NotUndone { .. } => Rule::Undone,
            Flaw::NoAnswer(_) => Rule::Answers,
            Flaw::Mislisted(_) => Rule::Listed,
            Flaw::Spans(_) => Rule::Spans,
        }
    }
}

impl From<Mislisted> for Flaw {
    fn from(mislisted: Mislisted) -> Flaw {
        Flaw::Mislisted(mislisted)
    }
}

/// How far a solid reaches from the origin, never less than one: the scale
/// every tolerance here is taken against.
pub fn reach(triangles: &[Triangle]) -> f64 {
    triangles
        .iter()
        .flatten()
        .fold(1.0f64, |far, corner| far.max(corner.abs().max_element()))
}

/// Where two answers first differ, bit for bit, if they do.
///
/// Bits rather than a tolerance: a kernel that answers the same input with two
/// results a hair apart has something in it that depends on more than its
/// input — an address, an order of iteration over a hash — and it will one day
/// answer with two different shapes.
pub fn first_difference(first: &[Triangle], second: &[Triangle]) -> Option<usize> {
    let bits = |triangle: &Triangle| triangle.map(|corner| corner.to_array().map(f64::to_bits));
    first
        .iter()
        .zip(second)
        .position(|(left, right)| bits(left) != bits(right))
        .or((first.len() != second.len()).then(|| first.len().min(second.len())))
}

/// The same input, answered twice, must be the same answer.
pub fn repeatable(first: &[Triangle], second: &[Triangle]) -> Result<(), Flaw> {
    match first_difference(first, second) {
        Some(at) => Err(Flaw::Unrepeatable { at }),
        None => Ok(()),
    }
}

#[cfg(test)]
pub(crate) mod tests;
