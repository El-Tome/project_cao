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
//!
//! Closes #725.
//! - every `Command` has an icon, looked up by command; a test fails on the
//!   first command without one —
//!   `every_command_wears_the_icon_the_redesign_drew_for_it`
//! - the SVGs are embedded — `every_command_wears_the_icon_the_redesign_drew_for_it`,
//!   which holds each icon's bytes against the file `docs/redesign/icons/`
//!   draws it in

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

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

#[test]
fn every_command_wears_the_icon_the_redesign_drew_for_it() {
    let redesign = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/redesign/icons");

    for &command in EVERY_COMMAND {
        let worn = icon(command);
        assert_eq!(
            worn.id(),
            drawn_for(command),
            "{command:?} wears another icon than the one docs/redesign/icons.md gives it",
        );

        let file = redesign.join(format!("{}.svg", worn.id()));
        let drawn = fs::read(&file).unwrap_or_else(|_| {
            panic!(
                "{command:?} wears `{}`, which the redesign never drew: there is no {}",
                worn.id(),
                file.display(),
            )
        });
        assert!(
            worn.svg() == drawn.as_slice(),
            "{command:?} wears `{}` but its bytes are not those of {}",
            worn.id(),
            file.display(),
        );
    }
}

/// Each command with the icon `docs/redesign/icons.md` gives it, written once.
///
/// The `match` drawn from the list has no wildcard, so a command added to
/// `cao_prefs` stops this file compiling until it is given an icon here.
/// `Command::ALL` could not say as much: it is the palette, and leaves out the
/// two commands the window keeps for itself.
macro_rules! icons_drawn {
    ($($command:ident => $icon:literal),+ $(,)?) => {
        const EVERY_COMMAND: &[Command] = &[$(Command::$command),+];

        fn drawn_for(command: Command) -> &'static str {
            match command {
                $(Command::$command => $icon),+
            }
        }
    };
}

icons_drawn! {
    NewSketch => "new-sketch",
    FinishSketch => "finish",
    RecenterOnSketch => "reframe",
    NextMatterInFront => "matter-in-front-hidden",
    SectionView => "section-view",
    Undo => "undo",
    Redo => "redo",
    CompactHistory => "compact-history",
    SelectAll => "select",
    ToolSelect => "select",
    ToolLine => "line",
    ToolLineSymmetric => "line-symmetric",
    ToolRectangle => "rectangle",
    ToolCircle => "circle-centre",
    ToolArc => "arc-by-centre",
    ToolEllipse => "ellipse-by-centre",
    ToolPoint => "point",
    ToolDimension => "dimension",
    ToolMeasure => "measure",
    ToolTrim => "trim",
    ToolSplit => "split",
    ToolChamfer => "chamfer",
    ToolFillet => "fillet",
    ToolMirror => "mirror",
    ToolCircularPattern => "circular-pattern",
    ToolRectangularPattern => "rectangular-pattern",
    ToggleConstruction => "construction",
    SwapChamferSides => "swap-chamfer-sides",
    CircleCenter => "circle-centre",
    CircleTwoPoints => "circle-two-points",
    CircleThreePoints => "circle-three-points",
    CircleTwoTangents => "circle-two-tangents",
    CircleThreeTangents => "circle-three-tangents",
    ArcByCenter => "arc-by-centre",
    ArcByEnds => "arc-by-ends",
    EllipseByCentre => "ellipse-by-centre",
    EllipseByEnds => "ellipse-by-ends",
    ChamferEqual => "chamfer-equal",
    ChamferAngled => "chamfer-angled",
    ChamferSided => "chamfer-sided",
    DimensionAuto => "dimension-auto",
    DimensionPointToPoint => "dimension-point-to-point",
    DimensionLength => "dimension-length",
    DimensionAngle => "dimension-angle",
    DimensionRadius => "dimension-radius",
    RulePerpendicular => "perpendicular",
    RuleParallel => "parallel",
    RuleEqual => "equal",
    RuleCoincident => "coincident",
    RuleCollinear => "collinear",
    RuleTangent => "tangent",
    RuleMidpoint => "midpoint",
    RuleFixed => "fixed",
    RuleConcentric => "concentric",
    ExtrusionAdd => "extrusion-add",
    ExtrusionCut => "extrusion-cut",
    ExtrusionStraight => "extrusion-straight",
    ExtrusionRevolution => "revolution",
    ExtrusionApply => "extrusion-apply",
    ExtrusionCancel => "extrusion-cancel",
    ExportStlCoarse => "stl-coarse",
    ExportStlNormal => "stl-normal",
    ExportStlFine => "stl-fine",
    ExportStep => "export-step",
    ToggleHistory => "history",
    ToggleExplorer => "files",
    ToggleToolbarDocked => "dock-toolbar",
    OpenSettings => "preferences",
    BackToMenu => "home",
}
