//! The geometry of the whole part, written down beside the design that
//! produces it.
//!
//! The design stays the source of truth: this is only what replaying it came
//! to last time, kept so that opening a part shows it at once instead of
//! extruding and cutting everything again. A cache that is missing, damaged or
//! no longer answers to the design is dropped and the design replayed — it can
//! never be the reason a part refuses to open.
//!
//! Written when the part is put away, and not at the end of every gesture: a
//! part closed is a part nothing more is coming to, where a gesture is only
//! ever followed by another one.
//!
//! JSON rather than a binary format: what the cache saves is the rebuild, not
//! the reading. On a part of forty features the replay takes some seventy
//! milliseconds where reading its geometry back takes three, so a codec would
//! buy a couple of milliseconds and cost a dependency.

use std::io::{Read, Seek, Write};

use serde::{Deserialize, Serialize};

use crate::errors::PartFileError;
use crate::state::PartState;

/// At the root of the archive: the rebuilt geometry answers to the part as a
/// whole, and not to any one step of its design.
pub(super) const GEOMETRY_ENTRY: &str = "geometry.json";

/// Bumped when replaying the same design stops giving the same geometry — a
/// fix in the solver, in an extrusion, in a boolean. What was cached before
/// then holds the shape as it was computed, which is no longer the shape the
/// design describes.
const REBUILT_BY: u32 = 1;

#[derive(Serialize, Deserialize)]
struct Cached<S> {
    rebuilt_by: u32,
    /// A print of the design this geometry was rebuilt from.
    design: u64,
    state: S,
}

pub(super) fn write<W: Write + Seek>(
    archive: &mut zip::ZipWriter<W>,
    options: zip::write::SimpleFileOptions,
    state: &PartState,
    design: &[&str],
) -> Result<(), PartFileError> {
    archive.start_file(GEOMETRY_ENTRY, options)?;
    archive
        .write_all(&encoded(state, design)?)
        .map_err(zip::result::ZipError::from)?;
    Ok(())
}

/// The geometry as the archive carries it: the state itself, and what it was
/// rebuilt from.
pub(super) fn encoded(state: &PartState, design: &[&str]) -> Result<Vec<u8>, PartFileError> {
    Ok(serde_json::to_vec(&Cached {
        rebuilt_by: REBUILT_BY,
        design: fingerprint(design),
        state,
    })?)
}

/// The geometry the archive carries, when it is still the geometry that design
/// rebuilds to.
pub(super) fn read<R: Read + Seek>(
    archive: &mut zip::ZipArchive<R>,
    design: &[&str],
) -> Option<PartState> {
    let mut entry = archive.by_name(GEOMETRY_ENTRY).ok()?;
    // Read whole rather than parsed as it inflates: serde_json asks an
    // unbuffered reader for one byte at a time, which costs more than the
    // replay this is here to save.
    let mut content = Vec::new();
    entry.read_to_end(&mut content).ok()?;
    let cached: Cached<PartState> = serde_json::from_slice(&content).ok()?;
    (cached.rebuilt_by == REBUILT_BY && cached.design == fingerprint(design))
        .then_some(cached.state)
}

/// Enough of the design to tell it apart from another one, in eight bytes.
///
/// Taken over the text of every file the design is written as — the index and
/// each step's folder, in the order the index names them — so that a design
/// edited by any hand other than a save, the only way the two can fall out of
/// step since they are written together, leaves its cache behind rather than
/// showing a shape the part no longer describes. A print over the index alone
/// would miss every stroke, which is where the drawing is.
fn fingerprint(design: &[&str]) -> u64 {
    design
        .iter()
        .flat_map(|text| text.bytes())
        .fold(0xcbf2_9ce4_8422_2325, |print, byte| {
            (print ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
        })
}

#[cfg(test)]
mod tests;
