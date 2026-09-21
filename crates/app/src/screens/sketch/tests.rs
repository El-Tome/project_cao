//! What app · screens/sketch.rs is held to.
//!
//! Closes #401.
//! - pointing the trim tool at a curve no longer lights the whole of it —
//!   `only_the_tool_that_takes_the_whole_thing_lights_the_whole_of_it`
//! - pointing Sélectionner at something still lights the whole of it —
//!   `only_the_tool_that_takes_the_whole_thing_lights_the_whole_of_it`

use super::*;

#[test]
fn only_the_tool_that_takes_the_whole_thing_lights_the_whole_of_it() {
    assert!(
        Tool::Select.lights_the_whole_of_it(),
        "picking takes the whole of what is pointed at",
    );

    for tool in [Tool::Trim, Tool::Split, Tool::Chamfer, Tool::Fillet] {
        assert!(
            !tool.lights_the_whole_of_it(),
            "{tool:?} takes a stretch, and lighting the whole promises otherwise",
        );
    }
}
