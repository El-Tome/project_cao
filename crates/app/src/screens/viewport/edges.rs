//! The edges of the part as the view draws and picks them: each under its
//! name, through the points the kernel draws it through.

use std::cell::RefCell;
use std::rc::Rc;

use cao_solid::{Body, DrawnEdge, EdgeName};
use glam::DVec3;

use super::ViewportState;

/// How far from an edge on screen, in points, the hand still takes it.
pub const EDGE_REACH: f32 = 6.0;

/// The edges of the last body asked for. The part's body changes only when it
/// is rebuilt, and drawing its edges costs the kernel a sampling of each.
#[derive(Default)]
pub(crate) struct DrawnEdges {
    last: RefCell<Option<(Body, Rc<[DrawnEdge]>)>>,
}

impl DrawnEdges {
    pub(crate) fn of(&self, body: &Body) -> Rc<[DrawnEdge]> {
        let mut last = self.last.borrow_mut();
        if let Some((kept, edges)) = last.as_ref()
            && kept == body
        {
            return Rc::clone(edges);
        }
        let edges: Rc<[DrawnEdge]> = body.drawn_edges().into();
        *last = Some((body.clone(), Rc::clone(&edges)));
        edges
    }
}

impl ViewportState {
    /// The points the edge of that name is drawn through on `body`, when the
    /// body still names one so.
    pub(crate) fn edge_points(&self, body: &Body, name: &EdgeName) -> Option<Vec<DVec3>> {
        self.edges
            .of(body)
            .iter()
            .find(|edge| edge.name == *name)
            .map(|edge| edge.points.clone())
    }
}
