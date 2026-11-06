# Home screen

Screenshots: `screens/08-home.png` and `screens/09-home-empty.png`.
- Top bar of 44 px: the « ⌂ Accueil » pill on the left, the « Préférences » link on the right.
- Body in two columns: the « Toutes les pièces » column (280 to 320 px) and the main content.

## A1 — All parts (new, on the left)
- **Header**: « Toutes les pièces » and a ↺ « Actualiser » button.
- **Actions**: « + Dossier », « Renommer », « Supprimer » (existing confirmation kept).
- **« Filtrer… » field**: filters the tree by name.
- **Tree of the parts folder**:
  - unfoldable folders with their number of parts;
  - one part per row with its 22 × 18 px thumbnail;
  - the chosen part is highlighted in amber.
- Behaviours (taken from the Files panel):
  - double-click to open;
  - drag a part onto a folder to move it;
  - the selected folder serves as the default location in A2.

## A2 — « Nouveau » card (replaces « Nouvelle pièce »)
- **Type**: C6 selector with **« Pièce »** (active) and **« Assemblage »**, disabled and marked « bientôt ». It is intended for the modes to come.
- **Name:** 40 px field.
- **« Enregistrer dans : »** (new)
  - a field with the folder icon, which shows the relative path (« Dossier des pièces / … »);
  - a « Choisir… » button that opens the folder selector;
  - the help « Par défaut : le dossier choisi à gauche. ».
- Primary button « Créer ↵ », full width, 44 px.
> Decided otherwise in #722 (inventory M2): « Enregistrer dans » only chooses a folder inside the parts folder, and « Choisir… » opens a list of those folders drawn by the application, not the system's dialog.

## A3 — Recent parts
- Card list. Each row holds:
  - a 64 × 48 thumbnail;
  - the name (14 px);
  - a detail line: folder · number of sketches · number of extrusions;
  - the date `%d/%m/%Y %H:%M` in monospace;
  - a › chevron.
- Counter « N pièces » on the right of the title.
- **Empty screen**: dashed frame, part icon, « Aucune pièce ouverte pour le moment. » and the help « Donnez un nom à gauche, puis Créer. ».

## A4 — Header
- « CAO » large: 52 px, amber, letter spacing 0.12 em.
- Subtitle: **text to supply** (marked `[Sous-titre de l'application]` in the mockup).
> Decided otherwise in #722 (inventory M5): the subtitle stays today's, « Nouvelle pièce ou reprise d'une pièce récente ».

## Sample data
The names in square brackets (`[Dossier A]`, `[résumé]`, `[JJ/MM/AAAA HH:MM]`) are placeholders to fill, not real values.
