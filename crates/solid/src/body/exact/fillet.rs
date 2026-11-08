//! The cut a fillet makes: a prism raised along the edge over the corner
//! between the edge's two faces and the round tangent to both, cut from the
//! body — or, where the faces meet inwards, joined to it, filling the corner
//! with a hollow round. Its wall on the round is the one face the step names.

mod full_round;

use glam::DVec3;

use super::Exact;
use super::wedge::{Prism, Wedge, too_short};
use crate::brep::{Declined, EdgeName, Scale, Surface};

impl Exact {
    /// The body with the edge each name finds rounded off by a cylinder of
    /// `radius`, or of the largest radius every edge gives with the others
    /// laid, tangent to its two faces — filled in by it where they meet inwards
    /// — and the one radius taken: each face stops where a round takes over,
    /// set back by the radius over the tangent of half the angle its two faces
    /// meet at, and two set-backs on a face two edges share stop short of
    /// crossing. Where they meet, the face is gone and the two rounds, about
    /// one axis, are one full round — a round bottom where the faces meet
    /// inwards — as [`Wedge::rounded_with`] lays it. The rounds are numbered,
    /// in the order the names come, where the steps' numbers ended; a full
    /// round answers to both its edges' numbers. Declined `Distance`: a radius
    /// that is no length, one whose round would stand off the faces it replaces
    /// by less than the kernel can set a cut back by — a larger radius stands
    /// further off, and on an edge bent nearly flat is what rounds it — and a
    /// round the kernel read onto those faces even so, or did not lay as one
    /// face. `Lost`: an edge its name no longer finds. `Unsupported`: an edge
    /// that is not straight between two flat faces, one running on tangentially
    /// into the next edge of a face, two names finding one edge, two edges
    /// sharing a face that are not parallel, an edge whose set-backs meet those
    /// of two others, and a cut that left anything but its round — a face of
    /// the prism, the round in two pieces or with a corner on neither of the
    /// edge's faces, a face the body held, but the edge's two, taken away whole
    /// — as where a round leaving the edge's end outwards bulges past the
    /// edge's face, and the prism's wall square to that face cuts the bulge
    /// off.
    pub fn filleted(&self, names: &[EdgeName], radius: f64) -> Result<(Exact, f64), Declined> {
        let (cut, taken, pairs) = self.cut_off(
            names,
            radius,
            |wedge| wedge.reach * wedge.half_tangent(),
            |wedge| wedge.half_tangent().recip(),
            Wedge::rounded,
            Some(Wedge::rounded_with),
        )?;
        let made = |rank: usize| self.next + rank as u32;
        let alone = names.iter().enumerate().all(|(rank, name)| {
            let pair = pairs
                .iter()
                .find(|&&(earlier, later)| rank == earlier || rank == later);
            match pair {
                Some(&(earlier, later)) => cut.rounded_as(name, &[made(earlier), made(later)]),
                None => cut.rounded_alone(name, made(rank)),
            }
        });
        if alone {
            Ok((cut, taken))
        } else {
            Err(Declined::Distance)
        }
    }

    /// Whether the round stands as a face of its own: some face answers to
    /// `made`, each that does answers to nothing else and lies on no plane,
    /// and the edge a name found is gone. A round the kernel read onto the
    /// faces it replaces fails it, as one standing nearer them than the
    /// floor [`Wedge::rounded`] declines would.
    fn rounded_alone(&self, name: &EdgeName, made: u32) -> bool {
        self.rounded_as(name, &[made])
    }

    /// Whether the round stands as a face of its own answering to `numbers`,
    /// as [`Exact::rounded_alone`] reads it of one: some face answers to the
    /// first, each that does answers to them all and nothing else, and lies
    /// on no plane, and the edge a name found is gone.
    fn rounded_as(&self, name: &EdgeName, numbers: &[u32]) -> bool {
        let mut faces = self.named(numbers[0] as usize).peekable();
        let curved = |face| {
            let face = self.brep.face(face);
            !matches!(self.brep.surface(face.surface), Surface::Plane(_))
        };
        faces.peek().is_some()
            && faces.all(|face| self.brep.numbers(face) == numbers && curved(face))
            && self.lost(name).is_some()
    }
}

impl Wedge {
    /// The sine and the cosine of half the angle between the edge's two
    /// faces where the round goes: inside the matter at an edge whose faces
    /// meet outwards, outside it at one whose faces meet inwards.
    fn half_angle(&self) -> (f64, f64) {
        let [first, second] = self.into;
        let across = (first - second).normalize();
        let inward = (first + second).normalize();
        (first.dot(across), first.dot(inward))
    }

    /// The tangent of half the angle between the edge's two faces where the
    /// round goes: what a radius is over the set-back of its round.
    fn half_tangent(&self) -> f64 {
        let (sine, cosine) = self.half_angle();
        sine / cosine
    }

    /// How far a round of radius one stands off the faces it replaces, at
    /// the middle of its turn, where it stands furthest from both: one less
    /// the sine of half their angle, read as the cosine's square over one
    /// and the sine, so that it keeps its digits on an edge nearly flat.
    fn standing(&self) -> f64 {
        let (sine, cosine) = self.half_angle();
        cosine * cosine / (1.0 + sine)
    }

    /// The prism, raised along the edge over the section
    /// [`Wedge::squared_off`] draws, its first run the round from where it
    /// touches the second face to where it touches the first, about a centre
    /// a radius off the first face. Declined `Distance` where the round would
    /// stand off the two faces by less than the kernel can set a cut back by:
    /// within that, the kernel reads it onto them. How far it stands grows
    /// with the radius, and an edge bent nearly flat needs a large one to
    /// stand clear.
    fn rounded(
        &self,
        radius: f64,
        scale: Scale,
        bounds: (DVec3, DVec3),
    ) -> Result<Prism, Declined> {
        if too_short(radius * self.standing(), scale) {
            return Err(Declined::Distance);
        }
        let set_back = radius / self.half_tangent();
        let centre = self.into[0] * set_back - self.outward[0] * radius * self.off();
        Ok(self.squared_off(set_back, Some(centre), scale, bounds))
    }
}

#[cfg(test)]
mod tests;
