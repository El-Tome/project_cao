use cao_part::PartDocument;
use cao_prefs::{Command, Edge, Item, Settings, ToolbarLayout};

use crate::screens::extrusion::{ExtrusionState, Shape};
use crate::screens::sketch::{CircleMode, DimensionMode, Rule, SketchEditor, Tool};
use crate::wording::{command as wording, shortcuts, toolbar::group};

/// How wide a toolbar starts when it is down one side.
const SIDE_WIDTH: f32 = 210.0;

/// The toolbar, drawn from the arrangement the user has set.
///
/// Nothing about which buttons exist or where they sit is decided here: the
/// tree comes from the settings, and this only knows how to draw a tree. That
/// is what lets the arrangement be changed, saved and handed to somebody else.
#[derive(Default)]
pub struct Ribbon {
    /// Which top-level group is open, by rank.
    pub tab: usize,
    pub history_open: bool,
}

impl Ribbon {
    pub fn new() -> Self {
        Self {
            tab: 0,
            history_open: true,
        }
    }

    /// Draws the bar and returns every command the user asked for this frame.
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        settings: &Settings,
        document: &PartDocument,
        editor: &SketchEditor,
        extrusion: &mut ExtrusionState,
    ) -> Vec<Command> {
        let layout = &settings.toolbar;
        let mut asked = Vec::new();

        match layout.edge {
            Edge::Floating => {
                egui::Window::new("Outils")
                    .default_pos(ui.max_rect().left_top() + egui::vec2(24.0, 80.0))
                    .resizable(false)
                    .show(ui.ctx(), |ui| {
                        asked = self.contents(ui, settings, document, editor, extrusion);
                    });
            }
            Edge::Top => {
                egui::Panel::top("ribbon").show(ui, |ui| {
                    asked = self.contents(ui, settings, document, editor, extrusion);
                });
            }
            Edge::Bottom => {
                egui::Panel::bottom("ribbon").show(ui, |ui| {
                    asked = self.contents(ui, settings, document, editor, extrusion);
                });
            }
            // A side bar has to be told how wide to start: left to itself it
            // takes the room its widest button asks for, which on a wide
            // window is the whole window.
            Edge::Left => {
                egui::Panel::left("ribbon")
                    .resizable(true)
                    .default_size(SIDE_WIDTH)
                    .show(ui, |ui| {
                        asked = self.contents(ui, settings, document, editor, extrusion);
                    });
            }
            Edge::Right => {
                egui::Panel::right("ribbon")
                    .resizable(true)
                    .default_size(SIDE_WIDTH)
                    .show(ui, |ui| {
                        asked = self.contents(ui, settings, document, editor, extrusion);
                    });
            }
        }

        asked
    }

    fn contents(
        &mut self,
        ui: &mut egui::Ui,
        settings: &Settings,
        document: &PartDocument,
        editor: &SketchEditor,
        extrusion: &mut ExtrusionState,
    ) -> Vec<Command> {
        let layout = &settings.toolbar;
        let mut asked = Vec::new();
        let vertical = layout.edge.is_vertical() || layout.edge == Edge::Floating;

        self.header(ui, layout, &mut asked, vertical);
        ui.separator();

        let state = Context {
            settings,
            document,
            editor,
            extrusion: &*extrusion,
        };

        // The open tab, then whatever sits at the top level outside any group.
        let open = layout.items.get(self.tab);
        if let Some(Item::Group { items, .. }) = open {
            lay_out(ui, items, 1, &state, &mut asked, vertical);
        }
        for item in &layout.items {
            if !matches!(item, Item::Group { .. }) {
                lay_out(
                    ui,
                    std::slice::from_ref(item),
                    1,
                    &state,
                    &mut asked,
                    vertical,
                );
            }
        }

        extrusion_row(ui, document, extrusion, &mut asked);
        asked
    }

    /// The logo, the tabs and the window toggles.
    fn header(
        &mut self,
        ui: &mut egui::Ui,
        layout: &ToolbarLayout,
        asked: &mut Vec<Command>,
        vertical: bool,
    ) {
        let mut row = |ui: &mut egui::Ui| {
            if layout.show_logo {
                ui.strong(&layout.logo_text);
                ui.separator();
            }
            for (rank, item) in layout.items.iter().enumerate() {
                let Item::Group { name, .. } = item else {
                    continue;
                };
                if ui.selectable_label(self.tab == rank, group(name)).clicked() {
                    self.tab = rank;
                }
            }
        };

        if vertical {
            ui.vertical(|ui| row(ui));
            ui.horizontal_wrapped(|ui| {
                ui.toggle_value(&mut self.history_open, "Historique");
                if ui.button("⚙").on_hover_text("Préférences").clicked() {
                    asked.push(Command::OpenSettings);
                }
            });
            return;
        }

        ui.horizontal(|ui| {
            row(ui);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("⚙").on_hover_text("Préférences").clicked() {
                    asked.push(Command::OpenSettings);
                }
                ui.toggle_value(&mut self.history_open, "Historique");
            });
        });
    }
}

