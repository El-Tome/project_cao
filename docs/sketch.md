# Sketch mode

Drawing in 2D on a plane, before going to 3D. This is the first brick of the
sketch → extrusion cycle.

See also: [viewport](viewport.md) · [architecture](ARCHITECTURE.md)

## How it goes

1. **Esquisse → Nouvelle esquisse**. The available planes appear as translucent
   squares in the view.
2. **Click a plane.** The camera swings round to face it and frames the working
   area, the grid comes up. Any plane works, a slanted one included: the view
   faces its normal whatever it is.
3. **Draw** with the tools below. The shape to come is drawn continuously up to
   the cursor and follows the snapping, so where it will land is visible before
   the click.
4. **Dimension.** Click what is to be measured, then type the value.
5. **Recadrer** puts the view exactly back facing the plane and frames the
   drawing. That is the button to use after orbiting to look behind.

### What one draws on

The three origin planes, and — as soon as there is matter — **any flat face of
the part**. Faces come in front of the three planes where they are, those
staying available everywhere else. See [extrusion.md](extrusion.md).

## The tools

| Tool | Gesture |
| --- | --- |
| **Selection** | Click to take, click-drag a point to move it, drag from empty space to box in. |
| **Line** | Successive clicks, each trait carrying on from the last. |
| **Rectangle** | Two clicks: two opposite corners. |
| **Circle** | Five ways to lay it down, see below. |
| **Point** | One click lays a lone point. |
| **Dimension** | Two clicks: what is measured, then where the annotation sits. |

### Taking several things at once

Dragging **from empty space** pulls a box, as on a desktop, and takes
everything it holds **entirely**: a trait counts when both its ends are in.
Half a trait cannot be deleted, so letting the box take it would promise
something the drawing cannot do.

Who answers the drag is decided **when the gesture starts** and stays so to the
end: a point under the cursor at press time moves, otherwise it is a box.
Without that the gesture would change nature halfway, the moment the cursor
passed over a point.

`Cmd`/`Ctrl` (or `Shift`) while clicking **adds or removes** one element at a
time, and works with the box too. That is what lets three traits be named that
no single frame can enclose on its own.

`Suppr` erases everything held **in a single history step**: a deleted
selection is one gesture, and one undo brings it back.

### `Échap` steps back one notch

One press gives up what is under way: the chain of traits, the first corner of
a shape, the dimension looking for its place. A second press — when there is
nothing left to give up — **returns to the Selection tool**.

A tool still in hand once its work is done is a tool that draws a stray trait
on the next click.

Line, rectangle and circle **reuse the points already there** when the cursor
is over one: shapes hold on to each other instead of piling points at the same
spot. Nothing ever requires laying those points first — the Point tool is
there for when one wants them explicitly.

The Selection tool also moves the **dimensions themselves**: grabbing an
annotation shifts it, which is how it is taken out of the way. The offset is
saved with the dimension.

Moving a point with the Selection tool does not break the dimensions already
placed: the drawing settles around it. The move is recorded only on
**release** — during the drag the point simply follows the cursor, which keeps
the history from filling with thousands of entries saying the same thing.

## Drawing to a value

While a shape is being drawn, two fields follow the cursor: the **length** and
the **angle with the horizontal** for a trait, the **width** and the **height**
for a rectangle, the **diameter** for a circle.

They are pinned to the pointer, bottom right: laid on the drawing they ended up
under the cursor, and a cursor on the fields is no longer a cursor on the
canvas — the shape stopped following it.

The first field **takes the keyboard as soon as it appears**, whole value
selected: one types, without a single Tab. Tab moves to the second. A field
left alone stays empty and shows the measurement as a ghost — keeping the
measurement *in* the field meant the first keystroke landed behind it, and "40"
typed over "0.000" read as 0.00040.

Left alone, they are plain readouts. **Typed into, they are decisions**: the
trait can take no other value, and the matching dimension is placed by itself
when the trait is validated. A trait drawn to a value does not have to be
measured afterwards.

Fixing one of the two leaves the other free, which is the whole point:

| What is typed | What stays free |
| --- | --- |
| An angle | The length: the trait grows and shrinks along that direction |
| A length | The direction: the trait turns at that distance |
| Both | Nothing; the click only validates |

