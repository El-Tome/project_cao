//! What a trait clicked into the panel of the mirror and the two patterns
//! brings with it: its two ends, taken and let go with it.
//!
//! Closes #704.
//! - While the panel's **Elements** slot is active, clicking a trait takes
//!   **the trait and its two ends**, and shows all three as selected —
//!   `a_trait_clicked_while_elements_is_active_is_held_with_its_two_ends_all_three_selected`;
//!   the panel's row counting all three on screen, in
//!   `crates/app/tests/copying_in_a_panel.rs`,
//!   `a_trait_clicked_into_the_panel_is_counted_with_its_two_ends`
//! - An end shared with another trait is taken as well —
//!   `an_end_shared_with_another_trait_is_taken_as_well`
//! - Clicking the trait again lets go of the trait and of the ends that no
//!   other taken trait still holds —
//!   `a_trait_clicked_again_lets_go_of_the_ends_no_other_held_trait_holds`
//! - A box pulled across the drawing, and a click on a circle, an arc or an
//!   ellipse, work as today — `a_box_takes_exactly_what_it_took_before`,
//!   `a_click_on_a_circle_an_arc_or_an_ellipse_takes_that_curve_alone`
//! - The same panel serves the circular and the rectangular pattern, which
//!   share its gather step: they behave the same way. Say so if that is not
//!   wanted — `both_patterns_take_and_let_go_of_a_trait_s_ends_as_the_mirror_does`
//! - must not break: a mirror, a circular pattern and a rectangular pattern of
//!   a selection give the same drawing as today (the points were already
//!   carried with their trait, or not: this issue only needs to be checked
//!   against `docs/sketch.md` on that) —
//!   `each_copy_lays_the_same_drawing_with_the_ends_held_as_without`
//! - must not break: the rules and dimensions copied with the elements (#320
//!   is not part of this) —
//!   `each_copy_leaves_the_same_rules_and_dimensions_with_the_ends_held_as_without`;
//!   that none is copied is held in `cao_sketch`'s `mirroring/tests.rs`,
//!   `the_copy_comes_out_bare_of_the_rules_the_original_carried`
//! - must not break: the preview in the panel —
//!   `the_preview_shows_the_same_copy_with_the_ends_held_as_without`, and
//!   `the_preview_shows_across_the_axis_held_not_under_the_cursor` in
//!   `panel.rs`

use cao_part::{Operation, PointRef};
use cao_prefs::Command;
use cao_sketch::{
    ArcId, CircleId, Constraint, DimensionTarget, Element, EllipseId, PointId, SegmentId,
    Selection, Sketch,
};
use glam::DVec2;

use super::super::{previewed, take_up};
use super::bench::Bench;
use super::panel::{a_trait_under_a_box, pulled_with};
use crate::screens::sketch::CopySlot;

const COPY_TOOLS: [Command; 3] = [
    Command::ToolMirror,
    Command::ToolCircularPattern,
    Command::ToolRectangularPattern,
];

const THE_FOOT: DVec2 = DVec2::new(-10.0, -5.0);
const THE_CORNER: DVec2 = DVec2::new(-10.0, 5.0);
const THE_FAR_END: DVec2 = DVec2::new(-20.0, 5.0);
const ON_THE_UPRIGHT: DVec2 = DVec2::new(-10.0, 0.0);
const ON_THE_TOP: DVec2 = DVec2::new(-15.0, 5.0);
const ON_THE_TARGET: DVec2 = DVec2::new(2.0, 0.0);
const THE_TARGET_S_FOOT: DVec2 = DVec2::new(2.0, -5.0);

const UPRIGHT: SegmentId = SegmentId(0);
const TOP: SegmentId = SegmentId(1);

