//! What `Entrée` lays for the chamfer and the fillet: the operation every
//! corner taken is cut by, the values typed as the drawing measures them, and
//! a value too big held back with the largest that fits offered instead.

use cao_part::history::ChamferAsked;
use cao_part::{Formula, Operation, Variables};
use cao_sketch::{Chamfer, ChamferMode, Corner, CutShape, LockedInput, SegmentId};

use crate::screens::SketchContext;

/// Why the values typed are held back rather than laid, as the key of the
/// sentence that says so — nothing when they fit every corner taken.
///
/// A value too big for a corner is not laid: the field offers the largest that
/// fits, and a second `Entrée` lays that.
pub(super) fn held_back(
    context: &mut SketchContext<'_>,
    index: usize,
    taken: &[Corner],
    asked: Chamfer,
    rounding: bool,
) -> Option<&'static str> {
    let sketch = context.document.sketches().get(index)?;
    let sides: Vec<(SegmentId, SegmentId)> = taken
        .iter()
        .filter_map(|corner| sketch.sides_of(*corner))
        .collect();
    let shape = shape_of(rounding, in_units(asked, context.document.scale()));
    let share = sketch.fitting_share(&sides, shape);
    if share <= 0.0 {
        return Some("sketch.corner_has_no_room");
    }
    if share < 1.0 {
        offer_the_largest(context, asked, share);
        return Some("sketch.cut_too_big");
    }
    None
}

/// The operation that lays every corner taken. Whether each of them fits is
/// the drawing's business, and a corner too tight is counted and said rather
/// than stopping the rest.
///
/// Each value goes in as it was typed, so that a corner cut from the part's
/// variables follows them: `written` is what the two fields hold, as written.
pub(super) fn laying(
    index: usize,
    taken: Vec<Corner>,
    asked: Chamfer,
    rounding: bool,
    written: [Option<Formula>; 2],
    construction: bool,
) -> Operation {
    let [first, second] = written;
    let as_typed = |typed: Option<Formula>, number: f64| typed.unwrap_or(Formula::Number(number));
    let mode = match asked {
        Chamfer::Equal(reach) if rounding => {
            return Operation::Fillet {
                sketch: index,
                corners: taken,
                radius: as_typed(first, reach),
                construction,
            };
        }
        Chamfer::Equal(reach) => ChamferAsked::Equal(as_typed(first, reach)),
        Chamfer::Angled { along, degrees } => ChamferAsked::Angled {
            along: as_typed(first, along),
            degrees: as_typed(second, degrees),
        },
        Chamfer::Sided {
            first: near,
            second: far,
        } => ChamferAsked::Sided {
            first: as_typed(first, near),
            second: as_typed(second, far),
        },
    };
    Operation::Chamfer {
        sketch: index,
        corners: taken,
        mode,
        construction,
    }
}

/// What a fillet or a chamfer draws on a corner, for the values typed.
pub(super) fn shape_of(rounding: bool, asked: Chamfer) -> CutShape {
    match (rounding, asked) {
        (true, Chamfer::Equal(radius)) => CutShape::Fillet { radius },
        (_, chamfer) => CutShape::Chamfer(chamfer),
    }
}

/// Puts the largest values that fit into the fields, in place of those typed:
/// every length cut down by the same share, an angle left as it was.
///
/// Read down to the fourth decimal, never up: what the field shows has to fit
/// once laid.
fn offer_the_largest(context: &mut SketchContext<'_>, asked: Chamfer, share: f64) {
    let largest = |value: f64| (value * share * 1e4).floor() / 1e4;
    let values = match asked {
        Chamfer::Equal(reach) => [Some(largest(reach)), None],
        Chamfer::Sided { first, second } => [Some(largest(first)), Some(largest(second))],
        Chamfer::Angled { along, .. } => [Some(largest(along)), None],
    };
    for (rank, value) in values.into_iter().enumerate() {
        let Some(value) = value else { continue };
        let field = context.editor.live.field(rank);
        field.text = format!("{value}");
        field.take(&Variables::default());
    }
}

/// The values the tool needs, once they have all been typed. A fillet asks for
/// a radius and nothing else, whichever mode the chamfer beside it is in.
pub(super) fn typed(locked: LockedInput, mode: ChamferMode, rounding: bool) -> Option<Chamfer> {
    let first = locked.first?;
    if rounding {
        return Some(Chamfer::Equal(first));
    }
    match mode.wants() {
        1 => Some(mode.of(first, first)),
        _ => Some(mode.of(first, locked.second?)),
    }
}

/// A chamfer as the drawing measures it, for asking whether it fits: the
/// values are typed in millimetres, and the sketch works in its own units.
pub(super) fn in_units(asked: Chamfer, millimeters_per_unit: f64) -> Chamfer {
    match asked {
        Chamfer::Equal(reach) => Chamfer::Equal(reach / millimeters_per_unit),
        Chamfer::Sided { first, second } => Chamfer::Sided {
            first: first / millimeters_per_unit,
            second: second / millimeters_per_unit,
        },
        Chamfer::Angled { along, degrees } => Chamfer::Angled {
            along: along / millimeters_per_unit,
            degrees,
        },
    }
}
