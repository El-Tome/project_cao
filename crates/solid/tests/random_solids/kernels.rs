//! The kernels a case can be run through and held to the arithmetic: the
//! exact kernel of #498, the body the application computes with, which puts
//! that kernel behind numbers, a drawing and its declines (#526), and the
//! flats written before it, on which the harness itself is held to the
//! arithmetic.

use std::f64::consts::TAU;

use cao_solid::Declined;
use cao_solid::brep::{Body, Listing};
use cao_solid::profile::{Contour, Frame, Profile, Run};
use cao_solid::soundness::Triangle;
use cao_solid::turning::Straight;
use glam::{DVec2, DVec3};

use super::{CIRCLE_STEPS, Case, Leaf, Mode, Outline, Turned};

/// What the check held to the arithmetic asks of a kernel.
pub trait Kernel {
    type Body: Clone;

    /// The body a leaf is raised into, or why the kernel declines.
    fn raised(&self, leaf: &Leaf) -> Result<Self::Body, Declined>;

    /// A body with a tool added to it or taken out of it, or why the kernel
    /// declines.
    fn combined(
        &self,
        body: &Self::Body,
        tool: &Self::Body,
        mode: Mode,
    ) -> Result<Self::Body, Declined>;

    /// The triangles a body is drawn with, and how far they may stand from
    /// the true surfaces it is made of.
    fn triangles(&self, body: &Self::Body) -> (Vec<Triangle>, f64);

    /// What a body lists itself as made of, and how far it reaches: nothing,
    /// for a kernel that keeps no such list.
    fn listing(&self, _body: &Self::Body) -> Option<(Listing, f64)> {
        None
    }

    /// Where the line through `origin` along `direction` passes through the
    /// exact body, each crossing with one going into the matter and minus one
    /// coming out: nothing, for a kernel that keeps no exact body, or where
    /// it cannot tell along that line.
    fn crossings(
        &self,
        _body: &Self::Body,
        _origin: DVec3,
        _direction: DVec3,
    ) -> Option<Vec<(f64, i32)>> {
        None
    }
}

/// The exact kernel of #498, `cao_solid::brep`.
pub struct Exact;

/// How far the exact kernel's triangles are asked to stand from its
/// surfaces, as a fraction of how far the body reaches.
pub const TESSELLATION: f64 = 1e-3;

impl Kernel for Exact {
    type Body = Body;

    fn raised(&self, leaf: &Leaf) -> Result<Body, Declined> {
        if let Some(turned) = leaf.as_turned() {
            return turned_exactly(&turned);
        }
        let (contour, holes, frame, travel) = exact_prism(leaf).ok_or(Declined::Profile)?;
        Body::raised(&contour, &holes, frame, travel)
    }

    fn combined(&self, body: &Body, tool: &Body, mode: Mode) -> Result<Body, Declined> {
        match mode {
            Mode::Add => body.joined(tool),
            Mode::Cut => body.cut_by(tool),
        }
    }

    fn triangles(&self, body: &Body) -> (Vec<Triangle>, f64) {
        let tolerance = TESSELLATION * body.scale().reach();
        (body.triangles(tolerance), tolerance)
    }

    fn listing(&self, body: &Body) -> Option<(Listing, f64)> {
        Some((body.listing(), body.scale().reach()))
    }

    fn crossings(&self, body: &Body, origin: DVec3, direction: DVec3) -> Option<Vec<(f64, i32)>> {
        body.crossings_along(origin, direction, body.scale().eps())
            .ok()
    }
}

/// The exact kernel as the application reaches it: behind `cao_solid::Body`,
/// which numbers the faces, draws the body as it builds it and declines what
/// it cannot draw, catching what stops on a bug.
pub struct Application;

impl Kernel for Application {
    type Body = cao_solid::Body;

    /// A turned leaf is handed over as the application hands an area, both
    /// ways, each side of a section across its axis turned apart and the two
    /// joined.
    fn raised(&self, leaf: &Leaf) -> Result<cao_solid::Body, Declined> {
        let tool = match leaf.as_turned() {
            Some(turned) => turned.tool(true)?,
            None => {
                let (contour, holes, frame, travel) = exact_prism(leaf).ok_or(Declined::Profile)?;
                let profile = Profile {
                    exact: Some((contour, holes)),
                    sampled: cao_solid::Loop::straight(&[]),
                    sampled_holes: Vec::new(),
                    triangles: &[],
                };
                cao_solid::Body::default().tool_raised(&profile, frame, travel)?
            }
        };
        cao_solid::Body::default().union(&tool)
    }

    fn combined(
        &self,
        body: &cao_solid::Body,
        tool: &cao_solid::Body,
        mode: Mode,
    ) -> Result<cao_solid::Body, Declined> {
        match mode {
            Mode::Add => body.union(tool),
            Mode::Cut => body.difference(tool),
        }
    }

    /// The triangles the application draws, held to the room the exact
    /// kernel's own are: the application draws finer.
    fn triangles(&self, body: &cao_solid::Body) -> (Vec<Triangle>, f64) {
        let reach = body
            .bounds()
            .map_or(1.0, |(low, high)| low.abs().max(high.abs()).max_element());
        (body.triangles(), TESSELLATION * reach)
    }
}

