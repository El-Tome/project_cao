# The toolbar and the panels

See also: [sketch](sketch.md) · [history](historique.md)

## Layout

```
┌──────────────────────────────────────────────┐
│ ⌂ Accueil │ part name │ view │ message…       │  title bar
├──────────────────────────────────────────────┤
│ Esquisse │ Extrusion │        Historique │ ⚙ │  tabs
│ ─────────────────────────────────────────────│
│ Nouvelle esquisse │ Dessin… │ Édition…        │  entries of the open tab
├───────────┬──────────────────────────────────┤
│ History   │                                  │
│  Sketch   │           viewport               │
│   1. Line │                                  │
└───────────┴──────────────────────────────────┘
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
contents are described in [historique.md](historique.md).

## What is missing

- The place is chosen in the settings: dragging the floating window to an edge
  does not dock it there by itself.
- The state of the panels — which tab the bar is open on, whether the history
  is hidden — is not saved from one session to the next.
