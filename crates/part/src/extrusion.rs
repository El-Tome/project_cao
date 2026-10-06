//! Turning a sketch's areas into matter, or taking matter away: a prism along
//! the plane's normal, or a sweep around an axis lying in the plane.

use std::borrow::Cow;

use cao_sketch::{Area, Region, Sketch};
use cao_solid::profile::{Frame, Profile};
use cao_solid::turning::{Axis, Lie, Turn};
use cao_solid::{Body, Declined};
use glam::DVec2;

use crate::broken::Broken;
use crate::formula::Formula;
use crate::history::{ExtrusionMode, RevolutionAxis};
use crate::profile::profile;
use crate::state::PartState;

impl PartState {
    /// Raises a step of matter, or cuts it, noting the faces of the part it
    /// made: a tool's faces are numbered above the part's, so they are the
    /// ones numbered from where the part's ended. Noted even when the step
    /// made nothing, so that the rank of a step is the rank of its note.
    pub(crate) fn raising(&mut self, raise: impl FnOnce(&mut Self)) {
        let from = self.body.faces_end();
        raise(self);
        self.made.push(from..self.body.faces_end());
    }

    /// The faces the part still holds of those the step of matter of that
    /// rank made, counting the steps in the order they replay.
    pub(crate) fn faces_made(&self, rank: usize) -> Vec<usize> {
        self.made.get(rank).map_or_else(Vec::new, |made| {
            made.clone()
                .filter(|face| self.body.has_face(*face))
                .collect()
        })
    }

    /// Sweeps the chosen areas around an axis of the sketch and joins the
    /// result to the part, or takes it out.
    ///
    /// An area across the axis would sweep through itself: it is cut along
    /// the axis first, and each side is turned on its own, as if the axis had
    /// been drawn across it. A turn of straight runs parallel or square to
    /// the axis is the exact kernel's; anything else sends the part to the
    /// flats.
    pub(crate) fn revolve(
        &mut self,
        index: usize,
        areas: &[Area],
        axis: RevolutionAxis,
        degrees: &Formula,
        mode: ExtrusionMode,
    ) {
        let Some(degrees) = self.size(degrees, |turn| turn.abs() > 1e-6 && turn.abs() <= 360.0)
        else {
            return;
        };
        let Some(sketch) = self.sketches.get(index) else {
            return;
        };
        let Some(axis) = axis_in_sketch(sketch, axis) else {
            return;
        };

        let frame = frame_of(sketch);
        let regions = sketch.regions();
        let (standing, lost) = self.all_standing_on(index, areas, &regions);
        let sides = sides_of(sketch, &standing, axis, degrees.to_radians());
        let profiles: Vec<(Profile, Turn)> = sides
            .iter()
            .map(|(region, triangles, turn)| (profile(region, triangles), *turn))
            .collect();
        let past_turns = |body: &mut Body| {
            for (profile, turn) in &profiles {
                body.count_past_turned(profile, frame, turn);
            }
        };

        let mut tool = Body::default();
        for (profile, turn) in &profiles {
            let turned = self.body.tool_turned(profile, frame, turn);
            match turned.and_then(|piece| tool.union(&piece)) {
                Ok(joined) => tool = joined,
                Err(reason) => return self.declined_with(reason, past_turns),
            }
        }

        // An area the drawing no longer encloses raises nothing, which is a
        // size that no longer holds.
        if lost {
            self.broke(Broken::Operation(self.replaying));
        }
        if let Err(reason) = self.combine(tool, mode) {
            self.declined_with(reason, past_turns);
        }
    }

    /// Joins a tool to the part, or takes it out. A step the kernel declines
    /// leaves the part as it was.
    pub(crate) fn combine(&mut self, tool: Body, mode: ExtrusionMode) -> Result<(), Declined> {
        if tool.is_empty() {
            return Ok(());
        }
        self.body = match mode {
            ExtrusionMode::Add => self.body.union(&tool),
            ExtrusionMode::Cut => self.body.difference(&tool),
        }?;
        Ok(())
    }

    /// Turns the chosen areas of a sketch into a prism and joins it to the
    /// part, or takes it out.
    ///
    /// Every area is turned into matter first and the lot is applied in one go:
    /// two areas extruded together must behave as one shape, not as two that
    /// happen to be cut one after the other.
    pub(crate) fn extrude(
        &mut self,
        index: usize,
        areas: &[Area],
        distance: &Formula,
        mode: ExtrusionMode,
    ) {
        let Some(distance) = self.size(distance, |travel| travel.abs() >= 1e-6) else {
            return;
        };
        self.fix_a_unit_at_a_millimetre();
        let scale = self.scale();
        let Some(sketch) = self.sketches.get(index) else {
            return;
        };

        let plane = sketch.plane;
        let travel = plane.normal() * (distance / scale);
        let regions = sketch.regions();

        let (standing, lost) = self.all_standing_on(index, areas, &regions);
        let triangles: Vec<_> = standing
            .iter()
            .map(|region| region.face_triangles())
            .collect();
        let profiles: Vec<Profile> = standing
            .iter()
            .zip(&triangles)
            .map(|(region, triangles)| profile(region, triangles))
            .collect();
        let frame = Frame {
            origin: plane.origin,
            u: plane.u,
            v: plane.v,
        };

        let mut tool = Body::default();
        for profile in &profiles {
            let raised = self.body.tool_raised(profile, frame, travel);
            match raised.and_then(|piece| tool.union(&piece)) {
                Ok(joined) => tool = joined,
                Err(reason) => return self.declined_with(reason, past_raises(&profiles)),
            }
        }

        // An area the drawing no longer encloses raises nothing, which is a
        // size that no longer holds.
        if lost {
            self.broke(Broken::Operation(self.replaying));
        }
        if let Err(reason) = self.combine(tool, mode) {
            self.declined_with(reason, past_raises(&profiles));
        }
    }

