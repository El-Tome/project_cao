//! Reading what is settled asks for every direction the drawing may still move
//! in. The arithmetic behind that is cubic and stays cubic; the copies made on
//! top of it were not part of the answer. A basis rebuilt on every column
//! allocated once per row it already held, so the count followed the square of
//! the drawing.
//!
//! Allocations rather than milliseconds: the count is exactly what the change
//! removes, and it does not answer to what else the machine is doing.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

use cao_sketch::{Sketch, WorkPlane};
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

/// A chain of segments hanging off the origin: nothing holds it, so every
/// unknown comes back as a free direction and the search does its full work.
fn chain(segments: usize) -> Sketch {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let mut previous = Sketch::ORIGIN;
    for step in 1..=segments {
        let next = sketch.add_point(DVec2::new(step as f64 * 10.0, 0.0));
        sketch.add_segment(previous, next);
        previous = next;
    }
    sketch
}

fn allocations_reading_what_is_settled(segments: usize) -> usize {
    let sketch = chain(segments);

    let before = ALLOCATIONS.load(Ordering::Relaxed);
    let settled = sketch.settled_points(SCALE);
    let after = ALLOCATIONS.load(Ordering::Relaxed);

    assert_eq!(settled.len(), segments + 1, "the whole chain was read");
    after - before
}

#[test]
fn twice_the_drawing_costs_twice_the_allocations_and_not_four_times() {
    let small = allocations_reading_what_is_settled(40);
    let large = allocations_reading_what_is_settled(80);

    assert!(
        large * 2 < small * 5,
        "twice the drawing allocated {large} against {small}: \
         the count is following the square of the drawing, not its size",
    );
}