A rectangle works the same way, side by side: a typed width fixes the width and
lets the height follow the cursor. Both sizes then arrive as dimensions on the
shape, along with its right angles. A circle takes its diameter the same way,
and a size too small to reach the points clicked is held at the smallest that
does reach them: typing 150 goes through 1 and 15 on the way, and a circle that
vanishes at the first keystroke takes with it the field being typed into.

Emptying a field takes the decision back. `Entrée` validates the shape without
having to find the canvas again with the mouse.

The sign follows the cursor: 30° typed means the 30° one is pointing at, not
the ones underneath. And as long as a value is fixed, a neighbouring point is
no longer snapped to — that would quietly give the trait another length.

## Right angles place themselves

Drawing on from a trait, coming within 4° of the perpendicular **sets the trait
at exactly 90°**, shows the little square of technical drawing in the corner,
and places the angle constraint on validation.

The square is shown **before** validating: a constraint that appears without
warning is a nasty surprise. The band is wide enough to be easy to aim at,
narrow enough not to steal an angle genuinely meant to be 80°.

## The smart dimension

One single tool, which measures what it is shown:

| What is clicked | What one gets |
| --- | --- |
| A trait | Its length, its width or its height, depending on where the dimension is placed |
| Two points | The distance between them, joined or not |
| Two traits that touch | The angle between them |
| A trait then a point | The distance from the point to the line, taken square |
| A point then a trait | The same, the other way round |
| A trait then a sketch axis | The angle with that direction |
| An axis then a trait | The same, the other way round |
| A circle | Its **diameter** |
| A circle then its centre | Its radius |

The first click takes an entity and **already shows what it measures on its
own** — the length of a trait. Clicking a second entity before placing the
dimension transforms it: another trait makes it an angle, a point makes it a
distance to the line. That is what one expects of a dimension called smart, and
it saves having to name the kind beforehand.

A dimension from a point to a line reads as if a perpendicular segment came
down from the point to the line. It really is the **line** that is measured,
not the drawn stretch of trait: when the foot falls beyond the end, a thin
trait extends the segment out to it, as on a drawing.

A point beats a trait under the same cursor: it is the smallest target, so
aiming at it is a deliberate act.

### A circle gives its diameter

A plain click on a circle takes its **diameter**: that is the size a hole is
drilled to and the size a round bar is turned to. The radius is asked for on
purpose, by clicking the **centre** next — the only thing a centre can add to a
circle already taken.

### One clicks what is measured, then where the dimension sits

The first click takes the geometry; the annotation **then follows the cursor**,
value included, until the second click puts it there. A dimension dropped by
default over the shape it measures has to be moved out of the way by hand
anyway: better that it arrive where it belongs.

The dimension is placed with the value the geometry already measures, so
**placing a dimension never deforms anything**. It is by typing another value
that the drawing is moved.

### A slanted trait reads three ways

Its length, its width or its height. Which one is chosen simply depends on
**where the dimension is placed**, the two ends of the trait bounding a box:

| Where the cursor goes | What is dimensioned |
| --- | --- |
| Above or below the box | The **width** — horizontal gap |
| Left or right | The **height** — vertical gap |
| Inside the box, or past a corner | The **length**, diagonally |

The preview shows which one before the click. A width dimension is drawn
horizontally, its two extension lines each coming down from its own end: they
are therefore of unequal length, as on a drawing.

All three can live together on one trait — width and height together fix it
completely, and the solver holds them separately: a width leaves the trait free
to slide vertically.

Only a trait **square on an axis** — 0, 90, 180, 270° — is not offered the
choice: its width *is* its length, and two names for one measurement is one
name too many. Everything else, however slightly slanted, has it.

### One measurement, one dimension

Clicking again on what is already dimensioned **reopens the existing
dimension** instead of laying a second one over it. Clicking an annotation
directly does the same, with the Dimension tool as with the Selection tool:
that is the obvious gesture for changing a number already under the cursor.

The two ways of naming one measurement — two traits one way or the other, two
points one way or the other — are brought back to one before being recorded.
Without that, the same dimension would exist twice, in two superimposed copies.

Validating a value that is already the one in force does **nothing**: pressing
✔ twice must not leave two identical steps in the history.

