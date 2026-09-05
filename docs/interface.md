# La barre d'outils et les panneaux

Voir aussi : [esquisse](esquisse.md) · [historique](historique.md)

## Disposition

```
┌──────────────────────────────────────────────┐
│ ⌂ Accueil │ nom de la pièce │ vue │ cote…     │  barre de titre
├──────────────────────────────────────────────┤
│ Esquisse │ Extrusion │ Assemblage             │  catégories
│ ─────────────────────────────────────────────│
│ Nouvelle esquisse │ Ligne │ Cote │ Annuler…   │  outils de la catégorie
├───────────┬──────────────────────────────────┤
│ Historique│                                  │
│  Esquisse │           viewport               │
│   1. Trait│                                  │
└───────────┴──────────────────────────────────┘
```

## La barre d'outils est détachable

Elle est **ancrée en haut par défaut**, et le bouton ⏏ la détache en une petite
fenêtre qu'on déplace où l'on veut ; 📌 la ré-ancre. C'est le même contenu dans
les deux cas — une seule fonction dessine l'intérieur, la seule différence est
le conteneur.

Deux rangées : les **catégories** (Esquisse, Extrusion, Assemblage) puis les
**outils** de celle qui est choisie. Les catégories sans outils sont grisées et
le disent au survol ; elles sont affichées quand même pour que la structure du
logiciel soit visible, et pour qu'en ajouter une revienne à remplir sa liste
d'outils.

## Le panneau Historique

À gauche, redimensionnable, masquable par le bouton « Historique » de la barre
d'outils. Son contenu est décrit dans [historique.md](historique.md).

## Ce qui manque

- Le ré-ancrage ne se fait qu'au bouton : faire glisser la fenêtre jusqu'en
  haut ne la rattache pas toute seule.
- La position de la fenêtre détachée et l'état des panneaux ne sont pas
  enregistrés d'une session à l'autre.
- Il n'y a pas de raccourcis clavier pour les outils.
