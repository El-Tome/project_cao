//! Draws a part to order and writes it, so that a rich part can be opened in
//! the app or measured.
//!
//! ```sh
//! cargo run --release -p cao_part --features test-support \
//!     --example draw_a_part -- /tmp/big.caopart sketches=4 sides=6 storeys=3
//! ```
//!
//! What follows the path are the counts of [`Recipe`], each `name=number`.

use std::path::{Path, PathBuf};
use std::time::Instant;

use cao_part::{FileError, Files, PartDocument, PartState, Recipe};
use chrono::Utc;

struct Disk;

impl Files for Disk {
    fn read(&self, path: &Path) -> Result<Vec<u8>, FileError> {
        std::fs::read(path).map_err(|_| FileError::Absent(path.to_path_buf()))
    }

    fn write(&self, path: &Path, bytes: &[u8]) -> Result<(), FileError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|_| FileError::Refused(path.to_path_buf()))?;
        }
        std::fs::write(path, bytes).map_err(|_| FileError::Refused(path.to_path_buf()))
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }
}

fn main() {
    let mut arguments = std::env::args().skip(1);
    let Some(path) = arguments.next().map(PathBuf::from) else {
        eprintln!(
            "draw_a_part <file.caopart> [sketches=n] [sides=n] [circles=n] [arcs=n] \
             [dimensions=n] [rules=n] [storeys=n]"
        );
        std::process::exit(2);
    };

    let mut recipe = Recipe::default();
    for argument in arguments {
        let Some((name, count)) = argument.split_once('=') else {
            eprintln!("{argument} is not a name=number");
            std::process::exit(2);
        };
        let Ok(count) = count.parse::<usize>() else {
            eprintln!("{count} is not a number");
            std::process::exit(2);
        };
        match name {
            "sketches" => recipe.sketches = count,
            "sides" => recipe.sides = count,
            "circles" => recipe.circles = count,
            "arcs" => recipe.arcs = count,
            "dimensions" => recipe.dimensions = count,
            "rules" => recipe.rules = count,
            "storeys" => recipe.storeys = count,
            other => {
                eprintln!("{other} is not one of the counts a recipe holds");
                std::process::exit(2);
            }
        }
    }

    let name = path
        .file_stem()
        .map_or_else(|| "Piece".to_string(), |stem| stem.to_string_lossy().into());

    let start = Instant::now();
    let document = recipe.drawn(name, Utc::now());
    let drawing = start.elapsed();

    let start = Instant::now();
    if let Err(error) = document.put_away(&Disk, &path, Utc::now()) {
        eprintln!("{}: {error}", path.display());
        std::process::exit(1);
    }
    let writing = start.elapsed();

    let start = Instant::now();
    let reopened = match PartDocument::load(&Disk, &path) {
        Ok(reopened) => reopened,
        Err(error) => {
            eprintln!("{}: {error}", path.display());
            std::process::exit(1);
        }
    };
    let opening = start.elapsed();

    let start = Instant::now();
    let replayed = PartState::rebuild(&reopened.history);
    let replaying = start.elapsed();

    let size = Disk.read(&path).map_or(0, |bytes| bytes.len());
    println!("{}", path.display());
    println!(
        "  {} steps, {} sketches, {} polygons, {} KB",
        document.history.applied(),
        document.sketches().len(),
        replayed.body.polygons.len(),
        size / 1024,
    );
    println!("  drawn in {drawing:?}, written in {writing:?}");
    println!("  opened in {opening:?}, where replaying its design takes {replaying:?}");
}