### Where a dimension stands

What is recorded is the position of the annotation **in drawing units**, not in
pixels. A dimension placed somewhere stays there: the old way, an offset in
pixels brought every annotation back onto the shape as soon as one zoomed out.
The automatic dimensions of the trait and the rectangle are frozen the same way
when they are placed.

Only the size of what must stay legible — the text, the arrowheads — goes on
counting in pixels.

### The little reminder trait

A value pulled off to the side, past the ends of the dimension, has nothing
left to say what it belongs to. A **reminder trait** then extends the dimension
line out under the number. Same for an angle whose value has left the opening
of the two traits.

**A trait lying on an axis** is selected by clicking it twice: the first click
takes the trait, the second — which necessarily lands on the same trait — is
read as "and now the axis it rests on". Without that a rectangle drawn along
the axes could never be constrained.

When two things overlap and the wrong one wins, the kind can be forced: the
commands **Cote intelligente**, **Cote point à point**, **Cote de trait**,
**Cote d'angle** and **Cote de rayon** each pin the dimension tool to one
reading. The standard toolbar carries no row for them — they are reached by a
shortcut, or by adding them to the bar from the palette
([configuration.md](configuration.md)).

## Circles

The **Cercles** menu, in the Dessin row, offers five ways:

| Way | What is clicked |
| --- | --- |
| **Centre and diameter** | The centre, then a point on the rim |
| **Two points on the rim** | Two opposite points; the centre is between them |
| **Two points then the centre** | Two points on the rim, then the centre |
| **Tangent to two lines** | Two traits, then the centre |
| **Tangent to three lines** | Three traits: nothing left to choose |

The two ways that end with the centre do not take it where one clicks: a centre
equidistant from two points can only be on their **perpendicular bisector**,
and a centre equidistant from two lines only on their **bisector**. The click
is brought onto it — the user says roughly where, the geometry says exactly
where.

A circle laid against traits **stays against them**: the tangency is recorded
as a constraint, since that is the whole point of having named them.

### The places clicked become points

A circle drawn through points keeps those points: they are points of the
drawing like any other, held on the rim. That is what gives a circle
**handles**, having none but its centre — they can be grabbed, measured from,
snapped to.

- **Pulling a rim handle grows the circle**: the radius is an unknown of the
  solver, and the point held on the rim moves it.
- **Pulling the centre moves the whole circle**, handles and all.

A tangency likewise carries its **contact point**. It is not free: it is on the
line, and square under the centre. Without that second half it would slide
along the line — sliding a point along a circle it touches changes nothing at
all to first order, so the solver would have nothing to correct.

That point is what one grabs to **slide a circle along the line it touches**,
without breaking the tangency.

## The constraints

A dimension says **how much**; a constraint says **how**. Both take freedom
away from the drawing and count the same when it comes to knowing what is still
free — they are kept apart only because one carries a value the user types and
the other does not.

The **Contraintes** menu, in the Dessin row, offers nine:

| Rule | What is clicked | What it holds |
| --- | --- | --- |
| **Perpendicular** | Two traits | They stay square |
| **Parallel** | Two traits | They keep the same direction |
| **Equal** | Two traits, or two circles | The second takes the size of the first |
| **Coincident** | A point and a trait, or two points | The point stays on the line; two points become one; a point laid on a circle stays on its rim |
| **Collinear** | Two traits, or a trait and an axis of the frame | They rest on the same line |
| **Tangent** | A circle and a trait | The trait grazes the circle, and the contact point is placed |
| **Midpoint** | A point and a trait | The point stays halfway along |
| **Fixed** | Anything: a point, a trait, a circle | It no longer moves from its place (its size is not fixed for all that) |
| **Concentric** | Two circles | They share one single centre |

The order of the clicks is free: a point and a trait make the same coincidence
either way round. What counts is **what** was clicked, so the rule is built
from the types gathered and not from their order.

**One exception: equality.** The first trait clicked is the one whose length
suits; the second comes to take it. A rule that moved both would leave neither
at the size asked for. A corner the two share does not move either, otherwise
stretching the second would twist the first.

