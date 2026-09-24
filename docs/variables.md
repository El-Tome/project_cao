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

**The fields at the cursor take every key** the moment they appear, letters
included: `largeur/2` is typed as it is, with no `=` before it — a leading `=`
is still read, and ignored. The letter shortcuts go quiet while the fields are
shown, as they did before variables existed; the ribbon is always there. A first
version kept a letter a shortcut until a digit or `=` started the field; tried
in the app, having to type `=` before a name was the part that got in the way,
and the choice went the other way.

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
  longer whole, a dimension the drawing can no longer hold, an area an
  extrusion stood on and would no longer find. The message names what would
  break, and the dimensions among it **blink for a few seconds** on the
  drawing, so that it is seen where it happens;
- **a change to how many elements a step lays** — a pattern counted by a
  variable, say — **while anything follows that step in its sketch, or is
  raised from that sketch**. The elements of a drawing are named by their
  rank, so whatever comes after would name something else: another trait,
  another pad. A count changes only while what it counts is the last thing
  done in its sketch and nothing is raised from that sketch; the message names
  the step and what follows it. A sketch that would lose the face it was laid
  on is refused the same way.

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
keeps its rank and its last formula, so a size written from it before it went
still replays to what it was.

**A value that leaves the drawing stops following the variables from that
moment**, whichever way it leaves — erased, the trait it measured erased, typed
again and taken, dropped by a cut: it replays against the table as it stood
when it went. Replayed against the table as it stands now, the shape it set
would go on moving with a variable nothing on the drawing shows. The replay
marks each value it sets from the variables with the operation that set it,
and the drawing carries the mark wherever it carries the value; so the replay
sees exactly where one leaves, and plays the history again with those worked
out against the table as it stood there — those that come to another number
against it, which is when the drawing would move. A value typed again and
refused never left, and still follows.

A value the variables as they now stand cannot give — a side of 500 in a
triangle of 100 and 90 — is refused where it is set, and so would never be
seen leaving. It is first tried against the table it was written with, which
shows whether it leaves, and where: a value gone from the drawing is never
held against a change it could not have taken.

A chamfer's, a fillet's and a pattern's sizes, like a step's, are the tool's
own and are listed with it: they follow their variable for as long as the tool
stands, whatever becomes of the values it laid on the drawing.

A value on a drawing remembers what it was written as, in the part's own
notation (`#0 / 2`): the drawing never reads it, and carries it wherever it
carries the value — onto a piece of a trait a cut went through, onto a corner a
chamfer put back. That is what editing it, compaction and the list of what uses
a variable read. A diameter read again as the radius of the arc a cut leaves is
half of it: the drawing halves the number, and the part — which, unlike the
drawing, reads formulas — halves what it was written from.

**Compacting** the history keeps the variables — the live ones, each after
those it leans on — and every formula written from them, said again in the
ranks they land on. A sketch holding a tool written from variables — a
pattern, a chamfer, a fillet — is left as it was drawn rather than flattened
into what the tool laid, which would take the formula from it.

## What is missing

- Functions in formulas, and variables shared between parts or read from a file.
- A variable driven by a measure of the drawing.
- A dimension written from variables looks like any other on the drawing: only
  opening it says so.
- A finer reading of what follows a count. A label moved after the pattern,
  or the original dragged, names nothing the pattern lays, and holds its count
  all the same: telling those apart from what names the copies — and naming
  elements by something steadier than their rank — is work of its own.
- The panel of mirror and patterns (#321): the patterns still take their values
  in fields at the cursor, which now show once `Entrée` has closed the
  selection — until then `Entrée` finished the sketch instead.
