//! What app · screens/viewport/render/cuts.rs is held to.
//!
//! Closes #663.
//! - A fillet's or a chamfer's own points are not lit while a box is held
//!   beside them (surfaced by review) —
//!   `a_fillet_s_point_a_box_is_held_beside_is_not_lit`; pointing at one with
//!   no button held still lights it —
//!   `a_fillet_s_point_pointed_at_with_no_button_held_is_lit`
//!
//! Closes #664.
//! - A fillet's or a chamfer's point is not lit while just pointing 22 px from
//!   it (surfaced by review) —
//!   `a_fillet_s_point_22_px_from_the_hand_is_not_lit_while_pointing`
//! - The issue's three criteria and the drags review surfaced — no test: held
//!   in `crates/app/tests/selecting_under_the_hand.rs` and
//!   `screens/viewport/input/selecting/tests.rs`, under their own
//!   `Closes #664` blocks
//!
//! The canvas's share of #621, whose criteria live with the part's tests in
//! `crates/part/tests/a_fillet_rounds_a_corner_with_a_curve.rs`: a curve
//! rounded at its end is drawn solid from its far end to the fillet and in
//! thin dashes on to its sharp corner, as a trait side is —
//! `a_curve_rounded_at_its_end_is_drawn_solid_to_the_fillet_and_dashed_to_its_sharp_corner`
//! — and whole beside a fillet that does not fit —
//! `a_curve_beside_a_fillet_that_does_not_fit_is_drawn_whole`; the stretch
//! it hides lights with its cut, as a trait side's does —
//! `the_dashes_of_a_curve_up_to_its_sharp_corner_light_with_their_cut` — and a
//! fillet beside a curve that does not fit is red —
//! `a_fillet_beside_a_curve_that_does_not_fit_is_painted_red`.

use cao_part::history::PointRef;
use cao_part::{Operation, PartDocument};
use cao_sketch::{Corner, CornerCutId, SegmentId, Selection, ToolState, WorkPlane};
use glam::DVec2;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::{SketchEditor, SketchPhase, Tool};
use cao_prefs::config::ViewportConfig;
use cao_render::camera::OrbitCamera;

/// A plate forty by twenty, its four corners rounded together by five.
fn a_rounded_plate() -> PartDocument {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(40.0, 20.0)),
        construction: false,
    });
    document.apply(Operation::Fillet {
        sketch: 0,
        corners: (0..4)
            .map(|side| Corner::Between(SegmentId(side).into(), SegmentId((side + 1) % 4).into()))
            .collect(),
        radius: 5.0.into(),
        construction: false,
    });
    document
}

fn lit_holding(held: Vec<Selection>, hovered: Option<Selection>) -> Lit {
    let mut document = a_rounded_plate();
    let mut editor = SketchEditor {
        phase: SketchPhase::Editing(0),
        tool: Tool::Select,
        tool_state: ToolState::Select(Box::new(cao_sketch::SelectState {
            held,
            ..Default::default()
        })),
        hovered,
        ..Default::default()
    };
    let (mut extrusion, lang) = (ExtrusionState::default(), Catalogue::french());
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    lit_cuts(&context.document.sketches()[0], &context)
}

