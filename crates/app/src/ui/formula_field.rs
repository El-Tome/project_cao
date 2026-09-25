//! The field a formula is typed into: every whole name it is offered stands
//! as a block, and the name being typed is completed from a list shown under
//! it — the names offered that go on from what was typed, case aside, each
//! with what it says beside it.
//!
//! Plain values in, what the user did out. What makes a name — which
//! characters open one, which go on one — is handed in, so that the list and
//! whatever reads the field agree on it.

use std::ops::Range;

mod blocks;

/// A name the field can complete to, and what is shown beside it.
#[derive(Clone, Debug, PartialEq)]
pub struct Offer {
    pub name: String,
    pub beside: String,
}

/// Which characters open a name, and which go on one.
#[derive(Clone, Copy)]
pub struct Naming {
    pub starts: fn(char) -> bool,
    pub goes_on: fn(char) -> bool,
}

/// What a field's list holds between frames.
#[derive(Clone, Default)]
struct Listed {
    /// Whether it was shown: what the keys it takes are read by.
    open: bool,
    /// What was typed, the last time it was shown.
    typed: Option<String>,
    /// The rank of the name chosen among those shown.
    chosen: usize,
    /// What was typed when Échap closed it: it stays closed until that
    /// changes.
    closed_on: Option<String>,
}

/// A single-line field, `width` wide, showing `hint` while empty, for a
/// formula that names what `offers` holds.
///
/// Every whole name offered is a block: shown on a tint, crossed by ← and →
/// in one step, erased whole by Retour arrière after it or Suppr before it,
/// and never entered — a click inside one puts the cursor at its nearer edge.
///
/// The start of a name opens a list under the field of the names that go on
/// from it. While it is shown the list answers first: the arrows move the
/// choice, Entrée or Tab puts the name chosen in place of the one typed, a
/// click on a name does too, and Échap closes the list and does nothing else.
/// The field keeps Tab and Échap for that while the list is shown — egui
/// would otherwise take the keyboard away on them before any field is drawn.
///
/// A name put in or a block erased marks the field changed, as typing would.
pub fn formula_field(
    ui: &mut egui::Ui,
    id: egui::Id,
    text: &mut String,
    (width, hint): (f32, &str),
    offers: &[Offer],
    naming: Naming,
) -> egui::text_edit::TextEditOutput {
    let names: Vec<&str> = offers.iter().map(|offer| offer.name.as_str()).collect();
    let kept = id.with("completion");
    let mut listed: Listed = ui.data(|data| data.get_temp(kept)).unwrap_or_default();
    let was_open = listed.open;
    let mut completed = false;
    if ui.memory(|memory| memory.has_focus(id)) {
        completed |= keys_around_blocks(ui, id, text, &names, naming);
    }
    if listed.open && ui.memory(|memory| memory.has_focus(id)) {
        let pressed =
            |key: egui::Key| ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, key));
        let (down, up) = (pressed(egui::Key::ArrowDown), pressed(egui::Key::ArrowUp));
        let taken = pressed(egui::Key::Enter) | pressed(egui::Key::Tab);
        let closed = pressed(egui::Key::Escape);
        listed.chosen = match (down, up) {
            (true, false) => listed.chosen + 1,
            (false, true) => listed.chosen.saturating_sub(1),
            _ => listed.chosen,
        };
        let named = egui::TextEdit::load_state(ui.ctx(), id)
            .and_then(|state| state.cursor.char_range())
            .and_then(|range| name_at(text, range.primary.index.0, naming));
        if let (true, Some((range, typed))) = (taken, &named)
            && let Some(offer) = chosen(&starting_with(offers, typed), listed.chosen)
        {
            let (put, at) = replaced(text, range, &offer.name);
            *text = put;
            place_the_cursor(ui.ctx(), id, at);
            completed = true;
        }
        if closed {
            listed.closed_on = named.map(|(_, typed)| typed);
        }
        if taken || closed {
            listed.open = false;
        }
    }

    let tint = ui.visuals().selection.bg_fill.gamma_multiply(0.45);
    let mut layouter = |ui: &egui::Ui, buffer: &dyn egui::TextBuffer, wrap_width: f32| {
        let text = buffer.as_str();
        let font = egui::FontSelection::default().resolve(ui.style());
        let color = ui
            .visuals()
            .override_text_color
            .unwrap_or_else(|| ui.visuals().widgets.inactive.text_color());
        let mut job = blocks::laid_out(
            text,
            &blocks::found(text, &names, naming),
            font,
            color,
            tint,
        );
        job.wrap.max_width = wrap_width;
        ui.painter().layout_job(job)
    };
    let mut output = egui::TextEdit::singleline(text)
        .id(id)
        .desired_width(width)
        .hint_text(hint)
        .layouter(&mut layouter)
        .event_filter(egui::EventFilter {
            horizontal_arrows: true,
            vertical_arrows: true,
            tab: listed.open,
            escape: listed.open,
        })
        .show(ui);

    let response = output.response.response.clone();
    if response.has_focus() {
        keep_out_of_blocks(ui.ctx(), id, &mut output.state, text, &names, naming);
    }
    // A click on the list lands outside the field, which gives the keyboard
    // away on it: the list is still drawn that frame, or the click is lost.
    let typing = response.has_focus() || (was_open && response.lost_focus());
    let named = output
        .state
        .cursor
        .char_range()
        .filter(|_| typing)
        .and_then(|range| name_at(text, range.primary.index.0, naming));
    let typed = named.as_ref().map(|(_, typed)| typed.clone());
    if listed.typed != typed {
        listed.chosen = 0;
        listed.typed.clone_from(&typed);
    }
    if listed.closed_on.is_some() && listed.closed_on != typed {
        listed.closed_on = None;
    }
    let shown = typed
        .as_deref()
        .map_or_else(Vec::new, |typed| starting_with(offers, typed));
    // A name typed out whole has nothing left to complete.
    let whole = matches!(shown.as_slice(), [only] if Some(&only.name) == typed.as_ref());
    listed.open = !shown.is_empty() && !whole && listed.closed_on.is_none() && !completed;
    listed.chosen = listed.chosen.min(shown.len().saturating_sub(1));

    if listed.open {
        let mut clicked = None;
        egui::Popup::from_response(&response)
            .open(true)
            .close_behavior(egui::PopupCloseBehavior::IgnoreClicks)
            .show(|ui| {
                for (rank, offer) in shown.iter().enumerate() {
                    let said = format!("{}   {}", offer.name, offer.beside);
                    if ui.selectable_label(rank == listed.chosen, said).clicked() {
                        clicked = Some(rank);
                    }
                }
            });
        if let (Some(rank), Some((range, _))) = (clicked, &named) {
            let (put, at) = replaced(text, range, &shown[rank].name);
            *text = put;
            place_the_cursor(ui.ctx(), id, at);
            // The click landed outside the field, which gave the keyboard
            // away on it; it goes back to the field, still being typed in.
            ui.memory_mut(|memory| memory.request_focus(id));
            listed.open = false;
            completed = true;
        }
        // The next key may be one the list takes, and the field only keeps
        // it from egui once it has been drawn with the list shown.
        if !was_open {
            ui.ctx().request_repaint();
        }
    }
    if completed {
        output.response.response.mark_changed();
    }
    ui.data_mut(|data| data.insert_temp(kept, listed));
    output
}

