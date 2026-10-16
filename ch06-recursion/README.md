# Chapitre 6 : récursion et pattern matching

> Prérequis : chapitres 3 à 5 (itérateurs). Tests : `cargo test -p ch06-recursion`.

## L'idée

Les langages fonctionnels purs n'ont pas de boucle : ils ont la récursion. Une fonction récursive décrit un problème
par sa **forme** : « une liste vide vaut ceci ; une liste non vide vaut le premier élément combiné au résultat sur le
reste ». En Rust, le pattern matching rend cette description presque littérale.

```rust
// Impératif : un accumulateur, une boucle
fn somme(liste: &[i64]) -> i64 {
    let mut total = 0;
    for x in liste {
        total += x;
    }
    total
}

// Récursif : la définition elle-même
fn somme(liste: &[i64]) -> i64 {
    match liste {
        [] => 0,
        [premier, reste @ ..] => premier + somme(reste),
    }
}
```

Sur une liste, un itérateur fait souvent mieux (`liste.iter().sum()`). La récursion devient indispensable dès que
la donnée est elle-même récursive : un arbre, une expression, un JSON, un système de fichiers.

## Les outils du chapitre

| Motif | Ce qu'il reconnaît | Exemple |
|---|---|---|
| `[]` | une slice vide | cas de base |
| `[x]` | exactement un élément | |
| `[premier, reste @ ..]` | au moins un élément ; `reste` est une `&[T]` | cas récursif |
| `[.., dernier]` | au moins un élément, lie le dernier | |
| `[a, b, ..]` | au moins deux éléments | |
| `motif if condition` | une garde : le motif ET la condition | `([x, ..], [y, ..]) if x <= y` |
| `A \| B` | or-pattern : l'un ou l'autre, même branche | `[] \| [_] => true` |
| `(a, b)` | filtrer deux valeurs à la fois | `match (gauche, droite)` |

## Une fonction récursive a deux branches

1. Le **cas de base** ne s'appelle pas lui-même (`[] => 0`, `Feuille => 0`).
2. Le **cas récursif** s'appelle sur un problème **strictement plus petit** (le reste de la liste, un sous-arbre).

Si l'une manque ou si le problème ne rétrécit pas, la récursion ne s'arrête jamais. Le `match` vous aide : il est
exhaustif, le compilateur refuse un cas oublié.

## Les types récursifs et `Box`

```rust
enum Arbre {
    Feuille,
    Noeud(Box<Arbre>, i64, Box<Arbre>),
}
```

Sans `Box`, `Arbre` contiendrait un `Arbre` qui contient un `Arbre`… : taille infinie, ``error[E0072]: recursive
type `Arbre` has infinite size``. `Box<Arbre>` est un pointeur vers le tas, de taille fixe. Un `Vec<Arbre>` marche aussi :
ses éléments sont déjà sur le tas. La fonction récursive suit le type : une branche par variante, un appel par
champ récursif.

## La limite de pile : Rust ne garantit pas l'optimisation des appels terminaux

Chaque appel récursif occupe une case de la **pile d'appels**, dont la taille est limitée : pour le thread
principal, elle est fixée par le système (en général 8 Mo sous Linux et macOS, 1 Mo sous Windows) ; les threads
lancés par Rust ont 2 Mo par défaut. Haskell, Scheme ou OCaml transforment un appel
terminal (un appel récursif qui est la toute dernière opération de la fonction, comme `return f(reste)`) en simple
saut : la pile ne grandit pas. **Rust ne le garantit
pas.** LLVM le fait parfois en `--release`, jamais de façon promise, et presque jamais en debug. Une récursion sur
une liste d'un million d'éléments peut donc finir en `thread 'main' has overflowed its stack`.

Règle pratique :

1. récursion sur un arbre équilibré ou une dichotomie (profondeur en log n) : aucun souci ;
2. récursion sur une donnée naturellement petite (une expression, un JSON de configuration) : aucun souci ;
3. récursion linéaire sur une grande liste : préférez un itérateur (`fold`, `sum`…), qui ne consomme pas de pile.

Les exercices 1 et 2 récursent sur des listes pour apprendre la forme ; en production, ce serait un itérateur.

## Les pièges

1. `[premier, reste @ ..]` sur une `&[i64]` lie `premier: &i64` : `*premier` ou `premier + …` (l'addition accepte
   les références).
2. L'ordre des branches compte : la première qui colle gagne. Mettez les cas particuliers (`[]`, gardes) avant le
   cas général.
3. Une garde rend le `match` moins précis pour le compilateur : il ne sait pas que `if x <= y` et la branche
   suivante couvrent tout, il faut une branche finale sans garde.