/// What the toolbar needs to know to draw a button in the right state.
struct Context<'a> {
    settings: &'a Settings,
    document: &'a PartDocument,
    editor: &'a SketchEditor,
    extrusion: &'a ExtrusionState,
}

/// Draws a run of entries, at the depth they sit in the tree.
///
/// A group one level in is spread out where it is, with its name beside it; any
/// deeper and it becomes a menu that opens on click. Laying a third level out
/// in place would push everything else off the bar, and there is no bottom to
/// how deep the user may go.
fn lay_out(
    ui: &mut egui::Ui,
    items: &[Item],
    depth: usize,
    state: &Context<'_>,
    asked: &mut Vec<Command>,
    vertical: bool,
) {
    let run = |ui: &mut egui::Ui| {
        for item in items {
            match item {
                Item::Command(command) => button(ui, *command, state, asked),
                Item::Separator => {
                    ui.separator();
                }
                Item::Group { name, items } if depth <= 1 => {
                    ui.group(|ui| {
                        let inner = |ui: &mut egui::Ui| {
                            ui.weak(group(name));
                            lay_out(ui, items, depth + 1, state, asked, vertical);
                        };
                        if vertical {
                            ui.vertical(inner);
                        } else {
                            ui.horizontal(inner);
                        }
                    });
                }
                Item::Group { name, items } => {
                    ui.menu_button(group(name), |ui| {
                        lay_out(ui, items, depth + 1, state, asked, true);
                    });
                }
            }
        }
    };

    if vertical {
        ui.vertical(run);
    } else {
        ui.horizontal_wrapped(run);
    }
}

fn button(ui: &mut egui::Ui, command: Command, state: &Context<'_>, asked: &mut Vec<Command>) {
    let label = if state.settings.toolbar.show_labels {
        match state.settings.shortcuts.chord_for(command) {
            Some(chord) => format!("{} ({})", wording::label(command), shortcuts::chord(chord)),
            None => wording::label(command).to_string(),
        }
    } else {
        wording::label(command).chars().take(2).collect()
    };

    let response = ui
        .add_enabled_ui(enabled(command, state), |ui| {
            ui.selectable_label(active(command, state), label)
        })
        .inner
        .on_hover_text(wording::hint(command));
    if response.clicked() {
        asked.push(command);
    }
}

/// Whether the command is the one currently in force, so its button shows as
/// pressed.
fn active(command: Command, state: &Context<'_>) -> bool {
    let tool = state.editor.tool;
    let mode = state.editor.dimension_mode;
    match command {
        Command::ToolSelect => tool == Tool::Select,
        Command::ToolLine => tool == Tool::Line,
        Command::ToolRectangle => tool == Tool::Rectangle,
        Command::ToolCircle => tool == Tool::Circle,
        Command::ToolPoint => tool == Tool::Point,
        Command::ToolDimension => tool == Tool::Dimension,
        Command::CircleCenter => tool == Tool::Circle && mode_is(state, CircleMode::Center),
        Command::CircleTwoPoints => tool == Tool::Circle && mode_is(state, CircleMode::TwoPoints),
        Command::CircleThreePoints => {
            tool == Tool::Circle && mode_is(state, CircleMode::ThreePoints)
        }
        Command::CircleTwoTangents => {
            tool == Tool::Circle && mode_is(state, CircleMode::TwoTangents)
        }
        Command::CircleThreeTangents => {
            tool == Tool::Circle && mode_is(state, CircleMode::ThreeTangents)
        }
        Command::RulePerpendicular => tool == Tool::Constrain(Rule::Perpendicular),
        Command::RuleParallel => tool == Tool::Constrain(Rule::Parallel),
        Command::RuleEqual => tool == Tool::Constrain(Rule::Equal),
        Command::RuleCoincident => tool == Tool::Constrain(Rule::Coincident),
        Command::RuleCollinear => tool == Tool::Constrain(Rule::Collinear),
        Command::RuleTangent => tool == Tool::Constrain(Rule::Tangent),
        Command::RuleMidpoint => tool == Tool::Constrain(Rule::Midpoint),
        Command::RuleFixed => tool == Tool::Constrain(Rule::Fixed),
        Command::RuleConcentric => tool == Tool::Constrain(Rule::Concentric),
        Command::DimensionAuto => mode == DimensionMode::Auto,
        Command::DimensionPointToPoint => mode == DimensionMode::PointToPoint,
        Command::DimensionLength => mode == DimensionMode::Length,
        Command::DimensionAngle => mode == DimensionMode::Angle,
        Command::DimensionRadius => mode == DimensionMode::Radius,
        Command::ExtrusionAdd => state.extrusion.mode == Some(cao_part::ExtrusionMode::Add),
        Command::ExtrusionCut => state.extrusion.mode == Some(cao_part::ExtrusionMode::Cut),
        Command::ExtrusionStraight => state.extrusion.shape == Shape::Straight,
        Command::ExtrusionRevolution => state.extrusion.shape == Shape::Revolution,
        _ => false,
    }
}

