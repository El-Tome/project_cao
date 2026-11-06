//! What a section kept in the part asks for: a plane to look through, never a
//! step of matter.

use cao_sketch::WorkPlane;
use serde::{Deserialize, Serialize};

use super::{FaceAnchor, Operation};

/// How the matter on one side of a section is drawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SideDrawing {
    Hidden,
    SeeThrough,
    Opaque,
}

/// A section as it is made or edited: the plane it starts from, the face that
/// plane was taken off when it was, how far along its normal the cut stands,
/// how each side of it is drawn, and how far the plane is turned about its own
/// `u` and `v`, in degrees.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SectionAsked {
    /// On a face, what is left to fall back on when the face is gone.
    pub plane: WorkPlane,
    pub on: Option<FaceAnchor>,
    pub offset: f64,
    pub behind: SideDrawing,
    pub in_front: SideDrawing,
    #[serde(default)]
    pub tilt_about_u_degrees: f64,
    #[serde(default)]
    pub tilt_about_v_degrees: f64,
}

impl Operation {
    /// The section this operation changes, when it changes one: what files it
    /// under that section's step.
    pub(crate) fn edits_section(&self) -> Option<usize> {
        match self {
            Self::EditSection { section, .. } | Self::DeleteSection { section } => Some(*section),
            _ => None,
        }
    }

    /// Whether the operation keeps, changes or drops a section, which the list
    /// of what was done leaves out: a section is a way of looking.
    pub fn is_about_a_section(&self) -> bool {
        matches!(
            self,
            Self::SaveSection(_) | Self::EditSection { .. } | Self::DeleteSection { .. }
        )
    }
}
