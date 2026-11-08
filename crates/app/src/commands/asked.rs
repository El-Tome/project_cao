use cao_prefs::Command;
use cao_render::MatterInFront;

/// What the user asks for: a command, or a state of the matter in front
/// picked from the view bar's menu, which no `Command` names and which is run
/// as one all the same.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Asked {
    Command(Command),
    MatterInFront(MatterInFront),
}

impl From<Command> for Asked {
    fn from(command: Command) -> Self {
        Self::Command(command)
    }
}
