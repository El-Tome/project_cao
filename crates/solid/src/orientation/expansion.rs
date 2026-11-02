//! Sums and products of floating-point numbers carried out with every
//! rounding kept: an expansion is a sum of parts, none of which overlaps the
//! next, kept from the smallest to the largest, so that its sign is the sign
//! of its largest part.

use std::cmp::Ordering;

#[derive(Clone, Debug, Default)]
pub(super) struct Expansion(Vec<f64>);

impl Expansion {
    /// `a - b` exactly: the rounded difference and what rounding left out.
    pub(super) fn difference(a: f64, b: f64) -> Expansion {
        let rounded = a - b;
        let virtual_a = rounded + b;
        let virtual_b = virtual_a - rounded;
        let mut expansion = Expansion::default();
        expansion.add((a - virtual_a) + (virtual_b - b));
        expansion.add(rounded);
        expansion
    }

    /// Adds `value`, dropping the parts that come out nought.
    pub(super) fn add(&mut self, value: f64) {
        let mut carried = value;
        let mut kept = 0;
        for index in 0..self.0.len() {
            let (sum, error) = two_sum(carried, self.0[index]);
            carried = sum;
            if error != 0.0 {
                self.0[kept] = error;
                kept += 1;
            }
        }
        self.0.truncate(kept);
        if carried != 0.0 {
            self.0.push(carried);
        }
    }

    /// Adds `a * b` exactly.
    pub(super) fn add_product(&mut self, a: f64, b: f64) {
        let rounded = a * b;
        self.add(a.mul_add(b, -rounded));
        self.add(rounded);
    }

    pub(super) fn times(&self, other: &Expansion) -> Expansion {
        let mut product = Expansion::default();
        for &one in &self.0 {
            for &another in &other.0 {
                product.add_product(one, another);
            }
        }
        product
    }

    pub(super) fn plus(mut self, other: &Expansion) -> Expansion {
        for &part in &other.0 {
            self.add(part);
        }
        self
    }

    pub(super) fn minus(mut self, other: &Expansion) -> Expansion {
        for &part in &other.0 {
            self.add(-part);
        }
        self
    }

    pub(super) fn sign(&self) -> Ordering {
        self.0.last().copied().unwrap_or(0.0).total_cmp(&0.0)
    }
}

fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let rounded = a + b;
    let virtual_b = rounded - a;
    let virtual_a = rounded - virtual_b;
    (rounded, (a - virtual_a) + (b - virtual_b))
}
