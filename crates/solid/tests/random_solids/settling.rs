//! A campaign's cases measured for whether the sampling's rounds settle
//! (#764), one line per seed drawn, appended to the file `CAO_SETTLING_OUT`
//! names; nothing is measured without it. A case is told by its debug text,
//! so that the cases a shrink tries, which no seed drew, are left out.
//!
//! Compiled only when asked for — `--features campaigns`.

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::fs::{File, OpenOptions};
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::sync::{Mutex, OnceLock};

use cao_solid::brep::settling::{self, Record, Settling};
use cao_solid::soundness::Flaw;

use super::Case;

fn print(case: &Case) -> u64 {
    let mut hasher = DefaultHasher::new();
    format!("{case:?}").hash(&mut hasher);
    hasher.finish()
}

fn seeds() -> &'static Mutex<HashMap<u64, u64>> {
    static SEEDS: OnceLock<Mutex<HashMap<u64, u64>>> = OnceLock::new();
    SEEDS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn out() -> Option<&'static Mutex<File>> {
    static OUT: OnceLock<Option<Mutex<File>>> = OnceLock::new();
    OUT.get_or_init(|| {
        let path = std::env::var("CAO_SETTLING_OUT").ok()?;
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .expect("the settling file to open");
        Some(Mutex::new(file))
    })
    .as_ref()
}

/// Names `case` as the one `seed` drew.
pub fn drawn(seed: u64, case: &Case) {
    if out().is_some() {
        seeds()
            .lock()
            .expect("no thread to have panicked holding the seeds")
            .insert(print(case), seed);
    }
}

fn severity(settling: Settling) -> usize {
    match settling {
        Settling::NoMeet => 0,
        Settling::Settled => 1,
        Settling::Later(_) => 2,
        Settling::Cycles(_) => 3,
        Settling::Drifts => 4,
        Settling::Grows => 5,
        Settling::Broke => 6,
    }
}

fn named(settling: Settling) -> String {
    match settling {
        Settling::NoMeet => "nomeet".to_string(),
        Settling::Settled => "settled".to_string(),
        Settling::Later(rounds) => format!("later{rounds}"),
        Settling::Cycles(period) => format!("cycles{period}"),
        Settling::Drifts => "drifts".to_string(),
        Settling::Grows => "grows".to_string(),
        Settling::Broke => "broke".to_string(),
    }
}

/// One tab-separated line: seed, kernel, verdict, how many tessellations
/// and of each class, the worst class, its sizes first and last, the
/// furthest a first extra round moved anything, whether every first extra
/// round was bit for bit, whether any wall took a ray, the copies' strays,
/// whether a first extra round moved a cone's curve and one #761 sampled
/// again, the microseconds the copies and the extra rounds took, and each
/// tessellation's class in order.
fn line(seed: u64, kernel: &str, verdict: &str, records: &[Record]) -> String {
    let count = |of: fn(&Settling) -> bool| records.iter().filter(|r| of(&r.settling)).count();
    let worst = records
        .iter()
        .max_by_key(|record| severity(record.settling));
    let sizes =
        |at: Option<&(usize, usize)>| at.map_or("-".to_string(), |(s, r)| format!("{s}/{r}"));
    let moved = records
        .iter()
        .filter(|record| record.settling != Settling::NoMeet)
        .map(|record| record.moved)
        .fold(0.0, f64::max);
    let micros = |at: usize| {
        records
            .iter()
            .map(|record| record.micros[at])
            .sum::<u64>()
            .to_string()
    };
    [
        seed.to_string(),
        kernel.to_string(),
        verdict.to_string(),
        records.len().to_string(),
        count(|s| *s == Settling::NoMeet).to_string(),
        count(|s| *s == Settling::Settled).to_string(),
        count(|s| matches!(s, Settling::Later(_))).to_string(),
        count(|s| matches!(s, Settling::Cycles(_))).to_string(),
        count(|s| *s == Settling::Drifts).to_string(),
        count(|s| *s == Settling::Grows).to_string(),
        count(|s| *s == Settling::Broke).to_string(),
        worst.map_or("none".to_string(), |record| named(record.settling)),
        sizes(worst.and_then(|record| record.sizes.first())),
        sizes(worst.and_then(|record| record.sizes.last())),
        format!("{moved:e}"),
        records.iter().all(|record| record.exact).to_string(),
        records.iter().any(|record| record.rays).to_string(),
        records
            .iter()
            .map(|record| record.astray)
            .sum::<usize>()
            .to_string(),
        records.iter().any(|record| record.on_a_cone).to_string(),
        records.iter().any(|record| record.chained).to_string(),
        micros(0),
        micros(1),
        records
            .iter()
            .map(|record| named(record.settling))
            .collect::<Vec<_>>()
            .join(","),
    ]
    .join("\t")
}

struct Written<'a> {
    case: &'a Case,
    kernel: &'a str,
    verdict: Option<String>,
}

impl Drop for Written<'_> {
    fn drop(&mut self) {
        let records = settling::taken();
        let Some(out) = out() else {
            return;
        };
        let Some(seed) = seeds()
            .lock()
            .map_or(None, |seeds| seeds.get(&print(self.case)).copied())
        else {
            return;
        };
        let verdict = self.verdict.as_deref().unwrap_or("Panicked");
        let text = line(seed, self.kernel, verdict, &records) + "\n";
        if let Ok(mut file) = out.lock() {
            let _ = file.write_all(text.as_bytes());
        }
    }
}

/// `check` run on `case`, every tessellation of it measured and the line of
/// its seed written, whatever the check ends on.
pub fn measured(
    case: &Case,
    kernel: &str,
    check: impl FnOnce() -> Result<(), Flaw>,
) -> Result<(), Flaw> {
    if out().is_none() {
        return check();
    }
    let mut written = Written {
        case,
        kernel,
        verdict: None,
    };
    settling::arm();
    let verdict = check();
    written.verdict = Some(match &verdict {
        Ok(()) => "holds".to_string(),
        Err(flaw) => format!("{:?}", flaw.rule()),
    });
    drop(written);
    verdict
}
