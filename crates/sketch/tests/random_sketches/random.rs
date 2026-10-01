//! A source of drawings random enough to find what nobody listed, and
//! repeatable enough that a drawing found once is found again from its seed.

/// SplitMix64, written out rather than taken from a crate: `cao_sketch` takes
/// nothing but `glam` and `serde`, and a generator whose every number is a
/// function of the seed alone is what makes a failure worth reporting.
#[derive(Clone, Debug)]
pub struct Random(u64);

impl Random {
    pub fn seeded(seed: u64) -> Self {
        Self(seed)
    }

    pub fn number(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut mixed = self.0;
        mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        mixed ^ (mixed >> 31)
    }

    /// A whole number from zero up to, not including, `count`.
    pub fn below(&mut self, count: usize) -> usize {
        (self.number() % count.max(1) as u64) as usize
    }

    /// A number from zero up to, not including, one.
    pub fn unit(&mut self) -> f64 {
        (self.number() >> 11) as f64 / (1u64 << 53) as f64
    }

    pub fn chance(&mut self, odds: f64) -> bool {
        self.unit() < odds
    }

    /// A multiple of `step` between `low` and `high`, both included: where a
    /// drawing on round numbers puts its corners, and where two shapes end up
    /// with their sides along each other.
    pub fn on_lattice(&mut self, low: f64, high: f64, step: f64) -> f64 {
        let first = (low / step).ceil() as i64;
        let last = (high / step).floor() as i64;
        let count = (last - first + 1).max(1) as usize;
        (first + self.below(count) as i64) as f64 * step
    }

    pub fn pick<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.below(items.len())]
    }
}
