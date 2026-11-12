use cao_part::PartDocument;
use cao_part::history::BlendShape;
use cao_prefs::Command;
use cao_sketch::Rule;

use crate::screens::SketchContext;
use crate::screens::extrusion::apply_extrusion_with;
use crate::screens::ribbon::Ribbon;
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::{
    ViewportState, corner_picks_with, swap_held_chamfer, switch_held_cuts, take_everything,
    take_up_the_copy,
};
use crate::wording;

mod asked;
mod compaction;
mod framing;
pub(crate) use asked::Asked;
pub(crate) use compaction::compact_history;
pub(crate) use framing::sketch_framing;

/// Carries out one command, wherever it came from.
///
/// A button and a shortcut both end up here: two paths would drift apart, and
/// a shortcut that does almost what its button does is worse than none.
/// Returns true when the part changed and needs saving.
pub(crate) fn run(
    asked: impl Into<Asked>,
    part: &mut SketchContext<'_>,
    ribbon: &mut Ribbon,
    viewport: &mut ViewportState,
) -> bool {
    use crate::screens::extrusion::Shape;
    use crate::screens::sketch::{
        ArcMode, ChamferMode, CircleMode, DimensionMode, EllipseMode, Tool,
    };

    let tool = |editor: &mut SketchEditor, wanted: Tool| {
        editor.tool = wanted;
        editor.reset_pending();
    };

    let SketchContext {
        document: doc,
        editor,
        extrusion,
        lang,
    } = part;
    editor.message = None;

    let command = match asked.into() {
        Asked::Command(command) => command,
        Asked::MatterInFront(state) => {
            editor.set_matter_in_front(state);
            return false;
        }
    };
    match command {
        // The shell keeps these: they reach the window or the disk, which
        // the part's state has no business holding.
        Command::OpenSettings
        | Command::BackToMenu
        | Command::ExportStlCoarse
        | Command::ExportStlNormal
        | Command::ExportStlFine
        | Command::ExportStep => false,
        Command::NewSketch => {
            extrusion.close();
            ribbon.blend.close();
            editor.start_choosing_plane(lang);
            false
        }
        // Finishing a drawing is where an extrusion naturally begins, so the
        // tool is offered right there rather than left to be found again.
        Command::FinishSketch => {
            if let Some(index) = editor.active_sketch() {
                extrusion.offer(index);
                ribbon.leave_sketch();
            }
            editor.close();
            false
        }
        Command::Reframe => {
            match editor.active_sketch().and(editor.plane) {
                Some(plane) => {
                    let (center, radius) = sketch_framing(doc, editor.active_sketch(), plane);
                    viewport.look_at_plane(plane, center, radius);
                }
                None => viewport.frame_the_part(doc.body()),
            }
            false
        }
        Command::OriginView => {
            viewport.turn_to_the_opening_view(doc.body());
            false
        }
        Command::NextMatterInFront => {
            editor.next_matter_in_front();
            false
        }
        Command::SectionView => {
            viewport.toggle_section(doc.body());
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
            extrusion.history_moved();
            viewport.light_out();
            if doc.body().is_empty() {
                viewport.close_section();
            }
            changed
        }
        // Compaction itself waits for its warning to be confirmed; this only
        // opens it.
        Command::CompactHistory => {
            ribbon.history_unfolded = true;
            ribbon.history_compact_confirm = true;
            false
        }
        Command::ToolSelect => {
            tool(editor, Tool::Select);
            false
        }
        Command::SelectAll => {
            take_everything(editor, doc);
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
        Command::ToolEllipse => {
            tool(editor, Tool::Ellipse);
            editor.message = Some(crate::wording::ellipse::asks_for(lang, editor.ellipse_mode));
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
        Command::ToolMeasure => {
            tool(editor, Tool::Measure);
            editor.message = Some(lang.t("sketch.point_at_what_to_measure"));
            false
        }
        Command::ToolTrim => {
            tool(editor, Tool::Trim);
            editor.message = Some(lang.t("sketch.click_a_stretch"));
            false
        }
        Command::ToolSplit => {
            tool(editor, Tool::Split);
            editor.message = Some(lang.t("sketch.click_a_crossing"));
            false
        }
        Command::ToolChamfer | Command::ToolFillet if editor.active_sketch().is_none() => {
            let shape = match command {
                Command::ToolChamfer => BlendShape::Chamfer,
                _ => BlendShape::Fillet,
            };
            extrusion.close();
            ribbon.blend.offer(shape);
            editor.message = wording::command::hint_on_the_part(lang, command);
            false
        }
        Command::ToolChamfer => {
            tool(editor, Tool::Chamfer);
            editor.message = Some(lang.t(corner_picks_with(Tool::Chamfer, editor.chamfer_mode)));
            false
        }
        Command::ChamferEqual | Command::ChamferAngled | Command::ChamferSided => {
            editor.chamfer_mode = match command {
                Command::ChamferAngled => ChamferMode::Angled,
                Command::ChamferSided => ChamferMode::Sided,
                _ => ChamferMode::Equal,
            };
            // `tool` empties whatever the previous mode had taken: a corner
            // named by its point gives an asymmetric mode no first side.
            tool(editor, Tool::Chamfer);
            editor.message = Some(lang.t(corner_picks_with(Tool::Chamfer, editor.chamfer_mode)));
            false
        }
        Command::ToolFillet => {
            tool(editor, Tool::Fillet);
            editor.message = Some(lang.t(corner_picks_with(Tool::Fillet, editor.chamfer_mode)));
            false
        }
        Command::ToolMirror | Command::ToolCircularPattern | Command::ToolRectangularPattern => {
            let held = editor.held_elements();
            let which = match command {
                Command::ToolCircularPattern => Tool::CircularPattern,
                Command::ToolRectangularPattern => Tool::RectangularPattern,
                _ => Tool::Mirror,
            };
            tool(editor, which);
            let mut context = SketchContext {
                document: doc,
                editor,
                extrusion,
                lang,
            };
            take_up_the_copy(&mut context, held);
            false
        }
        // With a fillet or a chamfer held, the toggle switches it; otherwise it
        // says what the next shape is drawn as.
        Command::ToggleConstruction => {
            let mut context = SketchContext {
                document: doc,
                editor,
                extrusion,
                lang,
            };
            if switch_held_cuts(&mut context) {
                return true;
            }
            context.editor.construction = !context.editor.construction;
            false
        }
        Command::SwapChamferSides => {
            let mut context = SketchContext {
                document: doc,
                editor,
                extrusion,
                lang,
            };
            swap_held_chamfer(&mut context)
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
        Command::EllipseByCentre | Command::EllipseByEnds => {
            editor.ellipse_mode = match command {
                Command::EllipseByEnds => EllipseMode::ByEnds,
                _ => EllipseMode::ByCentre,
            };
            tool(editor, Tool::Ellipse);
            editor.message = Some(crate::wording::ellipse::asks_for(lang, editor.ellipse_mode));
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
            ribbon.blend.close();
            extrusion.arm(cao_part::ExtrusionMode::Add);
            false
        }
        Command::ExtrusionCut => {
            ribbon.blend.close();
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
            let record = |doc: &mut PartDocument, operation| {
                viewport.record_where_read(doc, operation);
            };
            let changed = apply_extrusion_with(doc, extrusion, &mut editor.message, lang, record);
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
            ribbon.history_unfolded = !ribbon.part_tree_open || !ribbon.history_unfolded;
            ribbon.part_tree_open = true;
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

#[cfg(test)]
mod tests;
