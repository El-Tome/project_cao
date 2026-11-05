//! What app · wording/command.rs is held to.
//!
//! Closes #580.
//! - the three levels give the gaps they promise, each hint saying the gap
//!   its export asks for, from the one figure —
//!   `each_stl_level_s_hint_says_the_gap_its_export_asks_for`
//!
//! Closes #581.
//! - to try it: each part opens in FreeCAD — the command that writes it reads
//!   « Exporter en STEP… » — `the_step_export_reads_exporter_en_step`

use std::collections::BTreeMap;

use super::*;

#[test]
fn no_two_commands_of_the_palette_read_the_same() {
    let lang = Catalogue::french();
    let mut seen: BTreeMap<String, Command> = BTreeMap::new();

    for command in Command::ALL {
        assert!(
            !label(&lang, command).is_empty(),
            "{command:?} has no name, so its button is a blank",
        );
        if let Some(taken) = seen.insert(label(&lang, command), command) {
            panic!(
                "{taken:?} and {command:?} both read {:?}: the palette offers two \
                 entries a user cannot tell apart",
                label(&lang, command),
            );
        }
    }
}

#[test]
fn resting_on_a_command_says_more_than_its_button_already_shows() {
    let lang = Catalogue::french();

    for command in Command::ALL {
        assert!(
            !hint(&lang, command).is_empty(),
            "{command:?} has nothing to say on hover",
        );
        assert_ne!(
            hint(&lang, command),
            label(&lang, command),
            "{command:?} repeats its own name on hover instead of helping",
        );
    }
}

#[test]
fn no_two_families_of_the_palette_are_headed_the_same() {
    let lang = Catalogue::french();
    let mut seen: BTreeMap<String, CommandFamily> = BTreeMap::new();

    for command in Command::ALL {
        let family = command.family();
        assert!(
            !family_heading(&lang, family).is_empty(),
            "{family:?} heads its run of the palette with a blank",
        );
        if let Some(taken) = seen.insert(family_heading(&lang, family), family) {
            assert_eq!(
                taken,
                family,
                "{taken:?} and {family:?} both head {:?}: the palette reads as \
                 one family cut in two",
                family_heading(&lang, family),
            );
        }
    }
}

#[test]
fn each_stl_level_s_hint_says_the_gap_its_export_asks_for() {
    let lang = Catalogue::french();
    let document = cao_part::PartDocument::new("part", chrono::Utc::now());

    for level in [
        Command::ExportStlCoarse,
        Command::ExportStlNormal,
        Command::ExportStlFine,
    ] {
        let gap = crate::export::export_request(level, &document)
            .expect("a level asks for an export")
            .gap;
        let figure = format!("{} mm", crate::wording::dimension::short(gap));
        let said = hint(&lang, level);

        assert!(
            said.contains(&figure),
            "{level:?} asks for {figure} and its hint reads: {said}"
        );
    }
}

#[test]
fn the_step_export_reads_exporter_en_step() {
    assert_eq!(
        label(&Catalogue::french(), Command::ExportStep),
        "Exporter en STEP…"
    );
}
