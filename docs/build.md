# Compiler et distribuer

Voir aussi : [architecture](ARCHITECTURE.md)

## Lancer en développement

```sh
cargo run -p cao_app
```

## Exécutable Windows depuis macOS

Le projet se compile de façon croisée vers Windows sans machine Windows.

Prérequis, une seule fois :

```sh
rustup target add x86_64-pc-windows-gnu
brew install mingw-w64          # Debian/Ubuntu : apt install mingw-w64
```

Puis :

```sh
./scripts/build-windows.sh
```

L'exécutable sort dans `target/x86_64-pc-windows-gnu/release/cao.exe`
(~29 Mo). Il ne dépend que des DLL système de Windows — pas de DLL mingw à
copier à côté : le fichier `.exe` seul suffit, on le copie sur la machine
Windows et on double-clique.

Le linker est déjà configuré dans `.cargo/config.toml`, donc
`cargo build --release -p cao_app --target x86_64-pc-windows-gnu` fonctionne
aussi directement.

### Détails

- En release, l'exécutable est marqué comme application graphique
  (`windows_subsystem = "windows"`) : pas de fenêtre de console noire au
  lancement. En debug la console reste, pour voir les panics et les logs.
- La cible utilisée est `gnu` (mingw) et non `msvc`, parce que `msvc` demande
  les en-têtes et bibliothèques Microsoft, qui ne sont pas librement
  redistribuables. Le rendu passe par DX12 ou Vulkan dans les deux cas.

## Autres plateformes

Linux et macOS se compilent nativement avec `cargo build --release -p cao_app`.
Il n'y a pas encore d'intégration continue ni de paquets d'installation
(`.msi`, `.dmg`, AppImage) : c'est à faire quand le logiciel sera distribué.
