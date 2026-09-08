//! A drawing whose values contradict each other is corrected four hundred times
//! over and still does not settle. The arithmetic of those turns is the price of
//! asking; the memory is not. Every buffer a turn needs has exactly the size it
//! had on the turn before, so a sweep that allocates is a sweep buying the same
//! vector again.
//!
//! Allocations rather than milliseconds: the count is exactly what the change
//! removes, and it does not answer to what else the machine is doing.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

use cao_sketch::{DimensionTarget, Sketch, WorkPlane};
use glam::DVec2;

static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static COUNTER: Counting = Counting;

const SCALE: f64 = 1.0;
const SIDE: f64 = 40.0;
const SIDES: usize = 20;

/// A staircase of twenty sides, every one of them given a length and every
/// corner an angle. The last side is asked for three times the length its two
/// neighbours leave room for, so the correction is made and undone until the
/// solver gives up.
fn a_drawing_that_cannot_be_satisfied() -> Sketch {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let mut previous = Sketch::ORIGIN;
    let mut sides = Vec::new();

    for step in 1..=SIDES {
        let up = (step % 2) as f64;
        let next = sketch.add_point(DVec2::new(SIDE * (step as f64 - up), SIDE * up));
        sides.push(sketch.add_segment(previous, next));
        previous = next;
    }

    for (rank, side) in sides.iter().enumerate() {
        let wanted = match rank + 1 == sides.len() {
            true => SIDE * 3.0,
            false => sketch.segment_length(*side),
        };
        sketch.set_dimension(DimensionTarget::Length(*side), wanted, false);
    }
    for pair in sides.windows(2) {
        let target = DimensionTarget::Angle {
            first: pair[0],
            second: pair[1],
        };
        sketch.set_dimension(target, 90.0, false);
    }

    sketch
}

#[test]
fn a_drawing_that_never_settles_pays_for_its_turns_in_arithmetic_only() {
    let mut sketch = a_drawing_that_cannot_be_satisfied();

    let before = ALLOCATIONS.load(Ordering::Relaxed);
    sketch.resolve(SCALE);
    let allocations = ALLOCATIONS.load(Ordering::Relaxed) - before;

    // Thirty-nine equations, four hundred turns, several rounds: the number of
    // corrections is in the hundreds of thousands. What is allowed here is what
    // reading the drawing costs once per round — a buffer bought per correction
    // put this at two million.
    assert!(
        allocations < 20_000,
        "one resolve allocated {allocations} times: a sweep is buying its \
         buffers again on every turn",
    );
}
