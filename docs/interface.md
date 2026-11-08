# The toolbar and the panels

See also: [sketch](sketch.md) · [history](history.md)

## Layout

```
┌──────────────────────────────────────────────┐
│ ⌂ Accueil / part / (view)  (• message…)      │  title bar
├──────────────────────────────────────────────┤
│ CAO Esquisse Extrusion Export  Pièce Var. ┊ ⚙│  logo, tabs, then the panels
│ ─────────────────────────────────────────────│
│ Nouvelle esquisse │ Dessin… │ Édition…       │  entries of the open tab
├──────────┬───────────────────────────────────┤
│ History  │                                   │
│  Sketch  │              viewport             │
│   1. Line│     ┌Recadrer Coupe Matière │ X Y┐│  view bar
└──────────┴───────────────────────────────────┘
```

The title bar reads as a breadcrumb: « Accueil », which goes back to the home
screen, then the part's name, its file on hover, then where the view stands, in
a pill — the sketch open (« Esquisse 2 »), else the section view, else the
kept section shown (« Coupe 1 »), else the plane the view is turned to, else
« Vue 3D libre ». In the middle of the window, whatever the breadcrumb's
length, the message answering the last gesture — a hint, a refusal, a
confirmation — stands in a pill of its own behind an amber dot, and gives way
to the next. A write that fails — saving the part, an export, the settings —
takes the pill in red, the message before it gone, and gives way in
turn to the next message or the next command; a write that succeeds meanwhile
takes nothing back. A message too long for the room left is cut short, the
whole of it on hover. The home screen has no pill: its errors stand in a red
bar along the bottom.

At the bottom right of the view, the view bar carries « Recadrer », « Vue en
coupe », « Matière devant » and the cursor's coordinates; see
[viewport.md](viewport.md).

The « CAO » logo heads the tab row on a fresh install; the toolbar section of
the preferences switches it off. Right of the tabs stand the toggles of the
« Pièce » and « Variables » panels, a separator, and « Préférences ».

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

On the left, resizable, shown and hidden by `H`: it has no toggle on the tab
row. Its contents are described in [history.md](history.md).

## The Variables panel

On the left, beside the History panel, hidden by the "Variables" button at the
far end of the tab row, in a sketch and outside one. One row per variable of
the part — its name, its formula, what it comes to — and a row at the foot that
adds one. What it does is in [variables.md](variables.md). The "Pièce" panel
lists the variables too, read only.

## The Files panel

On the home screen only, on the left, resizable, and always there: choosing a
part is what that screen is for. The part screen has no Files panel and no
command for one. It lists the folders and the parts of the parts folder — the one
new parts land in — and nothing else: a file that is not a `.caopart` is left
out, and so is a symbolic link, which is what keeps a folder pointing at one of
its own parents from being walked for ever.

Double-clicking a part opens it in place of the home screen.

Each part shows a small picture of itself, as the 3D view frames it — taken
when the part was last put away and kept in its file, never drawn while the
folder is being listed. A part nobody has closed since is a quiet empty frame,
so the rows stay aligned; it gains its picture the next time. Only the rows on
screen are read and turned into textures.

The panel also makes a folder, renames a part or a folder, and sends one to the
system bin after a question. Renaming a part writes the new name inside the
archive as well as on the disk, so the panel and the title bar never disagree.

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
