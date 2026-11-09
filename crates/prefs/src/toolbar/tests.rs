//! What prefs · toolbar.rs is held to.
//!
//! Closes #731. How the ribbon draws what is stored here is held in
//! `crates/app/tests/ribbon_groups.rs`.
//! - The size is a setting of the group: large buttons, or small ones on 2 or
//!   3 rows — `a_group_built_without_a_size_is_large`
//! - stored with the toolbar layout —
//!   `a_group_s_size_is_stored_with_the_toolbar_layout`
//! - the default layout sets it: `edit` small on 2 rows, `drawing`, `extrusion`,
//!   `export` and `sketch` large — `the_default_layout_sets_the_size_of_each_group`,
//!   which now holds the groups of #732 too
//! - decided: a group can ask to be one split button whatever its depth, with
//!   a fourth size `Menu`, which `stl` carries —
//!   `the_stl_group_asks_to_be_one_split_button`
//! - editing it in the preferences — no test: out of scope, the issue of the
//!   toolbar editor; until then the default layout sets it
//! - every command of today's ribbon is still there, at the same place in the
//!   tree — no test: #732 regroups the default tree, and what it holds is held
//!   by `the_default_sketch_tab_holds_its_five_groups_and_their_commands_in_order`
//!   and `the_default_extrusion_and_export_tabs_hold_their_commands_at_today_s_places`
//! - old settings are ignored — no test: no migration is written, and the
//!   tests that old settings open on defaults, in `profiles/tests.rs`, stand

use super::*;

fn layout() -> ToolbarLayout {
    ToolbarLayout {
        items: vec![
            Item::Command(Command::Undo),
            Item::group("G", vec![Item::Command(Command::Redo)]),
            Item::Command(Command::NewSketch),
        ],
        ..ToolbarLayout::default()
    }
}

#[test]
fn an_entry_is_found_by_its_path() {
    let layout = layout();
    assert_eq!(layout.at(&[0]), Some(&Item::Command(Command::Undo)));
    assert_eq!(layout.at(&[1, 0]), Some(&Item::Command(Command::Redo)));
    assert_eq!(layout.at(&[9]), None);
    assert_eq!(layout.at(&[0, 0]), None, "a command has no children");
}

#[test]
fn shifting_swaps_with_the_neighbour() {
    let mut layout = layout();
    assert_eq!(layout.shift(&[0], true), Some(vec![1]));
    assert_eq!(layout.at(&[1]), Some(&Item::Command(Command::Undo)));
    assert_eq!(layout.shift(&[0], false), None, "nothing before the first");
}

/// Groups within groups within groups : nesting has no floor.
#[test]
fn nesting_goes_as_deep_as_asked() {
    let mut layout = ToolbarLayout {
        items: vec![
            Item::group("A", vec![]),
            Item::group("B", vec![]),
            Item::Command(Command::Undo),
        ],
        ..ToolbarLayout::default()
    };

    assert_eq!(layout.nest(&[2]), Some(vec![1, 0]), "into B");
    assert_eq!(layout.nest(&[1]), Some(vec![0, 0]), "B into A");
    assert_eq!(layout.at(&[0, 0, 0]), Some(&Item::Command(Command::Undo)));
}

#[test]
fn an_entry_comes_back_out_next_to_its_group() {
    let mut layout = layout();
    assert_eq!(layout.unnest(&[1, 0]), Some(vec![2]));
    assert_eq!(layout.at(&[2]), Some(&Item::Command(Command::Redo)));
    assert_eq!(layout.at(&[3]), Some(&Item::Command(Command::NewSketch)));
    assert_eq!(layout.unnest(&[0]), None, "already at the root");
}

#[test]
fn nesting_needs_a_group_to_go_into() {
    let mut layout = layout();
    assert_eq!(layout.nest(&[1]), None, "what comes before it is a command");
}

