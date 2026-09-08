//! What a set of equations genuinely says, and what it leaves free.
//!
//! Linear algebra, kept apart from the solver that runs it: Gram–Schmidt over
//! the gradients answers both questions, it knows nothing of what is drawn,
//! and it is worth testing on its own.

use crate::solver::Equation;

/// How many of a set of equations are genuinely independent.
///
/// Gram–Schmidt: each row is stripped of whatever the earlier rows already
/// said; what is left over, if anything, is new information. This is what tells
/// a constraint that adds nothing from one that pins a shape down further —
/// counting constraints could never see that a triangle's third side follows
/// from its other sides and angles.
pub(crate) fn rank(equations: &[Equation]) -> usize {
    independent_rows(equations, None).0
}

/// The ways the drawing can still move without breaking anything.
///
/// Each returned vector is a direction the coordinates may travel in. Where a
/// point has no component in any of them, it cannot move at all: that point is
/// settled, whatever the rest of the drawing is doing. This is what lets one
/// line be shown as fixed while its neighbour is still loose.
pub(crate) fn null_space(
    equations: &[Equation],
    pinned: &[bool],
    variables: usize,
) -> Vec<Vec<f64>> {
    let mut basis: Vec<Vec<f64>> = Vec::new();
    for equation in equations {
        if let Some(row) = reduce(&equation.gradient, &basis) {
            basis.push(row);
        }
    }

    // Anything left once the constraints have had their say is free movement.
    let mut free: Vec<Vec<f64>> = Vec::new();
    for index in 0..variables {
        // A pinned point cannot move, so it is not a direction to consider.
        if pinned.get(index / 2).copied().unwrap_or(false) {
            continue;
        }
        let mut candidate = vec![0.0; variables];
        candidate[index] = 1.0;

        let mut combined = basis.clone();
        combined.extend(free.iter().cloned());
        if let Some(direction) = reduce(&candidate, &combined) {
            free.push(direction);
        }
    }
    free
}

/// True when `candidate` says nothing the others do not already say.
pub(crate) fn is_dependent(equations: &[Equation], candidate: &Equation) -> bool {
    independent_rows(equations, Some(candidate)).1
}

fn independent_rows(equations: &[Equation], candidate: Option<&Equation>) -> (usize, bool) {
    let mut basis: Vec<Vec<f64>> = Vec::new();

    for equation in equations {
        if let Some(row) = reduce(&equation.gradient, &basis) {
            basis.push(row);
        }
    }

    let dependent = match candidate {
        Some(candidate) => reduce(&candidate.gradient, &basis).is_none(),
        None => false,
    };
    (basis.len(), dependent)
}

/// Removes from `row` everything the basis already covers, returning what is
/// left once normalised, or `None` when nothing is.
fn reduce(row: &[f64], basis: &[Vec<f64>]) -> Option<Vec<f64>> {
    let mut residual = row.to_vec();
    let original = norm(&residual);
    if original < 1e-9 {
        return None;
    }

    for existing in basis {
        let projection: f64 = residual
            .iter()
            .zip(existing)
            .map(|(value, base)| value * base)
            .sum();
        for (value, base) in residual.iter_mut().zip(existing) {
            *value -= projection * base;
        }
    }

    let length = norm(&residual);
    // Relative to the original: a row a thousand times shorter than it started
    // is numerical dust, not information.
    if length / original < 1e-4 {
        return None;
    }
    for value in &mut residual {
        *value /= length;
    }
    Some(residual)
}

pub(crate) fn norm(row: &[f64]) -> f64 {
    row.iter().map(|value| value * value).sum::<f64>().sqrt()
}
