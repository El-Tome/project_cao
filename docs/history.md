# History, undo and the file format

See also: [sketch](sketch.md) · [architecture](ARCHITECTURE.md)

## The principle: a part is its list of operations

The geometry of a part is **not** stored. What is stored is the run of
operations that produced it: "sketch on the XY plane", "trait from A to B",
"dimension of 100 mm". The geometry is rebuilt by replaying that list.

That choice makes three functions identical, instead of three separate
mechanisms that would end up contradicting each other:

| What the user does | What happens |
| --- | --- |
| Undo | The cursor steps back one notch |
| Redo | The cursor steps forward one notch |
| Click a step of the history | The cursor goes to that position |

In all three cases the part is then rebuilt from the beginning. There is
therefore no way for the display and the history to diverge.

A test checks explicitly that applying an operation live gives the same result
as replaying it: without which a drawing could change its look from the mere
fact of closing and reopening the part.

A dimension carries **where its annotation sits** in the same operation.
Placing a dimension is one single gesture of the user; reading "Dimension
60 mm" then "Dimension moved" at every click would have said nothing more. A
later move, with the mouse, stays an operation of its own.

A selection deleted as a block is likewise **one single operation**, with the
list of what goes — traits, dimensions and constraints together. One step per
element would have wanted as many undos as elements to take back a single
gesture. It is the only deletion: there is no separate operation to erase just
one.

## The cursor and the abandoned branch

The history keeps every operation and one position: what is before is applied,
what is after waits to be redone. That tail is **saved in the file**, so the
redo survives closing the program.

Drawing something new after an undo erases that tail: the part has taken
another direction, and keeping the old branch would leave a redo that no longer
follows from what is on screen.

## The points, and why they are not recomputed

A "trait" operation does not keep two positions but two **references**: either
an existing point, or a point to be created at such a position.

That matters: snapping depends on the zoom at the moment of the click (10
pixels on screen are worth more or fewer millimetres depending on the
distance). Replaying the snapping later could therefore weld different points
and rebuild another drawing. The decision is taken once, at the click, and
kept.

## The tree

The left panel lists the operations, grouped under the one that opened the
function under way — one line per sketch, unfoldable. Undone steps appear
greyed out below the current position. Clicking a line puts the part back in
the state it was in just after that step.

## Extrusion in the history

An extrusion is an operation like any other: it opens its own line in the tree,
and going back before it returns the part to the state of a drawing. The volume
is never stored — it is rebuilt by replaying the operations, exactly like the
geometry of the sketch.

The extruded area is remembered by **the position clicked** and not by its
rank, for the same reason as the points of a trait: a rank would move as soon
as another shape is drawn. See [extrusion.md](extrusion.md).

## Deleting takes nothing out of the list

A deletion is an operation like the others, and it **marks** what disappears
instead of removing it. Taking a trait out of the middle of the list would
shift the rank of every one after it, and each dimension recorded against those
ranks would then name another piece of the drawing — silently.

That is what allows undoing a deletion like any other step, and replaying it
identically. See [sketch.md](sketch.md).

## The file format

A `.caopart` is a **zip archive**, and no longer a single JSON object:

| File | Contents |
| --- | --- |
| `part.json` | The identity of the part: id, name, dates, schema version |
| `history.json` | The list of operations and the position of the cursor |

Separating the files allows each part to evolve independently, and leaves room
for what will come along (a thumbnail of the part, materials, exported meshes)
without rewriting the rest at every save.

### Earlier versions are not converted

A file written by an earlier version is **refused**, with the reason, instead
of being converted. As long as the tool moves this much, a conversion would be
likelier to rebuild a part askew than to save anything useful.

## What is missing

- The history cannot be edited: a step cannot be removed from the middle, nor
  reordered, nor can the parameters of a past operation be changed.
- No branches: one single line of history, with one single redo tail.
- A very long part is rebuilt entirely at every move of the cursor. That is
  instantaneous at the current sizes; it will want cached intermediate states
  the day it is not.
