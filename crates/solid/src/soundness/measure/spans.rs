//! The stretches of one line inside a solid, and how two of them make a
//! promise.

/// The stretches of one line that lie inside a solid, as distances along it,
/// in order and apart.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Spans {
    pub(super) stretches: Vec<(f64, f64)>,
    /// What a surface that bounds no solid counts along the line beyond its
    /// stretches: a shell left inside another wraps a place twice, one turned
    /// inside out on its own wraps it the wrong way. Neither is a stretch of
    /// more or less matter, yet both are a wrong answer, and one no other rule
    /// sees: every edge still has its two faces, no face crosses another.
    ///
    /// Always nothing for spans made from others: `union` and `without` build
    /// a promise, and a promise is only ever a place in or out.
    pub(super) surplus: f64,
}

impl Spans {
    /// Stretches in any order, overlapping or not, made into spans: sorted,
    /// joined where they meet, and the empty ones dropped. What a promise
    /// worked out by arithmetic starts from.
    pub fn gathered(mut stretches: Vec<(f64, f64)>) -> Spans {
        stretches.retain(|(from, to)| from < to);
        stretches.sort_by(|left, right| left.0.total_cmp(&right.0));
        let mut joined: Vec<(f64, f64)> = Vec::with_capacity(stretches.len());
        for (from, to) in stretches {
            match joined.last_mut() {
                Some(last) if from <= last.1 => last.1 = last.1.max(to),
                _ => joined.push((from, to)),
            }
        }
        Spans {
            stretches: joined,
            surplus: 0.0,
        }
    }

    pub fn stretches(&self) -> &[(f64, f64)] {
        &self.stretches
    }

    /// How much of the line lies inside.
    pub fn length(&self) -> f64 {
        self.stretches.iter().map(|(from, to)| to - from).sum()
    }

    /// The matter along the line by the surface's own account: every place
    /// counted as many times as the surface wraps it, and against it where it
    /// is wrapped the wrong way — what `enclosed` counts, one line at a time.
    pub(super) fn held(&self) -> f64 {
        self.length() + self.surplus
    }

    /// What the surface counts along the line beyond its stretches: nothing
    /// for a solid, more for a shell left inside another.
    pub fn surplus(&self) -> f64 {
        self.surplus
    }

    /// What is inside either: the promise for matter added.
    pub fn union(&self, other: &Spans) -> Spans {
        let both = self.stretches.iter().chain(&other.stretches);
        Spans::gathered(both.copied().collect())
    }

    /// What is inside this and not the other: the promise for matter taken
    /// away.
    pub fn without(&self, other: &Spans) -> Spans {
        let (mut left, mut first, holes) = (Vec::new(), 0, &other.stretches);
        for &(from, to) in &self.stretches {
            while holes.get(first).is_some_and(|&(_, end)| end <= from) {
                first += 1;
            }
            let mut start = from;
            for &(hole_from, hole_to) in holes[first..]
                .iter()
                .take_while(|(hole_from, _)| *hole_from < to)
            {
                if hole_from > start {
                    left.push((start, hole_from));
                }
                start = start.max(hole_to);
            }
            if start < to {
                left.push((start, to));
            }
        }
        Spans {
            stretches: left,
            surplus: 0.0,
        }
    }
}

/// The stretches of one line inside a surface, from where it crosses the
/// surface and which way. Crossings at the same distance are taken together,
/// so that a line through an edge leaves no stretch of no length.
pub(super) fn sweep(mut crossings: Vec<(f64, i32)>) -> Spans {
    crossings.sort_by(|left, right| left.0.total_cmp(&right.0));
    let mut stretches = Vec::new();
    let (mut depth, mut from, mut last, mut surplus) = (0, 0.0, 0.0, 0.0);
    for group in crossings.chunk_by(|left, right| left.0 == right.0) {
        let at = group[0].0;
        surplus += f64::from(depth - i32::from(depth > 0)) * (at - last);
        let after = depth + group.iter().map(|(_, step)| step).sum::<i32>();
        if depth <= 0 && after > 0 {
            from = at;
        } else if depth > 0 && after <= 0 {
            stretches.push((from, at));
        }
        (depth, last) = (after, at);
    }
    Spans { stretches, surplus }
}
