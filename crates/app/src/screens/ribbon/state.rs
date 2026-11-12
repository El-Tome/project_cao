use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::{SketchEditor, Tool};
use crate::ui::icon::Icon;
use crate::wording::{command as wording, matter};
use cao_part::PartDocument;
use cao_prefs::{Command, Settings};
use std::collections::HashMap;

use super::icons;
pub(super) use super::pressed::active;

mod runs;
pub(super) use runs::{Run, SPREAD_OUT, Split, runs};

/// The toolbar, drawn from the arrangement the user has set.
///
/// Nothing about which buttons exist or where they sit is decided here: the
/// tree comes from the settings, and this only knows how to draw a tree. That
/// is what lets the arrangement be changed, saved and handed to somebody else.
#[derive(Default)]
pub struct Ribbon {
    /// Which top-level group is open, by rank.
    pub tab: usize,
    /// The tab shown before the sketch being edited was entered.
    tab_before_sketch: Option<usize>,
    /// The rank of the sketch's tab in the layout last drawn, if it has one.
    pub(super) sketch_tab: Option<usize>,
    /// The sketch open when the ribbon last followed the editor.
    sketch_seen: Option<usize>,
    /// The tree of what the part is made of, open by default.
    pub part_tree_open: bool,
    /// Whether the run of operations that made the part stands unfolded at
    /// the bottom of that tree. Folded until asked for.
    pub history_unfolded: bool,
    /// The panel the variables are edited in, waiting to be asked for.
    pub variables_open: bool,
    /// Whether the history is asking to confirm a compaction. Held here
    /// rather than only in the panel's own frame, so the warning survives to
    /// the next one instead of closing the moment the mouse moves.
    pub history_compact_confirm: bool,
    /// The kept section the tree is asking to confirm the deletion of, held
    /// here for the reason the compaction's question is.
    pub section_delete_confirm: Option<usize>,
    /// Whether the viewport's section view is on, so its button shows pressed.
    pub section_on: bool,
    /// The sketches the part is read through, the ones the extrusion offers.
    pub sketches_read: Vec<usize>,
    /// The command last picked from each menu, by group, for the session.
    last_used: HashMap<String, Command>,
    /// How wide what a floating bar holds came out last frame.
    pub(super) floating_width: f32,
}

impl Ribbon {
    pub fn new() -> Self {
        Self {
            tab: 0,
            tab_before_sketch: None,
            sketch_tab: None,
            sketch_seen: None,
            part_tree_open: true,
            history_unfolded: false,
            variables_open: false,
            history_compact_confirm: false,
            section_delete_confirm: None,
            section_on: false,
            sketches_read: Vec::new(),
            last_used: HashMap::new(),
            floating_width: 0.0,
        }
    }

    /// The command a split button runs: the one before its menu, else what
    /// its menu last ran, else its first entry.
    pub(super) fn primary_of(
        &self,
        group: &str,
        before: Option<Command>,
        entries: &[Command],
    ) -> Option<Command> {
        if before.is_some() {
            return before;
        }
        if entries.contains(&Command::ExportStlNormal) {
            return Some(Command::ExportStlNormal);
        }
        self.last_used
            .get(group)
            .copied()
            .filter(|used| entries.contains(used))
            .or_else(|| entries.first().copied())
    }

    /// Entering a sketch turns to its tools. The tab it came from is kept
    /// from the first sketch entered, whichever is reopened after it.
    pub fn enter_sketch(&mut self) {
        self.tab_before_sketch.get_or_insert(self.tab);
        if let Some(tab) = self.sketch_tab {
            self.tab = tab;
        }
    }

    pub fn leave_sketch(&mut self) {
        if let Some(tab) = self.tab_before_sketch.take() {
            self.tab = tab;
        }
    }

    /// Once a frame, after every way into a sketch. A plane being chosen is
    /// on the way to a new sketch, so the tab from before the first is kept.
    pub fn follow(&mut self, open: Option<usize>, choosing_a_plane: bool) {
        if !choosing_a_plane {
            self.follow_the_canvas(self.sketch_seen, open);
        }
        self.sketch_seen = open;
    }

    /// A sketch left any other way than « Terminer » leaves no tab to go
    /// back to.
    pub fn follow_the_canvas(&mut self, before: Option<usize>, after: Option<usize>) {
        match after {
            None => self.tab_before_sketch = None,
            Some(_) if after != before => self.enter_sketch(),
            Some(_) => {}
        }
    }

    pub(super) fn used_from_menu(&mut self, group: &str, command: Command) {
        self.last_used.insert(group.to_owned(), command);
    }
}

