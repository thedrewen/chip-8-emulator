# Police CHIP-8 (sprites des caractères hexadécimaux)

Source : guide de Tobias V. Langhoff, section « Font » :
<https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#font>

C'est la police utilisée par la majorité des émulateurs. Tu peux en concevoir une autre : peu de jeux dépendent de la forme exacte des caractères.

## Rappels

- 16 caractères (`0`–`F`), **5 octets** chacun, soit **80 octets** au total.
- Chaque octet est une ligne du sprite : seuls les **4 bits de poids fort** sont utilisés (4 pixels de large), les 4 bits de poids faible valent toujours 0.
- Emplacement conventionnel en mémoire : `0x050`–`0x09F`.
- Adresse du caractère `n` : adresse de départ + 5 × `n`.

## Table

| Caractère | Octets | Aperçu (`#` = pixel allumé) |
|---|---|---|
| `0` | `0xF0, 0x90, 0x90, 0x90, 0xF0` | `####` / `#..#` / `#..#` / `#..#` / `####` |
| `1` | `0x20, 0x60, 0x20, 0x20, 0x70` | `..#.` / `.##.` / `..#.` / `..#.` / `.###` |
| `2` | `0xF0, 0x10, 0xF0, 0x80, 0xF0` | `####` / `...#` / `####` / `#...` / `####` |
| `3` | `0xF0, 0x10, 0xF0, 0x10, 0xF0` | `####` / `...#` / `####` / `...#` / `####` |
| `4` | `0x90, 0x90, 0xF0, 0x10, 0x10` | `#..#` / `#..#` / `####` / `...#` / `...#` |
| `5` | `0xF0, 0x80, 0xF0, 0x10, 0xF0` | `####` / `#...` / `####` / `...#` / `####` |
| `6` | `0xF0, 0x80, 0xF0, 0x90, 0xF0` | `####` / `#...` / `####` / `#..#` / `####` |
| `7` | `0xF0, 0x10, 0x20, 0x40, 0x40` | `####` / `...#` / `..#.` / `.#..` / `.#..` |
| `8` | `0xF0, 0x90, 0xF0, 0x90, 0xF0` | `####` / `#..#` / `####` / `#..#` / `####` |
| `9` | `0xF0, 0x90, 0xF0, 0x10, 0xF0` | `####` / `#..#` / `####` / `...#` / `####` |
| `A` | `0xF0, 0x90, 0xF0, 0x90, 0x90` | `####` / `#..#` / `####` / `#..#` / `#..#` |
| `B` | `0xE0, 0x90, 0xE0, 0x90, 0xE0` | `###.` / `#..#` / `###.` / `#..#` / `###.` |
| `C` | `0xF0, 0x80, 0x80, 0x80, 0xF0` | `####` / `#...` / `#...` / `#...` / `####` |
| `D` | `0xE0, 0x90, 0x90, 0x90, 0xE0` | `###.` / `#..#` / `#..#` / `#..#` / `###.` |
| `E` | `0xF0, 0x80, 0xF0, 0x80, 0xF0` | `####` / `#...` / `####` / `#...` / `####` |
| `F` | `0xF0, 0x80, 0xF0, 0x80, 0x80` | `####` / `#...` / `####` / `#...` / `#...` |

## Comment lire un octet

Prends `0x90` = `1001 0000` en binaire. Les 4 premiers bits (`1001`) donnent la ligne `#..#`, les 4 autres sont inutilisés. C'est ainsi que chaque ligne de l'aperçu est obtenue.

## Vérification après saisie

- 16 lignes × 5 octets = **80** valeurs. Un tableau de type `[u8; 80]` refuse toute autre longueur.
- Tous les octets se terminent par `0` en hexadécimal (`0xF0`, `0x90`, `0x20`, …).