#[test]
fn a_fillet_shown_to_a_tangent_is_drawn_as_picked() {
    let mut document = a_rounded_plate();
    let sketch = document.sketches()[0].clone();
    let shown = CornerCutId(0);
    let mut editor = SketchEditor {
        phase: SketchPhase::Editing(0),
        tool: Tool::Constrain(cao_sketch::Rule::Tangent),
        tool_state: ToolState::Constrain {
            picks: vec![cao_sketch::RulePick::CutCurve(shown)],
        },
        ..Default::default()
    };
    let (mut extrusion, lang) = (ExtrusionState::default(), Catalogue::french());
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let scale = ViewScale::of(
        &OrbitCamera::default(),
        SCREEN,
        1.0,
        &ViewportConfig::default(),
        1.0,
    );
    let theme = Theme::default();
    let mut painted = Vec::new();

    CutCanvas::read(&sketch, &context).push_cuts(
        &mut painted,
        &[],
        (true, &[]),
        (&theme, scale),
        &context,
    );

    assert!(!painted.is_empty(), "the plate's fillets were drawn");
    let picked = tint_at(theme.picked, 1.0);
    let drawn_picked: Vec<DVec2> = painted
        .iter()
        .filter(|vertex| vertex.color == picked)
        .map(|vertex| {
            sketch
                .plane
                .to_local(glam::Vec3::from(vertex.position).as_dvec3())
        })
        .collect();
    assert!(
        !drawn_picked.is_empty(),
        "the fillet shown to the rule is drawn as the others are, not as picked"
    );
    let CutCurve::Arc(arc) = sketch.drawn_cut(shown).expect("the fillet is drawn").curve else {
        panic!("a fillet draws an arc");
    };
    let radius = arc.centre.distance(arc.start);
    for place in drawn_picked {
        assert!(
            (place.distance(arc.centre) - radius).abs() < 1e-3,
            "drawn as picked at {place}, off the fillet shown to the rule"
        );
    }
}

#[test]
fn the_cut_held_is_lit_apart_from_the_cuts_laid_with_it() {
    let lit = lit_holding(vec![Selection::Cut(CornerCutId(2))], None);

    assert_eq!(lit.held, vec![CornerCutId(2)]);
    assert_eq!(
        lit.linked,
        vec![CornerCutId(0), CornerCutId(1), CornerCutId(3)]
    );
}

#[test]
fn the_cut_pointed_at_is_lit_apart_too() {
    let lit = lit_holding(Vec::new(), Some(Selection::Cut(CornerCutId(0))));

    assert_eq!(lit.held, vec![CornerCutId(0)]);
    assert_eq!(lit.linked.len(), 3);
}

#[test]
fn nothing_held_lights_no_cut() {
    assert_eq!(lit_holding(Vec::new(), None), Lit::default());
}

#[test]
fn a_cut_lit_at_a_point_carrying_another_lights_its_own_sides_alone() {
    let mut sketch = cao_sketch::Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::ZERO);
    let ends =
        [(0.0, 40.0), (30.0, 30.0), (40.0, 0.0)].map(|(x, y)| sketch.add_point(DVec2::new(x, y)));
    let [top, middle, bottom] = ends.map(|end| sketch.add_segment(corner, end));
    let shape = cao_sketch::CutShape::Fillet { radius: 2.0 };
    let near = sketch
        .cut_corner(top, middle, shape, false, 0)
        .expect("a corner");
    sketch
        .cut_corner(top, bottom, shape, false, 1)
        .expect("another corner of the same point");

    let lit = ends_of(&sketch, &[near]);

    let side = cao_sketch::CutSide::Trait;
    assert!(lit.contains(&(corner, side(top))) && lit.contains(&(corner, side(middle))));
    assert!(!lit.contains(&(corner, side(bottom))));
}

/// How many of a fillet's points are drawn lit, the cursor resting a few
/// pixels off the first one, with or without a box being pulled.
fn fillet_points_lit_with_the_cursor_beside_one(box_pulled: bool) -> usize {
    let mut document = a_rounded_plate();
    let scale = ViewScale::of(
        &OrbitCamera::default(),
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1280.0, 800.0)),
        1.0,
        &ViewportConfig::default(),
        1.0,
    );
    let sketch = document.sketches()[0].clone();
    let (_, tangent_point) = sketch.drawn_cuts()[0]
        .points()
        .next()
        .expect("a fillet offers its points");
    let beside = tangent_point + DVec2::new(1.0, 1.0).normalize() * scale.world_size_of(9.0);
    let mut editor = SketchEditor {
        phase: SketchPhase::Editing(0),
        tool: Tool::Select,
        tool_state: ToolState::Select(Box::new(cao_sketch::SelectState {
            band: box_pulled.then_some((DVec2::new(-30.0, -30.0), beside)),
            ..Default::default()
        })),
        cursor: Some(beside),
        ..Default::default()
    };
    let (mut extrusion, lang) = (ExtrusionState::default(), Catalogue::french());
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let theme = Theme::default();
    let mut painted = Vec::new();
    CutCanvas::read(&sketch, &context).push_points(&mut painted, &[], (&theme, scale), &context);
    let lit = tint_at(theme.highlight, 1.0);
    painted.iter().filter(|vertex| vertex.color == lit).count()
}

