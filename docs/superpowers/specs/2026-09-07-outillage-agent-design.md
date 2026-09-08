# Outillage agent — conception

Date : 2026-09-07
Branche : `refactor/architecture`
État : validé, prêt pour le plan d'implémentation

## 1. Pourquoi

Le dépôt n'a aucun outillage agent : pas de `.claude/`, pas de skill, pas de
hook, pas de CI, pas de configuration de toolchain. Il ne reste que
`CLAUDE.md` (37 lignes) et `docs/` (1 881 lignes).

Ce vide a des effets mesurables.

`CLAUDE.md` énonce des règles sans procédure : rien ne dit *comment* mener une
tâche, les tests n'y sont jamais mentionnés alors que le dépôt en compte 176,
et la consigne « faire un pull et vérifier s'il n'y a pas déjà une branche »
n'est adossée à aucun outil — donc oubliée à chaque session.

`docs/` est de la prose produit, pas une carte du code. `esquisse.md` décrit
sur 869 lignes ce que *fait* l'esquisse, jamais *où intervenir*. Aucun fichier
ne relie un comportement à un module, ni ne liste les invariants à ne pas
casser.

Et la couverture de tests, laissée à la discipline seule, s'est creusée là où
le risque est le plus grand :

| Zone | Lignes | Tests |
| --- | ---: | ---: |
| `sketch/sketch.rs` | 2 372 | 62 |
| `core/state.rs` | 1 280 | 26 |
| `sketch/solver.rs` | 1 470 | **0** |
| `sketch/constraints.rs` | 283 | **0** |
| `crates/app/` (entier) | ~6 400 | **0** |

Le solveur de contraintes n'a aucun test et concentre quatre correctifs
récents (`fix/solver-anchoring`, `fix/tangent-circles`, `fix/circle-handling`,
`fix/dimension-handling`). C'est exactement le profil d'une zone où une
régression passe inaperçue.

## 2. Mesures de référence

Relevées le 2026-09-07 sur Apple Silicon, Rust 1.98.1 stable, après
installation de la toolchain (absente de la machine jusque-là).

| Commande | Durée | Résultat |
| --- | ---: | --- |
| `cargo fmt --all --check` | < 1 s | **échoue** — 90 différences |
| `cargo clippy --workspace --all-targets -- -D warnings` | 13 s | **passe, 0 warning** |
| `cargo test --workspace` | 11 s (36 s à froid) | **passe** |

Deux conséquences pour la conception.

`clippy` en mode strict passe déjà sur 19 822 lignes : le gate peut être
intransigeant dès le premier jour sans rien casser.

`rustfmt` échoue, mais le réparer demande un commit de reformatage global qui
entrerait en conflit avec la vingtaine de branches distantes ouvertes. Un des
90 écarts n'est d'ailleurs pas une faute : la table de correspondance clavier
de `app.rs` est compactée à la main, en table, et rustfmt l'éclate en 26
lignes. Aucun réglage de `rustfmt.toml` ne préserve cette mise en forme ; seul
`#[rustfmt::skip]` le fait.

## 3. Décisions

| Question | Décision |
| --- | --- |
| Périmètre | **Outillage seul.** Aucun `.rs`, aucun `Cargo.toml`, aucune doc existante n'est modifié — sauf `CLAUDE.md`. |
| Niveau de contrainte | **Hooks bloquants**, pas de simples règles écrites. |
| Structure | **Skills spécifiques à ce workspace**, pas un pack générique importé. |
| Couches de gate | **Les trois** : hook Claude Code, hook git `pre-commit`, CI GitHub. |
| rustfmt | **Hors du gate** pour l'instant. `rustfmt.toml` est livré, la CI vérifie le format en avertissement non bloquant. — *reprise le 2026-09-08, voir sous la table* |
| MCP | **`context7` seul.** |