/// Retour arrière, Suppr, ← and → where a block stands next to the cursor,
/// read before the field sees them: a block goes whole, and is crossed in one
/// step — Maj held, the selection stretches over it. Says whether the text
/// changed.
fn keys_around_blocks(
    ui: &egui::Ui,
    id: egui::Id,
    text: &mut String,
    names: &[&str],
    naming: Naming,
) -> bool {
    let found = blocks::found(text, names, naming);
    let Some(range) = egui::TextEdit::load_state(ui.ctx(), id)
        .and_then(|state| state.cursor.char_range())
        .filter(|_| !found.is_empty())
    else {
        return false;
    };
    let (primary, secondary) = (range.primary.index.0, range.secondary.index.0);
    let pressed = |modifiers, key| ui.input_mut(|input| input.consume_key(modifiers, key));
    if primary == secondary {
        for (forward, key) in [(false, egui::Key::Backspace), (true, egui::Key::Delete)] {
            if let Some(block) = blocks::erased(primary, forward, &found)
                && pressed(egui::Modifiers::NONE, key)
            {
                let (put, at) = replaced(text, &block, "");
                *text = put;
                place_the_cursor(ui.ctx(), id, at);
                return true;
            }
        }
    }
    for (forward, key) in [(false, egui::Key::ArrowLeft), (true, egui::Key::ArrowRight)] {
        let Some(across) = blocks::stepped(primary, forward, &found) else {
            continue;
        };
        if primary == secondary && pressed(egui::Modifiers::NONE, key) {
            place_the_cursor(ui.ctx(), id, across);
        } else if pressed(egui::Modifiers::SHIFT, key) {
            place_the_selection(ui.ctx(), id, secondary, across);
        }
    }
    false
}

