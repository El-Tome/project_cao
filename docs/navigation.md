# Moving around the view

See also: [viewport](viewport.md) · [configuration](configuration.md)

## With a mouse (Fusion 360 habits)

| Gesture | Action |
| --- | --- |
| Wheel | Zoom |
| Middle click + drag | Pan |
| Shift + middle click + drag | Orbit |
| Click on the cube | See [viewport](viewport.md) |

## On a trackpad

| Gesture | Action |
| --- | --- |
| Two fingers | Pan |
| Shift + two fingers | Orbit |
| Pinch | Zoom |
| Alt + left click + drag | Orbit |
| Alt + Shift + left click + drag | Pan |

A mouse wheel and a two-finger scroll arrive in the same stream of events; they
are told apart by their unit (lines for a wheel, pixels for a trackpad), which
is what keeps "wheel = zoom" without the trackpad zooming at full tilt.

That is also why the wheel has a **sensitivity of its own**
(`wheel_zoom_sensitivity`): one notch of a wheel is worth *one line*, where a
trackpad gesture is worth hundreds of *pixels*. Sharing a single setting was
bound to make one of the two unusable — the wheel crept half a percent per
notch while Ctrl + wheel leapt.

Ctrl (or Cmd) + wheel and Ctrl + two fingers zoom too, at the same speed as the
gesture without the modifier: it is no longer a "turbo" shortcut.

The trackpad gestures are configurable (`TrackpadConfig`): each of the two
scrolls can be `Pan`, `Orbit`, `Zoom` or `Ignore`.

## Other habits

`NavigationPreset` also offers `SolidWorks` (middle click = orbit, Ctrl +
middle click = pan) and `Blender` (middle click = orbit, Shift + middle click =
pan). The preset is a field of the viewport configuration, and the settings
screen offers the three as buttons, under "Habitudes".

## How the camera behaves

The camera is **orbital**: it turns around a target point, with a yaw, a pitch
and a distance. It can therefore never roll, and the pitch is bounded to ±90°.

This parametrisation avoids a classic trap: with a `look_at` matrix and a fixed
"up" vector, looking straight down is a degenerate position that flips the
image. Here the top view is one angle like any other.

- **Zoom**: exponential, so one notch of the wheel has the same visual effect
  whether one is 1 mm or 10 m from the part. The near and far planes follow the
  distance, which keeps depth precision usable at every scale. The distance is
  bounded (`min_distance`, `max_distance`, 1 µm to 1 000 km by default): a bound
  has to exist, the camera and the rendering working in `f32`, but it is
  configurable and far enough out to frame a whole assembly.
- **Pan**: converted into world units according to the distance, so the part
  follows the cursor exactly.
- **Orbit**: always returns to the free 3D view (see [viewport](viewport.md)).

An orbit or a pan begun on the canvas carries on even if the cursor leaves it,
as in every CAD program.

## What does not exist yet

Touch and stylus are not handled: that is meant for the tablet port, and it
will want a real set of gestures (pinch to zoom, two fingers to orbit) on top
of the mouse bindings described here.