**Décision du 2026-09-08 (#60) — `rustfmt` bloque.** L'exemption avait un motif
nommé : la table de correspondance clavier de `app.rs`, compactée à la main, que
rustfmt éclatait en 26 lignes, et un commit de reformatage global qui serait
entré en conflit avec la vingtaine de branches distantes alors ouvertes (§ 2).
Ce commit a eu lieu — `4f81da6`, 22 fichiers — et `grep -rn 'rustfmt::skip'
crates/` ne renvoie rien : le formatage de rustfmt a été accepté partout, il n'y
a plus d'exception à protéger. `cargo fmt --all --check` sort en 0 sur l'arbre
actuel, donc le remettre dans le gate ne bloque rien de ce qui existe et coûte
environ 0,3 s par commit. `continue-on-error` disparaît de `ci.yml` — avec cette
option la conclusion du job était `success`, si bien qu'une branche mal formatée
n'allumait rien nulle part, même déclaré vérification requise.

### Pourquoi ces choix

**Trois couches de gate, parce qu'elles ne protègent pas les mêmes personnes.**
Un hook Claude Code n'intercepte que les commandes lancées par l'agent : il ne
voit rien d'un commit fait depuis un terminal ou depuis JetBrains. Le hook git
couvre tout commit local. La CI couvre la branche distante, donc les
contributeurs qui n'ont pas configuré leurs hooks.

| Couche | Bloque | Contournable par |
| --- | --- | --- |
| Hook Claude Code | l'agent | rien |
| Hook git `pre-commit` | tout commit local | `git commit --no-verify` |
| CI GitHub | toute branche poussée | rien |

**Un seul MCP.** Le projet dépend d'`egui 0.36`, `wgpu 30` et `glam 0.33` —
des crates dont l'API casse à chaque version mineure et sur lesquelles la
connaissance d'un modèle dérive. `context7` sert des docs versionnées et
répare ce manque précis. Les serveurs MCP enveloppant `rust-analyzer` sont peu
maintenus et `cargo check` fait mieux : aucun n'est ajouté.

**Des skills qui connaissent ce code.** Un skill générique sur SOLID ne vaut
rien. Un skill qui dit « avant de toucher `solver.rs`, écris un test
caractérisant l'existant, parce qu'il n'en a aucun » vaut quelque chose. C'est
le critère de tri de tout ce qui suit.

## 4. Ce qui est livré

```
.claude/
├── settings.json                       versionné : hooks + permissions
├── skills/
│   ├── carte-du-code/SKILL.md
│   ├── rust-tdd/SKILL.md
│   ├── architecture-rust/SKILL.md
│   ├── revue-rust/SKILL.md
│   └── ouvrir-une-tache/SKILL.md
├── agents/
│   └── revue-archi-rust.md
└── hooks/
    ├── session-start.sh
    └── gate-commit.sh
.githooks/pre-commit
.mcp.json
rust-toolchain.toml
rustfmt.toml
clippy.toml
.github/workflows/ci.yml
CLAUDE.md                               refondu
docs/carte-du-code.md                   nouveau
.gitignore                              + .claude/settings.local.json
```

## 5. Les skills

Chaque skill est un `SKILL.md` avec un frontmatter `name` + `description`. La
`description` dit **quand** déclencher le skill — c'est le seul champ lu avant
chargement, il doit contenir les mots que l'on emploierait naturellement.

Les skills sont rédigés en **français**, comme `CLAUDE.md` et `docs/`. Le code
et les noms de tests restent en anglais, conformément à l'usage établi du
dépôt (`a_crash_is_written_down_with_its_hour_and_its_stack`).

### 5.1 `carte-du-code`

Le pont manquant entre `docs/` et les 40 fichiers de `crates/`.

Contenu : une table comportement → crate → fichier → fonction d'entrée,
couvrant les cinq crates. Puis les invariants du projet, chacun avec sa
raison :

- le noyau calcule en `f64`, la caméra et le rendu en `f32`, et la conversion
  se fait au dernier moment à chaque passage de frontière ;
- la géométrie n'est jamais enregistrée : elle est rejouée depuis l'historique
  par `PartState::rebuild`, ce qui rend annulation, rétablissement et retour à
  une étape identiques ;
- un nouveau mode est une variante de `Screen` plus un module dans `screens/`,
  jamais une branche ajoutée à un module existant ;
- `cao_core` ne dépend d'aucune crate UI.

Déclenchement : dès qu'il s'agit de trouver où intervenir dans le code.

### 5.2 `rust-tdd`

La boucle rouge/vert/refactor appliquée à ce dépôt.

- **Un test à la fois.** Jamais tous les tests puis tout le code : des tests
  écrits en lot vérifient un comportement imaginé, pas le comportement réel.
- **Où le poser.** `mod tests` colocalisé en bas de fichier, la convention du
  dépôt ; `crates/<crate>/tests/` pour l'intégration, sur le modèle de
  `crates/core/tests/stress_tangent.rs`.
- **Comment boucler vite.** `cargo test -p cao_sketch <filtre>` plutôt que le
  workspace entier.
- **Jamais `==` entre deux `f64`.** Comparaison par tolérance, avec une
  tolérance choisie et justifiée, pas copiée.
- **Zones sans filet.** `solver.rs` et `constraints.rs` n'ont aucun test et
  concentrent quatre correctifs récents ; `crates/app/` n'en a aucun sur
  ~6 400 lignes. Toute intervention dans ces fichiers commence par un test qui
  caractérise l'existant avant de le modifier.

### 5.3 `architecture-rust`

SOLID et ports & adapters appliqués à ce workspace.

- Le graphe de dépendances autorisé entre les cinq crates.
- La règle des ports : aucun `std::fs`, `directories`, ni `chrono::Utc::now()`
  sous une frontière de domaine sans passer par un trait. Justification
  concrète : les tests de persistance écrivent aujourd'hui dans
  `std::env::temp_dir()` et font des `remove_dir_all`, ce qui les rend lents et
  non parallélisables sans risque.
- Le patron Rust pour introduire un port : trait, implémentation réelle,
  implémentation de test.
- **Les écarts actuels, nommés.** `cao_core` dépend de `cao_sketch` et
  `cao_solid` : c'est en réalité la couche application, pas le domaine, malgré
  ce qu'affirme sa documentation. L'I/O est en dur dans `document.rs`,
  `recents.rs`, `settings.rs` et `storage.rs`. Ces endroits sont de la dette
  connue et ne doivent jamais servir de modèle à recopier.

### 5.4 `revue-rust`

Checklist de relecture avant commit.

- SRP, avec des seuils chiffrés — `viewport.rs` fait 4 179 lignes et 105
  fonctions, `sketch.rs` 2 372 ; ce sont les repoussoirs.
- OCP sur les `match` exhaustifs, `PartState::apply` en exemple : chaque
  nouvelle opération oblige à rouvrir la fonction.
- `unwrap()`, `expect()`, `panic!()` hors code de test.
- `pub` posé par défaut alors que le champ ou la fonction pourrait rester privé.
- Erreurs : `thiserror`, jamais une variante portant une `String` libre.
- Allocation à l'intérieur d'une boucle de rendu.

### 5.5 `ouvrir-une-tache`

La procédure que `CLAUDE.md` demande sans l'outiller.

`git fetch`, comparaison aux branches distantes existantes — il y en a une
vingtaine — pour ne pas refaire une fonctionnalité déjà en cours, création
d'une branche `feat/…` ou `fix/…`, puis le style de message de commit du dépôt :
une phrase française évocatrice, sur le modèle de « Cercles : poignées,
tangences tenues, plantages tracés ».

## 6. Le sous-agent

`.claude/agents/revue-archi-rust.md` — sous-agent en lecture seule qui applique
`revue-rust` et `architecture-rust` à un diff et rend une liste de constats
classés par gravité. Outils : lecture, recherche, `cargo` en lecture seule. Il
ne modifie rien.

Il existe pour que la revue se fasse dans un contexte séparé du contexte
d'écriture : celui qui vient d'écrire le code est mal placé pour le juger.

## 7. Les hooks

Les trois scripts exportent `PATH="$HOME/.cargo/bin:$PATH"` en préambule : les
hooks tournent dans un shell non interactif qui ne lit pas `~/.zprofile`, et
`cargo` y serait autrement introuvable.

### 7.1 `session-start.sh` — informatif

Déclencheur : `SessionStart`. Ne bloque jamais.

Fait un `git fetch --prune`, puis affiche la branche courante, son écart avec
`origin/main`, les branches distantes les plus récentes, et l'état de l'arbre
de travail. Signale `cargo` introuvable le cas échéant.

Effet visé : la consigne « pull et vérifie les branches » de `CLAUDE.md` cesse
de dépendre de la mémoire de l'agent.

### 7.2 `gate-commit.sh` — bloquant

Déclencheur : `PreToolUse` sur `Bash`. Le script lit le JSON sur son entrée
standard et ne fait quelque chose que si la commande contient `git commit`.

Enchaîne, en s'arrêtant au premier échec :

1. `cargo fmt --all --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`

Le format est en tête parce qu'il est le moins cher — décision § 3.

En cas d'échec : sortie en code 2, le commit n'est jamais exécuté, la sortie de
la commande fautive est renvoyée à l'agent qui corrige et recommence. L'index
git n'est pas touché.

Si `cargo` est introuvable : refus explicite, message « rustup non installé,
gate non vérifiable », et non un `command not found` avalé.

Soupape : si `CAO_SKIP_GATE=1` est présent dans l'environnement, le gate
s'efface en le signalant. Le skill `ouvrir-une-tache` porte la consigne que
l'agent ne pose jamais cette variable de lui-même — elle appartient à
l'humain, pour ses travaux en cours.

### 7.3 `.githooks/pre-commit` — bloquant, versionné

Même enchaînement, pour tout commit local quel qu'en soit l'auteur. Versionné
dans le dépôt donc relisible et modifiable comme le reste du code.

Activation, une fois par clone : `git config core.hooksPath .githooks`. Cette
commande est documentée dans `CLAUDE.md` et rappelée par `session-start.sh`
tant qu'elle n'a pas été passée.

Contournable par `git commit --no-verify`, ce qui est le comportement attendu
d'un hook git et la raison d'être de la troisième couche.

## 8. `settings.json`

Versionné, donc partagé par l'équipe. Déclare les deux hooks et pré-autorise
les commandes de lecture et de vérification que l'agent lance en boucle :
`cargo test`, `cargo clippy`, `cargo check`, `cargo build`, `cargo fmt`,
`git status`, `git diff`, `git log`, `git fetch`.

`.claude/settings.local.json` — préférences personnelles — est ajouté au
`.gitignore`.

## 9. `.mcp.json`

Déclare `context7` en HTTP, avec sa clé d'API lue dans l'environnement
(`CONTEXT7_API_KEY`), sur le modèle du plugin déjà installé globalement. Un
skill n'est pas nécessaire : le déclencheur naturel est une question d'API sur
`egui`, `wgpu` ou `glam`, et `CLAUDE.md` porte la consigne d'y recourir plutôt
que de se fier à une mémoire datée.

## 10. Hors `.claude/`

### `rust-toolchain.toml`

Épingle `stable` 1.98.1 avec les composants `clippy` et `rustfmt`, pour que la
machine de chacun et la CI compilent le même code. L'édition 2024 du workspace
exige au moins 1.85.

### `rustfmt.toml` et `clippy.toml`

Livrés dès le premier jour, alors que `fmt` n'entrait pas encore dans le gate :
leur présence fixait le style visé en attendant qu'il y entre.

### `.github/workflows/ci.yml`

Sur chaque push, quelle que soit la branche. Toolchain épinglée par
`rust-toolchain.toml`, cache `Swatinem/rust-cache`.

| Job | Commande | Bloquant |
| --- | --- | --- |
| `fmt` | `cargo fmt --all --check` | oui |
| `clippy` | `cargo clippy --workspace --all-targets -- -D warnings` | oui |
| `test` | `cargo test --workspace` | oui |
| `build-windows` | `scripts/build-windows.sh` | oui |

Amendé le 2026-09-08 (#57) : `build-windows` ne tournait que sur `main`, donc
jamais avant une fusion. Il tourne maintenant sur chaque push ; seul l'envoi de
l'artefact reste réservé à `main`.

Amendé le 2026-09-08 (#58) : `on: push` couvrait `main` seulement, donc une
branche poussée sans PR ouverte n'était vérifiée nulle part. Il couvre
maintenant toute branche, et `on: pull_request` disparaît en échange — les deux
ensemble feraient tourner le workflow deux fois à chaque push. Ce qui est cédé :
GitHub vérifiait la branche *fusionnée* avec sa base, il vérifie maintenant sa
tête. Une branche verte mais en retard sur `main` reste possible ; c'est le
rebase avant fusion qui la rattrape.

Le runner Ubuntu reçoit une étape `apt-get` installant les dépendances système
de `winit`/`wgpu` (`libxkbcommon-dev`, `libwayland-dev`, `libxcb*`) : `eframe`
est compilé avec les features `x11` et `wayland`, et sans elles la CI échoue à
l'édition de liens, pas au test.

Aucun test GPU n'est exécuté. Les tests de `cao_render` (`camera.rs`,
`geometry.rs`, `cube.rs`) sont du calcul pur et passent sans carte graphique.
`examples/offscreen.rs` a besoin d'un vrai device `wgpu` : il est compilé, pas
exécuté. Le contrôle visuel des trois PNG reste manuel, en local.

### `CLAUDE.md` refondu

Les règles existantes sont conservées mot pour mot. S'y ajoutent : la boucle de
travail, un pointeur vers les skills, la commande d'activation des hooks git,
la consigne de recourir à `context7` pour les API `egui`/`wgpu`/`glam`.

Une correction : `CLAUDE.md` et `ARCHITECTURE.md` présentent `cao_core` comme
la crate des « types de domaine ». C'est faux — elle dépend de `cao_sketch` et
`cao_solid` et orchestre esquisse, solide, historique et persistance. C'est la
couche application. La formulation est corrigée dans `CLAUDE.md` ;
`ARCHITECTURE.md` n'est pas touché, le périmètre l'exclut, et le point est
consigné dans `architecture-rust` comme dette connue.

### `docs/carte-du-code.md`

Le contenu de référence du skill `carte-du-code`, sous une forme lisible aussi
par un humain qui arrive sur le projet.

## 11. Hors périmètre

Explicitement exclus de cette tâche :

- toute modification d'un `.rs` ou d'un `Cargo.toml` ;
- le commit de reformatage `cargo fmt --all` et les `#[rustfmt::skip]` associés ;
- le refactor architectural : inversion de `cao_core`, ports sur la
  persistance, découpe de `viewport.rs` ;
- l'écriture des tests manquants du solveur et de `crates/app/` ;
- la cible `x86_64-pc-windows-gnu` et `mingw-w64`, non installés.

Ces points sont réels et documentés, mais chacun est une tâche distincte à
décider séparément.

## 12. Critères d'acceptation

1. `.claude/skills/` contient cinq skills, chacun avec un frontmatter `name` et
   `description` valide.
2. Une session ouverte dans le dépôt affiche l'état git sans qu'on le demande.
3. Un `git commit` tenté alors qu'un test échoue est refusé, et le message
   d'échec du test est visible.
4. Le même commit passe une fois le test réparé.
5. `CAO_SKIP_GATE=1 git commit` passe, en signalant que le gate a été sauté.
6. Le gate refuse avec un message explicite si `cargo` est absent du `PATH`.
7. `git config core.hooksPath .githooks` suffit à faire jouer le hook git.
8. La CI passe sur la branche, `fmt` en avertissement, `clippy` et `test` au vert.
9. `cargo test --workspace` et `cargo clippy --workspace --all-targets -- -D
   warnings` restent au vert : aucun fichier source n'a été touché.
