//! The operation that opens a step, cut in two: what it stands on, which goes
//! into the index, and what it sets, which goes into its folder.

use super::{Opening, Stands};
use crate::errors::PartFileError;
use crate::history::{Operation, SectionAsked, StepKind};

pub(super) fn kind_of(stands_on: &Stands) -> StepKind {
    match stands_on {
        Stands::Sketch { .. } => StepKind::Sketch,
        Stands::Extrusion { .. } => StepKind::Extrusion,
        Stands::Revolution { .. } => StepKind::Revolution,
        Stands::Section { .. } => StepKind::Section,
        Stands::Blend { .. } => StepKind::Blend,
    }
}

/// The operation opening a step, split into what the index holds and what its
/// folder keeps.
pub(super) fn taken_apart(opening: &Operation) -> Result<(Stands, Option<Opening>), PartFileError> {
    Ok(match opening {
        Operation::CreateSketch { plane, on } => (
            Stands::Sketch {
                plane: *plane,
                on: *on,
                section: None,
            },
            None,
        ),
        Operation::CreateSketchOnSection { section, plane } => (
            Stands::Sketch {
                plane: *plane,
                on: None,
                section: Some(*section),
            },
            None,
        ),
        Operation::SaveSection(asked) => (
            Stands::Section {
                plane: asked.plane,
                on: asked.on,
            },
            Some(Opening::Section {
                offset: asked.offset,
                behind: asked.behind,
                in_front: asked.in_front,
                tilt_about_u_degrees: asked.tilt_about_u_degrees,
                tilt_about_v_degrees: asked.tilt_about_v_degrees,
                pivot: asked.pivot,
            }),
        ),
        Operation::Extrude {
            sketch,
            areas,
            distance,
            mode,
        } => (
            Stands::Extrusion {
                sketch: *sketch,
                areas: areas.clone(),
            },
            Some(Opening::Extrusion {
                distance: distance.clone(),
                mode: *mode,
            }),
        ),
        Operation::Revolve {
            sketch,
            areas,
            axis,
            angle,
            mode,
        } => (
            Stands::Revolution {
                sketch: *sketch,
                areas: areas.clone(),
                axis: *axis,
            },
            Some(Opening::Revolution {
                angle: angle.clone(),
                mode: *mode,
            }),
        ),
        Operation::Blend { edge, shape, size } => (
            Stands::Blend { edge: edge.clone() },
            Some(Opening::Blend {
                shape: *shape,
                size: size.clone(),
            }),
        ),
        _ => return Err(PartFileError::BrokenDesign),
    })
}

/// The same operation, back in one piece.
pub(super) fn put_together(
    stands_on: Stands,
    opening: Option<Opening>,
) -> Result<Operation, PartFileError> {
    Ok(match (stands_on, opening) {
        (
            Stands::Sketch {
                plane,
                on,
                section: None,
            },
            None,
        ) => Operation::CreateSketch { plane, on },
        (
            Stands::Sketch {
                plane,
                section: Some(section),
                ..
            },
            None,
        ) => Operation::CreateSketchOnSection { section, plane },
        (
            Stands::Section { plane, on },
            Some(Opening::Section {
                offset,
                behind,
                in_front,
                tilt_about_u_degrees,
                tilt_about_v_degrees,
                pivot,
            }),
        ) => Operation::SaveSection(SectionAsked {
            plane,
            on,
            offset,
            behind,
            in_front,
            tilt_about_u_degrees,
            tilt_about_v_degrees,
            pivot,
        }),
        (Stands::Extrusion { sketch, areas }, Some(Opening::Extrusion { distance, mode })) => {
            Operation::Extrude {
                sketch,
                areas,
                distance,
                mode,
            }
        }
        (
            Stands::Revolution {
                sketch,
                areas,
                axis,
            },
            Some(Opening::Revolution { angle, mode }),
        ) => Operation::Revolve {
            sketch,
            areas,
            axis,
            angle,
            mode,
        },
        (Stands::Blend { edge }, Some(Opening::Blend { shape, size })) => {
            Operation::Blend { edge, shape, size }
        }
        _ => return Err(PartFileError::BrokenDesign),
    })
}
