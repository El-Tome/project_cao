# Preferences

Screenshot: `screens/10-preferences-toolbar.png`. Window of 1120 × 760, radius 10.

## R1 — Header
- On the left: « ⚙ Préférences ».
- On the right:
  - the **« Profil »** list;
  - the « Dupliquer », « Exporter… », « Importer… » buttons, and « Supprimer » (disabled for the default profile);
  - the ✕ close cross.
- Profile management also stays in the « Profils » section.

## R2 — Left navigation (210 px)
- The 7 existing sections, each with its icon: « Profils », « Vue 3D », « Esquisse », « Navigation », « Apparence », « Raccourcis », « Barre d'outils ».
- Active section: background `background.hover_strong` and a 2 px amber line on the left.

## R3 — Toolbar section (the only mocked section)
- **« Position »**: C6 selector « En haut » / « En bas » / « À gauche » / « À droite » / « Flottante ».
- C7 switches: **« Logo »** and **« Noms des boutons »**.
- **Bar content** (on the left, 1.5 fr):
  - action bar: « Grouper », « Dégrouper », ↑, ↓, « Renommer », « + Séparateur »;
  - tree tab > group > command / menu, each node with its icon, its shortcut (C14) and its type (« onglet », « groupe · 8 », « menu · 5 »);
  - nodes are dragged to be moved.
> Decided otherwise in #722 (inventory F2, R1): « Retirer » is kept, though not drawn; and a group's size is edited in the tree.
- **Commands outside the bar** (on the right, 1 fr):
  - a search field;
  - the list of commands missing from the bar, each with an « Ajouter » button, and which can also be dragged into the tree. List: « Cote intelligente », « Cote point à point », « Cote de trait », « Cote d'angle », « Cote de rayon », « Inverser les côtés », « Appliquer l'extrusion », « Annuler l'extrusion », « Compacter l'historique », « Ancrer / détacher la barre », « Accueil ».
- Footer: the help « Glissez une commande vers l'arbre pour la placer. » and « Réinitialiser la barre d'outils ».
- **The tree's default content must reflect the new ribbon** (P4): groups « Dessin », « Cotes et règles », « Modifier », « Édition ».

## Sections not mocked
- « Profils », « Vue 3D », « Esquisse », « Navigation », « Apparence » and « Raccourcis »: keep the current content, laid out with the C6, C7 and C10 components.
- « Apparence » must offer the new alert colours (warning / error).
