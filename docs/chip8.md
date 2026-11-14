# CHIP-8 : référence conceptuelle

CHIP-8 est une machine virtuelle des années 70, conçue pour écrire des jeux simples sur de petits ordinateurs. Un « émulateur » CHIP-8 est donc en réalité un **interpréteur** : il lit des instructions de 2 octets et simule l'effet qu'elles ont sur un état interne (mémoire, registres, écran).

Ce document décrit *ce que fait la machine*, pas comment l'implémenter.

---

## 1. Vue d'ensemble de l'architecture

| Élément | Taille | Rôle |
|---|---|---|
| Mémoire | 4096 octets (`0x000`–`0xFFF`) | Contient police, programme et données |
| Registres généraux `V0`–`VF` | 16 × 8 bits | Calculs et données. `VF` sert aussi de **drapeau** (retenue, collision…) |
| Registre d'index `I` | 16 bits (12 utiles) | Pointe une adresse mémoire (sprites, données) |
| Compteur de programme `PC` | 16 bits (12 utiles) | Adresse de la prochaine instruction |
| Pile d'appels | 16 niveaux (adresses 16 bits) | Stocke les adresses de retour des sous-routines |
| Pointeur de pile | petit entier | Indique le sommet de la pile |
| Timer de délai (*delay*) | 8 bits | Décrémente à 60 Hz jusqu'à 0 |
| Timer de son (*sound*) | 8 bits | Décrémente à 60 Hz ; tant qu'il est > 0, un bip est émis |
| Écran | 64 × 32 pixels, 1 bit par pixel | Monochrome : un pixel est allumé ou éteint |
| Clavier | 16 touches (`0`–`F`) | Appuyée ou relâchée |

Points à retenir :

- Les adresses tiennent sur **12 bits** (4096 = 2¹²), mais on les stocke dans des types plus larges.
- Tous les registres `V` sont sur **8 bits** : les débordements (> 255) et sous-dépassements (< 0) font partie du comportement à gérer.
- Il n'y a pas de pile de données : la pile ne sert qu'aux adresses de retour.

## 2. Plan mémoire

```
0x000 ┌──────────────────────┐
      │ Réservé (historique) │  ← l'interpréteur d'origine vivait ici
0x050 │ Police (sprites 0-F) │  ← emplacement conventionnel, pas obligatoire
0x0A0 │ ...                  │
0x200 ├──────────────────────┤
      │ Programme (ROM)      │  ← les ROMs sont chargées ici
      │ + données            │
0xFFF └──────────────────────┘
```

- **Les programmes commencent à `0x200`.** `PC` doit donc y être initialisé et la ROM copiée à partir de cette adresse.
- **La police** : 16 caractères hexadécimaux (`0`–`F`), chacun est un sprite de **5 octets**. Tu dois les placer quelque part sous `0x200` (convention : `0x050`–`0x09F`). L'instruction `FX29` fournira l'adresse du caractère voulu. Les 80 octets de données de la police se trouvent facilement dans n'importe quelle spec.
- La taille maximale d'une ROM est donc 4096 − 512 = **3584 octets**.

## 3. Format des instructions

Chaque instruction (**opcode**) fait **2 octets**, stockés en **big-endian** (octet de poids fort en premier). On la lit donc en combinant deux octets consécutifs de la mémoire.

On découpe un opcode en *nibbles* (4 bits) et on utilise des noms conventionnels pour les champs :

```
  opcode = 0xABCD   (16 bits)

  ┌───┬───┬───┬───┐
  │ A │ B │ C │ D │   chaque lettre = 1 nibble (4 bits)
  └───┴───┴───┴───┘
```

| Champ | Définition | Usage |
|---|---|---|
| premier nibble | bits 15–12 | Famille d'instruction (sert au `match` principal) |
| `X` | bits 11–8 | Index d'un registre `VX` |
| `Y` | bits 7–4 | Index d'un registre `VY` |
| `N` | bits 3–0 | Constante de 4 bits |
| `NN` | bits 7–0 | Constante de 8 bits |
| `NNN` | bits 11–0 | Adresse de 12 bits |

Extraire ces champs est un exercice de **masques et décalages** (`&`, `>>`).

## 4. Le cycle d'exécution

À chaque « tick » du processeur :

