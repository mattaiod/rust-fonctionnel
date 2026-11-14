# Chapitre 4 : réduire et chercher

> Prérequis : chapitres 1 à 3. Tests : `cargo test -p ch04-reduire-chercher`.

## L'idée

Le chapitre 3 transformait une séquence en une autre (`map`, `filter`). Celui-ci la **réduit** à une seule valeur
(une somme, un produit, un maximum, un booléen) ou y **cherche** un élément. Là encore, la boucle et sa variable
d'accumulation disparaissent : on décrit seulement comment combiner deux valeurs, ou ce qu'on cherche.

## Les outils du chapitre

| Méthode | Ce qu'elle fait | Rend |
|---|---|---|
| `fold(init, \|acc, x\| …)` | fait traverser un accumulateur, à partir de `init` | la valeur finale |
| `reduce(\|acc, x\| …)` | comme `fold`, le premier élément servant de départ | `Option` (vide = `None`) |
| `sum()` / `product()` | additionne / multiplie (un `fold` spécialisé) | une valeur |
| `min()` / `max()` | le plus petit / le plus grand | `Option` |
| `min_by_key(f)` / `max_by_key(f)` | selon une clé calculée par `f` | `Option` |
| `find(p)` | le premier élément qui vérifie `p` | `Option` |
| `position(p)` | le rang du premier élément qui vérifie `p` | `Option<usize>` |
| `any(p)` / `all(p)` | au moins un / tous vérifient `p` | `bool` |

## `fold` : le couteau suisse

```rust
let total = [1, 2, 3, 4].iter().fold(0, |acc, x| acc + x); // 10
```

L'accumulateur part de `0` et traverse la séquence :

| Tour | `acc` | `x` | `acc + x` |
|---|---|---|---|
| 1 | 0 | 1 | 1 |
| 2 | 1 | 2 | 3 |
| 3 | 3 | 3 | 6 |
| 4 | 6 | 4 | 10 |

Valeurs successives de `acc` : 0, 1, 3, 6, 10. Tout ce qui réduit une séquence à une valeur s'écrit avec `fold` :
`sum`, `count`, `max` en sont des cas particuliers. Mais si la méthode spécialisée existe, prenez-la, elle se lit
mieux. Gardez `fold` pour ce qui n'a pas de nom (recomposer un nombre, enchaîner un état comme au chapitre 1).

`reduce` est un `fold` sans valeur de départ : il prend le premier élément. D'où son `Option` : une séquence vide
n'a pas de premier élément.

## Chercher sans `-1` ni `null`

```rust
let scores = [0.42, 0.91, 0.77, 0.95];
let premier = scores.iter().find(|&&s| s > 0.8);                // Some(&0.91)
let rang = scores.iter().position(|&s| s > 0.8);                // Some(1)
let valides = scores.iter().all(|s| (0.0..=1.0).contains(s));   // true
```

Les motifs `|&&s|` et `|&s|` retirent les références que reçoit le prédicat : deux pour `find`, une pour
`position` (ch00, section 4).

`find` et `position` rendent un `Option` : l'absence est dans le type, impossible d'oublier le cas « pas
trouvé ». `find`, `position`, `any` et `all` s'arrêtent dès que la réponse est connue (*short-circuit*) : pas de
travail inutile après le premier élément trouvé.

## `?` sur un `Option` : sortir tôt quand la valeur manque

`max()`, `min()` et `find()` rendent un `Option`. Pour calculer avec ce qu'il y a dedans, il faut d'abord l'ouvrir.
Dans une fonction qui rend elle-même un `Option`, l'opérateur `?` le fait en une lettre : sur `Some(x)`, il donne
`x` ; sur `None`, il quitte la fonction tout de suite en rendant `None`.

```rust
fn ecart(notes: &[i64]) -> Option<i64> {
    let haute = notes.iter().max()?; // liste vide : la fonction rend None ici
    let basse = notes.iter().min()?;
    Some(haute - basse)
}
```

Pas de `if`, pas de `unwrap()` qui panique sur la liste vide : le cas « rien à calculer » traverse la fonction
tout seul. Le chapitre 8 reviendra en détail sur `?`, qui marche aussi avec `Result`.

## Trois outils de plus pour les exercices

1. **`rev()`** parcourt une séquence à l'envers : `"abc".chars().rev()` donne `'c'`, `'b'`, `'a'`.
2. **`a.eq(b)`** compare deux itérateurs élément par élément. `==` ne marche qu'entre deux valeurs comparables
   (deux `Vec`, deux `String`) ; un `Vec` et un itérateur, non.
3. **`flat_map(f)`** est un `map(f)` dont chaque résultat est lui-même une séquence, mise à plat dans le flux
   (le chapitre 5 y revient). Exemple : `c.to_lowercase()` rend un itérateur, car certaines majuscules donnent
   plusieurs minuscules ; `flat_map(char::to_lowercase)` aligne toutes ces minuscules. `char::to_lowercase` est le
   nom de la méthode, passé à la place de la closure `|c| c.to_lowercase()` : même effet.

## Les pièges