/// Whether a menu right after this command is its variants. A toggle's state
/// is no tool a menu could offer variants of, and « Mesurer » only reads the
/// drawing: each may precede « Contraintes » without being one of them.
fn takes_a_menu(command: Command) -> bool {
    !matches!(command, Command::ToggleConstruction | Command::ToolMeasure)
}

pub(super) fn any_entry_enabled(entries: &[Command], state: &Context<'_>) -> bool {
    entries.iter().any(|entry| enabled(*entry, state))
}

/// What the toolbar needs to know to draw a button in the right state.
pub(super) struct Context<'a> {
    pub(super) settings: &'a Settings,
    pub(super) document: &'a PartDocument,
    pub(super) editor: &'a SketchEditor,
    pub(super) extrusion: &'a ExtrusionState,
    pub(super) lang: &'a Catalogue,
    pub(super) section_on: bool,
}

/// What the command's button reads: its name, or the state it is in for the
/// one button whose face says that.
pub(super) fn label(command: Command, state: &Context<'_>) -> String {
    match command {
        Command::NextMatterInFront => matter::in_front(state.lang, state.editor.matter_in_front),
        _ => wording::label(state.lang, command),
    }
}

/// What the command's button wears: its icon, or the state it is in for the
/// one button whose face says that.
pub(super) fn icon(command: Command, state: &Context<'_>) -> Icon {
    match command {
        Command::NextMatterInFront => icons::matter_in_front(state.editor.matter_in_front),
        _ => icons::of(command),
    }
}

pub(super) fn enabled(command: Command, state: &Context<'_>) -> bool {
    is_enabled(command, state.document, state.editor, state.extrusion)
}

/// Whether a command can be carried out right now.
///
/// Shared with the keyboard: a shortcut for a command whose button is greyed
/// out must do nothing either, or Enter would "finish" a sketch that is not
/// open.
pub fn is_enabled(
    command: Command,
    document: &PartDocument,
    editor: &SketchEditor,
    extrusion: &ExtrusionState,
) -> bool {
    let drawing = editor.active_sketch().is_some();
    match command {
        Command::Undo => document.history.can_undo(),
        Command::Redo => document.history.can_redo(),
        Command::CompactHistory => !document.history.is_empty(),
        Command::SelectAll => drawing && editor.tool == Tool::Select,
        Command::FinishSketch | Command::NextMatterInFront => drawing,
        Command::ToolSelect
        | Command::ToolLine
        | Command::ToolLineSymmetric
        | Command::ToolRectangle
        | Command::ToolCircle
        | Command::ToolArc
        | Command::ToolEllipse
        | Command::ToolPoint
        | Command::ToolDimension
        | Command::ToolTrim
        | Command::ToolSplit
        | Command::ToolMirror
        | Command::ToolCircularPattern
        | Command::ToolRectangularPattern => drawing,
        Command::ToolChamfer | Command::ToolFillet => drawing || !document.body().is_empty(),
        Command::ChamferEqual | Command::ChamferAngled | Command::ChamferSided => drawing,
        Command::DimensionAuto
        | Command::DimensionPointToPoint
        | Command::DimensionLength
        | Command::DimensionAngle
        | Command::DimensionRadius => drawing && editor.tool == Tool::Dimension,
        Command::CircleCenter
        | Command::CircleTwoPoints
        | Command::CircleThreePoints
        | Command::CircleTwoTangents
        | Command::CircleThreeTangents => drawing,
        Command::ArcByCenter | Command::ArcByEnds => drawing,
        Command::EllipseByCentre | Command::EllipseByEnds => drawing,
        Command::RulePerpendicular
        | Command::RuleParallel
        | Command::RuleEqual
        | Command::RuleCoincident
        | Command::RuleCollinear
        | Command::RuleTangent
        | Command::RuleMidpoint
        | Command::RuleFixed
        | Command::RuleConcentric => drawing,
        Command::ExtrusionAdd | Command::ExtrusionCut => extrusion.sketch.is_some(),
        // A step of matter reopened keeps its shape.
        Command::ExtrusionStraight | Command::ExtrusionRevolution => {
            extrusion.is_active() && extrusion.reopened.is_none()
        }
        Command::ExtrusionApply => extrusion.is_ready(document.variables()),
        Command::ExtrusionCancel => extrusion.is_active(),
        Command::ExportStlCoarse
        | Command::ExportStlNormal
        | Command::ExportStlFine
        | Command::ExportStep => !document.body().is_empty(),
        Command::SectionView => !drawing && !document.body().is_empty(),
        Command::OriginView => !drawing,
        _ => true,
    }
}

#[cfg(test)]
mod tests;
