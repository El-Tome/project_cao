//! What app · screens/title_bar/state.rs is held to: which of an error and a
//! message the pill keeps once a frame is over.
//!
//! Closes #726.
//! - a fresh error, raised in the frame or since the last one was settled (the
//!   Preferences, the copy panel's place), takes the pill, and the message
//!   under it is left where it was —
//!   `a_fresh_error_takes_the_pill_and_leaves_the_message_under_it`
//! - a standing error gives way to a command —
//!   `a_standing_error_gives_way_to_a_command`
//! - or to a message other than the one the frame began with —
//!   `a_standing_error_gives_way_to_a_message_said_since`
//! - a message said again unchanged every frame, as choosing a plane says
//!   « Choisissez un plan d'esquisse », does not make it give way —
//!   `a_message_said_again_unchanged_leaves_the_error_standing`
//! - when it gives way and the message is still the one it covered, that
//!   message goes too: the message before the error never comes back —
//!   `the_message_an_error_covered_does_not_come_back_once_it_gives_way`
//! - a fresh error wins over a message said in the same frame —
//!   `a_fresh_error_wins_over_a_message_said_in_the_same_frame`
//! - a save the disk keeps refusing is a fresh error at every frame, and keeps
//!   the pill whatever is said or asked — decided: a save that keeps failing
//!   stays seen — `a_save_the_disk_keeps_refusing_keeps_the_pill`
//! - a standing error alone stays, a later save that succeeds taking nothing
//!   back — `a_standing_error_alone_stays`
//!
//! The rule replaces one that cleared the message as a frame began, so that
//! any message found at its end read as said in it: choosing a plane says its
//! sentence again at every frame, and an error from the Preferences went at
//! the first move of the mouse. A message is now said in a frame when the one
//! at its end is not the one at its start; one cleared by a gesture, at its end
//! `None`, is not a message said, and the rule leaves that case to the
//! implementation.

use super::*;

const SAVE_FAILED: &str = "L'échange avec /CAO/Platine.caopart s'est arrêté en chemin.";
const CHOOSE_A_PLANE: &str = "Choisissez un plan d'esquisse";
const ARC_HINT: &str = "Cliquez le centre, puis le départ de la courbe, puis son arrivée";

/// What a frame hands the rule: the error the pill held as it began, the one
/// raised since, the message at its start, and whether a command ran in it.
struct Frame<'a> {
    standing: Option<Covering>,
    fresh: Option<&'a str>,
    at_start: Option<&'a str>,
    commanded: bool,
}

/// The error the pill keeps once the frame is over, and the message left in
/// the slot, from the message the frame ended with.
fn settled(frame: Frame<'_>, at_end: Option<&str>) -> (Option<Covering>, Option<String>) {
    let mut message = at_end.map(str::to_owned);
    let kept = settle(
        frame.standing,
        frame.fresh.map(str::to_owned),
        frame.at_start,
        &mut message,
        frame.commanded,
    );
    (kept, message)
}

fn covering(error: &str, covers: Option<&str>) -> Covering {
    Covering {
        error: error.to_owned(),
        covers: covers.map(str::to_owned),
    }
}

#[test]
fn a_fresh_error_takes_the_pill_and_leaves_the_message_under_it() {
    let frame = Frame {
        standing: None,
        fresh: Some(SAVE_FAILED),
        at_start: Some(ARC_HINT),
        commanded: false,
    };

    let (kept, message) = settled(frame, Some(ARC_HINT));

    assert_eq!(kept, Some(covering(SAVE_FAILED, Some(ARC_HINT))));
    assert_eq!(
        message.as_deref(),
        Some(ARC_HINT),
        "the error lies over the message, and does not wipe it",
    );
}

#[test]
fn a_standing_error_gives_way_to_a_command() {
    let frame = Frame {
        standing: Some(covering(SAVE_FAILED, None)),
        fresh: None,
        at_start: None,
        commanded: true,
    };

    let (kept, _) = settled(frame, None);

    assert_eq!(
        kept, None,
        "a command was asked, and the error is still kept"
    );
}

#[test]
fn a_standing_error_gives_way_to_a_message_said_since() {
    let frame = Frame {
        standing: Some(covering(SAVE_FAILED, Some(CHOOSE_A_PLANE))),
        fresh: None,
        at_start: Some(CHOOSE_A_PLANE),
        commanded: false,
    };

    let (kept, message) = settled(frame, Some(ARC_HINT));

    assert_eq!(
        kept, None,
        "a new message was said, and the error is still kept"
    );
    assert_eq!(
        message.as_deref(),
        Some(ARC_HINT),
        "the message said in the frame is the one the pill is left with",
    );
}

#[test]
fn a_message_said_again_unchanged_leaves_the_error_standing() {
    let standing = covering(SAVE_FAILED, Some(CHOOSE_A_PLANE));
    let frame = Frame {
        standing: Some(standing.clone()),
        fresh: None,
        at_start: Some(CHOOSE_A_PLANE),
        commanded: false,
    };

    let (kept, message) = settled(frame, Some(CHOOSE_A_PLANE));

    assert_eq!(
        kept,
        Some(standing),
        "the sentence choosing a plane says again at every frame took the error away",
    );
    assert_eq!(message.as_deref(), Some(CHOOSE_A_PLANE));
}

#[test]
fn the_message_an_error_covered_does_not_come_back_once_it_gives_way() {
    let frame = Frame {
        standing: Some(covering(SAVE_FAILED, Some(ARC_HINT))),
        fresh: None,
        at_start: Some(ARC_HINT),
        commanded: true,
    };

    let (kept, message) = settled(frame, Some(ARC_HINT));

    assert_eq!(kept, None);
    assert_eq!(
        message, None,
        "a command that leaves the message alone, as opening the Preferences does, \
         brought back the message the error had covered",
    );
}

#[test]
fn a_fresh_error_wins_over_a_message_said_in_the_same_frame() {
    let frame = Frame {
        standing: None,
        fresh: Some(SAVE_FAILED),
        at_start: Some(CHOOSE_A_PLANE),
        commanded: false,
    };

    let (kept, message) = settled(frame, Some(ARC_HINT));

    assert_eq!(kept, Some(covering(SAVE_FAILED, Some(ARC_HINT))));
    assert_eq!(message.as_deref(), Some(ARC_HINT));
}

#[test]
fn a_save_the_disk_keeps_refusing_keeps_the_pill() {
    let frame = Frame {
        standing: Some(covering(SAVE_FAILED, Some(CHOOSE_A_PLANE))),
        fresh: Some(SAVE_FAILED),
        at_start: Some(CHOOSE_A_PLANE),
        commanded: true,
    };

    let (kept, message) = settled(frame, Some(ARC_HINT));

    assert_eq!(
        kept,
        Some(covering(SAVE_FAILED, Some(ARC_HINT))),
        "a save failing again in a frame where a command ran and a hint was said is \
         not kept, or not over that hint",
    );
    assert_eq!(message.as_deref(), Some(ARC_HINT));
}

#[test]
fn a_standing_error_alone_stays() {
    let standing = covering(SAVE_FAILED, None);
    let frame = Frame {
        standing: Some(standing.clone()),
        fresh: None,
        at_start: None,
        commanded: false,
    };

    let (kept, message) = settled(frame, None);

    assert_eq!(
        kept,
        Some(standing),
        "a frame where nothing was said nor asked took the error away",
    );
    assert_eq!(message, None);
}