4. Construire un `Vec` en collant un élément devant un autre `Vec` coûte une copie à chaque niveau : quadratique.
   Acceptable pour apprendre, à éviter sur de gros volumes.

## Ce qui va coincer (et comment s'en sortir)

Voici les erreurs les plus probables du chapitre, telles que le compilateur les affiche. Lisez d'abord la première ligne (le code `E0…` et le résumé), puis les `^^^` qui montrent l'endroit
exact (ch00, section 1).

### 1. Le cas de base oublié

```console
error[E0004]: non-exhaustive patterns: `&[]` not covered
```

Le `match` est exhaustif : sans la branche `[] => …`, le compilateur refuse. Il vous protège ainsi d'une
récursion sans cas de base, qui ne s'arrêterait jamais. Voir ch00, section 1.

### 2. `[premier, reste]` au lieu de `[premier, reste @ ..]`

```console
error[E0308]: mismatched types
  |
4 |         [premier, reste] => premier + somme(reste),
  |                                       ----- ^^^^^ expected `&[i64]`, found `&i64`
```

`[premier, reste]` veut dire « exactement deux éléments », et `reste` est alors un seul nombre. Pour « tout le
reste », c'est `reste @ ..` (et pas `..reste`, qui donne `` error[E0425]: cannot find value `reste` ``). Voir
ch00, section 2.

### 3. Les éléments d'un slice pattern sont des références

```console
error[E0308]: mismatched types
  |
4 |         [premier, reste @ ..] => premier <= reste[0] && est_triee(reste),
  |                                             ^^^^^^^^ expected `&i64`, found `i64`
  |
help: consider dereferencing the borrow
  |
4 |         [premier, reste @ ..] => *premier <= reste[0] && est_triee(reste),
  |                                  +
```

Sur une `&[i64]`, `[premier, ..]` lie `premier: &i64`. L'addition accepte une référence, mais la comparaison
veut les deux côtés du même type : `*premier`. Même cause pour `std::iter::once(x).chain(un Vec<i64>)` (exercice 2)
(`` error[E0271]: type mismatch resolving `<Vec<i64> as IntoIterator>::Item == &i64` ``) : `once(*x)`. Voir
ch00, section 2.

### 4. Une garde rend le `match` non exhaustif

```console
error[E0004]: non-exhaustive patterns: `(&[_, ..], &[_, ..])` not covered
```

Le compilateur ne lit pas les conditions des gardes : `if x <= y` puis `if x > y` couvrent tout pour vous, pas
pour lui. Le dernier bras s'écrit sans garde. Voir ch00, section 1.

### 5. `*` sur une `Box`

```console
error[E0308]: mismatched types
  |
5 |         Arbre::Noeud(g, v, d) => somme_arbre(*g) + v + somme_arbre(*d),
  |                                  ----------- ^^ expected `&Arbre`, found `Box<Arbre>`
```

`match arbre` sur un `&Arbre` lie `g: &Box<Arbre>`, que la fonction accepte tel quel (une `&Box<T>` se
convertit seule en `&T`). L'étoile sort la `Box`, ce qu'on ne veut pas ; `match *arbre` produit l'erreur sœur
`` error[E0507]: cannot move out of `arbre` as enum variant `Noeud` which is behind a shared reference ``. Et si
vous écrivez vous-même un type récursif sans `Box` : `` error[E0072]: recursive type `Arbre` has infinite size ``.
Voir ch00, section 2.

## Les exercices

| # | Fichier | Ce qu'on pratique |
|---|---|---|
| 1 | `e01_longueur_somme.rs` | cas de base, cas récursif, `[premier, reste @ ..]` |
| 2 | `e02_fusionner.rs` | `match` sur une paire, gardes, or-pattern |
| 3 | `e03_arbre.rs` | un `enum` récursif avec `Box` ; la pile explicite contre la récursion |
| 4 | `e04_aplatir.rs` | récursion + `flat_map` sur une structure imbriquée |
| 5 | `e05_dichotomie.rs` | diviser pour régner, `Ordering`, décaler un index |
| 6 | `e06_expression.rs` | sans version impérative : évaluer et afficher un arbre d'expression |

## Pour aller plus loin

- Le Book, chapitre 19 « Patterns and Matching » (syntaxe complète des motifs, `@`, gardes).
- Le Book, chapitre 15.1 « Using `Box<T>` to Point to Data on the Heap » (la liste chaînée `Cons`).
- La [référence des slice patterns](https://doc.rust-lang.org/reference/patterns.html#slice-patterns).
- 100 Exercises, chapitre `06_ticket_management` pour les slices.