    /// A step the kernel declined: broken, named among the declined ones with
    /// the reason it gave, and counted past the numbers it would have given its
    /// faces, so that the steps after it number theirs as if it had stood.
    fn declined_with(&mut self, reason: Declined, count_past: impl FnOnce(&mut Body)) {
        self.broke(Broken::Operation(self.replaying));
        self.declined.insert(self.replaying, reason);
        count_past(&mut self.body);
    }

    /// The areas of a drawing these places fall in, each named by the curves
    /// that bound it.
    ///
    /// This is what a click means, and it is worked out at the moment of the
    /// click and never again: snapping and what lies under the cursor depend
    /// on the view at the time, the same reason `PointRef` records the point
    /// a click landed on.
    pub fn areas_at(&self, sketch: usize, places: &[DVec2]) -> Vec<Area> {
        let Some(drawing) = self.sketches.get(sketch) else {
            return Vec::new();
        };
        let regions = drawing.regions();
        places
            .iter()
            .filter_map(|place| {
                let rank = cao_sketch::area_under(&regions, *place)?;
                Some(Area::of(&regions[rank], *place))
            })
            .collect()
    }

    /// The areas a step of matter stands on, as the drawing holds them now,
    /// and whether one of them is lost.
    fn all_standing_on<'a>(
        &self,
        sketch: usize,
        areas: &[Area],
        regions: &'a [Region],
    ) -> (Vec<&'a Region>, bool) {
        let mut lost = false;
        let standing = areas
            .iter()
            .filter_map(|area| {
                let region = self.standing_on(sketch, area, regions);
                lost |= region.is_none();
                region
            })
            .collect();
        (standing, lost)
    }

    /// The area a step of matter stands on, as the drawing holds it now.
    ///
    /// The name it was given at the click is followed through every cut made
    /// since, and then asked of the drawing. Nothing when the drawing no
    /// longer encloses it — which is what the tree says, rather than raising
    /// matter somewhere the user never pointed at.
    fn standing_on<'a>(
        &self,
        sketch: usize,
        area: &Area,
        regions: &'a [Region],
    ) -> Option<&'a Region> {
        regions.get(self.area_rank(sketch, area, regions)?)
    }
}

/// Counts a body past the numbers a raise of each profile would have named.
fn past_raises<'a>(profiles: &'a [Profile<'a>]) -> impl FnOnce(&mut Body) + 'a {
    move |body| profiles.iter().for_each(|profile| body.count_past(profile))
}

/// Where a revolution's axis lies, in the sketch's own coordinates.
fn axis_in_sketch(sketch: &Sketch, axis: RevolutionAxis) -> Option<Axis> {
    match axis {
        RevolutionAxis::Sketch(axis) => Some(Axis {
            origin: DVec2::ZERO,
            direction: axis.direction(),
        }),
        RevolutionAxis::Segment(segment) => {
            if segment.0 >= sketch.segments().len() {
                return None;
            }
            let (start, end) = sketch.endpoints(segment);
            ((end - start).length() > 1e-6).then_some(Axis {
                origin: start,
                direction: end - start,
            })
        }
    }
}

/// The frame a sketch's areas stand in, for the kernels.
fn frame_of(sketch: &Sketch) -> Frame {
    Frame {
        origin: sketch.plane.origin,
        u: sketch.plane.u,
        v: sketch.plane.v,
    }
}

/// Each area as a turn takes it, with its triangles: whole when it lies on
/// one side of the axis, and cut along the axis into its sides when it lies
/// across it. Every side keeps the band its whole area was read at, so that
/// a corner the drawing left a hair off the axis lands on it whichever side
/// it ended up in.
fn sides_of<'a>(
    sketch: &Sketch,
    standing: &[&'a Region],
    axis: Axis,
    angle: f64,
) -> Vec<(Cow<'a, Region>, Vec<[DVec2; 3]>, Turn)> {
    let resolution = sketch.resolution();
    let mut sides = Vec::new();
    for &region in standing {
        let triangles = region.face_triangles();
        let whole = profile(region, &triangles);
        let turn = Turn::of(axis, angle, resolution, &whole);
        if turn.lie(&whole) != Lie::Across {
            sides.push((Cow::Borrowed(region), triangles, turn));
            continue;
        }
        for side in sketch.pieces_across(region, axis.origin, axis.direction) {
            let triangles = side.face_triangles();
            sides.push((Cow::Owned(side), triangles, turn));
        }
    }
    sides
}