What is **fixed** is drawn in a colour of its own, configurable like the
others: what no longer moves must read at a glance, not be guessed at by trying
to move it. Fixing a trait holds both its ends; fixing a circle holds its
centre and that alone, its radius staying free.

Two of them are not rules but **merges**: two points brought to coincide, and
two circles brought onto a single centre. Holding them at zero distance by an
equation would leave two superimposed points for ever — exactly what the
drawing does not want.

### How they are held

Each becomes one more equation in the same system as the dimensions:
perpendicularity and parallelism are a dot or cross product to cancel, equality
a difference of lengths, coincidence a distance to a line, the midpoint two
equations — being in the middle is two assertions, not one.

**Fixed** falls outside the solver: it goes through the pins, a fixed point
simply having nowhere to go, like the origin point.

### The size of a circle is an unknown like any other

The system counts two unknowns per point **and one per circle**. A circle held
against a trait gives on its size as readily as on its place, and a rule that
could only move it would have to be broken to grow it.

That is what makes three things work at once:

- pulling a corner of a triangle **grows its inscribed circle** instead of
  leaving it askew;
- changing the size of a tangent circle **slides it** so it goes on touching,
  instead of waiting for the next gesture;
- dimensioning the distance from the centre to the line is **a real dimension**,
  which drives the drawing. As long as the radius was not an unknown, the rank
  analysis saw it as identical to the tangency and laid it down read-only.

Radius, diameter and equal radii have therefore become ordinary equations, and
the two passes that caught them by hand have gone.

A rule disappears by itself when what it talks about is deleted.

### The marks

Each rule writes its mark (`|_`, `//`, `=`, `+`, `--`, `T`, `1/2`, `X`) **on
each of the things it holds**: pointing at one of them says what it is caught
by. A parallelism therefore places one on each trait, and not a single one
between the two.

The right angle is the exception: its mark goes **in the corner**, the only
place where it reads as an angle rather than as a note about two traits. A
tangency likewise goes **to the contact point**: the three tangencies of an
inscribed circle would otherwise all land in the same place.

Several rules can hold the same spot — a midpoint and a perpendicularity, for
instance. Marks that would land on one another are therefore **spread side by
side**.

They follow the drawing **during** a move, like the values of dimensions: read
off the recorded drawing, they lagged behind and only caught up on release.

The symbols of technical drawing — ⊥, ∥, ½ — are not in the fonts shipped with
the interface, and a mark that comes out as an empty box says less than
nothing.

## Snapping

The cursor is pulled, in this order:

| What pulls | Why it comes first |
| --- | --- |
| **An existing point** | It is what one aims at most often, and missing by a hair leaves geometry that only looks joined from afar |
| **The middle of a trait** | One aims at it on purpose, and nothing on screen says one is exactly halfway: a **little triangle** announces it |
| **The body of a trait** | Drawing on a trait already there is far more common than drawing beside it |
| **The grid** | The safety net, with the shortest reach |

A trait already drawn therefore pulls **harder than the grid**: its reach is
configurable separately ([configuration.md](configuration.md)).

The reach of a **click** (what is grabbed, what is dimensioned) is 18 physical
pixels, that is 9 points on a high-density screen. At ten, a point had to be
aimed at within four points: far finer than anyone aims.

### Two superimposed vertices make one

Dropping a point on another **merges** them: everything that pointed at the one
that goes now points at the one that stays, dimensions included. Two ends laid
on one another are one corner, not two — without which the outline looks closed
without being so, and nothing extrudes.

A trait whose two ends become the same point goes away: it has neither length
nor direction left. And the origin point is never the one that gives way.

The decision is taken on release and **recorded**, like the snapping: the
distance that counts depends on the zoom of the moment, so making it again on
replay could join another pair, or none.

The grid magnet bites on quarter squares: aiming roughly is enough to land on
the origin. No magnet bites beyond a few pixels, so a deliberately free
position stays possible.

Careful: **snapping is not constraining**. A trait laid nicely horizontal
thanks to the grid stays free to turn as long as no angle dimension holds it.

A rectangle is **one single operation** in the history, not four traits: that
is what one wants to see when reading the construction back.

### A rectangle arrives dimensioned