1. **`max_by_key` rend le DERNIER maximum, `min_by_key` le PREMIER minimum.** Sur `["chien", "lapin"]` (5 lettres
   chacun), `max_by_key(|m| m.len())` rend `"lapin"`. Si l'énoncé veut le premier, il faut `reduce` avec une
   comparaison stricte, ou un `rev()` avant. Même règle pour `max()` et `min()` sur des éléments égaux.
2. **`all` sur une liste vide rend `true`**, `any` rend `false`. C'est la logique (« aucun contre-exemple »), mais
   ça surprend dans un test.
3. **Les références.** Sur `iter()`, `find` passe un `&&T` au prédicat et rend un `Option<&T>` ; `position`, `any`
   et `all` passent un `&T`. `copied()` remet tout à plat.
4. **Le débordement.** `product()` sur des `u64` déborde vite (`21!` ne tient pas) : en `debug`, panique ; en
   `release`, résultat faux. `checked_mul` dans un `try_fold` si la question se pose.
5. **`min` et `max` sur des `f64`** ne compilent pas : `f64` n'est pas `Ord` (à cause de `NaN`). Utilisez
   `fold(f64::MIN, f64::max)` ou `reduce(f64::max)`, après un `copied()` (`f64::max` prend deux `f64`, pas des
   `&f64`).

## Ce qui va coincer (et comment s'en sortir)

Les erreurs que le compilateur affiche le plus souvent dans ces exercices, et comment s'en sortir. Chaque fonction à
écrire a aussi son bloc **Aide Rust** au-dessus de sa signature.

1. **Modifier l'accumulateur au lieu de le rendre.**

   ```text
   error[E0384]: cannot assign to immutable argument `acc`
   ```

   La closure d'un `fold` **rend** le nouvel accumulateur : `|acc, chiffre| acc * 10 + chiffre`. Si vous tenez à
   un bloc qui modifie, écrivez `|mut acc, chiffre| { acc = …; acc }` (autorisé : `acc` vous appartient), sans
   oublier le `acc` final, sinon ``E0308: expected `i64`, found `()` ``. (ch00, section 7)

2. **Une ou deux références selon la méthode.**

   ```text
   error[E0308]: mismatched types
     ... expected `&&i64`, found integer        (find sur iter())
     ... expected `&i64`, found integer         (any, all, position sur iter())
   ```

   `find` et `filter` prêtent l'élément, qui est déjà un `&i64` : `&&i64`. `any`, `all` et `position` le passent
   tel quel : `&i64`. Mettez `.copied()` tôt, ou une étoile par référence. (ch00, section 4)

3. **Calculer avec des `Option`.**

   ```text
   error[E0369]: cannot subtract `Option<&i64>` from `Option<&i64>`
   ```

   `max()` et `min()` rendent un `Option` (la liste peut être vide). Dans une fonction qui rend un `Option`, `?`
   ouvre la boîte ou sort avec `None` : `Some(iter().max()? - iter().min()?)`. (ch00, section 11)

4. **Rendre une référence au lieu d'une valeur.**

   ```text
   error[E0308]: mismatched types
     ... expected `Option<i64>`, found `Option<&i64>`
   help: use `Option::copied` to copy the value inside the `Option`
   ```

   `find` sur `iter()` rend un `Option<&i64>` : `.copied()` sur l'`Option`, ou sur l'itérateur avant le `find`.
   (ch00, section 11)

5. **Rendre un `&str` qui pointe dans une valeur temporaire.**

   ```text
   error[E0515]: cannot return value referencing temporary value
   ```

   Dans `plus_long_mot`, le `&str` rendu doit être un morceau de `texte`. Si vous découpez `texte.to_lowercase()`
   ou convertissez les mots en `String` (puis suivez le conseil `.as_deref()`), il pointe dans une valeur détruite à
   la fin de la fonction. Découpez `texte` lui-même et gardez les `&str`. (ch00, sections 5 et 13)

## Les exercices

| # | Fichier | Ce qu'on pratique |
|---|---|---|
| 1 | `e01_factorielle.rs` | `product` sur une *range*, et la même en récursif avec `match` (avant-goût du chapitre 6) |
| 2 | `e02_chiffres.rs` | un `fold` qui n'a pas de méthode toute faite |
| 3 | `e03_palindrome.rs` | `rev`, `flat_map`, comparer deux itérateurs avec `eq` |
| 4 | `e04_temperatures.rs` | `all`, `any`, `min`, `max`, l'opérateur `?` sur `Option` |
| 5 | `e05_chercher.rs` | `find` et `position` |
| 6 | `e06_plus_long_mot.rs` | sans version impérative ; le piège de `max_by_key`, `reduce` |

Chaque exercice a ses tests : des cas connus, la comparaison avec la version impérative sur des jeux aléatoires,
et le **style** (ni `let mut`, ni `for`, ni `while`, ni `loop`, ni `for_each` dans votre version).

## Pour aller plus loin

- La doc du trait [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) : lisez celle de
  [`max_by_key`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max_by_key) (« If several
  elements are equally maximum, the last element is returned »).
- 100 Exercises, chapitre `06_ticket_management`.
