//! The kernels a case can be run through and held to the arithmetic: the
//! exact kernel of #498, and the flats written before it, which is what shows
//! the harness sound while the exact kernel still answers nothing.

use std::f64::consts::TAU;

use cao_solid::Mesh;
use cao_solid::brep::{Body, Listing};
use cao_solid::profile::{Contour, Frame, Run};
use cao_solid::soundness::Triangle;
use glam::{DVec2, DVec3};

use super::{CIRCLE_STEPS, Case, Leaf, Mode, Outline};

/// What the check held to the arithmetic asks of a kernel.
pub trait Kernel {
    type Body: Clone;

    /// The body a leaf is raised into, or `None` when the kernel declines.
    fn raised(&self, leaf: &Leaf) -> Option<Self::Body>;

    /// A body with a tool added to it or taken out of it, or `None` when the
    /// kernel declines.
    fn combined(&self, body: &Self::Body, tool: &Self::Body, mode: Mode) -> Option<Self::Body>;

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
const TESSELLATION: f64 = 1e-3;

impl Kernel for Exact {
    type Body = Body;

    /// A rectangle as the profile's own rectangle, a circle as one whole turn
    /// from the angle its flats start at, a ring as two whole turns from
    /// nought, the outer one with the inner one as its hole, a rounded
    /// rectangle and a slot as their contour of straight runs and arcs; the
    /// plane as the frame it is drawn in, and the prism's height along the
    /// plane's normal.
    fn raised(&self, leaf: &Leaf) -> Option<Body> {
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
        Body::raised(
            &contour,
            &holes,
            Frame { origin, u, v },
            plane.normal() * *height,
        )
        .ok()
    }

    fn combined(&self, body: &Body, tool: &Body, mode: Mode) -> Option<Body> {
        match mode {
            Mode::Add => body.joined(tool),
            Mode::Cut => body.cut_by(tool),
        }
        .ok()
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
        Flats {
            tolerance: sagitta.max(ALONG_A_LINE * reach),
        }
    }
}

impl Kernel for Flats {
    type Body = Mesh;

    fn raised(&self, leaf: &Leaf) -> Option<Mesh> {
        leaf.solid()
    }

    fn combined(&self, body: &Mesh, tool: &Mesh, mode: Mode) -> Option<Mesh> {
        Some(match mode {
            Mode::Add => body.union(tool),
            Mode::Cut => body.difference(tool),
        })
    }

    fn triangles(&self, body: &Mesh) -> (Vec<Triangle>, f64) {
        (body.triangles(), self.tolerance)
    }
}