Drawing it and then having to say four times that its corners are square is
drudgery: that is what a rectangle *is*. It therefore receives on its own
**three right angles** — the fourth follows — and **a length on two adjacent
sides**, which fixes it exactly.

A value that would add nothing is left out, as for the trait. And since the
shape is at once entirely constrained, a dimension placed on it afterwards is
read-only: to change a size, one retypes the dimension already there.

## Deleting

With the **Selection** tool, clicking a trait, a point, a circle or a dimension
highlights it; `Suppr` or `Backspace` erases it. The smallest wins: a point
before a trait before a circle before a dimension, since the smaller the
target, the harder it is to aim at on purpose.

Deleting **takes with it whatever leaned on it**. A trait without its point is
not geometry, and a dimension measuring what is no longer there accounts for
nothing. The origin point does not erase: it is what everything else is
measured from.

### Why nothing is really removed

What is deleted is **marked**, not taken out of the list. A trait removed from
the middle would shift the rank of every one after it, and each dimension
already recorded against those ranks would quietly start naming another piece
of the drawing.

That is also what makes a deletion replay and undo like any other step
([history.md](history.md)).

## Reopening a sketch

The History panel shows an ✏ **Modifier** button under each sketch. It reopens
it to add traits, even after clicking "Terminer" or closing the part. The view
puts itself back facing the plane and frames the existing drawing.

The planes offered are the three origin planes (XY, XZ, YZ) **and every flat
face of the part**, as soon as there is matter. A face in front of a plane
takes it, whichever is nearer the camera winning, without ever making the
planes unreachable. The selection code does not care which it is: it tests a
ray against a `WorkPlane`, and a face of the part is one.

## Everything shows before it is placed

Every tool shows what a click would do, before doing it: the trait follows the
cursor, the rectangle and the circle draw themselves lightly, the point has its
marker, and **the smart dimension traces the annotation it would place** — in
the right spot, with its arrows and extension lines.

The preview of the dimension is computed by **the same reading of the cursor**
as the placing itself. Two separate readings would end up diverging, and a
preview that lies is worse than no preview at all.

### While a point is being moved

The point held under the cursor **does not give way**: the drawing settles
*around* it. The solver pins it for the length of the gesture, exactly like the
origin point. Without that the constraints pull it partly back, the shape comes
out from under the cursor — which is what made a tangent circle so tiresome to
move.

The drawing is shown **as it will settle** if released there: the solver runs
every frame, and the values already given pull the rest of the shape along with
the point. Before, only the point followed the cursor while the rest stood
still; the shape looked torn, and nothing showed where it was going to land.

The dimensions follow: their lines, their arrows **and their values** are read
off that same settling drawing, without which the numbers would lag behind
while the lines they belong to moved away.

Nothing is recorded for all that: the history receives one single operation, on
release.

### When the gesture is impossible

Holding the point is not always within the drawing's reach: a corner pulled
where no tangency can follow it, for instance. The values already given then
win over the cursor — everything comes back into place and settles the ordinary
way, the point going as far as the drawing lets it.

And if even that crushes a trait until nothing is left of it, the gesture is
**refused**: the point does not go there. A trait of zero length is not
geometry, and its equations can no longer even be written — the system would
declare itself satisfied while the drawing had fallen apart.

### Moving a whole figure in one block

A drag that starts **on something already selected** carries the whole
selection, as a desktop moves a group of icons. Every named point advances by
the same step: the shape is carried, never stretched, and the rest of the
drawing settles around it. One single line in the history for the whole block.

A drag that starts elsewhere stays what it was: a point under the cursor, or a
selection box.

## What dimensions draw

A dimension is not just a number laid beside the drawing: it is **traced**,
with its extension lines, its dimension line and its arrows for a length, an
arrowed arc for an angle, an arrowed radius for a circle. The value is written
on the tracing.

This is vector drawing produced by the code, not images: a few segments per
dimension, which follow the geometry when it moves and stay crisp at any zoom.
An image would have to be redone for every value and every angle.

A read-only dimension is traced more discreetly, in grey: it reports, it does
not decide.

### Moving a dimension

With the Selection tool, grabbing a dimension shifts it, and the offset is
recorded with it. It is **the whole annotation** that moves — the line, its
arrows and its value together.