#[test]
fn an_entry_can_be_taken_out_and_put_back() {
    let mut layout = layout();
    let taken = layout
        .remove(&[1, 0])
        .expect("the command inside the group");
    assert_eq!(taken, Item::Command(Command::Redo));
    assert!(layout.push_into(&[1], taken));
    assert_eq!(layout.at(&[1, 0]), Some(&Item::Command(Command::Redo)));
}

#[test]
fn only_a_group_can_be_renamed() {
    let mut layout = layout();
    assert!(layout.rename(&[1], "Other".to_string()));
    assert!(matches!(layout.at(&[1]), Some(Item::Group { name, .. }) if name == "Other"));
    assert!(!layout.rename(&[0], "Nothing".to_string()));
}

fn size_of(items: &[Item], wanted: &str) -> Option<GroupSize> {
    items.iter().find_map(|item| match item {
        Item::Group { name, size, .. } if name == wanted => Some(*size),
        Item::Group { items, .. } => size_of(items, wanted),
        _ => None,
    })
}

#[test]
fn a_group_built_without_a_size_is_large() {
    let group = [Item::group("G", vec![])];
    assert_eq!(size_of(&group, "G"), Some(GroupSize::Large));
}

#[test]
fn a_group_s_size_is_stored_with_the_toolbar_layout() {
    for size in [
        GroupSize::Large,
        GroupSize::SmallTwoRows,
        GroupSize::SmallThreeRows,
        GroupSize::Menu,
    ] {
        let saved = ToolbarLayout {
            items: vec![Item::Group {
                name: "G".to_string(),
                size,
                items: vec![Item::Command(Command::Undo)],
            }],
            ..ToolbarLayout::default()
        };

        let written = serde_json::to_string(&saved).expect("a layout writes itself out");
        let read: ToolbarLayout =
            serde_json::from_str(&written).expect("a layout reads itself back");

        assert_eq!(
            size_of(&read.items, "G"),
            Some(size),
            "read back from {written}"
        );
    }
}

#[test]
fn the_default_layout_sets_the_size_of_each_group() {
    let layout = ToolbarLayout::default();
    for (group, expected) in [
        ("new_sketch", GroupSize::Large),
        ("drawing", GroupSize::Large),
        ("dimensions", GroupSize::SmallThreeRows),
        ("modify", GroupSize::SmallThreeRows),
        ("edit", GroupSize::SmallTwoRows),
        ("extrusion", GroupSize::Large),
        ("export", GroupSize::Large),
        ("sketch", GroupSize::Large),
    ] {
        assert_eq!(
            size_of(&layout.items, group),
            Some(expected),
            "the group {group:?}"
        );
    }
}

#[test]
fn the_stl_group_asks_to_be_one_split_button() {
    let layout = ToolbarLayout::default();
    assert_eq!(size_of(&layout.items, "stl"), Some(GroupSize::Menu));
}

fn places(items: &[Item], within: &str) -> Vec<String> {
    items
        .iter()
        .flat_map(|item| match item {
            Item::Command(command) => vec![format!("{within}/{command:?}")],
            Item::Separator => vec![format!("{within}/|")],
            Item::Group { name, items, .. } => places(items, &format!("{within}/{name}")),
        })
        .collect()
}

fn tab<'a>(layout: &'a ToolbarLayout, name: &str) -> &'a [Item] {
    layout
        .items
        .iter()
        .find_map(|item| match item {
            Item::Group {
                name: named, items, ..
            } if named == name => Some(items.as_slice()),
            _ => None,
        })
        .unwrap_or_else(|| panic!("the default layout has no tab {name:?}"))
}

fn groups_of(items: &[Item]) -> Vec<(&str, GroupSize)> {
    items
        .iter()
        .filter_map(|item| match item {
            Item::Group { name, size, .. } => Some((name.as_str(), *size)),
            _ => None,
        })
        .collect()
}

