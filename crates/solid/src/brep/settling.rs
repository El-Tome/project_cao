//! Whether the rounds the sampling runs between the curves two walls meet
//! along and the rays the walls' circles take would move again (#764): a
//! measurement, kept out of every build that does not ask for the campaigns.
//!
//! Nothing is measured on a thread that did not [`arm`] it. Once armed, each
//! tessellation on that thread runs its rounds again aside, on its own
//! copies, then up to [`CAP`] rounds more, and leaves a [`Record`] to be
//! [`taken`]; what is drawn is what the sampling drew without it.
//!
//! A state is the samples of every curve two walls meet along, between its
//! ends, and the rays every wall's circles take. A round samples the meets on
//! the last state's rays, then the contacts again, with what each wall is
//! given on its own as the sampling last gave it. Two states are one when no
//! sample of an edge stands further than the body's `eps` from the nearest of
//! the other's for that edge, both ways, and no ray of a wall does — a ray
//! measured where it meets the wall's circle, so a direction moved by `δ` on
//! a wall of radius `r` has moved `r·δ`.

use std::cell::RefCell;
use std::collections::HashMap;

/// How many rounds past the sampling's own are run before calling a state
/// unsettled.
pub const CAP: usize = 8;

/// How the rounds behave past the sampling's own.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Settling {
    /// No curve two walls meet along: a round has nothing to sample.
    NoMeet,
    /// One more round leaves every sample and every ray where it was.
    Settled,
    /// The state stops moving after this many rounds more.
    Later(usize),
    /// The state comes back to one already met, this many rounds before.
    Cycles(usize),
    /// Still moving at the cap, with more samples and rays than ever before.
    Grows,
    /// Still moving at the cap, neither come back nor grown.
    Drifts,
    /// The rounds run aside panicked.
    Broke,
}

/// One tessellation's rounds.
#[derive(Clone, Debug)]
pub struct Record {
    pub settling: Settling,
    /// How many samples and rays each state holds, the sampling's own first.
    pub sizes: Vec<(usize, usize)>,
    /// The furthest one more round moves a sample or a ray; infinite where
    /// a curve or a wall gains or loses all of them.
    pub moved: f64,
    /// Whether one more round gives every sample and ray bit for bit.
    pub exact: bool,
    /// Whether any wall takes a ray in the sampling's own state.
    pub rays: bool,
    /// The drawn samples of the meets found on none of the copies': zero
    /// when the copy ran the same rounds as the sampling.
    pub astray: usize,
    /// Whether one more round moves a curve a cone meets a wall along, and
    /// one whose other wall also meets a wall that is no cone.
    pub on_a_cone: bool,
    pub chained: bool,
    /// How long the copy of the sampling's rounds took, then the rounds
    /// past them, in microseconds.
    pub micros: [u64; 2],
}

/// What was measured since [`arm`], and each body's record by its print,
/// so that a body drawn again is not measured again.
type Measured = (Vec<Record>, HashMap<u64, Record>);

thread_local! {
    static RECORDS: RefCell<Option<Measured>> = const { RefCell::new(None) };
}

/// Starts measuring every tessellation on this thread.
pub fn arm() {
    RECORDS.with(|records| *records.borrow_mut() = Some((Vec::new(), HashMap::new())));
}

/// What was measured since [`arm`], which stops the measuring.
pub fn taken() -> Vec<Record> {
    RECORDS.with(|records| {
        records
            .borrow_mut()
            .take()
            .map_or_else(Vec::new, |(taken, _)| taken)
    })
}

pub(in crate::brep) fn armed() -> bool {
    RECORDS.with(|records| records.borrow().is_some())
}

/// Whether the body printed as `key` was measured already, its record
/// taken once more if so.
pub(in crate::brep) fn recalled(key: u64) -> bool {
    RECORDS.with(|records| {
        let mut records = records.borrow_mut();
        let Some((taken, known)) = records.as_mut() else {
            return false;
        };
        let Some(record) = known.get(&key) else {
            return false;
        };
        taken.push(Record {
            micros: [0; 2],
            ..record.clone()
        });
        true
    })
}

pub(in crate::brep) fn record(key: u64, record: Record) {
    RECORDS.with(|records| {
        if let Some((taken, known)) = records.borrow_mut().as_mut() {
            known.insert(key, record.clone());
            taken.push(record);
        }
    });
}
