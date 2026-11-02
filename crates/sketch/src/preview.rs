use crate::corner_cut::CornerCutId;
use crate::duplicating::Duplicated;
use crate::element::Element;
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
    /// The fillets and chamfers the change laid, drawn with it.
    pub cuts: Vec<CornerCutId>,
}

/// What a change left behind, read as pieces of the drawing.
pub trait Laid {
    fn laid(&self) -> Vec<Element>;

    /// The corners it cut. A change that cuts none says nothing.
    fn cuts(&self) -> Vec<CornerCutId> {
        Vec::new()
    }
}

impl Laid for Duplicated {
    fn cuts(&self) -> Vec<CornerCutId> {
        self.cuts.clone()
    }

    fn laid(&self) -> Vec<Element> {
        let points = self.points.iter().copied().map(Element::Point);
        let segments = self.segments.iter().copied().map(Element::Segment);
        let circles = self.circles.iter().copied().map(Element::Circle);
        let arcs = self.arcs.iter().copied().map(Element::Arc);
        let ellipses = self.ellipses.iter().copied().map(Element::Ellipse);
        points
            .chain(segments)
            .chain(circles)
            .chain(arcs)
            .chain(ellipses)
            .collect()
    }
}

/// Every corner a chamfer or a fillet was shown, cut with the same values.
/// They are no piece of the drawing: what they draw is read off their corner.
impl Laid for Vec<CornerCutId> {
    fn laid(&self) -> Vec<Element> {
        Vec::new()
    }

    fn cuts(&self) -> Vec<CornerCutId> {
        self.clone()
    }
}

/// One gesture that lays several things at once.
impl Laid for Vec<Element> {
    fn laid(&self) -> Vec<Element> {
        self.clone()
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
            cuts: made.cuts(),
            sketch: trial,
        })
    }
}

#[cfg(test)]
mod tests;
