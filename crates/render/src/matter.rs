/// How the matter standing between the eye and the plane being drawn on is
/// shown.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MatterInFront {
    #[default]
    Hidden,
    SeeThrough,
    Opaque,
}

impl MatterInFront {
    /// The state one click on the button moves to.
    pub fn next(self) -> Self {
        match self {
            Self::Hidden => Self::SeeThrough,
            Self::SeeThrough => Self::Opaque,
            Self::Opaque => Self::Hidden,
        }
    }
}
