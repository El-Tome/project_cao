//! What app · wording/outcome.rs is held to.

use super::*;

fn said(outcome: DimensionOutcome) -> Option<String> {
    message(&Catalogue::french(), Some(Outcome::Dimension(outcome)))
}

#[test]
fn a_dimension_that_fixes_the_scale_says_so_whichever_call_site_applied_it() {
    assert_eq!(
        said(DimensionOutcome::ScaleDefined {
            millimeters_per_unit: 2.5,
        })
        .as_deref(),
        Some("Échelle définie : 1 unité = 2.5000 mm"),
    );
}

#[test]
fn a_redundant_dimension_applied_without_an_open_field_still_warns() {
    let lang = Catalogue::french();

    assert_eq!(
        said(DimensionOutcome::Reference).as_deref(),
        Some(dimension::redundant_warning(&lang).as_str()),
    );
}

#[test]
fn a_value_the_drawing_could_only_approach_warns_whichever_call_site_applied_it() {
    let lang = Catalogue::french();

    assert_eq!(
        said(DimensionOutcome::Geometry(LengthOutcome::BestEffort)).as_deref(),
        Some(dimension::conflict_warning(&lang).as_str()),
    );
}

#[test]
fn a_cut_says_how_much_of_the_drawing_went_with_the_trait() {
    let lang = Catalogue::french();
    let said = |rules, values| {
        message(
            &lang,
            Some(Outcome::Cut {
                rules,
                values,
                refused: 0,
            }),
        )
        .unwrap_or_default()
    };

    assert!(said(2, 0).contains("2 contrainte"), "{}", said(2, 0));
    assert!(said(0, 1).contains("1 cote"), "{}", said(0, 1));
    let both = said(2, 1);
    assert!(
        both.contains("2 contrainte") && both.contains("1 cote"),
        "{both}",
    );
}

#[test]
fn a_cut_that_took_nothing_with_it_says_nothing() {
    assert_eq!(
        message(
            &Catalogue::french(),
            Some(Outcome::Cut {
                rules: 0,
                values: 0,
                refused: 0
            })
        ),
        None,
    );
}

#[test]
fn an_ordinary_dimension_says_nothing() {
    assert_eq!(said(DimensionOutcome::Geometry(LengthOutcome::Exact)), None);
    assert_eq!(message(&Catalogue::french(), None), None);
}
