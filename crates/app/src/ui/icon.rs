//! The redesign's icons, embedded at build time from `docs/redesign/icons/`,
//! where they are drawn: the files there are the only copy.

/// One of the redesign's icons: the stem of the file it is drawn in, and that
/// file's SVG.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Icon {
    id: &'static str,
    svg: &'static [u8],
}

/// The sizes `docs/redesign/icons.md` draws an icon at.
// Only the ribbon draws an icon yet: the panel toggles and the menus of #725
// take the two others, and `expect` then asks for this to go.
#[cfg_attr(not(test), expect(dead_code))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconSize {
    /// A large ribbon button.
    Large,
    /// A small button, or a line of the tree.
    Small,
    /// A row of a menu or a list.
    Row,
}

impl IconSize {
    pub fn points(self) -> f32 {
        match self {
            Self::Large => 22.0,
            Self::Small => 15.0,
            Self::Row => 14.0,
        }
    }
}

impl Icon {
    pub fn id(&self) -> &str {
        self.id
    }

    pub fn svg(&self) -> &[u8] {
        self.svg
    }

    /// The icon at `size`, rasterised from its SVG at as many pixels as it
    /// covers on the screen.
    pub fn image(self, size: IconSize) -> egui::Image<'static> {
        egui::Image::new(self.uri()).fit_to_exact_size(egui::Vec2::splat(size.points()))
    }

    fn uri(self) -> String {
        format!("bytes://icon/{}.svg", self.id())
    }
}

/// Teaches `ctx` to draw every icon.
///
/// Each is handed over drawn in white rather than in `currentColor`, which the
/// rasteriser paints black: a widget's tint multiplies the picture, and only
/// white comes out in the colour of its text.
pub fn install(ctx: &egui::Context) {
    egui_extras::install_image_loaders(ctx);
    for icon in ALL {
        let white = String::from_utf8_lossy(icon.svg()).replace("currentColor", "white");
        ctx.include_bytes(icon.uri(), white.into_bytes());
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

        const ALL: &[Icon] = &[$($name),+];
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
    MATTER_IN_FRONT_OPAQUE = "matter-in-front-opaque";
    MATTER_IN_FRONT_SEE_THROUGH = "matter-in-front-see-through";
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

#[cfg(test)]
mod tests;
