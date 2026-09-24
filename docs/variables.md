# Variables and formulas

See also: [history](history.md) · [sketch](sketch.md) · [interface](interface.md)

## What they are

A part holds **one table of variables**, shared by every sketch and every step
of it: a width is used by the sketch that draws the plate and by the extrusion
that gives it its thickness. Each variable is a **name** and a **formula**.

A formula is a number or a small calculation: the four operations, parentheses,
numbers and other variables' names — `largeur / 2 + 5`, `(a + b) * 2`. A
decimal comma reads like a point, and a leading `=` is allowed and ignored.
There are no functions: `sin`, `sqrt` and `min` are a language to build and to
explain, and were left out on purpose.

A name is letters, digits and `_`, does not start with a digit, and is unique
among the variables the part has. Accented letters are letters.

## Where a formula is typed

Everywhere a size is typed, a formula can be typed instead:

| Where | Unit it is read in |
| --- | --- |
| A dimension's value, placed or edited | millimetres, or degrees for an angle |
| The fields at the cursor while drawing | the same |
| An extrusion's distance, a revolution's angle | millimetres, degrees |
| A chamfer's and a fillet's values | millimetres, degrees |
| A pattern's steps and counts | millimetres, degrees, a plain count |

A variable has no unit of its own: it is read in the unit of wherever it is
used. A count has to come out as a whole number above zero; a formula that
does not is refused with a message. A count typed as a plain number is rounded,
as it always was.

**A size written from variables keeps its formula**, never the number it came
to. The dimension a shape leaves behind keeps the formula typed at the cursor;
a dimension on the drawing shows its value, and opening it for editing shows
its formula. The history says a size written from variables as both —
`Extrusion épaisseur = 4 mm`.

### The fields at the cursor, and the keyboard

While nothing has been typed into the fields at the cursor, **a letter is a
shortcut**: `R` takes the rectangle tool rather than landing in the length.
A digit, a sign, a decimal separator or `=` starts the field, which then keeps
every key until `Entrée` or `Échap` — so `=largeur/2` is typed after `=`, as in
a spreadsheet. Before this, the first field took every key the moment it
appeared, and the letter shortcuts went quiet while drawing.

A field holding something that does not read refuses the click and `Entrée`,
and says why: the shape is not laid at the cursor with what was typed dropped
without a word.

## Changing a variable

Variables are made, edited and taken away in a **panel of their own**, beside
the history panel, opened from the ribbon — in a sketch and outside one. One
row each: the name, the formula, what it comes to. A row is asked of the part
when the keyboard leaves it, or on `Entrée`; `Échap` drops what was typed into
it. The row at the foot adds a variable.

**Changing a variable changes every size written from it**, and the part is
rebuilt. The formulas are resolved when the part is rebuilt rather than
rewritten into numbers when a variable changes: a change to the table replays
like everything else. **It is a step of the history like any other**: undo
puts the former formula back, and the part with it.

**Renaming a variable renames it everywhere.** A formula names a variable by
its rank in the table, not by its name, so a name is a label a formula follows.

Refused, with a message, and nothing applied:

- a name that does not do;
- a formula that does not read — the message says what is wrong with it;
- a loop — `a` from `b`, `b` from `a` — naming the variables in it;
- **taking away a variable something uses**, with the list of what uses it:
  the dimensions, the steps and tools, the other variables. Those are rewritten
  first; then it can go;
- **a change that would break a size**: a length at zero or below, a count no
  longer whole, a dimension the drawing can no longer hold. The message names
  what would break, and the dimensions among it **blink for a few seconds** on
  the drawing, so that it is seen where it happens.

A change is tried before it is made: the part is rebuilt with it, and what no
longer holds that held before is what the change is refused for. A size that
did not hold already is not held against it.

## How it is kept

A change to the table is an `Operation::Variable`, numbered like any other, and
filed under **no step**: the variables are a table of the part, not a step of
it. The index names the numbers of those changes, and `design/variables.json`
holds them — every change, undone ones included, so the redo tail survives
closing the part, as it does for everything else. A part with no variables
writes no such file, and an index that names none.

The table is played out of the history before any step, and every formula is
worked out against the table as it stands at the cursor. A variable taken away
keeps its rank and its last formula, so a size written from it before it went —
a dimension since erased, say — still replays to what it was.

A value on a drawing remembers what it was written as, in the part's own
notation (`#0 / 2`): the drawing never reads it, and carries it wherever it
carries the value — onto a piece of a trait a cut went through, onto a corner a
chamfer put back. That is what editing it, compaction and the list of what uses
a variable read. A diameter read again as an arc's radius is half the number it
was, and the drawing cannot say what half of a formula is: the number goes
over, the formula does not.

**Compacting** the history keeps the variables — the live ones, each after
those it leans on — and every formula written from them, said again in the
ranks they land on.

## What is missing

- Functions in formulas, and variables shared between parts or read from a file.
- A variable driven by a measure of the drawing.
- A dimension written from variables looks like any other on the drawing: only
  opening it says so.
- The panel of mirror and patterns (#321): the patterns still take their values
  in fields at the cursor, which now show once `Entrée` has closed the
  selection — until then `Entrée` finished the sketch instead.