#[test]
fn a_fillet_s_point_pointed_at_with_no_button_held_is_lit() {
    assert!(
        fillet_points_lit_with_the_cursor_beside_one(false) > 0,
        "the point under the cursor is lit, the way it offers itself to a click"
    );
}

#[test]
fn a_fillet_s_point_a_box_is_held_beside_is_not_lit() {
    assert_eq!(
        fillet_points_lit_with_the_cursor_beside_one(true),
        0,
        "the box will not take the fillet's point it is held beside, so the point is not lit"
    );
}

const SCREEN: egui::Rect = egui::Rect {
    min: egui::Pos2::ZERO,
    max: egui::pos2(1280.0, 800.0),
};

/// A plate away from the plane's axes, its corners rounded by five, seen
/// straight on: a pixel covers the same length whichever way it is measured.
struct PointingAtAPlate {
    state: crate::screens::viewport::ViewportState,
    document: PartDocument,
    editor: SketchEditor,
    egui: egui::Context,
}

impl PointingAtAPlate {
    fn new() -> Self {
        let mut state = crate::screens::viewport::ViewportState::default();
        state
            .camera
            .set_view_angles(0.0, std::f32::consts::FRAC_PI_2);
        let mut document = PartDocument::new("part", chrono::Utc::now());
        document.apply(Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        });
        document.apply(Operation::AddRectangle {
            sketch: 0,
            corner: PointRef::New(DVec2::new(20.0, 20.0)),
            opposite: PointRef::New(DVec2::new(60.0, 40.0)),
            construction: false,
        });
        document.apply(Operation::Fillet {
            sketch: 0,
            corners: (0..4)
                .map(|side| {
                    Corner::Between(SegmentId(side).into(), SegmentId((side + 1) % 4).into())
                })
                .collect(),
            radius: 5.0.into(),
            construction: false,
        });
        Self {
            state,
            document,
            editor: SketchEditor {
                phase: SketchPhase::Editing(0),
                tool: Tool::Select,
                tool_state: ToolState::Select(Box::default()),
                ..Default::default()
            },
            egui: egui::Context::default(),
        }
    }

    fn scale(&self) -> ViewScale {
        ViewScale::of(&self.state.camera, SCREEN, 1.0, &self.state.config, 1.0)
    }

    fn on_the_screen(&self, place: DVec2) -> egui::Pos2 {
        let view_projection = self
            .state
            .camera
            .view_projection(SCREEN.width() / SCREEN.height());
        let clip = view_projection * WorkPlane::XY.to_world(place).as_vec3().extend(1.0);
        let ndc = clip.truncate() / clip.w;
        egui::pos2(
            SCREEN.center().x + ndc.x * SCREEN.width() * 0.5,
            SCREEN.center().y - ndc.y * SCREEN.height() * 0.5,
        )
    }

    /// The hand rested there for a frame, through the input pass, then how
    /// many of the fillets' points are drawn lit.
    fn points_lit_with_the_hand_at(&mut self, place: DVec2) -> usize {
        let at = self.on_the_screen(place);
        let scale = self.scale();
        let (mut extrusion, lang) = (ExtrusionState::default(), Catalogue::french());
        let mut context = SketchContext {
            document: &mut self.document,
            editor: &mut self.editor,
            extrusion: &mut extrusion,
            lang: &lang,
        };
        for events in [vec![], vec![egui::Event::PointerMoved(at)], vec![]] {
            self.egui.begin_pass(egui::RawInput {
                screen_rect: Some(SCREEN),
                events,
                ..Default::default()
            });
            egui::Area::new(egui::Id::new("the canvas"))
                .fixed_pos(SCREEN.min)
                .show(&self.egui, |ui| {
                    ui.set_min_size(SCREEN.size());
                    let response = ui.interact(
                        SCREEN,
                        egui::Id::new("the canvas's surface"),
                        egui::Sense::click_and_drag(),
                    );
                    crate::screens::viewport::input::handle_sketch_input(
                        ui,
                        &mut self.state,
                        &response,
                        SCREEN,
                        scale,
                        &mut context,
                    );
                });
            self.egui.end_pass().textures_delta.clear();
        }
        let sketch = context.document.sketches()[0].clone();
        let theme = Theme::default();
        let mut painted = Vec::new();
        CutCanvas::read(&sketch, &context).push_points(
            &mut painted,
            &[],
            (&theme, scale),
            &context,
        );
        let lit = tint_at(theme.highlight, 1.0);
        painted.iter().filter(|vertex| vertex.color == lit).count()
    }
}

