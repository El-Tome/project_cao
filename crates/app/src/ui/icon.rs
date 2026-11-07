//! The redesign's icons, embedded at build time from `docs/redesign/icons/`,
//! where they are drawn: the files there are the only copy.

/// One of the redesign's icons: the stem of the file it is drawn in, and that
/// file's SVG.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Icon {
    id: &'static str,
    svg: &'static [u8],
}

// No button draws an icon yet: #725 wires them, and `expect` then asks for this to go.
#[cfg_attr(not(test), expect(dead_code))]
impl Icon {
    pub fn id(&self) -> &str {
        self.id
    }

    pub fn svg(&self) -> &[u8] {
        self.svg
    }
}

macro_rules! embedded {
    ($($name:ident = $id:literal;)+) => {
        $(
            pub const $name: Icon = Icon {
                id: $id,
                svg: include_bytes!(concat!("../../../../docs/redesign/icons/", $id, ".svg")),
            };
        )+
    };
}

embedded! {
    ARC_BY_CENTRE = "arc-by-centre";
    ARC_BY_ENDS = "arc-by-ends";
    CHAMFER = "chamfer";
    CHAMFER_ANGLED = "chamfer-angled";
    CHAMFER_EQUAL = "chamfer-equal";
    CHAMFER_SIDED = "chamfer-sided";
    CIRCLE_CENTRE = "circle-centre";
    CIRCLE_THREE_POINTS = "circle-three-points";
    CIRCLE_THREE_TANGENTS = "circle-three-tangents";
    CIRCLE_TWO_POINTS = "circle-two-points";
    CIRCLE_TWO_TANGENTS = "circle-two-tangents";
    CIRCULAR_PATTERN = "circular-pattern";
    COINCIDENT = "coincident";
    COLLINEAR = "collinear";
    COMPACT_HISTORY = "compact-history";
    CONCENTRIC = "concentric";
    CONSTRUCTION = "construction";
    DIMENSION = "dimension";
    DIMENSION_ANGLE = "dimension-angle";
    DIMENSION_AUTO = "dimension-auto";
    DIMENSION_LENGTH = "dimension-length";
    DIMENSION_POINT_TO_POINT = "dimension-point-to-point";
    DIMENSION_RADIUS = "dimension-radius";
    DOCK_TOOLBAR = "dock-toolbar";
    ELLIPSE_BY_CENTRE = "ellipse-by-centre";
    ELLIPSE_BY_ENDS = "ellipse-by-ends";
    EQUAL = "equal";
    EXPORT_STEP = "export-step";
    EXTRUSION_ADD = "extrusion-add";
    EXTRUSION_APPLY = "extrusion-apply";
    EXTRUSION_CANCEL = "extrusion-cancel";
    EXTRUSION_CUT = "extrusion-cut";
    EXTRUSION_STRAIGHT = "extrusion-straight";
    FILES = "files";
    FILLET = "fillet";
    FINISH = "finish";
    FIXED = "fixed";
    HISTORY = "history";
    HOME = "home";
    LINE = "line";
    LINE_SYMMETRIC = "line-symmetric";
    MATTER_IN_FRONT_HIDDEN = "matter-in-front-hidden";
    MEASURE = "measure";
    MIDPOINT = "midpoint";
    MIRROR = "mirror";
    NEW_SKETCH = "new-sketch";
    PARALLEL = "parallel";
    PERPENDICULAR = "perpendicular";
    POINT = "point";
    PREFERENCES = "preferences";
    RECTANGLE = "rectangle";
    RECTANGULAR_PATTERN = "rectangular-pattern";
    REDO = "redo";
    REFRAME = "reframe";
    REVOLUTION = "revolution";
    SECTION_VIEW = "section-view";
    SELECT = "select";
    SPLIT = "split";
    STL_COARSE = "stl-coarse";
    STL_FINE = "stl-fine";
    STL_NORMAL = "stl-normal";
    SWAP_CHAMFER_SIDES = "swap-chamfer-sides";
    TANGENT = "tangent";
    TRIM = "trim";
    UNDO = "undo";
}
