use serde::{Deserialize, Serialize};

use crate::command::Command;

/// A key, named without reference to any windowing toolkit so that a shortcut
/// survives being written to a profile and read back by another front-end.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Key {
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    Escape,
    Tab,
    Space,
    Enter,
    Backspace,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    Left,
    Right,
    Up,
    Down,
    Plus,
    Minus,
    Comma,
    Period,
}

impl Key {
    /// Every key offered when recording a shortcut.
    pub const ALL: [Self; 66] = [
        Self::A, Self::B, Self::C, Self::D, Self::E, Self::F, Self::G, Self::H,
        Self::I, Self::J, Self::K, Self::L, Self::M, Self::N, Self::O, Self::P,
        Self::Q, Self::R, Self::S, Self::T, Self::U, Self::V, Self::W, Self::X,
        Self::Y, Self::Z,
        Self::Num0, Self::Num1, Self::Num2, Self::Num3, Self::Num4,
        Self::Num5, Self::Num6, Self::Num7, Self::Num8, Self::Num9,
        Self::F1, Self::F2, Self::F3, Self::F4, Self::F5, Self::F6,
        Self::F7, Self::F8, Self::F9, Self::F10, Self::F11, Self::F12,
        Self::Escape, Self::Tab, Self::Space, Self::Enter, Self::Backspace,
        Self::Delete, Self::Home, Self::End, Self::PageUp, Self::PageDown,
        Self::Left, Self::Right, Self::Up, Self::Down, Self::Plus, Self::Minus,
        Self::Comma, Self::Period,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::A => "A", Self::B => "B", Self::C => "C", Self::D => "D",
            Self::E => "E", Self::F => "F", Self::G => "G", Self::H => "H",
            Self::I => "I", Self::J => "J", Self::K => "K", Self::L => "L",
            Self::M => "M", Self::N => "N", Self::O => "O", Self::P => "P",
            Self::Q => "Q", Self::R => "R", Self::S => "S", Self::T => "T",
            Self::U => "U", Self::V => "V", Self::W => "W", Self::X => "X",
            Self::Y => "Y", Self::Z => "Z",
            Self::Num0 => "0", Self::Num1 => "1", Self::Num2 => "2",
            Self::Num3 => "3", Self::Num4 => "4", Self::Num5 => "5",
            Self::Num6 => "6", Self::Num7 => "7", Self::Num8 => "8",
            Self::Num9 => "9",
            Self::F1 => "F1", Self::F2 => "F2", Self::F3 => "F3",
            Self::F4 => "F4", Self::F5 => "F5", Self::F6 => "F6",
            Self::F7 => "F7", Self::F8 => "F8", Self::F9 => "F9",
            Self::F10 => "F10", Self::F11 => "F11", Self::F12 => "F12",
            Self::Escape => "Échap",
            Self::Tab => "Tab",
            Self::Space => "Espace",
            Self::Enter => "Entrée",
            Self::Backspace => "Retour arrière",
            Self::Delete => "Suppr",
            Self::Home => "Début",
            Self::End => "Fin",
            Self::PageUp => "Page préc.",
            Self::PageDown => "Page suiv.",
            Self::Left => "←",
            Self::Right => "→",
            Self::Up => "↑",
            Self::Down => "↓",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Comma => ",",
            Self::Period => ".",
        }
    }
}

/// A key plus the modifiers held with it.
///
/// `command` is Ctrl on Windows and Linux, Cmd on macOS — the same shortcut
/// then reads correctly on every platform, which a profile shared between
/// machines depends on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Chord {
    pub key: Key,
    #[serde(default)]
    pub command: bool,
    #[serde(default)]
    pub shift: bool,
    #[serde(default)]
    pub alt: bool,
}

impl Chord {
    pub const fn new(key: Key) -> Self {
        Self {
            key,
            command: false,
            shift: false,
            alt: false,
        }
    }

    pub const fn cmd(mut self) -> Self {
        self.command = true;
        self
    }

    pub const fn shift(mut self) -> Self {
        self.shift = true;
        self
    }

    pub const fn alt(mut self) -> Self {
        self.alt = true;
        self
    }

    pub fn label(self) -> String {
        let mut text = String::new();
        if self.command {
            text.push_str("Cmd+");
        }
        if self.shift {
            text.push_str("Maj+");
        }
        if self.alt {
            text.push_str("Alt+");
        }
        text.push_str(self.key.label());
        text
    }
}

/// What each command is bound to.
///
/// A list of pairs rather than a map: a command may have several shortcuts, and
/// the order is what the settings screen shows.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Shortcuts {
    pub bindings: Vec<(Command, Chord)>,
}

impl Shortcuts {
    pub fn chord_for(&self, command: Command) -> Option<Chord> {
        self.bindings
            .iter()
            .find(|(bound, _)| *bound == command)
            .map(|(_, chord)| *chord)
    }

    /// What this chord triggers, if anything.
    pub fn command_for(&self, chord: Chord) -> Option<Command> {
        self.bindings
            .iter()
            .find(|(_, bound)| *bound == chord)
            .map(|(command, _)| *command)
    }

    /// Binds a command, replacing whatever it had and taking the chord away
    /// from any other command that was using it.
    ///
    /// Two commands on one chord would make the second unreachable and give no
    /// hint why, so the older binding gives way rather than both being kept.
    pub fn bind(&mut self, command: Command, chord: Chord) {
        self.bindings
            .retain(|(bound, existing)| *bound != command && *existing != chord);
        self.bindings.push((command, chord));
    }

    pub fn unbind(&mut self, command: Command) {
        self.bindings.retain(|(bound, _)| *bound != command);
    }
}

impl Default for Shortcuts {
    fn default() -> Self {
        use Command as C;
        Self {
            bindings: vec![
                (C::Undo, Chord::new(Key::Z).cmd()),
                (C::Redo, Chord::new(Key::Z).cmd().shift()),
                (C::NewSketch, Chord::new(Key::N).cmd()),
                (C::FinishSketch, Chord::new(Key::Enter)),
                (C::RecenterOnSketch, Chord::new(Key::F)),
                (C::ToolSelect, Chord::new(Key::S)),
                (C::ToolLine, Chord::new(Key::L)),
                (C::ToolRectangle, Chord::new(Key::R)),
                (C::ToolCircle, Chord::new(Key::C)),
                (C::ToolPoint, Chord::new(Key::P)),
                (C::ToolDimension, Chord::new(Key::D)),
                (C::ExtrusionAdd, Chord::new(Key::E)),
                (C::ExtrusionCut, Chord::new(Key::E).shift()),
                (C::ExtrusionApply, Chord::new(Key::Enter).cmd()),
                (C::ToggleHistory, Chord::new(Key::H)),
                (C::OpenSettings, Chord::new(Key::Comma).cmd()),
            ],
        }
    }
}