#[test]
fn a_fillet_s_point_22_px_from_the_hand_is_not_lit_while_pointing() {
    let mut plate = PointingAtAPlate::new();
    let pixel = plate.scale().world_size_of(1.0);
    let where_the_bottom_side_starts = plate.document.sketches()[0].drawn_cuts()[0]
        .points()
        .map(|(_, place)| place)
        .find(|place| (place.y - 20.0).abs() < 1e-6 && place.x > 20.0)
        .expect("a fillet meets the bottom side");
    let below = DVec2::new(0.0, -1.0);

    assert!(
        plate.points_lit_with_the_hand_at(where_the_bottom_side_starts + below * 9.0 * pixel) > 0,
        "the fillet's point 9 px from the hand is lit, the way it offers itself to a click"
    );

    assert_eq!(
        plate.points_lit_with_the_hand_at(where_the_bottom_side_starts + below * 22.0 * pixel),
        0,
        "22 px from the hand, the fillet's point is out of reach of a click, so it is not lit"
    );
}

/// A trait from (10, 10) to (50, 10) and an arc round (2, 10) of radius 8 from
/// that corner to `far`, the corner rounded by `radius`.
fn a_trait_and_an_arc_rounded(far: DVec2, radius: f64) -> PartDocument {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(10.0, 10.0),
        on: Vec::new(),
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(cao_sketch::PointId(1)),
        end: PointRef::New(DVec2::new(50.0, 10.0)),
        construction: false,
    });
    document.apply(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(DVec2::new(2.0, 10.0)),
        start: PointRef::Existing(cao_sketch::PointId(1)),
        end: PointRef::New(far),
        construction: false,
    });
    document.apply(Operation::Fillet {
        sketch: 0,
        corners: vec![Corner::At(cao_sketch::PointId(1))],
        radius: radius.into(),
        construction: false,
    });
    document
}

/// An arc of ellipse round (30, 20), its first axis (8, 6) and its second
/// reaching 5, drawn from the corner at (32.4, 26.8) round to (22, 14); and an
/// arc round (42.4, 26.8) of radius 10 from that corner down to (42.4, 16.8),
/// the corner rounded by two.
fn an_arc_of_ellipse_and_an_arc_rounded() -> PartDocument {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(32.4, 26.8),
        on: Vec::new(),
    });
    let back = PointRef::New(DVec2::new(22.0, 14.0));
    document.apply(Operation::AddEllipse {
        sketch: 0,
        center: PointRef::New(DVec2::new(30.0, 20.0)),
        first: [back.clone(), PointRef::New(DVec2::new(38.0, 26.0))],
        second: [
            PointRef::New(DVec2::new(33.0, 16.0)),
            PointRef::New(DVec2::new(27.0, 24.0)),
        ],
        construction: false,
        drawn: Some([PointRef::Existing(cao_sketch::PointId(1)), back]),
    });
    document.apply(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(DVec2::new(42.4, 26.8)),
        start: PointRef::Existing(cao_sketch::PointId(1)),
        end: PointRef::New(DVec2::new(42.4, 16.8)),
        construction: false,
    });
    document.apply(Operation::Fillet {
        sketch: 0,
        corners: vec![Corner::Between(
            cao_sketch::EllipseId(0).into(),
            cao_sketch::ArcId(0).into(),
        )],
        radius: 2.0.into(),
        construction: false,
    });
    document
}

