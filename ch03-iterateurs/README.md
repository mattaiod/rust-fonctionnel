# Chapitre 3 : itérateurs de base

> Prérequis : chapitres 1 (pureté) et 2 (closures). Tests : `cargo test -p ch03-iterateurs`.

## L'idée

Une boucle `for` mélange deux choses : le **parcours** (l'index, le `push`, la variable qui s'accumule) et le
**traitement** (ce qu'on fait à chaque élément). Un itérateur sépare les deux : le parcours est déjà écrit dans la
bibliothèque standard, on ne fournit que le traitement, sous forme de closure.

```rust
// Impératif : COMMENT faire
let mut somme = 0;
for n in &nombres {
    if n % 2 == 0 {
        somme += n * n;
    }
}

// Fonctionnel : QUOI faire, « garder les pairs, les mettre au carré, sommer »
let somme: i64 = nombres.iter().filter(|n| *n % 2 == 0).map(|n| n * n).sum();
```

## Les outils du chapitre

| Méthode | Ce qu'elle fait | Rend |
|---|---|---|
| `iter()` | parcourt une collection par référence (`&T`) | un itérateur |
| `into_iter()` | parcourt en prenant possession des éléments (`T`) | un itérateur |
| `map(f)` | transforme chaque élément | un itérateur |
| `filter(p)` | garde les éléments pour lesquels `p` rend `true` | un itérateur |
| `copied()` / `cloned()` | `&T` → `T` (copie / clone) | un itérateur |
| `sum()` / `count()` | additionne / compte | une valeur |
| `collect()` | range les éléments dans une collection (`Vec`, `String`, `HashMap`…) | une collection |

## Paresse : rien ne s'exécute avant la fin

`map` et `filter` ne font **rien** tout de suite : ils construisent une recette. Le travail n'a lieu que quand un
**consommateur** (`sum`, `count`, `collect`, `find`…) tire les éléments un par un.

```rust
let recette = nombres.iter().map(|n| n * 2); // rien n'est calculé
let doubles: Vec<i64> = recette.collect();   // maintenant, oui
```

Le compilateur le rappelle : un `map` oublié sans consommateur produit l'avertissement `unused_must_use`
(« iterators are lazy and do nothing unless consumed »).

## Zero-cost

En mode optimisé (`cargo build --release`), le compilateur transforme la chaîne `filter().map().sum()` en une boucle
aussi rapide que celle qu'on aurait écrite à la main. On gagne en lisibilité sans payer en performance : c'est ce
qu'on appelle une *zero-cost abstraction*.

## Le piège des références

`nombres.iter()` produit des `&i64`. `filter` passe à son prédicat une **référence vers l'élément**, donc un `&&i64` :
d'où les `*n` ou les `|&&n|` qu'on croise (ch00, section 4). Deux réflexes :

1. Mettre `.copied()` tôt dans la chaîne pour travailler sur des `i64`.
2. Lire le message du compilateur : il dit le type qu'il attendait.

Le tableau dit qu'`into_iter()` « prend possession » des éléments : sur un `Vec<T>` reçu par valeur, il vous les
donne (des `T`), et le `Vec` n'existe plus ensuite. Mais sur une tranche empruntée `&[i64]`, vous n'avez rien à
prendre : `into_iter()` y donne des `&i64`, exactement comme `iter()`. (ch00, section 3)

## Ce qui va coincer (et comment s'en sortir)

Voici les erreurs que le compilateur affiche le plus souvent dans ces exercices. Chaque fonction à écrire a aussi
son bloc **Aide Rust** au-dessus de sa signature.

1. **Oublier le consommateur.**

   ```text
   error[E0308]: mismatched types
     ... expected `Vec<i64>`, found `Map<Iter<'_, i64>, {closure@...}>`
   ```

   Vous rendez la recette (`Map`), pas le plat : ajoutez `.collect()` au bout. (ch00, section 12)

2. **Calculer sur un `&&i64` dans `filter`.**

   ```text
   error[E0369]: cannot calculate the remainder of `&&i64` divided by `{integer}`
   help: `%` can be used on `&i64` if you dereference the left-hand side
   ```

   `filter` ajoute une référence : `*n % 2` (une étoile suffit, Rust sait faire `%` sur un `&i64`) ou le motif
   `|&&n|`. Une comparaison donne ``expected `&&i64`, found integer`` : même remède. (ch00, section 4)

3. **Collecter des références dans un `Vec` de valeurs.**

   ```text
   error[E0277]: a value of type `Vec<i64>` cannot be built from an iterator over elements of type `&i64`
   error[E0277]: a value of type `Vec<String>` cannot be built from an iterator over elements of type `&str`
   ```

   Les éléments sont des emprunts, le `Vec` du retour veut des valeurs. Pour des nombres : `.copied()`. Pour du
   texte : `.map(|mot| mot.to_string())`, qui fabrique une `String` possédée ; `map(|s| s)` ne change rien.
   (ch00, sections 3 et 5)

4. **`sum` sans type.**

   ```text
   error[E0283]: type annotations needed
     ... cannot infer type of the type parameter `S` declared on the method `sum`
   ```

   `sum` sait produire plusieurs types : quand ni le `let` ni le retour ne le disent, écrivez `sum::<f64>()`.
   (ch00, section 12)

5. **Mélanger `f64` et `usize`.**

   ```text
   error[E0277]: cannot divide `f64` by `usize`
   ```

   `len()` rend un `usize` ; Rust ne convertit jamais tout seul : `notes.len() as f64`. (ch00, section 0)

## Les exercices

| # | Fichier | Ce qu'on pratique |
|---|---|---|
| 1 | `e01_doubler.rs` | `map` + `collect` |
| 2 | `e02_garder_pairs.rs` | `filter` + `copied` |
| 3 | `e03_somme_carres_pairs.rs` | une chaîne complète, l'exemple du cours |
| 4 | `e04_mots_longs.rs` | une closure qui capture un paramètre |
| 5 | `e05_moyenne.rs` | sans version impérative ; un retour `Option` |

Chaque exercice a ses tests : des cas connus, la comparaison avec la version impérative sur des jeux aléatoires
(sauf au dernier exercice, qui n'en a pas), et le **style** (ni `let mut`, ni `for`, ni `while`, ni `loop`, ni
`for_each` dans votre version). Un exercice est réussi quand tous ses tests passent : `cargo test -p
ch03-iterateurs e01` pour l'exercice 1.

## Pour aller plus loin

- La doc du trait [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) : plus de 70 méthodes.
- 100 Exercises, chapitre `06_ticket_management`.