/// Two traits of one polyline meeting at a corner — an upright and a top
/// running left from its head — and, clear of them, an upright trait to mirror
/// across, run along or turn about the foot of.
fn a_corner_and_a_target() -> Bench {
    let mut bench = Bench::drawn(vec![Bench::a_trait(THE_FOOT, THE_CORNER, false)]);
    let corner = bench.point_at(THE_CORNER);
    for operation in [
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(corner),
            end: PointRef::New(THE_FAR_END),
            construction: false,
        },
        Bench::a_trait(THE_TARGET_S_FOOT, DVec2::new(2.0, 5.0), false),
    ] {
        bench.document.apply(operation);
    }
    let sides = bench.document.sketches()[0].segments();
    assert_eq!(
        sides[UPRIGHT.0].end, sides[TOP.0].start,
        "the two traits share their corner"
    );
    bench
}

fn point(bench: &Bench, at: DVec2) -> Element {
    Element::Point(bench.point_at(at))
}

fn holds_exactly(bench: &Bench, expected: &[Element]) -> bool {
    let held = bench.elements();
    held.len() == expected.len() && expected.iter().all(|element| held.contains(element))
}

#[test]
fn a_trait_clicked_while_elements_is_active_is_held_with_its_two_ends_all_three_selected() {
    let mut bench = a_corner_and_a_target();
    bench.take(Command::ToolMirror);

    bench.click(ON_THE_UPRIGHT);

    let taken = vec![
        Element::Segment(UPRIGHT),
        point(&bench, THE_FOOT),
        point(&bench, THE_CORNER),
    ];
    assert_eq!(
        bench.elements(),
        taken,
        "the trait first, then the point it starts from and the one it ends on"
    );
    for element in taken {
        assert!(
            bench.editor.is_selected(Selection::Element(element)),
            "{element:?} is drawn selected"
        );
    }
    assert_eq!(bench.active(), CopySlot::Elements);
}

#[test]
fn an_end_shared_with_another_trait_is_taken_as_well() {
    let mut bench = a_corner_and_a_target();
    bench.take(Command::ToolMirror);

    bench.click(ON_THE_UPRIGHT);
    assert!(
        bench.elements().contains(&point(&bench, THE_CORNER)),
        "the corner the top also runs from is taken, the top not being held: {:?}",
        bench.elements()
    );
    bench.click(ON_THE_TOP);

    assert_eq!(
        bench.elements(),
        vec![
            Element::Segment(UPRIGHT),
            point(&bench, THE_FOOT),
            point(&bench, THE_CORNER),
            Element::Segment(TOP),
            point(&bench, THE_FAR_END),
        ],
        "the corner both traits stand on is held once"
    );
}

#[test]
fn a_trait_clicked_again_lets_go_of_the_ends_no_other_held_trait_holds() {
    let mut bench = a_corner_and_a_target();
    bench.take(Command::ToolMirror);
    bench.click(ON_THE_UPRIGHT);
    bench.click(ON_THE_TOP);

    bench.click(ON_THE_UPRIGHT);

    let still = [
        Element::Segment(TOP),
        point(&bench, THE_CORNER),
        point(&bench, THE_FAR_END),
    ];
    assert!(
        holds_exactly(&bench, &still),
        "the upright and its foot go; the corner stays with the top still held: {:?}",
        bench.elements()
    );
    assert!(
        !bench
            .editor
            .is_selected(Selection::Element(point(&bench, THE_FOOT)))
    );

    bench.click(ON_THE_TOP);

    assert_eq!(
        bench.elements(),
        Vec::new(),
        "nothing holds the corner any more"
    );
}

#[test]
fn both_patterns_take_and_let_go_of_a_trait_s_ends_as_the_mirror_does() {
    for command in [
        Command::ToolCircularPattern,
        Command::ToolRectangularPattern,
    ] {
        let mut bench = a_corner_and_a_target();
        bench.take(command);

        bench.click(ON_THE_UPRIGHT);
        assert_eq!(
            bench.elements(),
            bench.a_trait_and_its_ends(UPRIGHT),
            "{command:?} takes the trait with its two ends"
        );
        for element in bench.elements() {
            assert!(
                bench.editor.is_selected(Selection::Element(element)),
                "{command:?} draws {element:?} selected"
            );
        }

        bench.click(ON_THE_TOP);
        bench.click(ON_THE_UPRIGHT);
        let still = [
            Element::Segment(TOP),
            point(&bench, THE_CORNER),
            point(&bench, THE_FAR_END),
        ];
        assert!(
            holds_exactly(&bench, &still),
            "{command:?} lets go of the upright and its foot, not of the corner: {:?}",
            bench.elements()
        );
    }
}

