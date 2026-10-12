//! A case written out as the Rust that builds it again.
//!
//! Numbers are written the way `{:?}` writes an `f64`: the shortest text that
//! reads back as the very same bits, so a case pasted into a test is the case
//! that failed and not a neighbour of it.

use std::fmt::{self, Display, Formatter};

use glam::{DVec2, DVec3};

use super::{Along, Axis, Case, Leaf, Mode, Outline, Plane, Section, Step};

struct Pair(DVec2);

impl Display for Pair {
    fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
        write!(out, "[{:?}, {:?}]", self.0.x, self.0.y)
    }
}

struct Triple(DVec3);

impl Display for Triple {
    fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
        write!(out, "[{:?}, {:?}, {:?}]", self.0.x, self.0.y, self.0.z)
    }
}

impl Display for Case {
    fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
        writeln!(out, "Case::new(")?;
        writeln!(out, "    {},", self.start)?;
        if self.steps.is_empty() {
            writeln!(out, "    vec![],")?;
        } else {
            writeln!(out, "    vec![")?;
            for step in &self.steps {
                writeln!(out, "        {step},")?;
            }
            writeln!(out, "    ],")?;
        }
        write!(out, ")")
    }
}

impl Display for Step {
    fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
        let verb = match self.mode {
            Mode::Add => "add",
            Mode::Cut => "cut",
        };
        write!(out, "Step::{verb}({})", self.tool)
    }
}

impl Display for Leaf {
    fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Leaf::Prism {
                plane,
                outline,
                height,
            } => write!(out, "Leaf::prism({plane}, {outline}, {height:?})"),
            Leaf::Revolution {
                plane,
                low,
                high,
                degrees,
            } => write!(
                out,
                "Leaf::revolution({plane}, {}, {}, {degrees:?})",
                Pair(*low),
                Pair(*high)
            ),
            Leaf::Turned {
                plane,
                axis,
                section,
                degrees,
            } => write!(out, "Leaf::turned({plane}, {axis}, {section}, {degrees:?})"),
        }
    }
}

impl Display for Plane {
    fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Plane::Xy(offset) => write!(out, "Plane::xy({offset:?})"),
            Plane::Xz(offset) => write!(out, "Plane::xz({offset:?})"),
            Plane::Yz(offset) => write!(out, "Plane::yz({offset:?})"),
            Plane::Tilted { origin, turn } => {
                write!(out, "Plane::tilted({}, {})", Triple(*origin), Triple(*turn))
            }
        }
    }
}

impl Display for Outline {
    fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Outline::Rectangle { low, high } => {
                write!(out, "Outline::rectangle({}, {})", Pair(*low), Pair(*high))
            }
            Outline::Circle {
                center,
                radius,
                from,
            } => {
                if *from == 0.0 {
                    write!(out, "Outline::circle({}, {radius:?})", Pair(*center))
                } else {
                    write!(
                        out,
                        "Outline::circle_from({}, {radius:?}, {from:?})",
                        Pair(*center)
                    )
                }
            }
            Outline::Star { center, corners } => {
                write!(out, "Outline::star({}, &[", Pair(*center))?;
                for (index, corner) in corners.iter().enumerate() {
                    let separator = if index == 0 { "" } else { ", " };
                    write!(out, "{separator}{}", Pair(*corner))?;
                }
                write!(out, "])")
            }
            Outline::Ring {
                center,
                outer,
                inner,
            } => write!(
                out,
                "Outline::ring({}, {outer:?}, {inner:?})",
                Pair(*center)
            ),
            Outline::Rounded { low, high, radius } => write!(
                out,
                "Outline::rounded({}, {}, {radius:?})",
                Pair(*low),
                Pair(*high)
            ),
            Outline::Slot { from, to, radius } => write!(
                out,
                "Outline::slot({}, {}, {radius:?})",
                Pair(*from),
                Pair(*to)
            ),
        }
    }
}

impl Display for Axis {
    fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
        let along = match self.along {
            Along::First => "first",
            Along::Second => "second",
        };
        write!(out, "Axis::{along}({:?})", self.across)?;
        if self.backwards {
            write!(out, ".backwards()")?;
        }
        if self.lean != 0.0 {
            write!(out, ".leaning({:?})", self.lean)?;
        }
        Ok(())
    }
}

impl Display for Section {
    fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
        write!(out, "Section::bands({:?}, &[", self.from)?;
        for (index, [length, low, high]) in self.bands.iter().enumerate() {
            let separator = if index == 0 { "" } else { ", " };
            write!(out, "{separator}[{length:?}, {low:?}, {high:?}]")?;
        }
        write!(out, "])")?;
        if !self.sloping_to.is_empty() {
            write!(out, ".sloping_to(&[")?;
            for (index, [low, high]) in self.sloping_to.iter().enumerate() {
                let separator = if index == 0 { "" } else { ", " };
                write!(out, "{separator}[{low:?}, {high:?}]")?;
            }
            write!(out, "])")?;
        }
        if !self.holes.is_empty() {
            write!(out, ".with_holes(&[")?;
            for (index, [low, high]) in self.holes.iter().enumerate() {
                let separator = if index == 0 { "" } else { ", " };
                write!(out, "{separator}({}, {})", Pair(*low), Pair(*high))?;
            }
            write!(out, "])")?;
        }
        Ok(())
    }
}
