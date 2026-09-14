use cao_part::PartDocument;
use cao_prefs::Command;
use cao_sketch::{Rule, WorkPlane};
use glam::DVec3;

use crate::lang::Catalogue;
use crate::screens::extrusion::{ExtrusionState, apply_extrusion};
use crate::screens::ribbon::Ribbon;
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::ViewportState;
use crate::{screens, wording};

/// Carries out one command, wherever it came from.
///
/// A button and a shortcut both end up here: two paths would drift apart, and
/// a shortcut that does almost what its button does is worse than none.
/// Returns true when the part changed and needs saving.
pub(crate) fn run(
    command: Command,
    doc: &mut PartDocument,
    editor: &mut SketchEditor,
    extrusion: &mut ExtrusionState,
    ribbon: &mut Ribbon,
    viewport: &mut ViewportState,
    lang: &Catalogue,
) -> bool {
    use crate::screens::extrusion::Shape;
    use crate::screens::sketch::{ArcMode, CircleMode, DimensionMode, Tool};

    let tool = |editor: &mut SketchEditor, wanted: Tool| {
        editor.tool = wanted;
        editor.reset_pending();
    };

    editor.message = None;

    match command {
        Command::OpenSettings | Command::BackToMenu => false,
        Command::NewSketch => {
            extrusion.close();
            editor.start_choosing_plane(lang);
            false
        }
        // Finishing a drawing is where an extrusion naturally begins, so the
        // tool is offered right there rather than left to be found again.
        Command::FinishSketch => {
            if let Some(index) = editor.active_sketch() {
                extrusion.offer(index);
                ribbon.tab = 1;
            }
            editor.close();
            false
        }
        Command::RecenterOnSketch => {
            if let Some(plane) = editor.plane {
                let (center, radius) = sketch_framing(doc, editor.active_sketch(), plane);
                viewport.look_at_plane(plane, center, radius);
            }
            false
        }
        Command::Undo | Command::Redo => {
            let changed = if command == Command::Undo {
                doc.undo()
            } else {
                doc.redo()
            };
            if changed {
                clamp_editor_to_document(editor, doc);
            }
            changed
        }
        // Compaction itself waits for the warning in the history panel to be
        // confirmed; this only opens it.
        Command::CompactHistory => {
            ribbon.history_open = true;
            ribbon.history_compact_confirm = true;
            false
        }
        Command::ToolSelect => {
            tool(editor, Tool::Select);
            false
        }
        Command::ToolLine => {
            tool(editor, Tool::Line);
            false
        }
        Command::ToolLineSymmetric => {
            tool(editor, Tool::LineSymmetric);
            false
        }
        Command::ToolRectangle => {
            tool(editor, Tool::Rectangle);
            false
        }
        Command::ToolCircle => {
            tool(editor, Tool::Circle);
            false
        }
        Command::ToolArc => {
            tool(editor, Tool::Arc);
            editor.message = Some(crate::wording::arc::asks_for(lang, editor.arc_mode));
            false
        }
        Command::ToolPoint => {
            tool(editor, Tool::Point);
            false
        }
        Command::ToolDimension => {
            tool(editor, Tool::Dimension);
            false
        }
        Command::ToolTrim => {
            tool(editor, Tool::Trim);
            editor.message = Some(lang.t("sketch.click_a_stretch"));
            false
        }
        Command::ToggleConstruction => {
            editor.construction = !editor.construction;
            false
        }
        Command::DimensionAuto
        | Command::DimensionPointToPoint
        | Command::DimensionLength
        | Command::DimensionAngle
        | Command::DimensionRadius => {
            editor.dimension_mode = match command {
                Command::DimensionPointToPoint => DimensionMode::PointToPoint,
                Command::DimensionLength => DimensionMode::Length,
                Command::DimensionAngle => DimensionMode::Angle,
                Command::DimensionRadius => DimensionMode::Radius,
                _ => DimensionMode::Auto,
            };
            editor.reset_pending();
            false
        }
        Command::CircleCenter
        | Command::CircleTwoPoints
        | Command::CircleThreePoints
        | Command::CircleTwoTangents
        | Command::CircleThreeTangents => {
            editor.circle_mode = match command {
                Command::CircleTwoPoints => CircleMode::TwoPoints,
                Command::CircleThreePoints => CircleMode::ThreePoints,
                Command::CircleTwoTangents => CircleMode::TwoTangents,
                Command::CircleThreeTangents => CircleMode::ThreeTangents,
                _ => CircleMode::Center,
            };
            tool(editor, Tool::Circle);
            editor.message = Some(crate::wording::circle::asks_for(lang, editor.circle_mode));
            false
        }
        Command::ArcByCenter | Command::ArcByEnds => {
            editor.arc_mode = match command {
                Command::ArcByEnds => ArcMode::ByEnds,
                _ => ArcMode::ByCenter,
            };
            tool(editor, Tool::Arc);
            editor.message = Some(crate::wording::arc::asks_for(lang, editor.arc_mode));
            false
        }
        Command::RulePerpendicular
        | Command::RuleParallel
        | Command::RuleEqual
        | Command::RuleCoincident
        | Command::RuleCollinear
        | Command::RuleTangent
        | Command::RuleMidpoint
        | Command::RuleFixed
        | Command::RuleConcentric => {
            let rule = match command {
                Command::RuleParallel => Rule::Parallel,
                Command::RuleEqual => Rule::Equal,
                Command::RuleCoincident => Rule::Coincident,
                Command::RuleCollinear => Rule::Collinear,
                Command::RuleTangent => Rule::Tangent,
                Command::RuleMidpoint => Rule::Midpoint,
                Command::RuleFixed => Rule::Fixed,
                Command::RuleConcentric => Rule::Concentric,
                _ => Rule::Perpendicular,
            };
            tool(editor, Tool::Constrain(rule));
            editor.message = Some(wording::constraints::rule_asks_for(lang, rule));
            false
        }
        Command::ExtrusionAdd => {
            extrusion.arm(cao_part::ExtrusionMode::Add);
            false
        }
        Command::ExtrusionCut => {
            extrusion.arm(cao_part::ExtrusionMode::Cut);
            false
        }
        Command::ExtrusionStraight => {
            extrusion.shape = Shape::Straight;
            false
        }
        Command::ExtrusionRevolution => {
            extrusion.shape = Shape::Revolution;
            false
        }
        Command::ExtrusionCancel => {
            extrusion.close();
            ribbon.tab = 0;
            false
        }
        Command::ExtrusionApply => {
            let changed = apply_extrusion(doc, extrusion, &mut editor.message, lang);
            if changed {
                // Seen from straight above its own plane, a new prism looks
                // exactly like the drawing it came from.
                if let Some((min, max)) = doc.body().bounds() {
                    viewport.look_at_part((min + max) * 0.5, (max - min).length() * 0.6);
                }
            }
            changed
        }
        Command::ToggleHistory => {
            ribbon.history_open = !ribbon.history_open;
            false
        }
        Command::ToggleToolbarDocked => false,
    }
}

/// After the history moves, the sketch being edited may no longer exist. The
/// editor has to let go of it rather than point at nothing.
pub(crate) fn clamp_editor_to_document(editor: &mut SketchEditor, doc: &PartDocument) {
    editor.reset_pending();

    let Some(index) = editor.active_sketch() else {
        return;
    };
    match doc.sketches().get(index) {
        Some(sketch) => editor.plane = Some(sketch.plane),
        None => editor.close(),
    }
}

/// Centre and radius to frame: the sketch if it has anything in it, otherwise a
/// sensible patch of the plane around its origin.
pub(crate) fn sketch_framing(
    doc: &PartDocument,
    sketch: Option<usize>,
    plane: WorkPlane,
) -> (DVec3, f64) {
    let bounds = sketch
        .and_then(|index| doc.sketches().get(index))
        .and_then(|sketch| sketch.bounds());

    match bounds {
        Some((min, max)) if (max - min).length() > 1e-4 => {
            let center = plane.to_world((min + max) * 0.5);
            (center, (max - min).length() * 0.5)
        }
        _ => (plane.origin, screens::viewport::DEFAULT_SKETCH_RADIUS),
    }
}