#[test]
fn a_box_takes_exactly_what_it_took_before() {
    let (mut state, mut bench) = a_trait_under_a_box();

    pulled_with(
        &mut state,
        &mut bench,
        egui::PointerButton::Primary,
        egui::Modifiers::NONE,
    );

    let inside = bench.document.sketches()[0].segments()[0];
    assert_eq!(
        bench.elements(),
        vec![
            Element::Point(inside.start),
            Element::Point(inside.end),
            Element::Segment(SegmentId(0)),
        ],
        "the ends first, then the trait, as the box has always taken them"
    );
}

const THE_CIRCLE_S_CENTRE: DVec2 = DVec2::new(-30.0, 0.0);
const THE_ARC_S_CENTRE: DVec2 = DVec2::new(-30.0, 30.0);
const THE_ELLIPSE_S_CENTRE: DVec2 = DVec2::new(30.0, 30.0);

/// A circle, an arc and an ellipse, each well clear of the others.
fn three_curves() -> Bench {
    let at = |centre: DVec2, offset: DVec2| PointRef::New(centre + offset);
    Bench::drawn(vec![
        Operation::AddCircle {
            sketch: 0,
            center: PointRef::New(THE_CIRCLE_S_CENTRE),
            radius: 4.0,
            rim: vec![],
            construction: false,
        },
        Operation::AddArc {
            sketch: 0,
            center: PointRef::New(THE_ARC_S_CENTRE),
            start: at(THE_ARC_S_CENTRE, DVec2::new(4.0, 0.0)),
            end: at(THE_ARC_S_CENTRE, DVec2::new(0.0, 4.0)),
            construction: false,
        },
        Operation::AddEllipse {
            sketch: 0,
            center: PointRef::New(THE_ELLIPSE_S_CENTRE),
            first: [
                at(THE_ELLIPSE_S_CENTRE, DVec2::new(-6.0, 0.0)),
                at(THE_ELLIPSE_S_CENTRE, DVec2::new(6.0, 0.0)),
            ],
            second: [
                at(THE_ELLIPSE_S_CENTRE, DVec2::new(0.0, -3.0)),
                at(THE_ELLIPSE_S_CENTRE, DVec2::new(0.0, 3.0)),
            ],
            construction: false,
            drawn: None,
        },
    ])
}

#[test]
fn a_click_on_a_circle_an_arc_or_an_ellipse_takes_that_curve_alone() {
    let half = std::f64::consts::FRAC_1_SQRT_2;
    let clicked = [
        (
            THE_CIRCLE_S_CENTRE + DVec2::splat(4.0 * half),
            Element::Circle(CircleId(0)),
        ),
        (
            THE_ARC_S_CENTRE + DVec2::splat(4.0 * half),
            Element::Arc(ArcId(0)),
        ),
        (
            THE_ELLIPSE_S_CENTRE + DVec2::new(6.0 * half, 3.0 * half),
            Element::Ellipse(EllipseId(0)),
        ),
    ];
    for command in COPY_TOOLS {
        for (on_the_curve, curve) in clicked {
            let mut bench = three_curves();
            bench.take(command);

            bench.click(on_the_curve);

            assert_eq!(bench.elements(), vec![curve], "{command:?}");
        }
    }
}

/// The corner drawn with a rule between its two traits and a length on the
/// upright, so that a copy has something it could carry over.
fn a_ruled_corner_and_a_target() -> Bench {
    let mut bench = a_corner_and_a_target();
    for rule in [
        Operation::Constrain {
            sketch: 0,
            constraint: Constraint::Perpendicular {
                first: UPRIGHT,
                second: TOP,
            },
        },
        Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::Length(UPRIGHT),
            value: 10.0.into(),
            placement: None,
        },
    ] {
        bench.document.apply(rule);
    }
    bench
}

