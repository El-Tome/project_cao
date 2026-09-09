# The 3D viewport

What the user sees when a part is open: a 3D space with the X/Y/Z axes, an
orientation cube in a corner, and a grid when one is settled on a plane.

See also: [GPU rendering](render.md) · [navigation](navigation.md) ·
[configuration](configuration.md)

## The two modes

The viewport has exactly two states, described by `ViewMode`
(`crates/app/src/screens/viewport/mod.rs`). The camera and the gestures that
change it are handled there and in `input.rs`; `render.rs` pushes the axes,
the grid and the cube it draws below to the GPU. What the canvas draws *of the
sketch* — points, traits, dimensions — is decided by `cao_sketch` and only
carried here; see [sketch.md](sketch.md) for that half.

| Mode | What is shown | How one gets in |
| --- | --- | --- |
| `Free` | The 3 coloured axes only | By orbiting (as soon as the view turns) |
| `Plane(plane)` | The axes **and** the grid of the plane | By clicking a **face** of the orientation cube |

The rule is deliberately simple: orbiting necessarily leaves plane mode, since
the view is then no longer aligned on a plane. Pan and zoom, on the other hand,
keep the mode: framing or zooming on a plane is a normal gesture.

The grid only appears once the view has **settled** on the plane, not during
the animation: halfway the view is slanted, and a grid of finite size seen
edge-on reads as a disc floating in the middle of the screen.

The cube is clicked on three kinds of zone, cut like a 3×3 grid on each face:

| Zone clicked | View obtained | Mode |
| --- | --- | --- |
| **Face** (centre) | Straight view onto the plane | Grid |
| **Edge** (border) | View at 45° between two faces | Lines |
| **Corner** | Isometric view | Lines |

Only a face corresponds to a work plane: an edge or corner view is slanted, so
by definition aligned on no plane — it stays in line mode. Hovering an edge or
a corner highlights it on all the faces it touches at once.

| Face clicked | View | Grid plane |
| --- | --- | --- |
| DESSUS / DESSOUS | ±Z | XY |
| FACE / ARRIÈRE | ∓Y | XZ |
| DROITE / GAUCHE | ±X | YZ |

The move to the view is animated (~0.35 s, see `ViewTransition`) so one
understands how the part has turned rather than suffering a jump.

## The adaptive grid

The grid step follows the sequence 1 – 2 – 5 – 10: it is the smallest step
whose spacing on screen stays above `grid_pixel_spacing` (48 px by default).
Zooming in, a graduation of 10 becomes 5, then 2, then 1; zooming out, the
reverse. One line in ten is heavier.

The grid is centred on the camera target (rounded to the step) and not on the
origin, so that it follows the pan without ever stopping dead; its alpha falls
off with the distance to the centre, which avoids a hard edge. Lines that would
fall exactly on an axis are skipped, otherwise they would double the coloured
line of the axis.

## The orientation cube

The cube turns with the camera and therefore shows how one is looking at the
part. Its 6 faces carry a label (DESSUS, FACE, DROITE…) drawn by egui and not
by the GPU: showing text would want a font atlas on the rendering side, where
egui already has one.

Its position is configurable (`cube_corner`, top-right corner by default), as
are its size and its margin. Hovering highlights the zone aimed at.

Detecting the hovered zone is a ray cast on the CPU (`cube::pick_zone`), not a
GPU pixel read: the cube is axis-aligned in orthographic projection, so the
intersection fits in a few lines of arithmetic and stays in step with the
display.

## The ruler (scale bar)

Bottom left, a bar exactly one grid square long, with its value ("10 mm"). It
answers two questions at a glance: how big a square is, and how fast one is
zooming — the value changes by jumping from 1 to 2, 5, 10, which makes the zoom
legible.

The unit adapts to avoid endless numbers: µm, mm, m then km according to the
scale, so "50 m" and not "50000 mm". It can be pinned to a precise unit
(`unit: Fixed(…)`).

The step is chosen **in millimetres**, then converted into world units to draw
the grid. The reverse would be the natural way round, but wrong: once the first
dimension is placed, a world unit is no longer worth a millimetre, and choosing
the step in units put the ruler out by exactly that factor — the constant,
proportional error one used to see.

Its corner is configurable (`ruler_corner`), and it can be hidden
(`ruler_visible`).

## The axis pointing at us

In plane mode, the axis perpendicular to the plane is not drawn: seen head on
it comes down to a dot in the middle of the drawing, which reads as a smudge
and not as an axis.

## Frame and units

**Z up** convention (the usual one in mechanical CAD): the XY plane is the
"ground" plane, seen from above.

What a world unit is worth in millimetres is a property of the document
(`millimeters_per_unit`), **undefined as long as no dimension has been
placed**: it is then worth one millimetre, and the ruler shows mm. The first
dimension defines it — saying a trait is 100 mm, 5 m or 5 mm deforms nothing,
it is the scale of the document that is redefined and the picture does not
move. The following ones are ordinary constraints. See [sketch.md](sketch.md).

## The axes behind the part

The X, Y and Z axes and the grid are **hidden by the part** when it is in
front: without that, a red trait crossing a volume reads as an edge of that
volume, and one no longer knows what one is looking at.

They keep the depth test without writing to it, and are pulled a hair towards
the camera: a grid drawn on a face of the part is exactly as far away as that
face, and without the nudge the two would fight over every pixel. The nudge has
no slope term — where a face is seen edge-on its depth changes enormously from
pixel to pixel, and a slope-scaled nudge there would be enough to drag the
trait right through.

The sketch being edited, its dimensions and the orientation cube stay visible
throughout, on the contrary: a dimension buried in a block would be unusable.
