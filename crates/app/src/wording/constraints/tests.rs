//! What app · wording/constraints.rs is held to.

use std::collections::BTreeSet;

use cao_sketch::{ArcId, CircleId, Element, PointId, SegmentId};

use super::*;

const FIRST: SegmentId = SegmentId(0);
const SECOND: SegmentId = SegmentId(1);

/// Every rule, next to how it reads, so that adding a variant without
/// saying how it reads cannot slip past the way two parallel lists would.
fn named_rules() -> [(Constraint, &'static str); 13] {
    [
        (
            Constraint::Perpendicular {
                first: FIRST,
                second: SECOND,
            },
            "Perpendiculaire",
        ),
        (
            Constraint::Parallel {
                first: FIRST,
                second: SECOND,
            },
            "Parallèle",
        ),
        (
            Constraint::Equal {
                first: FIRST,
                second: SECOND,
            },
            "Égalité",
        ),
        (
            Constraint::EqualRadius {
                first: CircleId(0),
                second: CircleId(1),
            },
            "Égalité",
        ),
        (
            Constraint::EqualRadiusArc {
                first: ArcId(0),
                second: ArcId(1),
            },
            "Égalité",
        ),
        (
            Constraint::OnSegment {
                point: PointId(1),
                segment: FIRST,
            },
            "Coïncidence",
        ),
        (
            Constraint::OnCircle {
                point: PointId(1),
                circle: CircleId(0),
            },
            "Coïncidence",
        ),
        (
            Constraint::Collinear {
                first: FIRST,
                second: SECOND,
            },
            "Colinéaire",
        ),
        (
            Constraint::AxisCollinear {
                segment: FIRST,
                axis: SketchAxis::U,
            },
            "Colinéaire",
        ),
        (
            Constraint::Tangent {
                circle: CircleId(0),
                segment: FIRST,
                at: None,
            },
            "Tangence",
        ),
        (
            Constraint::ArcTangent {
                arc: ArcId(0),
                segment: FIRST,
                at: None,
            },
            "Tangence",
        ),
        (
            Constraint::Midpoint {
                point: PointId(1),
                segment: FIRST,
            },
            "Milieu",
        ),
        (
            Constraint::Fixed {
                element: Element::Point(PointId(1)),
            },
            "Fixe",
        ),
    ]
}

#[test]
fn rules_that_say_the_same_thing_read_the_same_and_the_others_do_not() {
    let lang = Catalogue::french();
    let named = named_rules();

    for (rule, reads) in named {
        assert_eq!(label(&lang, rule), reads, "{rule:?} reads {reads:?}");
    }

    let read: BTreeSet<&str> = named.iter().map(|(_, reads)| *reads).collect();

    assert_eq!(
        read.len(),
        8,
        "thirteen rules read as eight names, five of them shared: {read:?}",
    );
}

#[test]
fn every_mark_is_drawable_in_the_fonts_the_interface_ships_with() {
    for (rule, _) in named_rules() {
        let drawn = mark(rule);

        assert!(
            drawn.is_ascii(),
            "{rule:?} is marked {drawn:?}, which needs a font the interface has not got",
        );
    }
}

#[test]
fn the_two_axes_of_a_sketch_read_differently() {
    let lang = Catalogue::french();

    assert_eq!(axis(&lang, SketchAxis::U), "axe horizontal");
    assert_eq!(axis(&lang, SketchAxis::V), "axe vertical");
}

#[test]
fn every_rule_of_the_constraint_tool_names_itself_and_says_what_it_wants() {
    let lang = Catalogue::french();
    let rules = [
        Rule::Perpendicular,
        Rule::Parallel,
        Rule::Equal,
        Rule::Coincident,
        Rule::Collinear,
        Rule::Tangent,
        Rule::Midpoint,
        Rule::Fixed,
        Rule::Concentric,
    ];

    let already_there: BTreeSet<String> = rules
        .iter()
        .map(|rule| already_there_label(&lang, *rule))
        .collect();
    let asks: BTreeSet<String> = rules
        .iter()
        .map(|rule| rule_asks_for(&lang, *rule))
        .collect();

    assert_eq!(
        already_there.len(),
        rules.len(),
        "each rule says it is already there under its own name",
    );
    assert_eq!(
        asks.len(),
        rules.len(),
        "each rule tells the title bar what to point at, and none share the wording",
    );
}

#[test]
fn a_masculine_rule_name_and_a_feminine_one_each_agree_with_their_own_sentence() {
    let lang = Catalogue::french();
    let midpoint = Constraint::Midpoint {
        point: PointId(1),
        segment: FIRST,
    };

    assert_eq!(erased_label(&lang, midpoint), "Milieu supprimé");
    assert_eq!(
        already_there_label(&lang, Rule::Midpoint),
        "Milieu : déjà posé"
    );

    let parallel = Constraint::Parallel {
        first: FIRST,
        second: SECOND,
    };

    assert_eq!(erased_label(&lang, parallel), "Parallèle supprimée");
    assert_eq!(
        already_there_label(&lang, Rule::Parallel),
        "Parallèle : déjà posée",
    );
}
