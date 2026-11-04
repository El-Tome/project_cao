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
/// design describes. Also bumped when edges and vertices come to be named
/// another way, since a part opened on its cache trusts the names the cache
/// holds, and when the matter is written another way: 4
/// is the body that says which kernel computed it (#526). 5: a revolution of
/// straight runs is exact, and a declined step keeps its reason (#533). 6: a
/// revolution of a slanted run is exact (#536). 7: a plane meets a cylinder
/// or a cone at a slant along a conic (#552). 8: a plane a hair off square
/// to, or along, a wall's axis is laid exactly so, and a step built at a
/// lean the kernel reads unstably is noted (#552). 9: an ellipse raised is
/// exact, and the part stays exact after it (#571). 10: the solver stops at
/// a thousandth of a micron, and a replay lands the drawing there (#517).
/// 11: an area whose hole touches its outline is raised, pinched at the
/// touch, where the kernel declined it (#609). 12: an arc turned is a
/// sphere or a torus of the exact kernel, and the part stays exact after it
/// (#576). 13: a turn the exact kernel cannot make is a broken step, where
/// the flats made it and every step after it (#632). 14: a groove tangent to
/// its walls is laid about an axis leaning a hair, where the reading declined
/// it, and its end stays at the tangent (#633). 15: a fillet or a chamfer is
/// drawn from its sharp corner rather than laid as geometry, and a replay
/// draws it there (#617). 16: a coaxial surface a hair from touching a
/// sphere or a torus, or from a rim on it, is moved onto it, and a rim
/// another surface crosses along parts a ring (#626). 17: an arc of an
/// ellipse a hair from round is an arc of the circle of its mean, its
/// corners laid on it (#574). 18: an exact body carries the names of its
/// edges and its vertices, and the number each piece of a parted face
/// replaced (#582).
const REBUILT_BY: u32 = 18;

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