/// A curve side of a corner as the test laid it, leaving the corner the way
/// its drawing runs when `counter_clockwise`.
#[derive(Clone, Copy, Debug)]
enum CurveSide {
    Arc {
        centre: DVec2,
        radius: f64,
        counter_clockwise: bool,
    },
    ArcOfEllipse {
        centre: DVec2,
        first: DVec2,
        second: f64,
        counter_clockwise: bool,
    },
}

impl CurveSide {
    /// How far round from the corner a place lies, the way the side leaves
    /// it — an angle round an arc, a turn round an ellipse, within half a turn
    /// either way — when the place lies on the side's curve.
    fn along(self, corner: DVec2, place: DVec2) -> Option<f64> {
        const ON: f64 = 1e-5;
        let (turn, counter_clockwise): (Box<dyn Fn(DVec2) -> f64>, bool) = match self {
            Self::Arc {
                centre,
                radius,
                counter_clockwise,
            } => {
                if (place.distance(centre) - radius).abs() > ON * radius {
                    return None;
                }
                (
                    Box::new(move |at: DVec2| (at - centre).to_angle()),
                    counter_clockwise,
                )
            }
            Self::ArcOfEllipse {
                centre,
                first,
                second,
                counter_clockwise,
            } => {
                let squashed = move |at: DVec2| {
                    let out = at - centre;
                    DVec2::new(
                        out.dot(first.normalize()) / first.length(),
                        out.dot(first.perp().normalize()) / second,
                    )
                };
                if (squashed(place).length() - 1.0).abs() > ON {
                    return None;
                }
                (
                    Box::new(move |at: DVec2| squashed(at).to_angle()),
                    counter_clockwise,
                )
            }
        };
        let turned = turn(place) - turn(corner);
        let turned = match counter_clockwise {
            true => turned,
            false => -turned,
        };
        let tau = std::f64::consts::TAU;
        Some(turned - tau * (turned / tau).round())
    }
}