/// A leaf as the exact kernel raises it: its outline and holes as runs, the
/// frame it is drawn in and its travel; nothing for a leaf it does not raise.
///
/// A rectangle as the profile's own rectangle, a circle as one whole turn
/// from the angle its flats start at, a ring as two whole turns from nought,
/// the outer one with the inner one as its hole, a rounded rectangle and a
/// slot as their contour of straight runs and arcs; the plane as the frame it
/// is drawn in, and the prism's height along the plane's normal.
fn exact_prism(leaf: &Leaf) -> Option<(Contour, Vec<Contour>, Frame, DVec3)> {
    let Leaf::Prism {
        plane,
        outline,
        height,
    } = leaf
    else {
        return None;
    };
    let (contour, holes) = match outline {
        Outline::Rectangle { low, high } => (Contour::rectangle(*low, *high), Vec::new()),
        Outline::Circle {
            center,
            radius,
            from,
        } => (whole_circle(*center, *radius, *from), Vec::new()),
        Outline::Ring {
            center,
            outer,
            inner,
        } => (
            whole_circle(*center, *outer, 0.0),
            vec![whole_circle(*center, *inner, 0.0)],
        ),
        Outline::Rounded { .. } | Outline::Slot { .. } => (outline.contour()?, Vec::new()),
        Outline::Star { .. } => return None,
    };
    let (origin, u, v) = plane.frame();
    Some((
        contour,
        holes,
        Frame { origin, u, v },
        plane.normal() * *height,
    ))
}

/// A turned leaf as the exact kernel turns it: each side of the section laid
/// square to its axis at no drawing's resolution, turned, and the two sides
/// joined.
fn turned_exactly(turned: &Turned) -> Result<Body, Declined> {
    let (frame, turn) = (turned.frame(), turned.turn(0.0));
    let mut sides = turned.drawn().into_iter().map(|side| {
        let (outline, holes) = side.contours();
        let straight =
            Straight::of(&outline, &holes, frame, &turn, 0.0).ok_or(Declined::Profile)?;
        Body::turned(&straight, frame, &turn)
    });
    let first = sides.next().ok_or(Declined::Profile)??;
    sides.try_fold(first, |joined, side| joined.joined(&side?))
}

/// A whole circle as one run round from a single corner, at `from` degrees.
pub fn whole_circle(center: DVec2, radius: f64, from: f64) -> Contour {
    Contour {
        corners: vec![center + DVec2::from_angle(from.to_radians()) * radius],
        runs: vec![Run::Round { center, turn: TAU }],
    }
}

/// The flats written before the exact kernel: circles laid as the flats the
/// application samples them into, held to the true circles within the
/// deepest of their sagittas.
pub struct Flats {
    tolerance: f64,
}

/// How far the flats may stand from a promise along a line wherever nothing
/// curves, as a fraction of the reach: the room the harness's own lines of
/// measure leave them.
const ALONG_A_LINE: f64 = 1e-6;

/// How many flats the flats lay round a whole turn, at most: a partial turn
/// takes its share of them, never fewer than three.
const TURN_STEPS: f64 = 64.0;

impl Flats {
    pub fn for_case(case: &Case) -> Flats {
        let reach = case
            .leaves()
            .filter_map(Leaf::bounds)
            .map(|(low, high)| low.abs().max(high.abs()).max_element())
            .fold(1.0, f64::max);
        let sagitta = case
            .leaves()
            .filter_map(|leaf| match leaf {
                Leaf::Prism {
                    outline: Outline::Circle { radius, .. },
                    ..
                } => Some(*radius),
                Leaf::Prism {
                    outline: Outline::Ring { outer, .. },
                    ..
                } => Some(*outer),
                Leaf::Prism {
                    outline: Outline::Rounded { radius, .. } | Outline::Slot { radius, .. },
                    ..
                } => Some(*radius),
                _ => None,
            })
            .map(|radius| radius * (1.0 - (std::f64::consts::PI / CIRCLE_STEPS as f64).cos()))
            .fold(0.0, f64::max);
        let turned = case
            .leaves()
            .filter_map(Leaf::as_turned)
            .flat_map(|turned| turned.section.pieces())
            .map(|piece| piece.away[1] * (1.0 - (std::f64::consts::PI / TURN_STEPS).cos()))
            .fold(0.0, f64::max);
        let sagitta = sagitta.max(turned);
        Flats {
            tolerance: sagitta.max(ALONG_A_LINE * reach),
        }
    }
}

impl Kernel for Flats {
    type Body = cao_solid::Body;

    fn raised(&self, leaf: &Leaf) -> Result<cao_solid::Body, Declined> {
        leaf.solid().ok_or(Declined::Profile)
    }

    fn combined(
        &self,
        body: &cao_solid::Body,
        tool: &cao_solid::Body,
        mode: Mode,
    ) -> Result<cao_solid::Body, Declined> {
        match mode {
            Mode::Add => body.union(tool),
            Mode::Cut => body.difference(tool),
        }
    }

    fn triangles(&self, body: &cao_solid::Body) -> (Vec<Triangle>, f64) {
        (body.triangles(), self.tolerance)
    }
}