#[test]
fn the_default_sketch_tab_holds_its_five_groups_and_their_commands_in_order() {
    let layout = ToolbarLayout::default();
    let sketch = tab(&layout, "sketch");

    assert_eq!(
        groups_of(sketch),
        [
            ("new_sketch", GroupSize::Large),
            ("drawing", GroupSize::Large),
            ("dimensions", GroupSize::SmallThreeRows),
            ("modify", GroupSize::SmallThreeRows),
            ("edit", GroupSize::SmallTwoRows),
        ],
    );
    assert_eq!(
        places(sketch, ""),
        [
            "/new_sketch/NewSketch",
            "/drawing/ToolSelect",
            "/drawing/ToolLine",
            "/drawing/ToolLineSymmetric",
            "/drawing/ToolRectangle",
            "/drawing/ToolCircle",
            "/drawing/circles/CircleCenter",
            "/drawing/circles/CircleTwoPoints",
            "/drawing/circles/CircleThreePoints",
            "/drawing/circles/CircleTwoTangents",
            "/drawing/circles/CircleThreeTangents",
            "/drawing/ToolArc",
            "/drawing/arcs/ArcByCenter",
            "/drawing/arcs/ArcByEnds",
            "/drawing/ToolEllipse",
            "/drawing/ellipses/EllipseByCentre",
            "/drawing/ellipses/EllipseByEnds",
            "/drawing/ToolPoint",
            "/dimensions/ToolDimension",
            "/dimensions/ToolMeasure",
            "/dimensions/constraints/RulePerpendicular",
            "/dimensions/constraints/RuleParallel",
            "/dimensions/constraints/RuleEqual",
            "/dimensions/constraints/RuleCoincident",
            "/dimensions/constraints/RuleCollinear",
            "/dimensions/constraints/RuleTangent",
            "/dimensions/constraints/RuleMidpoint",
            "/dimensions/constraints/RuleFixed",
            "/dimensions/constraints/RuleConcentric",
            "/modify/ToolTrim",
            "/modify/ToolSplit",
            "/modify/ToolMirror",
            "/modify/ToolFillet",
            "/modify/ToolChamfer",
            "/modify/chamfers/ChamferEqual",
            "/modify/chamfers/ChamferAngled",
            "/modify/chamfers/ChamferSided",
            "/modify/ToggleConstruction",
            "/modify/ToolCircularPattern",
            "/modify/ToolRectangularPattern",
            "/edit/Undo",
            "/edit/Redo",
        ],
    );
}

#[test]
fn the_default_extrusion_and_export_tabs_hold_their_commands_at_today_s_places() {
    let layout = ToolbarLayout::default();

    assert_eq!(
        places(tab(&layout, "extrusion"), ""),
        [
            "/ExtrusionAdd",
            "/ExtrusionCut",
            "/|",
            "/ExtrusionStraight",
            "/ExtrusionRevolution",
        ],
    );
    assert_eq!(
        places(tab(&layout, "export"), ""),
        [
            "/stl/ExportStlCoarse",
            "/stl/ExportStlNormal",
            "/stl/ExportStlFine",
            "/ExportStep",
        ],
    );
}

#[test]
fn the_four_moved_commands_are_nowhere_in_the_default_tree_and_keep_their_keys() {
    const MOVED: [Command; 4] = [
        Command::Reframe,
        Command::SectionView,
        Command::NextMatterInFront,
        Command::FinishSketch,
    ];

    let held = places(&ToolbarLayout::default().items, "");
    let found: Vec<&String> = held
        .iter()
        .filter(|place| {
            MOVED
                .iter()
                .any(|command| place.ends_with(&format!("/{command:?}")))
        })
        .collect();
    assert!(found.is_empty(), "the default tree still holds {found:?}");

    for command in MOVED {
        assert!(
            Command::ALL.contains(&command),
            "{command:?} stopped being a command",
        );
    }
    let keys = crate::Shortcuts::default();
    assert_eq!(
        keys.chord_for(Command::Reframe),
        Some(crate::Chord::new(crate::Key::F)),
    );
    assert_eq!(
        keys.chord_for(Command::FinishSketch),
        Some(crate::Chord::new(crate::Key::Enter)),
    );
}
