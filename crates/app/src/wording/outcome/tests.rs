//! What app · wording/outcome.rs is held to.
//!
//! Closes #467.
//! - the message reads « Règle impossible : elle contredit le dessin » —
//!   `a_refused_rule_says_it_contradicts_the_drawing`
//!
//! Closes #920 — the whole transcription is in
//! `crates/part/tests/an_edge_is_chamfered_or_rounded.rs`; this bullet is
//! what the edge panel says of a decline.
//! - A decline of the kernel at « Appliquer » records nothing: the panel stays
//!   open and shows the reason in French — the sentence for each reason the
//!   kernel can give, in the user's words: an edge the step does not handle
//!   yet, an edge no longer in the part, and every failure of the computation
//!   alike, since the kernel makes no distinction the user could act on; said
//!   in the panel, never in the message bar as well —
//!   `a_chamfer_or_a_fillet_declined_says_why_in_the_user_s_words`
//! - The step replays when a size changes upstream. An edge it no longer finds
//!   makes it a broken step carrying its reason — the sentence its alert
//!   reads, for a step already laid, which no panel is open to change: its
//!   edge gone, an edge the step does not handle, or the computation failing —
//!   `a_chamfer_or_a_fillet_broken_on_a_later_replay_says_why_in_the_user_s_words`

use cao_part::history::BlendShape;
use cao_solid::{Declined, Lost};

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

#[test]
fn a_refused_rule_says_it_contradicts_the_drawing() {
    assert_eq!(
        message(&Catalogue::french(), Some(Outcome::RuleRefused)).as_deref(),
        Some("Règle impossible : elle contredit le dessin"),
    );
}

#[test]
fn a_chamfer_or_a_fillet_declined_says_why_in_the_user_s_words() {
    let lang = Catalogue::french();
    let lost = "Cette arête n'est plus dans la pièce : choisissez-en une autre.";
    for (shape, not_yet, failed) in [
        (
            BlendShape::Chamfer,
            "Ce chanfrein n'est pas encore possible sur cette arête : la pièce reste comme elle était.",
            "Le calcul de ce chanfrein n'a pas abouti : la pièce reste comme elle était.",
        ),
        (
            BlendShape::Fillet,
            "Ce congé n'est pas encore possible sur cette arête : la pièce reste comme elle était.",
            "Le calcul de ce congé n'a pas abouti : la pièce reste comme elle était.",
        ),
    ] {
        assert_eq!(
            blend_declined(&lang, shape, Declined::Unsupported),
            not_yet,
            "{shape:?}",
        );
        for reason in [
            Declined::Lost(Lost::Gone),
            Declined::Lost(Lost::Split { now: 2 }),
        ] {
            assert_eq!(
                blend_declined(&lang, shape, reason),
                lost,
                "{shape:?} {reason:?}"
            );
        }
        for reason in [
            Declined::Profile,
            Declined::Travel,
            Declined::Tie,
            Declined::Unverified,
            Declined::Unfinished,
            Declined::Undrawn,
            Declined::Panicked,
        ] {
            assert_eq!(
                blend_declined(&lang, shape, reason),
                failed,
                "{shape:?} {reason:?}"
            );
        }
    }
    assert_eq!(
        message(&lang, Some(Outcome::BlendDeclined(Declined::Unsupported))),
        None,
        "the panel says why; the message bar does not say it a second time",
    );
}

#[test]
fn a_chamfer_or_a_fillet_broken_on_a_later_replay_says_why_in_the_user_s_words() {
    let lang = Catalogue::french();
    for (shape, lost, not_any_more, failed) in [
        (
            BlendShape::Chamfer,
            "L'arête de ce chanfrein n'est plus dans la pièce : il n'est pas construit.",
            "Ce chanfrein n'est plus possible sur son arête : il n'est pas construit.",
            "Le calcul de ce chanfrein n'a pas abouti : il n'est pas construit.",
        ),
        (
            BlendShape::Fillet,
            "L'arête de ce congé n'est plus dans la pièce : il n'est pas construit.",
            "Ce congé n'est plus possible sur son arête : il n'est pas construit.",
            "Le calcul de ce congé n'a pas abouti : il n'est pas construit.",
        ),
    ] {
        let said = |reason| crate::wording::alerts::blend_not_built(&lang, shape, reason);
        for reason in [
            Declined::Lost(Lost::Gone),
            Declined::Lost(Lost::Split { now: 2 }),
        ] {
            assert_eq!(said(reason), lost, "{shape:?} {reason:?}");
        }
        assert_eq!(said(Declined::Unsupported), not_any_more, "{shape:?}");
        for reason in [
            Declined::Profile,
            Declined::Travel,
            Declined::Tie,
            Declined::Unverified,
            Declined::Unfinished,
            Declined::Undrawn,
            Declined::Panicked,
        ] {
            assert_eq!(said(reason), failed, "{shape:?} {reason:?}");
        }
    }
}