The annotation follows the cursor throughout the gesture, while nothing is
recorded before release: without that it would stand still and jump at the end,
and the move would look as though it had done nothing.

Grabbing a dimension means knowing where it is drawn: it stands a fixed number
of **pixels** from what it measures, so looking for it at another scale than
the screen's puts it where it is not — and it becomes impossible to grab.

A length dimension only moves away **perpendicularly** to what it measures: the
part of the move along the trait is discarded. The dimension line therefore
stays parallel to what it measures, with its two extension lines perpendicular
and of equal length. Otherwise it is a pair of arrows askew, which no longer
reads as a measurement. Only the value can still slide along the line, which
lets two dimensions in the same direction stop overlapping.

An angle stays hooked to the corner it measures: pulling it opens its arc
instead of tearing it off. A radius turns around its circle.

## The colours: where the drawing stands

| Colour | What it means |
| --- | --- |
| **Yellow** | Freedom is left: this element can still move. |
| **Green** | Entirely constrained: this point can no longer move at all. |
| **Grey** | A sketch other than the one being edited. |

The colour is **per element, not per sketch**: one outline can be entirely
frozen while its neighbour still floats, and that is precisely what shows what
is left to do. A trait is green only if both its ends are.

### The origin point

Every sketch has, from its creation, **a point at its origin**. One does not
place it: it is there. It is told apart by a diamond, never moves, and serves
as the reference for everything else.

It is what keeps a drawing from sliding, in two ways:

- by **snapping** a vertex onto it — a click nearby joins it rather than laying
  a second point at the same spot;
- by **measuring from it** — a point-to-point dimension between the origin and
  a vertex positions it without its having to touch it.

*(Later, in 3D, a vertex of an existing part will be able to play the same
role.)*

### What it takes to get to green

Three things:

1. **The shape values needed** — lengths and angles. "Needed" and not "all": in
   a triangle with two sides and the angle between them given, the third side
   **follows** and can no longer be imposed.
2. **An attachment to the origin**, by snapping or by dimension.
3. **Enough to say which way round the shape is laid** — see below.

**Example, a rectangle** with one corner on the origin: two sides and **three**
right angles (the fourth follows) are enough. With a single right angle the
quadrilateral can still deform into a parallelogram.

### Only the origin attaches

A drawing that holds on to nothing can be anywhere on the plane, and declaring
it finished would be saying it is done while it is attached to nothing. The
only point that attaches is therefore **the origin** — and, later in 3D, a
vertex or a face of the part.

**Fixed** does not attach. It holds an element still while the drawing
settles — that is what it is for — but it does not say *where*: the figure it
alone holds back could be elsewhere, so it does not turn it green.

### Orientation, when the shape says it by itself

Turning a whole drawing around the origin changes no length and no angle: **no
dimension can see that rotation**. One therefore used to have to place a 0°
angle dimension on an axis, purely to say "and it stays that way round".

That is no longer needed **when the shape says so by itself**: a trait laid
along an axis says which way round the figure lies. Square to the frame is one
way round like any other, and the most common. The figure then keeps the way
round it was drawn in, without anything having to be written.

A **slanted** shape says nothing. As soon as none of its traits is at 0, 90,
180 or 270°, it has to be said: an angle dimension against an axis, and it
turns green. Without that it would be declared finished while it can still be
swung round.

The rule holds **per group of connected geometry**: two shapes drawn apart can
turn relative to one another, so each answers for its own way round. A single
shared rule would leave both free to swing against each other, and neither
would ever be frozen.

Placing an angle with an axis on a shape already square takes nothing away: as
soon as a dimension says which way round a shape is laid, the implicit rule
steps aside for that group — without which the same freedom would be removed
twice and a drawing still free to slide would pass for frozen.

### How it is computed

By the **rank** of the system of equations, not by counting dimensions. Each
dimension gives an equation; one looks at how many of them say something new.
That is the only way to see that the third side of a triangle follows from the
others — a count would never see it.

To know whether a given point is frozen, the **moves still possible** are
computed (the kernel of the system): if none of them displaces that point, it
can no longer move.

What is counted opposite are the **unknowns**: two per point, one per circle.
Only the origin is not among them — it is the one point known in advance not to
move.

