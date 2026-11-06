# Part screen (main screen)

Screenshots: `screens/01` to `screens/07`. Reference size: 1440 × 900.
From top to bottom:
1. Title bar (44 px)
2. Tab row (36 px)
3. Ribbon (about 94 px)
4. Body: left panel (290 px) and 3D view (the rest)

## P1 — Title bar
- **On the left**, breadcrumb:
  - « ⌂ Accueil » (clickable, back to the home screen);
  - `/` then the part's name;
  - `/` then the current view in a pill (« Esquisse 3 » or « Vue 3D libre »).
- **In the centre**, the existing one-line message area, shaped as a pill: a 7 px amber dot, then the active tool's instruction.
  - Example: « Ligne — cliquez le point d'arrivée ou tapez une longueur ».
  - It keeps the instructions, refusals and confirmations; **the part's problems no longer go there** (see P2).
- **On the right**: the Alerts button (P2).
> Decided otherwise in #722 (inventory T4, T1): file, save and export errors also go to the pill, in red; and the native OS frame stays.

## P2 — Alerts (new)
- Pill button « ⚠ N alertes »: background `state.error_background`, border in light red.
  - Visible only when there is at least one alert.
  - N = the number of elements with a lost, refused, unstable or drifting badge.
- A click opens a floating list of 340 px, under the button. Each alert holds:
  - a `warning` (yellow) or `error` (red) icon;
  - the title « Élément · problème »;
  - a sentence of explanation;
  - the actions « Montrer » (selects the element and lights it in the view) and a repair action specific to the problem.
- Alert level:
  - **warning**: unstable, drifting;
  - **error**: lost, refused, and any error that stops the computation.
- The same colours appear in the tree's badges (P5) and in the step reader (P7).
> Decided otherwise in #722 (inventory A1, A2, A3): each kind of problem has its level, and « cote perdue » is only an example; the button takes the colour of the worst level; and the actions are « Montrer » and « Modifier ».

## P3 — Tab row
- On the left: the text logo « CAO », optional (amber, letter spacing 0.14 em).
- The C4 tabs: « Esquisse », « Extrusion », « Export ».
- On the right, the C5 panel toggles: « Pièce », « Historique », « Variables », « Fichiers ». Then a separator and « Préférences ».
> Decided otherwise in #722 (inventory T6, T8): only the « Pièce » and « Variables » toggles stay, then « Préférences »; the history sits at the bottom of the Part panel; the files live on the home screen.

## P4 — Ribbon (2nd and 3rd levels)

**« Esquisse »** (the default content is reorganised; see the README, section "Changes from the current interface"):

| Group | Content |
|---|---|
| « Esquisse » | « Nouvelle esquisse » (large, ⌘N) |
| « Dessin » | « Sélection » S · **« Ligne » L** · « Ligne sym. » · « Rectangle » R · « Cercle » C ▾ (menu « Cercles », 5 variants) · « Arc » ▾ (2) · « Ellipse » ▾ (2) · « Point » P — all large |
| « Cotes et règles » | « Cote » D · « Mesurer » M · « Contraintes » ▾ (9 rules) — small, 3 rows |
| « Modifier » | « Ajuster » · « Diviser » · « Symétrie » · « Congé » · « Chanfrein » ▾ (3 variants) · « Construction » · « Réseau circulaire » · « Réseau rectangulaire » — small, 3 rows in columns |
| « Édition » | « Annuler » ⌘Z · « Rétablir » ⇧⌘Z (disabled when there is nothing to restore) — small, 2 rows |

