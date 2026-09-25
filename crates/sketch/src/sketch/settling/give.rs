//! Where a dragged point is free to go without its shape turning, read once
//! from the drawing as the press found it.

use glam::DVec2;

use super::kept::Kept;
use crate::equation::Equation;
use crate::independence::{null_space, turns_nothing};
use crate::sketch::{PointId, Sketch};

/// How a dragged point can follow the hand, its shape keeping its way up.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Give {
    /// Anywhere: the shape stretches to follow.
    Free,
    /// Along one line only, this way or the other: the shape stretches that
    /// way and no further.
    Along(DVec2),
    /// Nowhere, and the shape can turn about the point that stays: it pivots.
    Nowhere,
    /// Nowhere, and turning would break a rule: nothing moves.
    Stuck,
}

impl Sketch {
    /// How `point` can follow the hand once `pins` stay put and `lines` are
    /// kept, everything outside its shape held still.
    ///
    /// Read off the drawing's own equations as it stands — the directions the
    /// point can travel in without any of them giving. When a turn of the
    /// whole shape about the place it would pivot on is itself one of those
    /// motions, it is taken out first: all that question asks is whether the
    /// point can go anywhere without the shape turning.
    pub(crate) fn give_of(
        &self,
        point: PointId,
        shape: &[PointId],
        pins: &[PointId],
        lines: &[Kept],
        about: Option<PointId>,
        millimeters_per_unit: f64,
    ) -> Give {
        let mut pinned: Vec<bool> = self.pinned_points();
        for (index, pinned) in pinned.iter_mut().enumerate() {
            if !shape.contains(&PointId(index)) {
                *pinned = true;
            }
        }
        for pin in pins {
            pinned[pin.0] = true;
        }
        pinned[point.0] = false;

        let mut system = self.rows_touching(shape, &pinned, lines, millimeters_per_unit);
        let turn = about
            .filter(|about| *about != point)
            .map(|about| self.turn_about(shape, about, &pinned))
            .filter(|turn| system.iter().all(|row| turns_nothing(row, turn)));
        let turning_is_free = turn.is_some();
        system.extend(turn);
        let free = null_space(&system, &pinned, self.variables());
        let span = spanned(
            free.iter()
                .map(|way| DVec2::new(way[point.0 * 2], way[point.0 * 2 + 1])),
        );

        match (span.as_slice(), turning_is_free) {
            ([], _) => match about
                .is_some_and(|about| self.turns_freely(shape, about, millimeters_per_unit))
            {
                true => Give::Nowhere,
                false => Give::Stuck,
            },
            ([only], false) => Give::Along(*only),
            _ => Give::Free,
        }
    }

    /// Whether the whole shape can be turned about `about` without a single
    /// rule of the drawing giving — the lines a drag keeps left out, since
    /// turning is exactly what they refuse — and with nothing else holding
    /// it somewhere else.
    pub(crate) fn turns_freely(
        &self,
        shape: &[PointId],
        about: PointId,
        millimeters_per_unit: f64,
    ) -> bool {
        let mut pinned = self.pinned_points();
        let place = self.point(about);
        if shape
            .iter()
            .any(|each| pinned[each.0] && self.point(*each).distance(place) > 1e-9)
        {
            return false;
        }
        pinned[about.0] = true;
        let turn = self.turn_about(shape, about, &pinned);
        self.equations_pinned_by(millimeters_per_unit, &pinned)
            .iter()
            .all(|row| turns_nothing(row, &turn))
    }

    /// The rows that speak of anything the shape is free to move: its points
    /// and the sizes of the circles centred in it.
    fn rows_touching(
        &self,
        shape: &[PointId],
        pinned: &[bool],
        lines: &[Kept],
        millimeters_per_unit: f64,
    ) -> Vec<Equation> {
        let mut columns: Vec<usize> = shape
            .iter()
            .filter(|each| !pinned[each.0])
            .flat_map(|each| [each.0 * 2, each.0 * 2 + 1])
            .collect();
        columns.extend(
            self.live_circles()
                .filter(|(_, round)| shape.contains(&round.center))
                .filter_map(|(id, _)| self.radius_column(id)),
        );
        let mut rows = self.equations_pinned_by(millimeters_per_unit, pinned);
        rows.extend(lines.iter().filter_map(|line| self.kept_row(line, pinned)));
        rows.retain(|row| columns.iter().any(|column| row.gradient[*column] != 0.0));
        rows
    }

    /// A turn of the shape's free points about `about`, written as a row.
    fn turn_about(&self, shape: &[PointId], about: PointId, pinned: &[bool]) -> Equation {
        let centre = self.point(about);
        let mut turn = Equation::new(self.variables());
        for each in shape.iter().filter(|each| !pinned[each.0]) {
            turn.add(*each, (self.point(*each) - centre).perp());
        }
        turn
    }
}

/// The directions a handful of vectors span in the plane, as at most two unit
/// vectors square to each other. What is too short to say anything is dust.
fn spanned(ways: impl Iterator<Item = DVec2>) -> Vec<DVec2> {
    let mut basis: Vec<DVec2> = Vec::new();
    for way in ways {
        let rest = basis
            .iter()
            .fold(way, |rest, each| rest - *each * rest.dot(*each));
        if rest.length() > 1e-3 {
            basis.push(rest.normalize());
        }
        if basis.len() == 2 {
            break;
        }
    }
    basis
}