The reading is taken **once per state of the drawing**, not once per image
drawn. Painting the sketch asks which points are held on every frame, and the
answer costs a cubic pass over the whole drawing — tens of milliseconds on a
few hundred points, which is several frames' worth. It is kept until the
drawing it was read from changes, and the drawing is recognised by a print
taken of everything the reading answers to. Positions are part of that print:
a trait lying square to the sketch is what lets its group keep the direction it
was drawn in without being told, so moving one point can settle another.

Both answers come out of the same Gram–Schmidt pass: each direction is stripped
of what the equations already hold, and whatever survives is a move still
possible. A direction found that way is itself something the next one must be
stripped of, so it joins the basis instead of being kept beside it — the two
were once held apart and reconciled by copying the basis on every column, which
cost as many copies as the drawing has unknowns squared. The arithmetic behind
the answer is cubic and stays cubic; a drawing of a few hundred points is read
in tens of milliseconds.

### A frozen point no longer moves with the mouse

A green vertex does not answer the Selection tool. Pulling it would silently
undo a value that was typed; to move it, one changes what holds it.

## Closed surfaces

As soon as an outline closes, the area it encloses is **lightly tinted**. Four
separate traits become a face, and one sees at a glance whether a shape is
really closed.

A shape drawn **inside another** is tinted more strongly: without that an
outline and the pocket in it would blend into one another. The tint of the
outer area is not taken away underneath — the doubling is the cue. What
**becomes matter** is another matter: an inner outline is a hole there, and two
circles one inside the other extrude to a tube and not a rod. See
[extrusion.md](extrusion.md).

Outlines are found as a map finds its countries: one walks along each trait
always turning as tightly as possible, and the walk comes back on itself around
exactly one area. Counting the traits would not do — one and the same side
belongs to two areas when two shapes share it.

### Construction geometry

The **Construction** toggle, next to the drawing tools, marks the next shape a
tool places as construction: still real, still draggable, still solved and
snapped onto like any other trait or circle, but excluded from the area of any
region it sits inside or across, drawn dashed to say so. It stays pressed
across shapes — off again is a second click — the same way a circle mode
stays chosen until another is picked.

It is there to help build a profile without becoming part of it: a
symmetry line, a construction axis to dimension an angle against, a helper
circle to centre a pattern on.

### The value field is on the dimension

Once placed, the dimension carries its input field **right beside it**, in the
viewport. It used to be in the title bar, an arm's length from the drawing: the
eye had to leave the measured shape to find the number belonging to it.

The field **takes the keyboard** as soon as the dimension is placed, value
selected: one types the new one and that is all — reaching it with Tab would
mean crossing the whole toolbar first, and without the selection "40" typed
over "60.88" would read 60.8840.

`Entrée` validates, and **that keystroke is consumed on the spot**: the field
has just handed back the keyboard, so without that the same press would also
fire the shortcut bound to it — and finish the sketch.

## Dimensions too many

Placing a dimension whose value already follows from the others adds nothing.
The application detects it and places it **read-only** rather than refusing it:
it shows the measured value, in brackets and in grey, and its field cannot be
edited. A message says so at the moment of placing it.

It is useful: reading a length stays interesting even when fixing it makes no
sense. And since it always shows what the geometry measures, it stays right
when the drawing moves afterwards.

## Dimensions, and the scale

A dimension behaves differently depending on whether it is the first of the
document:

- **The first dimension defines the scale.** Nothing moves: saying that a trait
  is 100 mm simply teaches the document how many millimetres a world unit is
  worth. That is what allows drawing by eye and then giving the drawing its
  size afterwards, without deforming it.
- **The following ones are constraints.** The geometry moves to respect the
  length asked for.

### Angles

An angle dimension is placed on two traits that touch, and turns the second
around the shared point to the angle asked for, carrying along whatever is
attached to it — the same spirit as a length. With the Angle tool, the second
click can also land on **an axis of the sketch**: the angle is then measured
against that fixed direction. That is no longer required to freeze a drawing —
the orientation is implicit — but it is what serves to lay a shape at a wanted
angle.

The way the corner opens is kept: asking for 30° on a corner turning one way
does not flip it. An angle can never define the scale of the document: degrees
say nothing about a size.

### How the geometry moves