fn mode_is(state: &Context<'_>, mode: CircleMode) -> bool {
    state.editor.circle_mode == mode
}

fn enabled(command: Command, state: &Context<'_>) -> bool {
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
        Command::FinishSketch | Command::RecenterOnSketch => drawing,
        Command::ToolSelect
        | Command::ToolLine
        | Command::ToolRectangle
        | Command::ToolCircle
        | Command::ToolPoint
        | Command::ToolDimension => drawing,
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
        Command::ExtrusionStraight | Command::ExtrusionRevolution => extrusion.is_active(),
        Command::ExtrusionApply => extrusion.is_ready(),
        Command::ExtrusionCancel => extrusion.is_active(),
        _ => true,
    }
}

/// The values an extrusion needs, shown only while one is being set up.
///
/// These are not commands: they are numbers being typed, and a toolbar entry
/// cannot stand for a field the user is in the middle of filling in.
fn extrusion_row(
    ui: &mut egui::Ui,
    document: &PartDocument,
    extrusion: &mut ExtrusionState,
    asked: &mut Vec<Command>,
) {
    if extrusion.sketch.is_none() {
        if document.sketches().is_empty() {
            return;
        }
        ui.horizontal_wrapped(|ui| {
            ui.label("Extruder l'esquisse :");
            for index in 0..document.sketches().len() {
                if ui.button(format!("{}", index + 1)).clicked() {
                    extrusion.offer(index);
                }
            }
        });
        return;
    }
    if !extrusion.is_active() {
        return;
    }

    ui.horizontal_wrapped(|ui| {
        if extrusion.is_revolving() {
            ui.label("Angle :");
            ui.add(
                egui::TextEdit::singleline(&mut extrusion.angle_input)
                    .desired_width(60.0)
                    .hint_text("°"),
            );
            ui.label("Autour de :");
            for axis in [cao_sketch::SketchAxis::U, cao_sketch::SketchAxis::V] {
                let chosen = extrusion.axis == cao_part::RevolutionAxis::Sketch(axis);
                if ui.selectable_label(chosen, axis.label()).clicked() {
                    extrusion.axis = cao_part::RevolutionAxis::Sketch(axis);
                }
            }
            if let cao_part::RevolutionAxis::Segment(segment) = extrusion.axis {
                ui.selectable_label(true, format!("trait {}", segment.0))
                    .on_hover_text("Cliquer un autre trait de l'esquisse pour en changer");
            } else {
                ui.weak("ou cliquer un trait");
            }
        } else {
            ui.label("Hauteur :");
            ui.add(
                egui::TextEdit::singleline(&mut extrusion.distance_input)
                    .desired_width(70.0)
                    .hint_text("mm"),
            );
        }
        ui.checkbox(&mut extrusion.reversed, "Sens inverse")
            .on_hover_text("Pousser la matière de l'autre côté du plan");

        ui.separator();
        ui.weak(format!("{} aire(s)", extrusion.picks.len()));
        ui.add_enabled_ui(extrusion.is_ready(), |ui| {
            if ui.button("Appliquer").clicked() {
                asked.push(Command::ExtrusionApply);
            }
        });
        if ui.button("Annuler").clicked() {
            asked.push(Command::ExtrusionCancel);
        }
    });
}
