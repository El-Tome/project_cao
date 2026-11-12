//! The cut a fillet makes: a prism raised along the edge over the corner
//! between the edge's two faces and the round tangent to both, cut from the
//! body — or, where the faces meet inwards, joined to it, filling the corner
//! with a hollow round. Its wall on the round is the one face the step names.

mod full_round;

use glam::DVec3;

use super::Exact;
use super::wedge::{
    Around, Closing, Meeting, Prism, Together, Wedge, hollowed_across, rounded_across, together,
    too_short,
};
use crate::brep::{Declined, EdgeName, Scale, Surface};

impl Exact {
    /// The body with the edge each name finds rounded off by a cylinder of
    /// `radius` tangent to its two faces — filled in by it where they meet
    /// inwards — or of the largest radius every edge gives: alone, and for any
    /// two not meeting at a corner, with their set-backs short of reaching one
    /// point of a straight edge bounding a face of each, as [`Exact::cut_off`]
    /// reads it. Each face stops where a round takes over, set back by the
    /// radius over the tangent of half the angle its two faces meet at, and
    /// two set-backs on such a face stop short of crossing. Where they meet,
    /// the face is gone and the two rounds, about one axis, are one full round
    /// — a round bottom where the faces meet inwards — as
    /// [`Wedge::rounded_with`] lays it, at the radius they meet at; the edge
    /// at its end, bounding the face that is gone and meeting both its edges
    /// at a corner, is rounded at that same radius and mitred against it as
    /// against any round meeting it at a convex corner, the two rounds meeting
    /// on two ellipses up to the crown, where they touch; any upright meeting
    /// one of its edges at a convex corner where no third edge of the list
    /// ends — at either end, along the top or the floor, the two at one end a
    /// standing full round — is rounded at that radius too and mitred against
    /// it on one ellipse, from where the full round leaves the face the two
    /// share to where the upright's round leaves its other face, the crown at
    /// a square end; with the edge at its end, the three closed at their
    /// corner by a piece of sphere, as below — and at the tip of a tongue as
    /// wide as two radii, the end edge's round closing to nothing between the
    /// two corners, by one quarter of a ball answering to its number and both
    /// corners'; the two edges of a pointed end, rounded at that radius too,
    /// with the upright at its tip closed there by a piece of sphere about the
    /// crown's axis, which the full round touches at the crown alone. Where
    /// matter lies past an end of an edge rounded off, behind the one flat face
    /// it runs into there — the inner corner of an L, a pocket's rim, the foot
    /// of a block standing on another — its round stops flush on that face,
    /// which grows by the round's end and keeps its number, as
    /// [`Exact::laid_flush`] stops it. Two edges meeting at a convex corner are
    /// mitred there, on the ellipse where their rounds cross, from where they
    /// leave the face the edges share down to the corner's third edge, left
    /// sharp. Two rounded off where they meet beside a concave edge left sharp
    /// — the L's two top edges at its inner corner, two edges of a pocket's rim
    /// — each stop flush there and run on past the corner into a mitre, on the
    /// ellipse where their rounds cross, from where they leave the face the
    /// edges share down to the concave edge, which ends one radius short of the
    /// corner, no face laid there. Two filled in where they meet beside a
    /// convex edge left sharp — two feet of a block standing on another — each
    /// stop flush there and run on past the corner into a mitre outside the
    /// convex edge, on the ellipse where their rounds cross, no face laid
    /// there. Two filled in where they meet beside a concave edge left sharp —
    /// two edges of a pocket's floor, or one and the upright it meets — each
    /// stop flush there and meet in a valley on the ellipse where their rounds
    /// cross, the third edge ending one radius short, no face laid there. Three
    /// edges meeting at a convex corner are closed there by a piece of sphere
    /// of the radius, centred one radius from the corner's three faces, meeting
    /// each round along the round's end section, square to its edge through the
    /// centre, and numbered after the rounds, as [`rounded_across`] lays it,
    /// and three at a concave corner, a pocket's bottom, by a hollow piece of
    /// sphere centred one radius out from its three faces, facing its centre,
    /// as [`hollowed_across`] lays it; declined `Undrawn` where the kernel
    /// cannot draw it. The three at a standing block's foot corner, two feet
    /// and the upright above them, go round the upright's round: the corner
    /// filled up to the planes through its axis square to the feet, every
    /// round then laid against that — the upright's ending one radius above
    /// the lower block's top, each foot's on the plane through the axis square
    /// to its wall — and the corner closed by a piece of torus about that
    /// axis, its tube of the radius and its centre circle twice the radius
    /// from the axis, tangent to the four faces it meets and numbered after
    /// the rounds; and the three at the top of an L's inner corner, its
    /// mirror: the inner upright's fill ending one radius below the top, each
    /// top edge's round on the plane through the fill's axis square to its
    /// wall, and the corner closed by a piece of torus cut out about that
    /// axis, its centre circle one radius below the top. An upright named
    /// with its block's top edges closes its top corner with them by a piece
    /// of sphere, as above: every edge of a standing block in one step. A
    /// post's upright between a floor plate and a ceiling plate goes round at
    /// both its ends, the piece of torus under the ceiling hanging from it,
    /// the mirror of the floor's. A
    /// foot and the upright above it, the other foot left sharp, go round the
    /// upright's round the same way, the piece of torus over the whole arc and
    /// stopping on the plane square to the sharp foot where that foot's wall
    /// begins, its end section a flat face numbered after the piece; and a
    /// top edge at an L's inner corner and the upright below it, the other top
    /// edge left sharp, the mirror, the piece of torus cut out over the whole
    /// arc and its end section dug into the top, facing back along the sharp
    /// top edge.
    /// Rounds that would meet only on a curved edge are not measured, and
    /// declined where they meet. The rounds are numbered, in the
    /// order the names come, where the steps' numbers ended; a full round
    /// answers to both its edges' numbers. Where every edge meets two others
    /// so, in one closed chain of cuts about one axis — the four long edges of
    /// a square bar at half its side — the rounds close on themselves into
    /// one cylinder with no seam, answering to every edge's number, as
    /// [`Exact::closed_round`] cuts it (#790). Declined `Distance`: a radius that
    /// is no length, one whose round would stand off the faces it replaces by
    /// less than the kernel can set a cut back by — a larger radius stands
    /// further off, and on an edge bent nearly flat is what rounds it — and a
    /// round the kernel read onto those faces even so, or did not lay as one
    /// face. `Lost`: an edge its name no longer finds. `Unsupported`: a list of
    /// no edge, an edge that is not straight between two flat faces, one
    /// running on tangentially into the next edge of a face on the body as
    /// given, one edge named twice, two edges meeting at any other corner where
    /// an edge meets inwards, an edge whose set-backs meet those of two others
    /// — rounds closing on themselves, #790 — an edge of a full round meeting
    /// another edge of the list at a corner but the edge at its end, the two
    /// of a pointed or a V-notched end or an upright, a notch's two where the
    /// upright at its bottom is named too and an upright where a third edge
    /// of the list ends but the edge at its end, one of a round bottom meeting
    /// any, and a cut that left anything but its round —
    /// a face of the prism, the round in two
    /// pieces or with a corner on neither of the edge's faces, a face the body
    /// held, but the edge's two, taken away whole — as where a round leaving
    /// the edge's end outwards bulges past the edge's face, and the prism's
    /// wall square to that face cuts the bulge off, or two rounds meet where
    /// the radius was not measured for them.
    pub fn filleted(&self, names: &[EdgeName], radius: f64) -> Result<(Exact, f64), Declined> {
        let (cut, taken, pairs) = self.cut_off(
            names,
            radius,
            |wedge| wedge.reach * wedge.half_tangent(),
            |wedge| wedge.half_tangent().recip(),
            Wedge::rounded,
            Meeting {
                together: Together::Raised(Wedge::rounded_with),
                closing: Closing {
                    outwards: rounded_across,
                    inwards: hollowed_across,
                },
                around: Around::Torus,
            },
        )?;
        let made = |rank: usize| self.next + rank as u32;
        let alone = names
            .iter()
            .enumerate()
            .all(|(rank, name)| match together(rank, &pairs)[..] {
                [_] => {
                    cut.rounded_alone(name, made(rank))
                        || cut.closed_between(name, made(rank), made(names.len()))
                }
                ref ranks => {
                    cut.rounded_as(name, &ranks.iter().map(|&at| made(at)).collect::<Vec<_>>())
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

    /// Whether the round closed to nothing between two corners whose centres
    /// are one (#837): the faces answering to `made` answer to nothing but it
    /// and numbers from `closing` on, where the pieces closing the step's
    /// corners are numbered, and [`Exact::rounded_as`] holds of all those
    /// numbers.
    fn closed_between(&self, name: &EdgeName, made: u32, closing: u32) -> bool {
        let numbers: Vec<u32> = (self.named(made as usize))
            .flat_map(|face| self.brep.numbers(face).to_vec())
            .collect();
        numbers
            .iter()
            .all(|&number| number == made || number >= closing)
            && self.rounded_as(name, &numbers)
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