1. **Fetch** : lire les 2 octets à l'adresse `PC`, les assembler en un opcode.
2. **Avancer `PC` de 2** (généralement *avant* d'exécuter, car certaines instructions le modifient).
3. **Decode** : identifier l'instruction via le premier nibble puis, selon la famille, d'autres champs.
4. **Execute** : appliquer l'effet.

Cadences :
- Le CPU tourne typiquement à **~500–700 instructions/seconde** (ce n'est pas fixé par la spec, c'est à régler selon le jeu).
- Les **timers** et l'**affichage** sont à **60 Hz**, indépendamment du CPU. Il faut donc découpler ces deux cadences dans ta boucle principale.

## 5. Jeu d'instructions (35 opcodes)

Notation : `VX`/`VY` = registres désignés par `X`/`Y`. « Saute » = `PC` avance de 2 de plus (on ignore l'instruction suivante).

### Contrôle de flux

| Opcode | Effet |
|---|---|
| `00E0` | Efface l'écran |
| `00EE` | Retour de sous-routine : dépile l'adresse et la met dans `PC` |
| `1NNN` | Saut : `PC` = `NNN` |
| `2NNN` | Appel de sous-routine : empile `PC` actuel, puis `PC` = `NNN` |
| `3XNN` | Saute si `VX` == `NN` |
| `4XNN` | Saute si `VX` != `NN` |
| `5XY0` | Saute si `VX` == `VY` |
| `9XY0` | Saute si `VX` != `VY` |
| `BNNN` | Saut : `PC` = `NNN` + `V0` (voir *quirks*) |

### Registres et arithmétique

| Opcode | Effet |
|---|---|
| `6XNN` | `VX` = `NN` |
| `7XNN` | `VX` += `NN` (**sans** toucher `VF`, débordement silencieux) |
| `8XY0` | `VX` = `VY` |
| `8XY1` | `VX` \|= `VY` (OU binaire) |
| `8XY2` | `VX` &= `VY` (ET binaire) |
| `8XY3` | `VX` ^= `VY` (OU exclusif) |
| `8XY4` | `VX` += `VY`; `VF` = 1 s'il y a retenue (> 255), sinon 0 |
| `8XY5` | `VX` -= `VY`; `VF` = 1 s'il **n'y a pas** d'emprunt (`VX` ≥ `VY`), sinon 0 |
| `8XY6` | Décalage à droite de 1 bit ; `VF` = bit sorti (voir *quirks*) |
| `8XY7` | `VX` = `VY` − `VX`; `VF` = 1 s'il n'y a pas d'emprunt, sinon 0 |
| `8XYE` | Décalage à gauche de 1 bit ; `VF` = bit sorti (voir *quirks*) |
| `CXNN` | `VX` = nombre aléatoire (0–255) ET `NN` |

### Index et mémoire

| Opcode | Effet |
|---|---|
| `ANNN` | `I` = `NNN` |
| `FX1E` | `I` += `VX` |
| `FX29` | `I` = adresse du sprite du caractère hexadécimal contenu dans `VX` |
| `FX33` | Décomposition décimale de `VX` : centaines à `I`, dizaines à `I+1`, unités à `I+2` |
| `FX55` | Copie `V0`..`VX` (inclus) en mémoire à partir de `I` |
| `FX65` | Charge `V0`..`VX` (inclus) depuis la mémoire à partir de `I` |

### Affichage

| Opcode | Effet |
|---|---|
| `DXYN` | Dessine un sprite (voir section 6) |

### Clavier

| Opcode | Effet |
|---|---|
| `EX9E` | Saute si la touche dont le numéro est dans `VX` est **appuyée** |
| `EXA1` | Saute si la touche dont le numéro est dans `VX` est **relâchée** |
| `FX0A` | **Bloque** jusqu'à ce qu'une touche soit appuyée, puis stocke son numéro dans `VX` |

### Timers

| Opcode | Effet |
|---|---|
| `FX07` | `VX` = valeur du timer de délai |
| `FX15` | Timer de délai = `VX` |
| `FX18` | Timer de son = `VX` |

> `0NNN` (appel d'une routine machine) existe dans la spec d'origine mais est ignoré par les émulateurs modernes.

## 6. L'écran et l'instruction `DXYN`

L'écran est une grille de 64 × 32 pixels, chacun allumé (1) ou éteint (0).

**Un sprite** est une image de **8 pixels de large** et de `N` lignes de haut (1 à 15). Chaque ligne est **un octet** : chaque bit est un pixel, le bit de poids fort étant le pixel le plus à gauche.

`DXYN` fonctionne ainsi :

1. Les coordonnées de départ sont `VX` (colonne) et `VY` (ligne). Elles sont **ramenées dans l'écran par modulo** (64 et 32).
2. Les `N` octets du sprite sont lus en mémoire à partir de l'adresse `I` (`I` n'est **pas** modifié).
3. Chaque bit du sprite est combiné avec le pixel correspondant par **XOR** : un bit à 1 *inverse* le pixel.
4. `VF` = 1 si au moins un pixel **allumé a été éteint** (collision), sinon 0.
5. Un sprite qui dépasse du bord de l'écran est **coupé** (pas de retour de l'autre côté) pour le comportement de référence.

Le XOR est ce qui permet d'effacer un sprite en le redessinant au même endroit, et de détecter les collisions.

## 7. Le clavier

Le clavier d'origine avait 16 touches disposées en grille 4×4 :

```
CHIP-8        Disposition courante sur un clavier moderne
1 2 3 C       1 2 3 4
4 5 6 D       A Z E R     (AZERTY)  ou  Q W E R  (QWERTY)
7 8 9 E       Q S D F     (AZERTY)  ou  A S D F  (QWERTY)
A 0 B F       W X C V     (AZERTY)  ou  Z X C V  (QWERTY)
```

Le choix du mapping est libre. L'important est de garder la **forme de la grille**.

## 8. Les timers et le son

- Deux compteurs 8 bits, décrémentés **de 1, 60 fois par seconde**, tant qu'ils sont > 0.
- Le timer de délai sert aux jeux pour mesurer le temps.
- Le timer de son déclenche un bip tant qu'il est > 0. Un simple son continu suffit.

## 9. Ambiguïtés et *quirks*

C'est le principal piège de CHIP-8 : plusieurs implémentations historiques se comportent différemment, et **les ROMs sont écrites pour l'une ou l'autre**.

| Instruction | Variante « CHIP-8 d'origine » | Variante « SUPER-CHIP / moderne » |
|---|---|---|
| `8XY6` / `8XYE` | Copie `VY` dans `VX`, puis décale | Décale `VX` directement, `VY` ignoré |
| `FX55` / `FX65` | `I` est incrémenté pendant la copie | `I` reste inchangé |
| `BNNN` | `PC` = `NNN` + `V0` | `BXNN` : `PC` = `XNN` + `VX` |
| `8XY1/2/3` | `VF` remis à 0 | `VF` inchangé |

Conseil : prends le comportement qui fait passer les ROMs de test que tu utilises, puis ajuste si un jeu se comporte bizarrement. Tu peux rendre ces variantes configurables plus tard.

Autre subtilité : pour `8XY4`, `8XY5`, `8XY7` et les décalages, si `X` vaut `F`, l'ordre entre « écrire le résultat dans `VX` » et « écrire le drapeau dans `VF` » change le résultat final. À toi de décider lequel doit gagner.

## 10. Ressources

- **Guide de Tobias V. Langhoff** : « Guide to making a CHIP-8 emulator », très pédagogique, idéal pour ton niveau. <https://tobiasvl.github.io/blog/write-a-chip-8-emulator/>
- **Cowgod's CHIP-8 Technical Reference** : la référence classique pour la liste des opcodes.
- **Suite de tests de Timendus** : <https://github.com/Timendus/chip8-test-suite> : ROMs qui valident chaque instruction et les quirks, avec un message à l'écran. Indispensable.
- **Rust** : *The Rust Book* (chapitre sur les types entiers et le débordement), doc de la std sur `wrapping_add`, `overflowing_add`, `checked_add` et leurs variantes pour la soustraction.

## 11. Questions pour t'aider à réfléchir

Avant d'écrire du code, essaie de répondre à ça :

- Que se passe-t-il si `7XNN` dépasse 255 ? Et `8XY4` ? Quelle différence de comportement dois-tu implémenter ?
- Pourquoi l'écran est-il mieux représenté par une collection de booléens plutôt que de caractères ?
- Quelle est la différence entre « avancer `PC` avant d'exécuter » et « après » pour `2NNN` ? Qu'empile-t-on ?
- Comment faire tourner le CPU à ~700 Hz et les timers à 60 Hz sans les lier à la vitesse de l'écran ?
