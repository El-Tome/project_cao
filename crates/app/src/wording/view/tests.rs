//! What app · wording/view.rs is held to: the name the title bar gives the
//! view, in its pill at the end of the breadcrumb.
//!
//! Closes #726.
//! - the view pill reads « Esquisse N » when a sketch is open, N its rank as
//!   the Part tree words it — `an_open_sketch_names_the_view_by_its_rank`, and
//!   ahead of the section view, so that the two cannot trade places unseen —
//!   `a_sketch_open_with_the_section_view_on_names_the_view_by_the_sketch`
//! - and so ahead of the kept section the sketch stands on, which the bar read
//!   before — `a_sketch_open_on_a_kept_section_names_the_view_by_the_sketch`
//! - else « Vue en coupe » while the section view is on, a kept section shown
//!   or not — `with_the_section_view_on_the_pill_names_the_section_view`
//! - else « Coupe N » while a kept section is shown —
//!   `a_kept_section_shown_names_the_view_by_its_rank`
//! - else the plane's label while the view stands on a plane with no sketch
//!   open, as today — `a_view_landed_on_a_plane_names_the_plane`
//! - else « Vue 3D libre » — `the_free_view_is_named_the_free_3d_view`
//!
//! The order is the rule, so each case is written with every fact that the
//! cases below it would answer to, and has to win over them. Ranks are handed
//! in as the Part tree counts them, from one: the wording prints a rank and
//! does not work one out, since which sketch is the third is the tree's
//! business. That `crates/app/tests/title_bar.rs` hands it what the screen
//! shows is held there.

use super::*;

fn named(standing: Standing) -> String {
    standing_on(&Catalogue::french(), standing)
}

#[test]
fn an_open_sketch_names_the_view_by_its_rank() {
    let view = named(Standing {
        open_sketch_rank: Some(3),
        section_view_on: false,
        kept_section_rank: None,
        plane: Some(WorkPlane::XY),
    });

    assert_eq!(view, "Esquisse 3");
}

#[test]
fn a_sketch_open_on_a_kept_section_names_the_view_by_the_sketch() {
    let view = named(Standing {
        open_sketch_rank: Some(2),
        section_view_on: false,
        kept_section_rank: Some(1),
        plane: Some(WorkPlane::YZ),
    });

    assert_eq!(
        view, "Esquisse 2",
        "a sketch open on a kept section is named after the section rather than itself",
    );
}

#[test]
fn a_sketch_open_with_the_section_view_on_names_the_view_by_the_sketch() {
    let view = named(Standing {
        open_sketch_rank: Some(2),
        section_view_on: true,
        kept_section_rank: None,
        plane: Some(WorkPlane::YZ),
    });

    assert_eq!(
        view, "Esquisse 2",
        "a sketch open with the section view on is named after the section view",
    );
}

#[test]
fn with_the_section_view_on_the_pill_names_the_section_view() {
    let view = named(Standing {
        open_sketch_rank: None,
        section_view_on: true,
        kept_section_rank: Some(1),
        plane: Some(WorkPlane::XY),
    });

    assert_eq!(view, "Vue en coupe");
}

#[test]
fn a_kept_section_shown_names_the_view_by_its_rank() {
    let view = named(Standing {
        open_sketch_rank: None,
        section_view_on: false,
        kept_section_rank: Some(2),
        plane: Some(WorkPlane::XY),
    });

    assert_eq!(view, "Coupe 2");
}

#[test]
fn a_view_landed_on_a_plane_names_the_plane() {
    let view = named(Standing {
        open_sketch_rank: None,
        section_view_on: false,
        kept_section_rank: None,
        plane: Some(WorkPlane::YZ),
    });

    assert_eq!(view, "Plan YZ");
}

#[test]
fn the_free_view_is_named_the_free_3d_view() {
    let view = named(Standing {
        open_sketch_rank: None,
        section_view_on: false,
        kept_section_rank: None,
        plane: None,
    });

    assert_eq!(view, "Vue 3D libre");
}
