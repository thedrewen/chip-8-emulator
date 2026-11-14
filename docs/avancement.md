# Avancement du projet et notes de passation

Document destiné à un agent (ou à une personne) qui reprend l'accompagnement. Dernière mise à jour : 2026-11-10.

## Contexte et règles du jeu

- Projet d'apprentissage : l'utilisateur construit un **émulateur CHIP-8 en Rust** pour apprendre le langage.
- Le rôle de l'agent est celui d'un **mentor**, pas d'un développeur. Les règles complètes sont dans `CLAUDE.md` à la racine : à lire en premier.
- Résumé des règles :
  - ne jamais écrire ni modifier le code du projet ;
  - pas de solution déguisée (pas de pseudo-code détaillé ni de squelette) ;
  - exemples de code uniquement **génériques**, sans rapport avec l'émulateur (ex. liste de courses, minuteur), sans reprendre ses noms ni ses types ;
  - **expliquer directement la théorie** nouvelle pour lui (la question vient après l'explication, pas à la place) ;
  - indices légers d'abord pour les choix de conception ;
  - réponses concises, en français.
- Préférence explicite de l'utilisateur : **aucun test pour l'instant**, il les écrira à la fin du projet. Ne pas en suggérer, ne pas signaler les tests cassés ou absents. Se limiter à `cargo check` et `cargo clippy`.
- Niveau : bases de Rust (syntaxe, structs, enums, traits simples, borrowing de base). Les opérations sur les bits, les bases de numération (hexadécimal, binaire), `Result` et `?`, et les notions de slice ont dû être expliquées. Il est débutant sur l'hexadécimal : l'expliquer avec des tables (1 chiffre hexa = 4 bits) plutôt qu'avec des conversions décimales.

## Documentation du dépôt

- `docs/chip8.md` : référence conceptuelle de l'architecture CHIP-8 (mémoire, registres, format des opcodes, 35 instructions, `DXYN`, clavier, timers, quirks).
- `docs/police.md` : table des 80 octets de la police intégrée, avec aperçu en pixels.
- `tests/test.ch8` : ROM de test utilisée par `main`.

## État du code

Fichiers : `src/main.rs`, `src/computer.rs`. Pas de dépendances (`Cargo.toml` vide), édition 2024.

### Fait

- Struct `Computer` avec tous les champs de la machine : `pc: u16`, `memory: [u8; 4096]`, `registers: [u8; 16]`, `index_register: u16`, `call_stack: [u16; 16]`, `stack_pointer: u8`, `delay_timer`, `sound_timer: u8`, `screen: [[bool; 64]; 32]` (indexé `[ligne][colonne]`), `keyboard: [bool; 16]`.
- `Computer::new()` : initialise tous les champs, `pc` à `0x200`, timers à 0, et charge la police via `initialize_memory()` (fonction associée privée qui renvoie la mémoire avec la police recopiée en `80..160`, c'est-à-dire `0x50..0xA0`).
- Constantes `FONTS_MIN_ADDR` (80) et `FONTS_MAX_ADDR` (160, borne **exclue**).
- `insert_rom(&mut self, file: String)` : lit le fichier avec `fs::read`, refuse plus de 3584 octets (`panic!`), copie à partir de `0x200`.
- `main` : crée la machine, charge `tests/test.ch8`, affiche l'état en `{:?}`.
- Les 80 octets de la police ont été vérifiés par l'agent : conformes à `docs/police.md`.

### État du compilateur

`cargo check` : un seul avertissement, `dead_code` sur les champs jamais lus. Il disparaîtra quand les opcodes les utiliseront. Aucune erreur. (Les tests unitaires ont été retirés par l'utilisateur.)

### Points de qualité identifiés mais non traités (à son rythme)

- `insert_rom` utilise `expect` et `panic!` : à remplacer par un `Result` avec `?`. L'explication de `Result`, `?`, et des choix de type d'erreur (`String`, `Box<dyn Error>`, enum personnalisé) a déjà été donnée. Il y a deux causes d'échec distinctes : lecture du fichier et ROM trop grosse.
- Le paramètre `file: String` pourrait être une référence ou un type générique convertible en chemin (`AsRef<Path>`).
- Valeurs magiques : `3584` (= 4096 − 0x200) et `0x200` (dupliqué entre `new` et `insert_rom`) gagneraient à être des constantes nommées.
- Le nom `FONTS_MAX_ADDR` désigne une borne exclue, ce qui peut prêter à confusion (`END` est une convention plus claire).
- Le tableau de la police est en variable locale ; une `const` au niveau du fichier serait plus lisible. `cargo fmt` a aplati la table (une ligne par caractère perdue) ; `#[rustfmt::skip]` permet de la conserver si souhaité.

## Étapes suivantes (ordre proposé)

1. **Fetch** : lire les 2 octets à `pc` et les assembler en un opcode de 16 bits (big-endian), attention à la conversion `u16` vers `usize` pour indexer. Avancer `pc` de 2.
2. **Decode** : extraire le premier nibble, `X`, `Y`, `N`, `NN`, `NNN` par masques et décalages. Le dernier point d'explication donné est précisément cette technique (masque `&`, décalage `>>`, assemblage `|`/`<<`). Un exercice était en attente de réponse : calculer `0x6A2F & 0x00FF`, `0x6A2F & 0xF000` et `(0x6A2F & 0xF000) >> 12`.
3. **Premiers opcodes** simples : `00E0`, `1NNN`, `6XNN`, `7XNN`, `ANNN`, puis le reste (voir `docs/chip8.md`).
4. **Affichage** : `DXYN` et choix d'une crate de fenêtre (`minifb`, `pixels`, `sdl2`…).
5. **Clavier, timers (60 Hz), boucle principale** (CPU ~700 Hz découplé des timers).
6. **Validation** avec la suite de ROMs de Timendus (<https://github.com/Timendus/chip8-test-suite>) et les quirks (section 9 de `docs/chip8.md`).
7. **Tests unitaires**, à la demande de l'utilisateur, à la fin.

## Comment l'utilisateur travaille

- Il écrit un morceau de code, puis dit « voilà » : l'agent le relit (lecture seule), pointe ce qui va bien, ce qui est non idiomatique, les bugs, sans réécrire.
- Il demande souvent des explications théoriques ciblées (slices, `&mut`, bases de numération). Y répondre directement, avec un exemple sans rapport avec le projet.
- Il lance lui-même ses modifications ; l'agent peut exécuter `cargo build/check/test/clippy` pour lire les retours du compilateur, jamais pour corriger.