/// How far round from the corner the canvas strokes a curve side, solid and
/// thin, as the frame paints the drawing with no tool in hand.
fn stroked_along(
    document: &mut PartDocument,
    corner: DVec2,
    side: CurveSide,
) -> (Vec<f64>, Vec<f64>) {
    let sketch = document.sketches()[0].clone();
    let mut editor = SketchEditor {
        phase: SketchPhase::Editing(0),
        ..Default::default()
    };
    let (mut extrusion, lang) = (ExtrusionState::default(), Catalogue::french());
    let context = SketchContext {
        document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let scale = ViewScale::of(
        &OrbitCamera::default(),
        SCREEN,
        1.0,
        &ViewportConfig::default(),
        1.0,
    );
    let theme = Theme::default();
    let (mut painted, mut surfaces) = (Vec::new(), Vec::new());
    let shown = super::super::drawing::Shown {
        sketch: &sketch,
        laid: &[],
        cuts: &[],
        going: None,
        active: true,
    };
    super::super::drawing::push_sketch(
        &mut painted,
        &mut surfaces,
        &shown,
        &theme,
        scale,
        &context,
    );
    let (mut solid, mut thin) = (Vec::new(), Vec::new());
    for stroke in painted.chunks(2) {
        let ends = [stroke[0], stroke[1]].map(|vertex| {
            sketch
                .plane
                .to_local(glam::Vec3::from(vertex.position).as_dvec3())
        });
        let (Some(from), Some(to)) = (side.along(corner, ends[0]), side.along(corner, ends[1]))
        else {
            continue;
        };
        match stroke[0].width < theme.sketch_width * 0.9 {
            true => thin.extend([from, to]),
            false => solid.extend([from, to]),
        }
    }
    (solid, thin)
}

/// How far round from the corner the fillet the drawing shows touches a
/// curve side.
fn touched_along(document: &PartDocument, corner: DVec2, side: CurveSide) -> f64 {
    let drawn = document.sketches()[0].drawn_cuts();
    assert_eq!(drawn.len(), 1, "one fillet is drawn: {drawn:?}");
    drawn[0]
        .touches
        .iter()
        .find_map(|touch| side.along(corner, *touch))
        .expect("the fillet touches the curve")
}

const ALONG: f64 = 1e-4;

fn lowest(along: &[f64]) -> f64 {
    along.iter().copied().fold(f64::INFINITY, f64::min)
}

fn highest(along: &[f64]) -> f64 {
    along.iter().copied().fold(f64::NEG_INFINITY, f64::max)
}

#[test]
fn a_curve_rounded_at_its_end_is_drawn_solid_to_the_fillet_and_dashed_to_its_sharp_corner() {
    let ellipse_corner = DVec2::new(32.4, 26.8);
    let cases = [
        (
            a_trait_and_an_arc_rounded(DVec2::new(2.0, 18.0), 5.0),
            DVec2::new(10.0, 10.0),
            CurveSide::Arc {
                centre: DVec2::new(2.0, 10.0),
                radius: 8.0,
                counter_clockwise: true,
            },
            DVec2::new(2.0, 18.0),
            "an arc rounded beside a trait",
        ),
        (
            an_arc_of_ellipse_and_an_arc_rounded(),
            ellipse_corner,
            CurveSide::ArcOfEllipse {
                centre: DVec2::new(30.0, 20.0),
                first: DVec2::new(8.0, 6.0),
                second: 5.0,
                counter_clockwise: true,
            },
            DVec2::new(22.0, 14.0),
            "an arc of ellipse rounded beside an arc",
        ),
        (
            an_arc_of_ellipse_and_an_arc_rounded(),
            ellipse_corner,
            CurveSide::Arc {
                centre: DVec2::new(42.4, 26.8),
                radius: 10.0,
                counter_clockwise: true,
            },
            DVec2::new(42.4, 16.8),
            "an arc rounded beside an arc of ellipse",
        ),
    ];
    for (mut document, corner, side, far, what) in cases {
        let touch = touched_along(&document, corner, side);
        let far = side
            .along(corner, far)
            .expect("the far end is on the curve");
        let (solid, thin) = stroked_along(&mut document, corner, side);

        assert!(
            !solid.is_empty() && (lowest(&solid) - touch).abs() < ALONG,
            "{what}: drawn solid from where the fillet touches it, {touch} round from the \
             corner, not from {}",
            lowest(&solid),
        );
        assert!(
            (highest(&solid) - far).abs() < ALONG,
            "{what}: drawn solid up to its far end, {far} round, not {}",
            highest(&solid),
        );
        assert!(
            !thin.is_empty(),
            "{what}: the stretch the fillet hides, from the touch to the sharp corner, is \
             drawn in thin dashes"
        );
        assert!(
            thin.iter()
                .all(|along| (-ALONG..=touch + ALONG).contains(along)),
            "{what}: the thin dashes run between the sharp corner and the touch, {touch} \
             round, and nowhere else: {thin:?}"
        );
    }
}

#[test]
fn a_curve_beside_a_fillet_that_does_not_fit_is_drawn_whole() {
    let corner = DVec2::new(10.0, 10.0);
    let far = DVec2::new(2.0 + 96.0 / 13.0, 10.0 + 40.0 / 13.0);
    let side = CurveSide::Arc {
        centre: DVec2::new(2.0, 10.0),
        radius: 8.0,
        counter_clockwise: true,
    };
    let mut document = a_trait_and_an_arc_rounded(far, 8.0);
    assert!(
        document.sketches()[0]
            .drawn_cuts()
            .iter()
            .all(|cut| !cut.fits),
        "a fillet of eight touches the arc past its end"
    );

    let (solid, thin) = stroked_along(&mut document, corner, side);

    let far = side.along(corner, far).expect("the far end is on the arc");
    assert!(
        !solid.is_empty() && lowest(&solid).abs() < ALONG && (highest(&solid) - far).abs() < ALONG,
        "the arc is drawn solid from its corner to its far end, {far} round: from {} to {}",
        lowest(&solid),
        highest(&solid),
    );
    assert!(thin.is_empty(), "nothing of the arc is dashed: {thin:?}");
}

/// A trait rising from (46, 34) to (46, 54), and an arc of ellipse ten wide
/// and five high round (40, 30) drawn from that corner over its top to
/// (30, 30), the corner rounded by three.
fn a_trait_and_an_arc_of_ellipse_rounded() -> PartDocument {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(46.0, 34.0),
        on: Vec::new(),
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(cao_sketch::PointId(1)),
        end: PointRef::New(DVec2::new(46.0, 54.0)),
        construction: false,
    });
    let back = PointRef::New(DVec2::new(30.0, 30.0));
    document.apply(Operation::AddEllipse {
        sketch: 0,
        center: PointRef::New(DVec2::new(40.0, 30.0)),
        first: [back.clone(), PointRef::New(DVec2::new(50.0, 30.0))],
        second: [
            PointRef::New(DVec2::new(40.0, 25.0)),
            PointRef::New(DVec2::new(40.0, 35.0)),
        ],
        construction: false,
        drawn: Some([PointRef::Existing(cao_sketch::PointId(1)), back]),
    });
    document.apply(Operation::Fillet {
        sketch: 0,
        corners: vec![Corner::At(cao_sketch::PointId(1))],
        radius: 3.0.into(),
        construction: false,
    });
    document
}