Every dimension is **re-solved together** at each change. That is what makes a
value stay true after another has been modified: the whole system is taken up
again, instead of applying each dimension once and then forgetting it.

The method is projection: each equation is corrected a little, in turn, until
nothing moves any more. Points laid on the origin never move. The process is
deterministic — same drawing, same order, same number of iterations — which is
what allows rebuilding a part identically by replaying its history.

If the values contradict one another, the solver stops at the end of its
iteration quota and says so, instead of stopping silently on one of them.

#### It starts again as long as it gains

One pass is not always enough. What is held rigid and what is allowed to give
is read **on the shape as it is**; once the drawing has moved, that reading is
out of date, and starting again takes a fresh one. The solver therefore goes
back to the beginning as long as each round removes at least a tenth of the
remaining error, and stops as soon as it gains no more.

That used to happen by accident: a drawing left halfway straightened itself as
soon as the next change gave it a new quota. One then saw a tangent circle stay
visibly out of shape until something else was touched. It now settles on
release.

#### What the change need not deform

A typed value, a moved point: a few equations are no longer true, and **they
alone say where the drawing is allowed to give**. An angle opens between its
two traits; a length stretches its own trait. All the rest is welded.

The solver therefore cuts the drawing into **blocks** — bundles of traits
welded together — and leaves them nothing but a move as a whole: carried,
turned, never bent. A block sharing a point with a block already in place
**turns around that point**: that is what makes a figure swing around its
corner instead of stretching. A block holding a point that cannot move — the
origin — does not move at all.

Concretely, on a chain laid on the origin with an angle dimensioned in the
middle: changing that angle leaves the anchored side exactly where it is and
swings the other, shapes preserved, instead of deforming the whole drawing a
little everywhere.

This shaping is applied **at every step** of correction, and not as a touch-up
at the end: a correction spread over the points and then straightened
afterwards is a correction thrown away three-quarters over, and the drawing
then takes dozens of rounds to converge.

Keeping the shapes is not always possible — the value asked for may want
exactly what was being held, like the height of a rectangle. The drawing is
then solved the old way, bending where it must.

#### Angles are judged in angles

The error of an equation is compared to the size of the drawing, which makes
sense for a length and not for an angle. A drawing of 150 units could thus be
declared solved with a corner a tenth of a degree out. An angle is now judged
on itself, in radians.

#### The drawing does not drift

A group that nothing holds straight can be turned without breaking a single
dimension, so **the solver is free to turn it** — and it did: each correction
is a step of finite size, and what each leaves behind adds up. A rectangle
whose height was changed came out several degrees askew, still declaring itself
entirely constrained — and it was: it had simply turned.

The way round of each group free to turn is therefore **read before solving**
(the direction of its first trait) and **restored afterwards**. Turning as a
block a group that nothing orients leaves all its dimensions exactly as they
were — that is the very definition of "free to turn" — so the drawing is set
straight without anything it measures changing.

A block also used to **grow** from being turned, more quietly. The turn is read
off as a torque over a spread, which is the *tangent* of the angle and not the
angle: laid along the perpendicular of each arm, it leaves that arm
`sqrt(1 + turn²)` longer. The error is one-sided — never shorter — so a figure
turned four hundred times came out measurably larger than it went in. The arm
is now turned rather than nudged sideways, and the block keeps its size however
often it is swung.

### What it is not

The solver is of the projection kind, not Newton: it converges well on drawings
of this size, but there is neither conflict detection beforehand nor a
diagnosis explaining *which* dimensions contradict each other — only the
observation that it did not get there.

## Saving

Every gesture becomes an operation recorded in the `.caopart` — a zip archive,
described in [history.md](history.md). The drawing itself is not stored:
it is rebuilt by replaying those operations.

## Undoing

`Ctrl+Z` undoes, `Ctrl+Y` (or `Ctrl+Shift+Z`) redoes. The History panel also
allows going straight back to any step. See [history.md](history.md).

## What is still missing

- No snapping to the alignments (horizontal, vertical) of an existing point:
  the magnets hold only to points, midpoints, the body of a trait and the grid.
- An outline that crosses itself is not tinted.
- The solver does not say *which* dimensions contradict each other when it does
  not get there.
