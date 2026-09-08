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

#[cfg(test)]
mod tests {
    use super::*;

    /// Two points, so a system has four unknowns: x0, y0, x1, y1.
    const UNKNOWNS: usize = 4;

    /// Only the gradient is under test here. How far off an equation currently
    /// is says nothing about whether it repeats what another one already says.
    fn saying(gradient: [f64; UNKNOWNS]) -> Equation {
        Equation {
            error: 0.0,
            angular: false,
            gradient: gradient.to_vec(),
        }
    }

    fn dot(a: &[f64], b: &[f64]) -> f64 {
        a.iter().zip(b).map(|(a, b)| a * b).sum()
    }

    fn loose() -> [bool; 2] {
        [false, false]
    }

    #[test]
    fn a_row_the_basis_already_covers_leaves_nothing_behind() {
        let basis = vec![vec![1.0, 0.0, 0.0, 0.0]];
        assert_eq!(reduce(&[1.0, 0.0, 0.0, 0.0], &basis), None);
        assert_eq!(
            reduce(&[7.0, 0.0, 0.0, 0.0], &basis),
            None,
            "a row seven times longer points the same way and adds as little",
        );
    }

    #[test]
    fn a_row_that_holds_nothing_down_leaves_nothing_behind() {
        assert_eq!(reduce(&[0.0; UNKNOWNS], &[]), None);
    }

    #[test]
    fn what_is_left_is_a_unit_row_at_right_angles_to_the_basis() {
        let basis = vec![vec![1.0, 0.0, 0.0, 0.0]];
        let left = reduce(&[3.0, 4.0, 0.0, 0.0], &basis).expect("y is new information");

        assert!(
            (norm(&left) - 1.0).abs() < 1e-9,
            "what is left is normalised"
        );
        assert!(
            dot(&left, &basis[0]).abs() < 1e-9,
            "and holds none of what the basis already said",
        );
    }

    #[test]
    fn an_equation_repeated_is_counted_once() {
        let alone = saying([1.0, 0.0, 0.0, 0.0]);
        let again = saying([1.0, 0.0, 0.0, 0.0]);
        let scaled = saying([-2.0, 0.0, 0.0, 0.0]);

        assert_eq!(rank(std::slice::from_ref(&alone)), 1);
        assert_eq!(rank(&[alone, again, scaled]), 1);
    }

    #[test]
    fn each_direction_none_of_the_others_reach_raises_the_rank() {
        assert_eq!(rank(&[]), 0);
        assert_eq!(rank(&[saying([1.0, 0.0, 0.0, 0.0])]), 1);
        assert_eq!(
            rank(&[saying([1.0, 0.0, 0.0, 0.0]), saying([0.0, 1.0, 0.0, 0.0])]),
            2,
        );
        assert_eq!(
            rank(&[
                saying([1.0, 0.0, 0.0, 0.0]),
                saying([0.0, 1.0, 0.0, 0.0]),
                saying([1.0, 1.0, 0.0, 0.0]),
            ]),
            2,
            "the third is the first two added together, and says nothing more",
        );
    }

    #[test]
    fn the_rank_never_passes_the_number_of_unknowns() {
        let system = [
            saying([1.0, 0.0, 0.0, 0.0]),
            saying([0.0, 1.0, 0.0, 0.0]),
            saying([0.0, 0.0, 1.0, 0.0]),
            saying([0.0, 0.0, 0.0, 1.0]),
            saying([1.0, -2.0, 3.0, 0.5]),
        ];
        assert_eq!(rank(&system), UNKNOWNS);
    }

    #[test]
    fn the_order_the_equations_come_in_does_not_change_what_they_say() {
        let forwards = rank(&[
            saying([1.0, 1.0, 0.0, 0.0]),
            saying([0.0, 1.0, 1.0, 0.0]),
            saying([1.0, 2.0, 1.0, 0.0]),
        ]);
        let backwards = rank(&[
            saying([1.0, 2.0, 1.0, 0.0]),
            saying([0.0, 1.0, 1.0, 0.0]),
            saying([1.0, 1.0, 0.0, 0.0]),
        ]);
        assert_eq!(forwards, backwards);
    }