/// Every stroke the frame paints of the drawing, the cursor resting on
/// `hovered`: its two ends, its colour and its width.
fn strokes_hovering(
    document: &mut PartDocument,
    hovered: Option<Selection>,
) -> Vec<([DVec2; 2], [f32; 4], f32)> {
    let sketch = document.sketches()[0].clone();
    let mut editor = SketchEditor {
        phase: SketchPhase::Editing(0),
        tool: Tool::Select,
        tool_state: ToolState::Select(Box::default()),
        hovered,
        ..Default::default()
    };
    let (mut extrusion, lang) = (ExtrusionState::default(), Catalogue::french());
    let context = SketchContext {
        document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };
    let scale = ViewScale::of(
        &OrbitCamera::default(),
        SCREEN,
        1.0,
        &ViewportConfig::default(),
        1.0,
    );
    let (mut painted, mut surfaces) = (Vec::new(), Vec::new());
    let shown = super::super::drawing::Shown {
        sketch: &sketch,
        laid: &[],
        cuts: &[],
        going: None,
        active: true,
    };
    super::super::drawing::push_sketch(
        &mut painted,
        &mut surfaces,
        &shown,
        &Theme::default(),
        scale,
        &context,
    );
    painted
        .chunks(2)
        .map(|stroke| {
            (
                [stroke[0], stroke[1]].map(|vertex| {
                    sketch
                        .plane
                        .to_local(glam::Vec3::from(vertex.position).as_dvec3())
                }),
                stroke[0].color,
                stroke[0].width,
            )
        })
        .collect()
}

/// The colours of the thin strokes painted between the corner and the touch
/// on a side: `along` says how far round from the corner a place of the side
/// lies, when it lies on it.
fn colours_of_the_dashes(
    strokes: &[([DVec2; 2], [f32; 4], f32)],
    touch: f64,
    along: impl Fn(DVec2) -> Option<f64>,
) -> Vec<[f32; 4]> {
    let thin = Theme::default().sketch_width * 0.9;
    strokes
        .iter()
        .filter(|(_, _, width)| *width < thin)
        .filter(|(ends, _, _)| {
            ends.iter().all(|end| {
                along(*end).is_some_and(|along| (-ALONG..=touch + ALONG).contains(&along))
            })
        })
        .map(|(_, colour, _)| *colour)
        .collect()
}

