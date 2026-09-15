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

```
.caopart
├── part.json                     the identity of the part: id, name, dates,
│                                 schema version
├── geometry.json                 what replaying the design below came to, a
│                                 cache
├── picture.json                  how big the picture below is, when there is
│                                 one
├── picture.rgba                  a picture of the part, rows of pixels, four
│                                 bytes each
└── design/
    ├── history.json              the ordered index: what each major step is,
    │                             its number, and what it is raised from
    ├── sketch-1/steps.json       the operations of that sketch
    ├── extrusion-2/steps.json
    └── sketch-3/steps.json
```

Separating the files allows each part to evolve independently, and leaves room
for what will come along (materials, exported meshes) without rewriting the rest
at every save.

### What designed the part sits apart from what the part is

`design/` holds how the part was arrived at; the root holds what it is. The
split is what gives the two things that are coming a place they do not have to
argue over: the rebuilt geometry of a step, cached beside the operations that
produce it, and a `simulation/` folder run against the part as a whole.

### The index says what the part is made of, the folders say how

`design/history.json` names the **major steps** in the order they were made —
today a sketch, an extrusion, a revolution; tomorrow a drilling, a section
view. Per step it holds three things and nothing else: its kind, its number,
and the step it is raised from. What a step is made of is not in there: the
operations live in a folder of its own, `design/sketch-1/steps.json`.

That is what makes the index readable on its own — the shape of the part
without the strokes — and what gives an edit of a past step one file to
rewrite instead of the whole design.

**Every major step gets a folder, a one-operation extrusion included.** A short
step written straight into the index would be a second layout every reader has
to know, and it would want a folder the day that extrusion's distance is edited
or its geometry cached.

**The number naming a folder is handed out once and never reused**, not even
after the step is undone, and not even by the compaction that rewrites the
design — it goes on counting from where the old one stopped. A rank would not
do: drawing after an undo throws away the abandoned tail, so what sits at a
given rank changes from one save to the next, and every folder past the cut
would have to be renamed on disk.

**Every operation carries a number too**, in the order the user did it, kept in
the index beside the step holding it. Nothing reads it yet; it is what undo
will walk the day the tree can be edited, when the order things were done in
stops being the order the part is rebuilt in.

The kinds are **written out by name**, so a kind added later costs no change of
format: a part written before that kind existed names only the kinds it knew.

The grouping this describes is no longer worked out from the list of
operations. The history records it, `Feature::all` turns it into the positions
the panel speaks, and the panel shows exactly what it showed before.

### The geometry is cached at the root, and the design stays the truth

`geometry.json` holds what replaying the design last came to — the drawing of
every sketch and the matter of the part. It sits at the root, since it answers
to the part as a whole and not to any one step of its design. Opening a part of
240 steps takes some seventy milliseconds of replay against three of reading
that geometry back.

It is written **when the part is put away** — closed, or left for the start
menu — and not at the end of every gesture like the rest of the archive. A part
closed is a part nothing more is coming to; a gesture is only ever followed by
another one, and geometry written there is geometry written again a second
later. The picture is written at that same moment, for the same reason. A
gesture that saves the design alone leaves no geometry behind it, so there is
nothing to invalidate: the entry is either the one the part was put away with,
or absent.

It is a cache and never the truth. It carries a print of the design it was
rebuilt from — folded over the index **and** every step's folder, since none of
the drawing is in the index — and a part whose cache is missing, damaged, or
answers to another design replays its design instead of refusing to open — so a design edited by
any hand other than a save can never show a shape the part no longer describes.

It also carries the version of the tool that rebuilt it, bumped by hand the day
replaying the same design stops giving the same geometry — a fix in the solver,
in an extrusion, in a boolean. Without that, a part fixed by such a change would
go on showing the shape it was cached with until somebody edited it.

JSON rather than the `.bin` the layout first called for: what the cache saves is
the rebuild, not the reading, and a binary codec would buy a couple of
milliseconds on the reading and cost a dependency — the same argument as the
picture below.

There is no `metadata.json`. `part.json` holds the identity, and a file named
after no particular content is where fields with nowhere else to go come to
pile up; the day something concrete needs writing down, it is named then.

### The picture is kept, because the geometry is not

The two `picture` entries hold what the part looked like when it was last put
away, so the files panel can show a folder without opening what is in it. That
is the whole reason they are there: no geometry is saved, so a picture drawn
while listing means replaying every history in the folder — measured at fifteen
times the cost on parts averaging seven operations, and linear in them after
that.

No image format. The archive already deflates what it holds, and a rendering is
mostly flat ground with a few strokes on it: a 128×128 picture is 64 KB of
pixels and about 3 KB in the file. A codec would buy nothing and cost a
dependency.

A part written before this carries neither entry and opens exactly as it did,
so no version was bumped for it. A picture that cannot be read whole is a part
with no picture, never a part that refuses to open: nothing of the drawing is
in there.

### Earlier versions are not converted

A file written by an earlier version is **refused**, with the reason, instead
of being converted. As long as the tool moves this much, a conversion would be
likelier to rebuild a part askew than to save anything useful. The flat list
that `design/history.json` used to hold is one of those, as was the flat layout
before `design/` existed: no reader is kept for either, and the schema version
went up each time so that a part written under one of them says so rather than
opening with no history at all.

## What is missing

- The history cannot be edited: a step cannot be removed from the middle, nor
  reordered, nor can the parameters of a past operation be changed. The file is
  laid out for it — one folder per step, a number that names it for good — but
  nothing reads that yet beyond writing it back.
- A sketch started on a face records the plane it was given and stays there.
  The face can move under it, and nothing says so.
- No branches: one single line of history, with one single redo tail.
- A very long part is rebuilt entirely at every move of the cursor. That is
  instantaneous at the current sizes; it will want cached intermediate states
  the day it is not.