    #[test]
    fn an_equation_the_others_already_imply_says_nothing_new() {
        let system = [saying([1.0, 0.0, 0.0, 0.0]), saying([0.0, 1.0, 0.0, 0.0])];

        assert!(is_dependent(&system, &saying([1.0, 0.0, 0.0, 0.0])));
        assert!(
            is_dependent(&system, &saying([2.0, -3.0, 0.0, 0.0])),
            "any mixture of the two is already covered",
        );
        assert!(
            is_dependent(&system, &saying([0.0; UNKNOWNS])),
            "an equation that holds nothing down adds nothing either",
        );
        assert!(!is_dependent(&system, &saying([0.0, 0.0, 1.0, 0.0])));
    }

    #[test]
    fn saying_nothing_new_is_exactly_leaving_the_rank_where_it_was() {
        let held = rank(&[saying([1.0, 1.0, 0.0, 0.0]), saying([0.0, 1.0, 1.0, 0.0])]);

        for gradient in [
            [1.0, 2.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
            [3.0, 3.0, 0.0, 0.0],
            [0.5, 0.0, -0.5, 0.0],
        ] {
            let system = [saying([1.0, 1.0, 0.0, 0.0]), saying([0.0, 1.0, 1.0, 0.0])];
            let grown = [
                saying([1.0, 1.0, 0.0, 0.0]),
                saying([0.0, 1.0, 1.0, 0.0]),
                saying(gradient),
            ];
            assert_eq!(
                is_dependent(&system, &saying(gradient)),
                rank(&grown) == held,
                "an equation says nothing new exactly when adding it moves no rank",
            );
        }
    }

    #[test]
    fn with_nothing_asked_of_it_the_drawing_moves_every_way_it_can() {
        assert_eq!(null_space(&[], &loose(), UNKNOWNS).len(), UNKNOWNS);
    }

    #[test]
    fn moving_along_a_free_direction_breaks_none_of_the_equations() {
        let system = [saying([1.0, 1.0, 0.0, 0.0]), saying([0.0, 0.0, 1.0, 0.0])];
        let free = null_space(&system, &loose(), UNKNOWNS);

        for direction in &free {
            for equation in &system {
                assert!(
                    dot(direction, &equation.gradient).abs() < 1e-9,
                    "a direction the constraints allow changes no error",
                );
            }
        }
    }

    #[test]
    fn no_free_direction_repeats_another() {
        let free = null_space(&[saying([1.0, 0.0, 0.0, 0.0])], &loose(), UNKNOWNS);

        for (index, direction) in free.iter().enumerate() {
            assert!((norm(direction) - 1.0).abs() < 1e-9);
            for other in &free[index + 1..] {
                assert!(dot(direction, other).abs() < 1e-9);
            }
        }
    }

    #[test]
    fn a_pinned_point_is_not_a_direction_to_move_in() {
        let free = null_space(&[], &[true, false], UNKNOWNS);

        assert_eq!(free.len(), 2, "only the loose point is still free to move");
        for direction in &free {
            assert!(
                direction[0].abs() < 1e-9 && direction[1].abs() < 1e-9,
                "and no direction asks the pinned point to budge",
            );
        }
    }

    #[test]
    fn what_is_held_and_what_is_free_add_up_to_the_whole() {
        for system in [
            vec![],
            vec![saying([1.0, 0.0, 0.0, 0.0])],
            vec![saying([1.0, 1.0, 0.0, 0.0]), saying([0.0, 1.0, 1.0, 0.0])],
            vec![
                saying([1.0, 0.0, 0.0, 0.0]),
                saying([0.0, 1.0, 0.0, 0.0]),
                saying([1.0, 1.0, 0.0, 0.0]),
            ],
        ] {
            let free = null_space(&system, &loose(), UNKNOWNS);
            assert_eq!(
                rank(&system) + free.len(),
                UNKNOWNS,
                "every unknown is either held by an equation or free to move",
            );
        }
    }
}
