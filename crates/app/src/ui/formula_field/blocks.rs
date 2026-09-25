//! The names in a formula field as blocks: every whole name the field is
//! offered stands as one, shown on a tint, stepped over and erased whole, so
//! that nothing written can split a name.

use std::ops::Range;

use super::Naming;

/// Every whole name among `names` in `text`, counted in characters: a run of
/// characters that go on a name, opened by one that starts a name, and
/// equal to one of the names as it is cased.
pub(super) fn found(text: &str, names: &[&str], naming: Naming) -> Vec<Range<usize>> {
    let characters: Vec<char> = text.chars().collect();
    let mut blocks = Vec::new();
    let mut at = 0;
    while at < characters.len() {
        if !(naming.goes_on)(characters[at]) {
            at += 1;
            continue;
        }
        let start = at;
        while at < characters.len() && (naming.goes_on)(characters[at]) {
            at += 1;
        }
        let run: String = characters[start..at].iter().collect();
        if (naming.starts)(characters[start]) && names.contains(&run.as_str()) {
            blocks.push(start..at);
        }
    }
    blocks
}

/// Where a step to the right (`forward`) or the left from `cursor` lands
/// when a block stands on that side: at its far edge. Nothing otherwise — the
/// field steps one character.
pub(super) fn stepped(cursor: usize, forward: bool, blocks: &[Range<usize>]) -> Option<usize> {
    blocks.iter().find_map(|block| match forward {
        true => (block.start == cursor).then_some(block.end),
        false => (block.end == cursor).then_some(block.start),
    })
}

/// The block Suppr (`forward`) or Retour arrière takes whole from `cursor`:
/// the one right after it, or right before it.
pub(super) fn erased(
    cursor: usize,
    forward: bool,
    blocks: &[Range<usize>],
) -> Option<Range<usize>> {
    blocks
        .iter()
        .find(|block| match forward {
            true => block.start == cursor,
            false => block.end == cursor,
        })
        .cloned()
}

/// A place inside a block moved to the block's nearer edge; any other left
/// where it is.
pub(super) fn snapped(at: usize, blocks: &[Range<usize>]) -> usize {
    blocks
        .iter()
        .find(|block| block.start < at && at < block.end)
        .map_or(at, |block| match at - block.start <= block.end - at {
            true => block.start,
            false => block.end,
        })
}

/// The text laid out as the field shows it: the blocks on `tint`, the rest
/// as plain text.
pub(super) fn laid_out(
    text: &str,
    blocks: &[Range<usize>],
    font: egui::FontId,
    color: egui::Color32,
    tint: egui::Color32,
) -> egui::text::LayoutJob {
    let plain = egui::TextFormat::simple(font, color);
    let tinted = egui::TextFormat {
        background: tint,
        ..plain.clone()
    };
    let characters: Vec<char> = text.chars().collect();
    let mut job = egui::text::LayoutJob::default();
    let mut at = 0;
    for block in blocks {
        let before: String = characters[at..block.start].iter().collect();
        let name: String = characters[block.clone()].iter().collect();
        job.append(&before, 0.0, plain.clone());
        job.append(&name, 0.0, tinted.clone());
        at = block.end;
    }
    let after: String = characters[at..].iter().collect();
    job.append(&after, 0.0, plain);
    job
}

#[cfg(test)]
mod tests;
