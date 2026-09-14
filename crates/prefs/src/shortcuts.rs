use serde::{Deserialize, Serialize};

use crate::command::Command;

/// A key, named without reference to any windowing toolkit so that a shortcut
/// survives being written to a profile and read back by another front-end.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
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
        Self::A,
        Self::B,
        Self::C,
        Self::D,
        Self::E,
        Self::F,
        Self::G,
        Self::H,
        Self::I,
        Self::J,
        Self::K,
        Self::L,
        Self::M,
        Self::N,
        Self::O,
        Self::P,
        Self::Q,
        Self::R,
        Self::S,
        Self::T,
        Self::U,
        Self::V,
        Self::W,
        Self::X,
        Self::Y,
        Self::Z,
        Self::Num0,
        Self::Num1,
        Self::Num2,
        Self::Num3,
        Self::Num4,
        Self::Num5,
        Self::Num6,
        Self::Num7,
        Self::Num8,
        Self::Num9,
        Self::F1,
        Self::F2,
        Self::F3,
        Self::F4,
        Self::F5,
        Self::F6,
        Self::F7,
        Self::F8,
        Self::F9,
        Self::F10,
        Self::F11,
        Self::F12,
        Self::Escape,
        Self::Tab,
        Self::Space,
        Self::Enter,
        Self::Backspace,
        Self::Delete,
        Self::Home,
        Self::End,
        Self::PageUp,
        Self::PageDown,
        Self::Left,
        Self::Right,
        Self::Up,
        Self::Down,
        Self::Plus,
        Self::Minus,
        Self::Comma,
        Self::Period,
    ];
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

    /// Binds what `reference` offers and this does not hold yet.
    ///
    /// A profile saved before a command existed would never hear of it, the
    /// way a toolbar would not. What the user set stays put, and a chord they
    /// have already given to something else is left alone: taking it back
    /// would break a shortcut they use for one they never asked for.
    pub fn adopt_new_bindings(&mut self, reference: &Self) {
        for (command, chord) in &reference.bindings {
            let known = self.bindings.iter().any(|(bound, _)| bound == command);
            let taken = self.bindings.iter().any(|(_, held)| held == chord);
            if !known && !taken {
                self.bindings.push((*command, *chord));
            }
        }
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
                (C::ToggleExplorer, Chord::new(Key::B)),
                (C::OpenSettings, Chord::new(Key::Comma).cmd()),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_profile_saved_before_a_command_existed_still_gets_its_chord() {
        let mut saved = Shortcuts {
            bindings: vec![(Command::Undo, Chord::new(Key::Z).cmd())],
        };

        saved.adopt_new_bindings(&Shortcuts::default());

        assert_eq!(
            saved.chord_for(Command::ToggleHistory),
            Some(Chord::new(Key::H)),
        );
        assert_eq!(
            saved.chord_for(Command::Undo),
            Some(Chord::new(Key::Z).cmd()),
            "what the user set stays put",
        );
    }

    #[test]
    fn a_chord_the_user_gave_to_something_else_is_not_taken_back() {
        let mut saved = Shortcuts {
            bindings: vec![(Command::ToolLine, Chord::new(Key::H))],
        };

        saved.adopt_new_bindings(&Shortcuts::default());

        assert_eq!(
            saved.command_for(Chord::new(Key::H)),
            Some(Command::ToolLine)
        );
        assert_eq!(
            saved.chord_for(Command::ToggleHistory),
            None,
            "a shortcut the user uses is broken for one they never asked for",
        );
    }

    #[test]
    fn no_two_commands_answer_to_the_same_chord_on_a_fresh_installation() {
        let mut taken: Vec<(Chord, Command)> = Vec::new();

        for (command, chord) in Shortcuts::default().bindings {
            if let Some((_, first)) = taken.iter().find(|(bound, _)| *bound == chord) {
                panic!(
                    "{chord:?} answers for both {first:?} and {command:?}: the second is unreachable and says nothing about why"
                );
            }
            taken.push((chord, command));
        }
    }
}
