use crate::chamfer::Chamfered;
use crate::duplicating::Duplicated;
use crate::element::Element;
use crate::fillet::Rounded;
use crate::sketch::Sketch;

/// A drawing as a tool would leave it, and what the tool would have put there.
///
/// The preview is the operation: the same call the click commits is run on a
/// copy of the drawing, so what is shown cannot say anything the click would
/// not do.
#[derive(Clone, Debug)]
pub struct Preview {
    pub sketch: Sketch,
    pub laid: Vec<Element>,
}

/// What a change left behind, read as pieces of the drawing.
pub trait Laid {
    fn laid(&self) -> Vec<Element>;
}

impl Laid for Duplicated {
    fn laid(&self) -> Vec<Element> {
        let points = self.points.iter().copied().map(Element::Point);
        let segments = self.segments.iter().copied().map(Element::Segment);
        let circles = self.circles.iter().copied().map(Element::Circle);
        let arcs = self.arcs.iter().copied().map(Element::Arc);
        points.chain(segments).chain(circles).chain(arcs).collect()
    }
}

impl Laid for Rounded {
    fn laid(&self) -> Vec<Element> {
        vec![Element::Arc(self.arc)]
    }
}

impl Laid for Chamfered {
    fn laid(&self) -> Vec<Element> {
        vec![Element::Segment(self.cut)]
    }
}

impl Sketch {
    /// The drawing as the change would leave it, without touching this one.
    ///
    /// Nothing when the change itself cannot happen — a corner too tight to
    /// round, an axis naming no direction — so a preview never shows what a
    /// click would refuse.
    pub fn preview<T: Laid>(
        &self,
        change: impl FnOnce(&mut Sketch) -> Option<T>,
    ) -> Option<Preview> {
        let mut trial = self.clone();
        let made = change(&mut trial)?;
        Some(Preview {
            laid: made.laid(),
            sketch: trial,
        })
    }
}

#[cfg(test)]
mod tests;