/// The tool taken on the ruled corner with both its traits held — by clicking
/// them, or handed over as the two traits alone — then its target picked and
/// its values typed, up to Validate.
fn on_validate(command: Command, clicked: bool) -> Bench {
    let mut bench = a_ruled_corner_and_a_target();
    bench.take(command);
    if clicked {
        bench.click(ON_THE_UPRIGHT);
        bench.click(ON_THE_TOP);
        bench.enter();
    } else {
        let traits = vec![Element::Segment(UPRIGHT), Element::Segment(TOP)];
        take_up(&mut bench.context(), traits);
    }
    assert_eq!(bench.active(), CopySlot::Target, "{command:?}");
    match command {
        Command::ToolCircularPattern => bench.click(THE_TARGET_S_FOOT),
        _ => bench.click(ON_THE_TARGET),
    }
    let values: &[&str] = match command {
        Command::ToolCircularPattern => &["90", "4"],
        Command::ToolRectangularPattern => &["10", "2", "10", "2"],
        _ => &[],
    };
    for (rank, value) in values.iter().enumerate() {
        bench.type_value(rank, value);
    }
    bench.click_the_slot(CopySlot::Validate);
    bench
}

fn laid(command: Command, clicked: bool) -> Bench {
    let mut bench = on_validate(command, clicked);
    let traits_before = bench.document.sketches()[0].segments().len();
    bench.enter();
    assert!(
        bench.document.sketches()[0].segments().len() > traits_before,
        "{command:?} laid its copy"
    );
    bench
}

/// A direct geometric comparison: both drawings went through the same
/// transform of the same places.
const TOLERANCE: f64 = 1e-9;

fn same_places(first: &[DVec2], second: &[DVec2]) -> bool {
    first.len() == second.len()
        && first
            .iter()
            .zip(second)
            .all(|(one, other)| (*one - *other).length() <= TOLERANCE)
}

fn sides(sketch: &Sketch) -> Vec<(PointId, PointId, bool)> {
    sketch
        .segments()
        .iter()
        .map(|side| (side.start, side.end, side.construction))
        .collect()
}

#[test]
fn each_copy_lays_the_same_drawing_with_the_ends_held_as_without() {
    for command in COPY_TOOLS {
        let clicked = laid(command, true);
        let handed = laid(command, false);
        let [clicked, handed] = [&clicked, &handed].map(|bench| &bench.document.sketches()[0]);

        assert!(
            same_places(clicked.points(), handed.points()),
            "{command:?} lays the same points, in the same order:\n{:?}\n{:?}",
            clicked.points(),
            handed.points()
        );
        assert_eq!(sides(clicked), sides(handed), "{command:?}");
    }
}

#[test]
fn each_copy_leaves_the_same_rules_and_dimensions_with_the_ends_held_as_without() {
    for command in COPY_TOOLS {
        let clicked = laid(command, true);
        let handed = laid(command, false);
        let [clicked, handed] = [&clicked, &handed].map(|bench| &bench.document.sketches()[0]);

        assert_eq!(clicked.constraints(), handed.constraints(), "{command:?}");
        let measured = |sketch: &Sketch| {
            sketch
                .dimensions()
                .iter()
                .map(|dimension| (dimension.target, dimension.driven))
                .collect::<Vec<_>>()
        };
        assert_eq!(measured(clicked), measured(handed), "{command:?}");
        let values = |sketch: &Sketch| {
            sketch
                .dimensions()
                .iter()
                .map(|dimension| DVec2::new(dimension.value, 0.0))
                .collect::<Vec<_>>()
        };
        assert!(
            same_places(&values(clicked), &values(handed)),
            "{command:?} keeps the same values"
        );
    }
}

#[test]
fn the_preview_shows_the_same_copy_with_the_ends_held_as_without() {
    for command in COPY_TOOLS {
        let [clicked, handed] = [true, false].map(|clicked| {
            let bench = on_validate(command, clicked);
            previewed(&bench.document.sketches()[0], &bench.editor, 1.0)
                .unwrap_or_else(|| panic!("{command:?} shows a preview on Validate"))
        });

        assert_eq!(clicked.laid, handed.laid, "{command:?}");
        assert!(
            same_places(clicked.sketch.points(), handed.sketch.points()),
            "{command:?}"
        );
    }
}
