# Le rendu GPU (`cao_render`)

Le crate `cao_render` dessine le contenu du viewport avec `wgpu`. Il ne dépend
d'aucune bibliothèque d'interface : il reçoit une description de la frame et la
dessine. C'est ce qui permettra de le réutiliser tel quel derrière un futur
front-end tablette ou web.

Voir aussi : [viewport](viewport.md) · [navigation](navigation.md)

## Découpage des fichiers

| Fichier | Rôle |
| --- | --- |
| `camera.rs` | Caméra orbitale, faces du cube, plans, animation de vue |
| `geometry.rs` | Construction des sommets : axes, grille adaptative, couleurs |
| `cube.rs` | Géométrie du cube d'orientation et détection de face cliquée |
| `renderer.rs` | Pipelines wgpu, buffers, uniformes, appels de dessin |
| `shaders/scene.wgsl` | Les deux shaders : lignes épaisses et faces pleines |

## Le contrat : `SceneFrame`

La couche UI construit à chaque frame une `SceneFrame` (matrices, sommets,
rectangle du cube en pixels physiques) et la passe au renderer. Aucun état de
caméra ni de souris ne vit dans `cao_render` — juste des données de dessin.

## Deux pipelines, pas de depth buffer

Il n'y a **aucun tampon de profondeur**, et ce n'est pas un oubli :

- la scène n'est faite que de lignes, dont l'ordre de dessin suffit ;
- le cube est un solide convexe dessiné en dernier : le simple *back-face
  culling* suffit à ne montrer que les faces avant. Ses arêtes ne sont émises
  que pour les faces visibles, sinon celles de derrière transperceraient le
  solide.

Le jour où de vraies pièces ombrées seront affichées, il faudra ajouter un
depth buffer — ce qui suppose de configurer l'attachement correspondant côté
egui.

## Lignes épaisses

`wgpu` ne sait pas dessiner de ligne large : la topologie `LineList` donne
toujours 1 pixel, illisible sur un écran haute densité. Chaque segment est donc
envoyé comme **une instance** et déplié dans le vertex shader en un quad de la
largeur voulue, en espace écran.

Astuce du format : le buffer reste une simple suite de paires de sommets ; c'est
la `VertexBufferLayout` qui la relit avec un pas de deux sommets par instance.
Les constructeurs de géométrie n'ont donc rien de spécial à faire.

Un segment qui traverse le plan de la caméra est coupé au plan proche dans le
shader : diviser par un `w` négatif renverrait le point de l'autre côté de
l'écran. C'est le cas courant des axes, qui traversent toute la scène.

## Couleurs

Les couleurs sont manipulées en **linéaire** dans les shaders. `geometry::srgb`
convertit une couleur sRGB (celle qu'affiche un sélecteur de couleur) vers le
linéaire. Si la surface de destination n'est pas sRGB, le fragment shader fait
lui-même la conversion inverse — d'où le drapeau passé dans les uniformes.

## Vérifier le rendu sans fenêtre

```sh
cargo run -p cao_render --example offscreen -- /tmp
```

Écrit trois PNG (vue libre, plan XY, survol du cube) en exécutant le vrai
pipeline GPU. Utile pour contrôler un rendu ou comparer avant/après une
modification, sans avoir à ouvrir l'application.

Les tests unitaires (`cargo test -p cao_render`) couvrent ce qu'une image ne
vérifie pas : le pas de grille, les angles de caméra de chaque face et le
lancer de rayon du cube.
