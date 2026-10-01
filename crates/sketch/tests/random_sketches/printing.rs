//! A drawing written out as the Rust that lays it again.
//!
//! Numbers are written the way `{:?}` writes an `f64`: the shortest text that
//! reads back as the very same bits, so a drawing pasted into a test is the
//! drawing that failed and not a neighbour of it. A number that is one of the
//! constants Clippy refuses to see written out is written as that constant,
//! and one Clippy would take for a constant cut short — it reads the value,
//! however the literal is written — is written by its bits.

use std::fmt::{self, Display, Formatter};

use super::{Axis, Gesture};

/// A list of gestures, written as the slice that lays them.
pub struct Drawing<'a>(pub &'a [Gesture]);

impl Display for Drawing<'_> {
    fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
        writeln!(out, "&[")?;
        for gesture in self.0 {
            writeln!(out, "    {gesture},")?;
        }
        write!(out, "]")
    }
}

struct Number(f64);

impl Display for Number {
    fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
        use std::f64::consts;
        let named = [
            (consts::PI, "PI"),
            (consts::TAU, "TAU"),
            (consts::FRAC_PI_2, "FRAC_PI_2"),
            (consts::FRAC_PI_3, "FRAC_PI_3"),
            (consts::FRAC_PI_4, "FRAC_PI_4"),
            (consts::FRAC_PI_6, "FRAC_PI_6"),
            (consts::SQRT_2, "SQRT_2"),
            (consts::FRAC_1_SQRT_2, "FRAC_1_SQRT_2"),
            (consts::E, "E"),
            (consts::LN_2, "LN_2"),
        ];
        for (sign, written) in [(1.0, ""), (-1.0, "-")] {
            if let Some((_, name)) = named.iter().find(|(value, _)| *value * sign == self.0) {
                return write!(out, "{written}std::f64::consts::{name}");
            }
        }
        match taken_for_a_constant(&format!("{:?}", self.0.abs())) {
            true => write!(out, "f64::from_bits({:#018x})", self.0.to_bits()),
            false => write!(out, "{:?}", self.0),
        }
    }
}

/// The constants Clippy's `approx_constant` knows, and how many characters a
/// literal needs before it is taken for one of them.
const CLIPPY_KNOWS: [(f64, usize); 19] = {
    use std::f64::consts::*;
    [
        (E, 4),
        (FRAC_1_PI, 4),
        (FRAC_1_SQRT_2, 5),
        (FRAC_2_PI, 5),
        (FRAC_2_SQRT_PI, 5),
        (FRAC_PI_2, 5),
        (FRAC_PI_3, 5),
        (FRAC_PI_4, 5),
        (FRAC_PI_6, 5),
        (FRAC_PI_8, 5),
        (LN_2, 5),
        (LN_10, 5),
        (LOG2_10, 5),
        (LOG2_E, 5),
        (LOG10_2, 5),
        (LOG10_E, 5),
        (PI, 3),
        (SQRT_2, 5),
        (TAU, 3),
    ]
};

/// Whether Clippy would refuse a literal as a constant cut short or rounded.
fn taken_for_a_constant(written: &str) -> bool {
    CLIPPY_KNOWS.iter().any(|(constant, digits)| {
        written.len() > *digits
            && (constant.to_string().starts_with(written)
                || format!("{constant:.*}", written.len() - 2) == written)
    })
}

struct Pair([f64; 2]);

impl Display for Pair {
    fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
        write!(out, "[{}, {}]", Number(self.0[0]), Number(self.0[1]))
    }
}

struct Places<'a>(&'a [[f64; 2]]);

impl Display for Places<'_> {
    fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
        write!(out, "vec![")?;
        for (index, at) in self.0.iter().enumerate() {
            let separator = if index == 0 { "" } else { ", " };
            write!(out, "{separator}{}", Pair(*at))?;
        }
        write!(out, "]")
    }
}

impl Display for Axis {
    fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Axis::U => write!(out, "Axis::U"),
            Axis::V => write!(out, "Axis::V"),
            Axis::Trait(at) => write!(out, "Axis::Trait({})", Pair(*at)),
        }
    }
}

impl Display for Gesture {
    fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Gesture::Chain {
                through,
                closed,
                construction,
            } => write!(
                out,
                "Gesture::Chain {{ through: {}, closed: {closed}, construction: {construction} }}",
                Places(through)
            ),
            Gesture::Rectangle {
                corner,
                opposite,
                construction,
            } => write!(
                out,
                "Gesture::Rectangle {{ corner: {}, opposite: {}, construction: {construction} }}",
                Pair(*corner),
                Pair(*opposite)
            ),
            Gesture::Circle {
                centre,
                radius,
                construction,
            } => write!(
                out,
                "Gesture::Circle {{ centre: {}, radius: {}, construction: {construction} }}",
                Pair(*centre),
                Number(*radius)
            ),
            Gesture::Arc {
                centre,
                start,
                degrees,
                construction,
            } => write!(
                out,
                "Gesture::Arc {{ centre: {}, start: {}, degrees: {}, construction: {construction} }}",
                Pair(*centre),
                Pair(*start),
                Number(*degrees)
            ),
            Gesture::Ellipse {
                centre,
                reach,
                across,
                construction,
            } => write!(
                out,
                "Gesture::Ellipse {{ centre: {}, reach: {}, across: {}, construction: {construction} }}",
                Pair(*centre),
                Pair(*reach),
                Number(*across)
            ),
            Gesture::HalfEllipse {
                from,
                to,
                rise,
                construction,
            } => write!(
                out,
                "Gesture::HalfEllipse {{ from: {}, to: {}, rise: {}, construction: {construction} }}",
                Pair(*from),
                Pair(*to),
                Number(*rise)
            ),
            Gesture::Point { at } => write!(out, "Gesture::Point {{ at: {} }}", Pair(*at)),
            Gesture::Fillet { at, radius } => write!(
                out,
                "Gesture::Fillet {{ at: {}, radius: {} }}",
                Pair(*at),
                Number(*radius)
            ),
            Gesture::Chamfer { at, length } => write!(
                out,
                "Gesture::Chamfer {{ at: {}, length: {} }}",
                Pair(*at),
                Number(*length)
            ),
            Gesture::Mirror { of, axis } => write!(
                out,
                "Gesture::Mirror {{ of: {}, axis: {axis} }}",
                Places(of)
            ),
            Gesture::PatternAround {
                of,
                centre,
                degrees,
                count,
            } => write!(
                out,
                "Gesture::PatternAround {{ of: {}, centre: {}, degrees: {}, count: {count} }}",
                Places(of),
                Pair(*centre),
                Number(*degrees)
            ),
            Gesture::PatternAlong {
                of,
                axis,
                along,
                across,
            } => write!(
                out,
                "Gesture::PatternAlong {{ of: {}, axis: {axis}, along: ({}, {}), across: ({}, {}) }}",
                Places(of),
                Number(along.0),
                along.1,
                Number(across.0),
                across.1
            ),
            Gesture::Divide { at } => write!(out, "Gesture::Divide {{ at: {} }}", Pair(*at)),
            Gesture::Trim { at } => write!(out, "Gesture::Trim {{ at: {} }}", Pair(*at)),
            Gesture::Erase { at } => write!(out, "Gesture::Erase {{ at: {} }}", Pair(*at)),
        }
    }
}
