//! What sketch · element.rs is held to.
//!
//! Closes #336.
//! - one of every kind is drawn, and a fifth kind stops the build at the match
//!   rather than at an assertion — no test: the fixture is `kinds` beside this
//!   file, whose matches carry no wildcard arm, and it is the compiler that
//!   refuses. Tried: a fifth variant on `Element` fails with `E0004` at nine
//!   matches and at not one assertion
//! - erasing answers for every kind — `erasing_answers_for_one_of_every_kind`
//! - the box, the click and the copy answer for every kind too — no test: each
//!   test lives beside the sweep it covers, in banding.rs, picking.rs and
//!   duplicating.rs
//! - a ratchet names the four sweeps and their tests — no test: it is
//!   OPERATIONS_THAT_SWEEP_THE_WHOLE_DRAWING, in crates/app/tests/architecture.rs
//! - the .caopart round trip is written down as owed rather than dropped — no
//!   test: it is owed, in that same constant's documentation

use super::kinds::{name_of, one_of_every_kind, still_drawn};
use crate::plane::WorkPlane;
use crate::sketch::Sketch;

#[test]
fn erasing_answers_for_one_of_every_kind() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let drawn = one_of_every_kind(&mut sketch);

    for element in drawn {
        assert!(
            still_drawn(&sketch, element),
            "the fixture drew a {} that the drawing does not hold",
            name_of(&element),
        );
        sketch.erase(element);
        assert!(
            !still_drawn(&sketch, element),
            "a {} was erased and the drawing still holds it",
            name_of(&element),
        );
    }
}