- **Moved out of the ribbon**: « Recadrer », « Vue en coupe » and « Matière devant » go into the view bar (P10); « Terminer » goes into the context pill (P9).
- **« Extrusion »**: group « Matière » (« Ajout de matière », « Enlèvement de matière »), group « Forme » (« Extrusion droite », « Révolution »), group « Édition ».
- **« Export »**: « Exporter en STL… ▾ » opens the C8 menu (« Grossière » « rapide », « Normale » « par défaut », « Fine » « lourd »); « Exporter en STEP… ».
- A C9 tooltip appears when hovering each button (screenshot 01: hovering « Ligne »).
> Decided otherwise in #722 (inventory P3, R1, R8): « Cotes et contraintes » instead of « Cotes et règles »; a group has a size setting; and the ribbon can sit on the left or the right.

## P5 — Part panel (tree)
- **Header**: « Pièce » and the part's name.
- **« ESQUISSES » section**:
  - one C10 row per sketch, with the 2-state eye, a chevron and the plane (XY, « face »…);
  - the active sketch is selected and unfolded;
  - its children: segments (with their length), circles (Ø), « Cotes » (count), « Règles » (count).
- **« VOLUMES » section**:
  - one row per extrusion or revolution, with the **3-state** visibility button C11;
  - an addition icon (green) or cut icon (red), the name, the possible badge, then the state written on the right.
- **« TOUT LE CORPS » section**: a C6 selector « Invisible » / « Transparent » / « Opaque », which sets every volume row at once. It is highlighted when all volumes are in the same state.
- **« VARIABLES » section** (read-only): name in `variable.text`, value on the right.
- The existing behaviours stay: hover = the element lights up in the view; click = selection.
> Decided otherwise in #722 (inventory P1, P7, P3): the sections are ordered « Esquisses », « Volumes », « Coupes », « Tout le corps », « Variables »; hover lighting is new; and « aire » and « contrainte » are used throughout.

## P6 — History (folded by default)
- At the bottom of the panel. The header row « ▸ Historique … N actions » unfolds on click.
- Once unfolded, the list of actions (at most 230 px high, scrolling):
  - each row: number in monospace, name, then the detail line;
  - a click goes back to that action;
  - « Modifier l'esquisse » stays available.
- **« Compacter » is no longer a header button**: it is a quiet link « Compacter l'historique… » at the bottom of the unfolded list (the existing confirmation is kept).
> Decided otherwise in #722 (inventory T7): H folds and unfolds the history.

## P7 — Step reader (new, a tool distinct from the history)
- Small card above the history: background `background.floating`, border, radius 8.
- **Shows only the main steps**, in order: sketch → extrusion → sketch → … (one box per sketch and per volume, not per action).
- A box = 22 × 24 px with the `item-sketch` or `extrusion-add` icon. Between two boxes, a small → arrow. ◀ ▶ buttons at both ends.
- **Click on a box = reading point**:
  - the part is computed up to that step included;
  - the following steps turn dashed and are not applied;
  - the arrows joining them go dark.
- A box in **warning** is yellow, a box in **error** is red. A 7 px dot in the corner stays visible even on a dashed step.
- Under the row:
  - the current step's name, in its alert colour if there is one;
  - « · la suite est masquée » if not at the end;
  - the counter « 3 / 6 » on the right;
  - a warning / error legend.
- Screenshot 07: reading point on « Esquisse 2 » (in error).
> Decided otherwise in #722 (inventory L2, L7, L3, L5): a step added while the reading point is at step 3 is inserted after step 3, and the reading point then stays on the new step; export always takes the whole part; and sections are not steps.

## P8 — 3D view
- **Background**:
  - sketch: plain `background.sketch_view`;
  - free view: radial gradient `background.view3d_centre` → `background.view3d_edge`.
  - The existing background settings still apply.
- **Grid** (in a sketch): thin lines every 20 px and main lines every 100 px, aligned on the origin. X (red), Y (green), Z (blue) axes at 75% opacity.
- **Sketch**:
  - settled traits of 2 px `sketch.settled_stroke`, vertices as 8 px squares;
  - construction in dashes;
  - closed areas tinted `sketch.area_fill`;
  - dimensions `sketch.dimension` with arrows, the value in a cartouche;
  - constraint marks in small boxes of 18 px (⊥, ∥…).
