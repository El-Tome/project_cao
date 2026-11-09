//! What app · screens/ribbon/icons.rs is held to.
//!
//! Closes #725.
//! - every `Command` has an icon, looked up by command; a test fails on the
//!   first command without one —
//!   `every_command_wears_the_icon_the_redesign_drew_for_it`
//! - the SVGs are embedded — `every_command_wears_the_icon_the_redesign_drew_for_it`,
//!   which holds each icon's bytes against the file `docs/redesign/icons/`
//!   draws it in
//!
//! The lookup sits with the ribbon, its one reader, rather than in `wording/`:
//! an icon is drawn, and `wording/` says.

use std::fs;
use std::path::Path;

use cao_prefs::Command;

use super::*;

#[test]
fn every_command_wears_the_icon_the_redesign_drew_for_it() {
    let redesign = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/redesign/icons");

    for &command in EVERY_COMMAND {
        let worn = of(command);
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
    Reframe => "reframe",
    OriginView => "home-view",
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
    ToggleToolbarDocked => "dock-toolbar",
    OpenSettings => "preferences",
    BackToMenu => "home",
}