/// Puts a cursor, or either end of a selection, that landed inside a block
/// at the block's nearer edge — a click, a drag, a jump by word.
fn keep_out_of_blocks(
    ctx: &egui::Context,
    id: egui::Id,
    state: &mut egui::text_edit::TextEditState,
    text: &str,
    names: &[&str],
    naming: Naming,
) {
    let Some(range) = state.cursor.char_range() else {
        return;
    };
    let found = blocks::found(text, names, naming);
    let (primary, secondary) = (range.primary.index.0, range.secondary.index.0);
    let kept = (
        blocks::snapped(primary, &found),
        blocks::snapped(secondary, &found),
    );
    if kept != (primary, secondary) {
        place_the_selection(ctx, id, kept.1, kept.0);
        state
            .cursor
            .set_char_range(Some(egui::text::CCursorRange::two(
                egui::text::CCursor::new(kept.1),
                egui::text::CCursor::new(kept.0),
            )));
    }
}

fn place_the_selection(ctx: &egui::Context, id: egui::Id, from: usize, to: usize) {
    let mut state = egui::TextEdit::load_state(ctx, id).unwrap_or_default();
    state
        .cursor
        .set_char_range(Some(egui::text::CCursorRange::two(
            egui::text::CCursor::new(from),
            egui::text::CCursor::new(to),
        )));
    state.store(ctx, id);
}

/// Whether the field holding the keyboard has its list shown, which is when
/// Échap is the list's to close and nobody else's to read.
pub fn keeps_escape(ctx: &egui::Context) -> bool {
    ctx.memory(|memory| memory.focused()).is_some_and(|id| {
        ctx.data(|data| data.get_temp::<Listed>(id.with("completion")))
            .is_some_and(|listed| listed.open)
    })
}

fn chosen<'a>(shown: &[&'a Offer], rank: usize) -> Option<&'a Offer> {
    shown.get(rank.min(shown.len().saturating_sub(1))).copied()
}

pub(super) fn place_the_cursor(ctx: &egui::Context, id: egui::Id, at: usize) {
    let mut state = egui::TextEdit::load_state(ctx, id).unwrap_or_default();
    state
        .cursor
        .set_char_range(Some(egui::text::CCursorRange::one(
            egui::text::CCursor::new(at),
        )));
    state.store(ctx, id);
}

/// The name the cursor stands in, counted in characters: the whole run of
/// characters that go on a name around the cursor, and what of it stands
/// before the cursor — what the list goes by. Nothing when that does not
/// open a name: after a digit, what is typed is a number.
fn name_at(text: &str, cursor: usize, naming: Naming) -> Option<(Range<usize>, String)> {
    let characters: Vec<char> = text.chars().collect();
    let cursor = cursor.min(characters.len());
    let start = characters[..cursor]
        .iter()
        .rposition(|character| !(naming.goes_on)(*character))
        .map_or(0, |at| at + 1);
    let end = characters[cursor..]
        .iter()
        .position(|character| !(naming.goes_on)(*character))
        .map_or(characters.len(), |at| cursor + at);
    let typed: String = characters[start..cursor].iter().collect();
    typed
        .chars()
        .next()
        .is_some_and(naming.starts)
        .then_some((start..end, typed))
}

/// The names offered that go on from what was typed, case aside, in the
/// order they were offered.
fn starting_with<'a>(offers: &'a [Offer], typed: &str) -> Vec<&'a Offer> {
    let typed = typed.to_lowercase();
    offers
        .iter()
        .filter(|offer| offer.name.to_lowercase().starts_with(&typed))
        .collect()
}

/// The text with `name` in place of the characters `range` covers, and the
/// character the cursor stands at after it.
pub(super) fn replaced(text: &str, range: &Range<usize>, name: &str) -> (String, usize) {
    let characters: Vec<char> = text.chars().collect();
    let mut completed: String = characters[..range.start].iter().collect();
    completed.push_str(name);
    let at = completed.chars().count();
    completed.extend(&characters[range.end.min(characters.len())..]);
    (completed, at)
}

#[cfg(test)]
mod tests;
