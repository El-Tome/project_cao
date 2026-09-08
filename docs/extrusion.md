# Extrusion: from the drawn area to a volume

See also: [sketch](sketch.md) · [history](history.md) ·
[rendering](render.md) · [architecture](ARCHITECTURE.md)

## How it goes

1. One draws a sketch and clicks **Terminer**. The sketch does **not** need to
   be entirely constrained: what is there is what gets extruded.
2. The bar switches by itself to the **Extrusion** category, offering the two
   tools. That is where the wish to extrude arrives, rather than having to find
   the tool afterwards.
3. One chooses **adding matter** or **taking matter away**.
4. One clicks **one to n closed areas** of the drawing. A second click on an
   area already taken removes it.
5. One gives a height in millimetres, possibly the other way round, and
   applies.
6. The view swings to a slant: seen head-on from its own plane, a prism looks
   exactly like the drawing it came from.

The two tools are the same work: they differ only in what they do with the
volume at the end.

## Straight or a revolution

The same pair of tools builds the volume in two ways:

| Shape | What one gives |
| --- | --- |
| **Straight** | A height, in millimetres. The matter goes perpendicular to the plane. |
| **Revolution** | An angle, in degrees, and an axis. The area turns around that axis. |

The axis is either one of the two axes of the sketch, or **a trait one has
drawn oneself**: in revolution mode, clicking a trait takes it as the axis. A
trait is a far smaller target than an area, so it is offered first.

A full turn closes on itself and has no ends; a partial turn is closed at both
ends by the profile itself.

The profile has to sit **entirely on one side of the axis**. Astride it, it
would pass through itself while turning, and no precaution afterwards recovers
a shape obtained that way: nothing is produced, and the application says so.

## What an area is, and why the tube works

An area is a closed outline **minus what is drawn directly inside it**. Two
circles one inside the other therefore give:

| What one clicks | The area obtained |
| --- | --- |
| The ring (the light tint) | The tube: the middle stays empty |
| The middle (the deeper tint) | The inner disc alone |

That is exactly the rule "one selects the area of the same colour": what is
tinted one shade is one area, what is tinted more strongly is another. What is
drawn **inside a hole** is matter again, and forms an area of its own.

The chosen area is filled on screen with the colour of the matter it is about
to become — green for an addition, red for a removal — holes included, so what
is shown solid is exactly what will become solid.

### Cutting up an area with a hole

A ring cannot be cut into triangles as it is: no walk leaves both the hole
empty and the outline closed. A **corridor** is therefore dug from the hole out
to the outline, and walked down one side and back up the other. The two edges
of the corridor coincide: it has no area, and the face is unchanged.

## An area is named by a point, not by its rank

The recorded operation does not keep "the second area" but **the position
clicked**. A rank would move as soon as something else is drawn in the sketch,
and the extrusion would silently start applying elsewhere. On replay, the area
is found as the one containing that point — the innermost if there are several.

It is the same principle as for the points of a trait
([history.md](history.md)): the decision is taken at the click and kept.

## Adding and taking away matter

The part is **one single volume**, not a pile of pieces: a pocket dug in a
block must really be a hole in that block.

Both operations go through a **binary space partition** (BSP): each volume
becomes a tree of planes taken from its own faces, and the faces of the other
volume are pushed into it. Each comes back labelled inside or outside, and cut
in two where the plane crosses it. Union and difference are then a matter of
keeping the right halves and turning a volume inside out.

This method works on any shape, convex or not — a pocket in a block is
precisely the case that breaks the simpler methods.

When several areas are extruded together, they are first joined into one single
tool, then applied at once: two areas extruded together must behave as one
shape.

## The direction

The matter goes along the **normal of the plane** of the sketch. The *Sens
inverse* box pushes it the other way; that is often what is wanted for digging,
since the matter is not always on the side the plane points at.

A removal that meets nothing says so, instead of letting one believe the tool
is broken.

## The scale

The height is given in millimetres, like the dimensions. It is converted into
world units with the scale of the document, so an extrusion of 25 mm stays
25 mm even if a dimension redefines the scale afterwards… except that the
volume is rebuilt by replaying the history, so it is the scale **at the time of
the replay** that applies.

## Sketching on a face of the part

Once there is matter, **its flat faces are sketch planes**. Choosing a plane
offers them first where they are, and the three origin planes stay available
everywhere else — they fade visually so as not to hide the part.

The face on top wins over the three planes rather than "the nearest to the
camera": the origin planes are infinite sheets crossing the part, and the
nearest would almost always be one of them.

Every face sharing the same plane lights up together: a curved surface and a
cut surface are both stored in several flat pieces, and lighting only one of
them would read as choosing a fragment.

The origin of the sketch falls where the origin of the world projects onto the
face, and the view settles on the spot clicked. A sketch laid on a face is
recorded with **its complete plane**, not with a reference to the face: if the
part changes afterwards, the drawing stays where it was made rather than
following a face that may no longer exist.

## Why the application used to close

A case met in use: a cylinder of revolution, then a pocket dug into it from its
own face — the application closed all at once, with no message, and not always.

The cause was in the partition of space. It sorts the faces by the plane they
lie on, and the first plane comes from a face taken at random: that face is on
that plane by definition. Except that in `f32` — what the core used at the
time — thirty units from the origin, the dot product already carries a few
millionths of error, more than the tolerance in use. A triangle therefore came
back **on both sides of its own plane**, was cut in two, and each half started
again: an endless cutting, which filled the stack and stopped the program.

Three things changed:

- the tolerance is **relative** to the distance to the origin, and no longer
  fixed;
- the face that gave the plane is set aside instead of being sorted, which
  guarantees that at each round there are strictly fewer faces left to place;
- the walk of the tree no longer goes through the call stack at all — the nodes
  live in an array and name each other by position. A part made of hundreds of
  facets gives a tree in a chain, and a recursive descent into it ends up
  overflowing even with no error of computation.

The exact case, with its real measurements, is a test.

## What is still missing

- No "up to the next face" and no "through everything": only a height, or an
  angle, given.
- No draft and no sweep along a curve.
- A sketch laid on a face does not follow that face if the part changes: it
  stays on the plane where it was made.
- An extrusion cannot be edited afterwards: one has to go back in the history
  and do it again.
- The mesh is not exported (no STL/STEP).
- Two exactly coplanar faces can still leave shards of surface. Moving the core
  to `f64` brought the coplanarity tolerance from a millionth to a billionth,
  so there are a thousand times fewer, but the case is not handled for itself.
