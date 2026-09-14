# The toolbar and the panels

See also: [sketch](sketch.md) · [history](history.md)

## Layout

```
┌──────────────────────────────────────────────┐
│ ⌂ Accueil │ part name │ view │ message…       │  title bar
├──────────────────────────────────────────────┤
│ Esquisse │ Extrusion │ Panneaux │           ⚙ │  tabs
│ ─────────────────────────────────────────────│
│ Nouvelle esquisse │ Dessin… │ Édition…        │  entries of the open tab
├──────────┬──────────┬────────────────────────┤
│ Fichiers │ History  │                        │
│  Brides  │  Sketch  │        viewport        │
│  support │   1. Line│                        │
└──────────┴──────────┴────────────────────────┘
```

## The toolbar goes where the user wants it

It attaches **at the top, at the bottom, on the left or on the right**, or
floats in a small window one moves about. The choice is made in the settings
screen and travels with the profile. One single function draws the inside in
all five cases — the only difference is the container, and the direction it
imposes on the rows.

Its contents are a **tree the user arranges**, not a fixed list: the groups at
the first level are the tabs, those one level down are spread out where they
are with their name beside them, deeper still they become menus that open on
click. The detail is in [configuration.md](configuration.md). Every button
shows its shortcut in brackets when it has one, and is greyed out when the
command makes no sense where one is.

## The History panel

On the left, resizable, hidden by the "Historique" button of the toolbar. Its
contents are described in [history.md](history.md).

## The Files panel

On the left too, resizable, opened by the "Panneaux" group of the toolbar or by
its shortcut. It lists the folders and the parts of the parts folder — the one
new parts land in — and nothing else: a file that is not a `.caopart` is left
out, and so is a symbolic link, which is what keeps a folder pointing at one of
its own parents from being walked for ever.

Double-clicking a part opens it: in place when the start menu is showing, in a
**second window** when a part is already open. Tabs are what that is to become;
until the shell can hold more than one part at a time, the second window is a
second process, and what the two share is what the installation remembers.

The panel also makes a folder, renames a part or a folder, and sends one to the
system bin after a question. Renaming a part writes the new name inside the
archive as well as on the disk, so the panel and the title bar never disagree.
The part the window is drawing is left alone: it is being written to behind the
panel's back.

The library is walked when the panel opens and when something is known to have
changed — never every frame. The ↺ button reads it again.

## What is missing

- The place is chosen in the settings: dragging the floating window to an edge
  does not dock it there by itself.
- The state of the panels — which tab the bar is open on, whether the history
  is hidden — is not saved from one session to the next.
- The Files panel browses the parts folder and cannot be pointed anywhere else.
- It walks the whole tree at once, folded folders included, rather than reading
  a folder when it is unfolded. On a parts folder held on a network share, that
  walk is felt.
- Two windows on two parts share one settings file and one recent list: the
  last one to write a change is the one that stands.
- Nothing stops a part being renamed or thrown away from one window while
  another window has it open.
