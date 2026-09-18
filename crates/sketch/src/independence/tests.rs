//! What sketch · independence.rs is held to.

use super::*;

const POINTS: usize = 2;
const UNKNOWNS: usize = POINTS * 2;

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

    assert_eq!(forwards, 2);
    assert_eq!(backwards, forwards);
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
fn a_row_a_shade_off_one_the_basis_holds_is_dust_rather_than_information() {
    let held = [saying([1.0, 0.0, 0.0, 0.0])];

    assert!(
        is_dependent(&held, &saying([1.0, 1e-6, 0.0, 0.0])),
        "what is left over is judged against the row it came from, not              against an absolute size: a millionth of a unit is rounding",
    );
    assert!(!is_dependent(&held, &saying([1.0, 0.1, 0.0, 0.0])));
}

#[test]
fn the_size_of_a_circle_is_free_though_only_points_can_be_pinned() {
    let circles = 1;
    let free = null_space(&[], &[true, false], UNKNOWNS + circles);

    assert_eq!(
        free.len(),
        3,
        "the loose point moves two ways, and the radius is a third",
    );
    assert!(
        free.iter().any(|direction| direction[UNKNOWNS].abs() > 0.5),
        "a column past the last point is nobody's coordinate, and free",
    );
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