- **Tool in progress** (amber), while drawing:
  - the provisional trait, and a dashed snap guide;
  - the snap ring with a label (« Coïncidence »);
  - an angle arc with its value (« 90° »);
  - a **length field near the cursor**, amber border, with a text cursor.
- Scale bar at the bottom left (« 50 mm »).
> Decided otherwise in #722 (inventory E4): the grid keeps its spacing in millimetres, following the zoom.

## P9 — Sketch context pill (new)
- At the top centre of the view, only during a sketch.
- Content: « **Esquisse 3** · Plan XY », « N libertés restantes », then the primary button « Terminer ↵ » (which replaces the ribbon's « Terminer » button).

## P10 — View bar (new)
- At the bottom right of the view, on a floating background.
- Content:
  - « Recadrer » (F);
  - « Vue en coupe »;
  - « Matière devant : cachée ▾ » (hidden / see-through / opaque menu);
  - a separator;
  - the cursor's coordinates in monospace (« X 0,00 Y 0,00 mm »).
> Decided otherwise in #722 (inventory V4, V3): in the free 3D view the coordinates are the X Y Z of the point under the cursor; and « Recadrer » frames the whole part outside a sketch.

## P11 — Orientation cube
- **A real 3D cube** of 104 px at the top right, drawn in isometric.
  - The names (« DESSUS », « FACE », « DROITE »…) are laid **in perspective on the faces**.
  - A small X/Y/Z triad is drawn in the corner, with a rotation arrow.
  - The corner and the size stay settable (existing preference).
- In a sketch, the face seen from the front (« DESSUS » for the XY plane) is highlighted in amber.
- « Vue d'origine » button under the cube.
- Click on a face: the camera turns towards that face (existing behaviour).
> Decided otherwise in #722 (inventory V6, V9, V7): two arrows turn the view a quarter turn either way; and « Vue d'origine » is the isometric view the part opens with.

## P12 — Extrusion panel (C13)
- Content, in order:
  - **« Zones »**: chips of the chosen areas and « + cliquer dans la vue »;
  - **« Matière »**: « Ajout » / « Enlèvement » selector;
  - **« Profondeur »**: formula field with a variable block, and « = 30 mm » on the right;
  - **« Dépouille »**: in degrees;
  - « Autour d'un axe » and « Inverser le sens » switches;
  - « Annuler » / « Appliquer ↵ ».
- In the view: a preview of the volume in the addition colour (faces at 22, 30 and 40% opacity) and an amber height handle with its value (« 30 mm »).
> Decided otherwise in #722 (inventory O6, O7, O8): the panel ships with no draft field; the preview draws the volume alone, not yet fused; and the depth is dragged by a handle in the view.

## P13 — Circular pattern panel (C13 in steps) — model of the tools with several fields
1. **« Éléments à copier »**: chips, and « + cliquer dans la vue ».
2. **« Centre »**: a « Point (90 ; 50) » chip and a « Changer » link.
3. **« Nombre »** (−/+) and **« Pas »** (formula « 360 / 6 » → 60°).
4. **« Sens »**: « Anti-horaire » / « Horaire ».

- Then « Annuler » / « Valider ↵ ».
- In the view:
  - copies in preview (amber dashed circles), the circle joining the copies, and the radius;
  - centre mark with the label « Centre »;
  - arrowed arc with the step (« 60° »);
  - the source element is highlighted in amber.
- The title bar's message follows the step: « Réseau circulaire — étape 3 : nombre de copies ».
- **Same model** for: « Symétrie » (elements, axis), « Réseau rectangulaire » (elements, direction, count, step), « Chanfrein » and « Congé » (edge, values), « Vue en coupe » (face, distance, « Devant » / « Derrière », « Inverser »).
> Decided otherwise in #722 (inventory O4): the sketch's chamfer and fillet keep their fields by the cursor; only « Inverser les côtés » moves into a small panel.
