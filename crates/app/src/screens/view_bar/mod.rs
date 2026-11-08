//! The bar at the bottom right of the view that gathers the view's own tools:
//! « Recadrer », « Vue en coupe », « Matière devant » and the cursor's
//! coordinates.

// Only its tests read it until the bar is drawn.
#[cfg_attr(not(test), expect(dead_code))]
mod state;
mod view;
