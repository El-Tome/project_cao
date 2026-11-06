# CAO — interface redesign package

Delivered by the designer on 2026-11-06 as `cao-interface.zip`, and translated into English in #723.

Reference package for splitting the redesign into tickets in the Rust / egui application.
The mockups were made in HTML, but nothing here is to be ported as is: the reference is the **specs**, the **tokens** and the **screenshots**.

## Contents

| Path | Role |
|---|---|
| `components.md` | Shared widgets (C1 … C15): sizes, states, colours. |
| `part-screen.md` | Part screen, area by area (P1 … P13). |
| `home-screen.md` | Home screen (A1 … A4). |
| `preferences.md` | Preferences window (R1 … R3). |
| `icons.md` | The 110 icons: id, name, usage. |
| `tokens.json` | Colours, typography, spacing, radii, sizes (logical px = egui points). |
| `icons/*.svg` | One icon per feature, 24 × 24, `currentColor`. |
| `screens/*.png` | Screenshots at 2880 × 1800 (×2) of each screen and state. |
| `source/*.html` | Original mockups, to check an exact value when in doubt. |

The designer's `backlog.md` — 27 proposed tickets (UI-01 … UI-42), with dependencies, references and acceptance criteria — is replaced by GitHub issue #722, whose body carries every decision.

## Screenshots

| File | Screen / state |
|---|---|
| `screens/01-part-sketch-line.png` | Part, sketch, Line tool in progress (snap, field near the cursor), hover tooltip |
| `screens/02-part-sketch-circular-pattern.png` | Tool panel in steps, preview of the copies |
| `screens/03-part-extrusion.png` | Extrusion panel, free 3D view, preview of the volume |
| `screens/04-part-export.png` | Export ribbon, STL menu open |
| `screens/05-part-alerts-open.png` | Alert list open |
| `screens/06-part-history-unfolded.png` | History unfolded |
| `screens/07-part-reading-step-3.png` | Step reader stopped on a step in error |
| `screens/08-home.png` / `screens/09-home-empty.png` | Home, with and without parts |
| `screens/10-preferences-toolbar.png` | Preferences > Toolbar |
| `screens/11-icons.png` | Icon sheet |

## Changes from the current interface (inventory of 2026-11-05)

> Where #722 decided otherwise, each spec says so in a quoted line that starts « Decided otherwise in #722 ». Here: the step reader also inserts a step at its reading point (inventory L2), and the save location is a folder inside the parts folder (inventory M2).

1. **Sketch ribbon reorganised** into groups. Frequent tools are large buttons, the others small buttons over 2 or 3 rows. It fits in 1440 px.
2. **« Recadrer », « Vue en coupe » and « Matière devant »** move into a view bar at the bottom right of the 3D. **« Terminer »** moves into a context pill at the top of the view.
3. **Alerts**: a dedicated button and a list, with two levels (yellow warning, red error), separate from the help messages.
4. **Visibility per volume** with 3 states (opaque / see-through / hidden). « Corps » becomes « Tout le corps ».
5. **History folded** by default. Compacting becomes a simple link.
6. **Step reader**: a new tool distinct from the history, which keeps only the main steps and colours them after the alerts.
7. **Tool panel in steps**: a common model for the tools with several fields.
8. **Orientation cube** in real 3D, with the sketch's face highlighted.
9. **Home**: tree of all parts, document type (« Pièce » / « Assemblage » to come), choice of save location.

Nothing is removed: every command of the inventory stays present, in the ribbon, in a menu, or among the commands left out of the bar, in the preferences.

## egui correspondences (indicative)

- Title bar, tabs, ribbon → stacked `TopBottomPanel::top`. Left panel → resizable `SidePanel::left`. View → `CentralPanel` with its own rendering.
- Floating panels, alert list, menus → `egui::Area` or `Window` without decoration, anchored.
- The cube and the view's annotations are drawn with the `Painter` over the 3D rendering.
- Theme → `Visuals` and `Style` built from `tokens.json`.
- The panel names are the designer's: on egui 0.36, which this project uses, side and top panels are `egui::Panel`.

## Points to supply or check

- The texts in square brackets (`[Sous-titre de l'application]`, `[Dossier A]`, `[résumé]`, dates) are placeholders to fill.
- The measurements, extrusion names and history steps of the mockups are examples.
- Faults of the HTML mockup not to reproduce:
  - « + Dossier » and « Vue d'origine » wrap onto two lines; they must stay on one line.
  - At 900 px high, the « Variables » section is clipped by the reader card: the Part panel must scroll, and the card must never cover its content.
- Only the « Barre d'outils » section of the « Préférences » is mocked; the others keep their current content with the new components.
