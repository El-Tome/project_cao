# Shared components

Building blocks reused by several screens. Each should become a single widget in `crates/app/src/ui/`, beside the primitives that already exist (eye, floating panel, formula field, slider, text, tool button, drag and drop).
Colours, sizes and radii refer to `tokens.json`.

## C1 — Ribbon button, large (`tb`)
- At least 50 px wide, 64 px high, radius 6. Icon of 22 px above the name (11.5 px, weight 500).
- Shortcut in the top right corner: 9.5 px, monospace, colour `text.shortcut`.
- Long names wrap onto two lines, 70 px wide (« Nouvelle esquisse », « Ajout de matière »).
- **States**:
  - normal: transparent background, text `text.button`;
  - hover: background `background.hover_strong`;
  - **active** (tool in progress): background `accent.amber_background`, border `accent.amber_border`, text `accent.amber_light`;
  - disabled: text `text.disabled`, no hover.
- With a menu (3rd level): a chevron of 8 px follows the name, and a click opens C8.
> Decided otherwise in #722 (inventory R2): a button with a menu is a split button — the icon runs the tool, the chevron opens the menu.

## C2 — Ribbon button, small (`sb`)
- 20 px high (26 px in a 2-row group). Icon of 15 px to the left of the name. Shortcut aligned on the right.
- Same states as C1.
- Laid out in a grid in columns: 3 rows of 20 px or 2 rows of 26 px, fitting inside the ribbon's 64 px.

## C3 — Ribbon group
- A column holding the buttons and, at the bottom, the group's name: 10 px, UPPERCASE, letter spacing 0.08 em, `text.quiet`.
- A vertical 1 px separator `border.group` between two groups, horizontal margin of 10 px.

## C4 — First-level tab
- 36 px high, horizontal margin of 14 px, text 13 px.
- The active tab has a 2 px amber underline and the text `text.title`.

## C5 — Panel toggle (« Pièce », « Historique », « Variables », « Fichiers », « Préférences »)
- 28 px high, 15 px icon and name.
- Enabled: background `#20242C` and border `border.control`.
> Decided otherwise in #722 (inventory T6, T8): only « Pièce » and « Variables » stay as toggles, then « Préférences »; the history sits at the bottom of the Part panel and the files on the home screen.

## C6 — Segmented selector
- Container with background `#15181D`, border `#2A2E37`, radius 6, inner margin of 2 px. Buttons of 24 px (28 to 30 px on wide screens).
- Chosen segment: background `background.active_neutral`, text `text.title`. A segment can be disabled, with a « bientôt » label.

## C7 — Switch
- 30 × 18 px, round knob of 12 px.
- Off: background `background.active_neutral`. On: amber background, knob `text.on_accent`.

## C8 — Dropdown menu (ribbon 3rd level)
- 230 px wide, background `#22262E`, border `#353A45`, radius 8, shadow `shadow.menu`.
- Rows of 30 px with, on the right, an indication in monospace (e.g. « par défaut »).

## C9 — Hover tooltip
- 230 px wide, background `#262A33`.
- Holds the name, the key in a frame (C14) and a sentence of help.

## C10 — Tree row
- 28 px high, radius 5, text 12.5 px.
- Value on the right in monospace 11 px, colour `text.quiet`.
- Variants:
  - selected: amber background at 12% opacity, text `#F7D29E`;
  - dimmed (hidden element): text `text.quiet`.
- Indentation: 26 px, with a 1 px vertical line `#2A2E37`.
- Chevron of 10 px to unfold or fold.

## C11 — Visibility button (« eye »)
- 22 × 22 px, icon of 15 px.
- **Sketches**: two states, `eye` / `eye-off`.
- **Volumes** (extrusions, revolutions): three states that a click cycles through:
  - `body-opaque` (full circle);
  - `body-see-through` (half circle);
  - hidden (`eye-off`, and the row turns dimmed).
- The state is written out in full to the right of the row (« opaque », « transparente », « invisible »).
> Decided otherwise in #722 (inventory P4): a volume's state applies to the faces of the part made by that step, not to the volume drawn alone.

## C12 — Badge
- Pill of 10 px, weight 600.
- Warning: background `state.warning_background`, text `state.warning_text` (e.g. « instable »).
- Error: background `state.error_background`, text `state.error_text` (e.g. « cote perdue »).
> Decided otherwise in #722 (inventory A1): « cote perdue » is only an example; it does not exist today. Each kind of problem has its level there.

## C13 — Floating tool panel (common model)
- 300 px wide, placed 14 px from the top left corner of the 3D view.
- Background `background.floating` at 97% opacity, border `border.control`, radius 10, shadow `shadow.floating`, inner margin of 14 px, spacing of 14 px.
- Header: tool name (14 px, weight 600) and, on the right, the context (« Esquisse 3 »).
- **Numbered steps** (tools in several stages): a 22 px dot per step.
  - done: green background at 18% opacity and a ✓ check;
  - current: amber background and number;
  - to come: background `background.active_neutral`.
  - The current step's field has an amber border.
- Footer: « Annuler » (secondary button) and the amber primary button « Valider ↵ » or « Appliquer ↵ ».
- Used for: « Extrusion », « Réseau circulaire », and to be declined for « Symétrie », « Réseau rectangulaire », « Chanfrein », « Congé », « Vue en coupe ».
> Decided otherwise in #722 (inventory O1): the panel can still be dragged by hand and keeps its place, as today.
> Decided otherwise in #722 (inventory O4): the sketch's « Chanfrein » and « Congé » keep their fields by the cursor; the 3D chamfer and fillet take this panel.

## C14 — Key, chip, field
- **Key**: monospace 10.5 px, border `#454B57`, radius 3.
- **Chip** (chosen element): 24 px high, radius 12. Addition variant (green), selection variant (amber), « + cliquer dans la vue » variant (dashed border).
- **Number or formula field**: 34 px high, background `background.field`, border `border.control` (amber when the field has focus).
  - The variable block is a `variable` chip (e.g. `épaisseur`).
  - The computed result is shown on the right in `text.quiet` (« = 30 mm »).
  - Number fields have − / + buttons.

## C15 — Primary / secondary button
- **Primary**: amber background, text `text.on_accent`, weight 600, radius 6 to 8, 32 to 44 px high. The ↵ key is recalled in the label.
- **Secondary**: transparent background, border `#353A45`.
