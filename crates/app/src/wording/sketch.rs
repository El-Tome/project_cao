//! What a sketch is called: « Esquisse » and the number the part gave it,
//! which no sketch put in before it, nor dropped before it, changes.

use cao_part::PartDocument;

use crate::lang::Catalogue;

/// The number a sketch goes by. An index the part numbers no sketch at, which
/// nothing should still hold, goes by its rank from one rather than by none.
pub fn number(document: &PartDocument, sketch: usize) -> u32 {
    document
        .sketch_number(sketch)
        .unwrap_or_else(|| u32::try_from(sketch + 1).unwrap_or(u32::MAX))
}

/// A sketch's name, the one its row, its line in the history and its pill go
/// by.
pub fn name(lang: &Catalogue, document: &PartDocument, sketch: usize) -> String {
    numbered(lang, number(document, sketch))
}

/// The name a sketch bearing that number goes by.
pub fn numbered(lang: &Catalogue, number: u32) -> String {
    lang.t_with("part_tree.sketch", &[("rank", &number.to_string())])
}
