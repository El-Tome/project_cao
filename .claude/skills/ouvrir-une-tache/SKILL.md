---
name: ouvrir-une-tache
description: La procédure d'ouverture et de clôture d'une tâche sur ce dépôt CAO. À utiliser au tout début de chaque tâche — avant de lire du code — pour vérifier l'état du dépôt et les branches en cours, et à la fin pour nommer la branche, écrire le message de commit et pousser.
---

# Ouvrir, puis clore une tâche

## Avant de lire une ligne de code

Plusieurs personnes travaillent sur ce dépôt, et il y a une vingtaine de
branches distantes ouvertes. La première question n'est pas « comment faire »,
c'est « quelqu'un le fait-il déjà ».

```sh
git fetch --prune
git status
git for-each-ref --sort=-committerdate --count=15 \
    --format='%(committerdate:short)  %(refname:short)' refs/remotes/origin
```

Le hook `SessionStart` affiche déjà l'essentiel à l'ouverture d'une session.
S'il signale un retard sur `origin/main`, mets-toi à jour avant de commencer.

**Si une branche existante semble faire le travail demandé, dis-le et demande**
plutôt que de repartir de zéro : `git log origin/feat/xxx --oneline` en dit
souvent assez.

## La branche

Depuis `main` à jour :

```sh
git checkout main && git pull
git checkout -b feat/description-courte
```

`feat/` pour une fonctionnalité, `fix/` pour un correctif — c'est l'usage du
dépôt : `feat/circles`, `feat/oriented-dimensions`, `fix/solver-anchoring`,
`fix/tangent-circles`. En anglais, en tirets, court.

Ne travaille pas directement sur `main`.

## Pendant

Le test d'abord, puis le code — voir `rust-tdd`. Pour trouver où intervenir,
`carte-du-code`. Avant de valider, `revue-rust`.

Pour une question d'API sur `egui`, `wgpu` ou `glam`, **utilise `context7`**
plutôt que ta mémoire : le projet est sur `egui 0.36`, `wgpu 30` et
`glam 0.33`, des crates dont l'API casse à chaque version mineure.

## Le message de commit

Le dépôt écrit ses commits en **phrases françaises évocatrices**, qui disent ce
que le logiciel sait faire maintenant — pas ce que le développeur a tapé :

```
Cercles : poignées, tangences tenues, plantages tracés
Validation : l'origine seule rattache, l'orientation doit se dire
Seule l'origine rattache, et une forme penchée doit dire son sens
Le solveur recommence tant qu'il avance, et une figure se déplace d'un bloc
```

Une ligne de titre. Un corps seulement s'il apporte quelque chose : le pourquoi,
une alternative écartée, une conséquence non évidente. Jamais la liste des
fichiers touchés — `git` la porte déjà.

C'est aussi l'endroit où va le récit de la modification, celui qu'on est tenté
de mettre en commentaire dans le code.

## Le gate

`git commit` déclenche `clippy -D warnings` puis `cargo test --workspace`
(~12 s). Si ça échoue, le commit n'est pas exécuté et l'index n'est pas touché :
corrige et recommence.

Il existe une soupape, `CAO_SKIP_GATE=1`, pour commiter un travail en cours en
connaissance de cause. **Elle appartient à l'humain.** Ne la pose jamais de
toi-même, même après plusieurs échecs : un gate qui échoue dit quelque chose de
vrai sur le code. Si tu n'arrives pas à le faire passer, explique pourquoi et
demande.

Sur un clone neuf, le hook git demande une activation, une fois :

```sh
git config core.hooksPath .githooks
```

## Clore

```sh
git push -u origin feat/description-courte
```

Puis dis ce qui a été fait, et surtout ce qui **ne** l'a pas été : une partie
laissée de côté, une décision reportée, un test qu'on n'a pas su écrire. Ce qui
n'est pas dit à ce moment-là est perdu.
