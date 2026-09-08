# GPU rendering (`cao_render`)

The `cao_render` crate draws the contents of the viewport with `wgpu`. It
depends on no interface library: it receives a description of the frame and
draws it. That is what will allow reusing it as it is behind a future tablet or
web front-end.

See also: [viewport](viewport.md) · [navigation](navigation.md)

## How the files are split

| File | Role |
| --- | --- |
| `camera.rs` | Orbital camera, cube faces, planes, view animation |
| `geometry.rs` | Building the vertices: axes, adaptive grid, colours |
| `cube.rs` | Geometry of the orientation cube and detection of the clicked face |
| `renderer.rs` | wgpu pipelines, buffers, uniforms, draw calls |
| `shaders/scene.wgsl` | The shaders: thick lines, solid faces, background in screen coordinates |

## The contract: `SceneFrame`

The UI layer builds a `SceneFrame` every frame (matrices, vertices, cube
rectangle in physical pixels) and hands it to the renderer. No camera or mouse
state lives in `cao_render` — nothing but drawing data.

## The pipelines and the depth buffer

Since extrusion there is a **depth buffer**, asked of the window by `eframe`
(`depth_buffer`) and whose format is decided in `cao_render`, where the
pipelines are built (`SceneRenderer::DEPTH_FORMAT`). The pipelines share the
same shader and differ mostly in what they do with that buffer:

| Pipeline | Depth |
| --- | --- |
| `solid` | tests and **writes** |
| `world_line` — axes and grid | tests without writing, pulled a hair towards the camera |
| `line`, `triangle`, `surface`, `background` | always pass, without writing |

A solid is the only geometry with a volume, and therefore the only one whose
near faces must hide its far ones: it is the only one that writes. The axes and
the grid are **hidden by the part** when it is in front, without which a red
trait crossing a block would read as an edge of that block; the nudge they get
is explained in [viewport.md](viewport.md). Everything else goes over the top
deliberately: a sketch or a dimension buried in a block would be unusable, and
the orientation cube must never be hidden by the part.

The cube is still drawn last, and it is plain back-face culling that shows only
its near faces. Its edges are emitted only for the visible faces, otherwise the
ones behind would pierce the solid.

Solids are lit **flat**, one shade per face computed once from its orientation
and baked into the vertex colour: a part made of flat faces reads better flat
than smoothed, and it avoids carrying a normal in the vertex format for
geometry without curves. The light follows the camera, so turning the part
never leaves it facing an unlit side.

## The painting order

The scene goes **first**. Everything egui paints over the viewport — the values
of the dimensions, the input fields that go with them, the scale bar, the cube
labels — is added to the same layer, in order, and the scene now fills the
whole area with its background: painted last, it erased them all.

## Thick lines

`wgpu` cannot draw a wide line: the `LineList` topology always gives 1 pixel,
illegible on a high-density screen. Each segment is therefore sent as **one
instance** and unfolded in the vertex shader into a quad of the wanted width,
in screen space.

A trick of the format: the buffer stays a plain run of vertex pairs; it is the
`VertexBufferLayout` that reads it back with a stride of two vertices per
instance. The geometry builders have nothing special to do.

Each segment is clipped in the shader against five planes — the near plane and
the four sides of a box twice as wide as the screen — **in homogeneous
coordinates, before any division by `w`**. That is the delicate point:

- dividing by a negative `w` would send the point to the other side of the
  screen;
- cutting at the near plane "as close as possible" (a minuscule `w`) gives
  screen coordinates of the order of 10⁵, and adding half a line width to them
  then changes nothing at all in `f32`: the line thins out and vanishes in full
  screen. That is exactly the bug we had on the axes, which cross the whole
  scene and therefore pass behind the camera.

Once clipped, the quad is emitted directly in screen coordinates (`w = 1`),
which keeps the thickness exact whatever the length of the line. The depth is
carried by hand: each end of the quad takes that of the end it belongs to, so a
line sinking behind the part disappears gradually rather than going out at
once.

## Colours

Colours are handled **linearly** in the shaders. `geometry::srgb` converts an
sRGB colour (the one a colour picker shows) to linear. If the destination
surface is not sRGB, the fragment shader does the reverse conversion itself —
hence the flag passed in the uniforms.

## Checking the rendering without a window

```sh
cargo run -p cao_render --example offscreen -- /tmp
```

Writes three PNGs (free view, XY plane, cube hover) by running the real GPU
pipeline. Useful for checking a rendering or comparing before and after a
change, without having to open the application.

The unit tests (`cargo test -p cao_render`) cover what an image does not check:
the grid step, the camera angles of each face and the ray cast of the cube.