#[test]
fn the_dashes_of_a_curve_up_to_its_sharp_corner_light_with_their_cut() {
    let cases = [
        (
            a_trait_and_an_arc_rounded(DVec2::new(2.0, 18.0), 5.0),
            DVec2::new(10.0, 10.0),
            DVec2::new(50.0, 10.0),
            CurveSide::Arc {
                centre: DVec2::new(2.0, 10.0),
                radius: 8.0,
                counter_clockwise: true,
            },
            "an arc rounded beside a trait",
        ),
        (
            a_trait_and_an_arc_of_ellipse_rounded(),
            DVec2::new(46.0, 34.0),
            DVec2::new(46.0, 54.0),
            CurveSide::ArcOfEllipse {
                centre: DVec2::new(40.0, 30.0),
                first: DVec2::new(10.0, 0.0),
                second: 5.0,
                counter_clockwise: true,
            },
            "an arc of ellipse rounded beside a trait",
        ),
    ];
    for (mut document, corner, far, curve, what) in cases {
        let cut = &document.sketches()[0].drawn_cuts()[0];
        let on_the_trait = |place: DVec2| {
            let along = (far - corner).normalize();
            ((place - corner).perp_dot(along).abs() < 1e-4).then(|| (place - corner).dot(along))
        };
        let trait_touch = cut
            .touches
            .iter()
            .find_map(|touch| on_the_trait(*touch))
            .expect("the fillet touches the trait");
        let curve_touch = touched_along(&document, corner, curve);

        let quiet = strokes_hovering(&mut document, None);
        let lit = strokes_hovering(&mut document, Some(Selection::Cut(CornerCutId(0))));

        let trait_lit = colours_of_the_dashes(&lit, trait_touch, on_the_trait);
        let trait_quiet = colours_of_the_dashes(&quiet, trait_touch, on_the_trait);
        assert!(
            !trait_lit.is_empty() && trait_lit.iter().all(|colour| !trait_quiet.contains(colour)),
            "{what}: the trait's dashes light with the cut pointed at: {trait_lit:?}"
        );
        let curve_lit =
            colours_of_the_dashes(&lit, curve_touch, |place| curve.along(corner, place));
        assert!(
            !curve_lit.is_empty(),
            "{what}: the curve's stretch up to the corner is dashed"
        );
        assert!(
            curve_lit.iter().all(|colour| trait_lit.contains(colour)),
            "{what}: the curve's dashes light with the cut pointed at, as the trait's do: \
             {curve_lit:?}, where the trait's are {trait_lit:?}"
        );
    }
}

#[test]
fn a_fillet_beside_a_curve_that_does_not_fit_is_painted_red() {
    let mut document =
        a_trait_and_an_arc_rounded(DVec2::new(2.0 + 96.0 / 13.0, 10.0 + 40.0 / 13.0), 8.0);
    let drawn = document.sketches()[0].drawn_cuts();
    assert_eq!(drawn.len(), 1, "the fillet is drawn: {drawn:?}");
    assert!(
        !drawn[0].fits,
        "a fillet of eight touches the arc past its end"
    );
    let CutCurve::Arc(fillet) = drawn[0].curve else {
        panic!("a fillet is drawn as an arc");
    };
    let radius = fillet.centre.distance(fillet.start);

    let strokes = strokes_hovering(&mut document, None);

    let on_the_fillet: Vec<[f32; 4]> = strokes
        .iter()
        .filter(|(ends, _, _)| {
            ends.iter()
                .all(|end| (end.distance(fillet.centre) - radius).abs() < 1e-4)
        })
        .map(|(_, colour, _)| *colour)
        .collect();
    let refused = tint(Theme::default().refused);
    assert!(
        !on_the_fillet.is_empty() && on_the_fillet.iter().all(|colour| *colour == refused),
        "the fillet that does not fit is painted in the colour of a refusal: {on_the_fillet:?}"
    );
}
