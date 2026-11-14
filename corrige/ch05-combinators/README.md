# Chapitre 5 : combinators

> Prérequis : chapitres 3 (itérateurs de base) et 4 (réduire, chercher). Tests : `cargo test -p ch05-combinators`.

## L'idée

Au chapitre 3, chaque élément traversait la chaîne seul : on le transformait (`map`) ou on le jetait (`filter`).
Beaucoup de boucles réelles font autre chose : elles avancent sur deux listes à la fois, gardent l'index, regardent
l'élément d'avant, s'arrêtent au premier problème, rangent les résultats dans une table. Pour chacun de ces
parcours, l'itérateur a déjà un *combinator* : une méthode qui prend un itérateur et en rend un autre. Le travail
du chapitre, c'est de **reconnaître le parcours** dans une boucle et de nommer le combinator qui le fait.

```rust
// Impératif : un index partagé par deux listes
let mut total = 0;
for i in 0..prix.len() {
    total += prix[i] * quantites[i];
}

// Fonctionnel : « associer chaque prix à sa quantité, multiplier, sommer »
let total: i64 = prix.iter().zip(&quantites).map(|(p, q)| p * q).sum();
```

## Les outils du chapitre

| Parcours dans la boucle | Combinator | Exemple |
|---|---|---|
| deux listes en parallèle | `a.iter().zip(b)` | `[1, 2]` et `[3, 4]` → `(1, 3), (2, 4)` |
| l'index à côté de l'élément | `enumerate()` | `["a", "b"]` → `(0, "a"), (1, "b")` |
| chaque élément donne plusieurs résultats | `flat_map(f)` | des articles → tous leurs tags |
| une liste de listes à mettre à plat | `flatten()` | `[[1, 2], [3]]` → `1, 2, 3` |
| prendre / sauter tant qu'une condition tient | `take_while(p)` / `skip_while(p)` | s'arrêter à la première ligne vide |
| ouvrir des `Option` jusqu'au premier `None` | `map_while(f)` | des relevés jusqu'à la panne |
| l'élément et son voisin | `windows(2)` (sur une slice) | `[1, 4, 9]` → `[1, 4], [4, 9]` |
| un état qui avance, un résultat à chaque pas | `scan(etat, f)` | sommes cumulées |
| regrouper les voisins égaux | `chunk_by(f)` (sur une slice, Rust 1.77+) | `[1, 1, 2]` → `[1, 1], [2]` |
| ranger dans une table | `fold` + `entry` | compter les mots |

Et `collect` remplit n'importe quelle collection qui implémente `FromIterator` (le trait « je sais me construire à
partir d'un itérateur ») : `Vec`, `String` (à partir de
`char` ou de `String`), `HashMap` et `BTreeMap` (à partir de paires `(clé, valeur)`), `HashSet` et `BTreeSet`
(dédoublonnés au passage ; les versions `BTree` sont triées).

## `zip` s'arrête à la plus courte

```rust
let a = [1, 2, 3];
let b = [10, 20];
let paires: Vec<(i32, i32)> = a.into_iter().zip(b).collect(); // [(1, 10), (2, 20)] : le 3 a disparu
```

Aucune erreur, aucun avertissement. Si des longueurs différentes sont une faute (produit scalaire, comparaison
vérité / prédiction), vérifiez-le **avant** et dites-le dans le type de retour (`Option`, `Result`).

## `fold` avec un accumulateur qu'on modifie

Replier dans une collection, c'est un `fold` dont l'accumulateur est la collection. La recréer à chaque étape
serait coûteux ; on la modifie sur place :

```rust
use std::collections::HashMap;

let compte = ["a", "b", "a"]
    .into_iter()
    .fold(HashMap::new(), |mut table, lettre| {
        *table.entry(lettre).or_insert(0) += 1;
        table
    }); // {"a": 2, "b": 1}
```

`|mut table, …|` n'est pas une entorse au style fonctionnel : la closure **possède** la table, la modifie et la
rend. Personne d'autre n'a pu l'observer entre deux étapes. C'est une mutation locale, invisible de l'extérieur,
comme le `sort` caché dans `pf_outils::trier_par`.

La ligne `*table.entry(lettre).or_insert(0) += 1;` se lit de gauche à droite :

1. `table.entry(lettre)` désigne la case de la clé `lettre`, qu'elle existe ou non ;
2. `.or_insert(0)` la crée avec la valeur `0` si elle n'existe pas, puis rend une référence mutable (`&mut`) vers
   la valeur de la case ;
3. `*… += 1` ajoute 1 à la valeur pointée (l'étoile, parce qu'on modifie à travers la référence).

Une seule recherche dans la table, et plus de couple `if contains_key … else insert`.

## `scan` : un `fold` qui montre ses étapes

`fold` rend la valeur finale. `scan` rend toutes les valeurs intermédiaires :

```rust
let cumuls: Vec<i64> = [3, 1, 4].iter().scan(0, |total, x| {
    *total += x;
    Some(*total)
}).collect(); // [3, 4, 8]
```

L'état (`total`) arrive dans la closure en `&mut i64` : on le modifie à travers l'étoile (`*total += x`) et on rend
une copie de sa valeur (`Some(*total)`). La closure rend un `Option` : `Some` produit un élément, `None` arrête
l'itération.

## Les pièges

1. `windows` et `chunk_by` sont des méthodes de **slice** (`&[T]`), pas d'itérateur : on les appelle sur le
   tableau, avant `iter()`. Une `&str` n'est pas une slice de caractères : `chars().collect::<Vec<char>>()` d'abord.
2. `windows(2)` sur une liste de 0 ou 1 élément ne donne rien : c'est en général le bon comportement.
3. `take_while` consomme l'élément qui l'arrête : il est perdu pour la suite de la chaîne.
4. L'ordre d'une `HashMap` n'est pas garanti : pour un affichage ou un test stable, `BTreeMap`.
5. `flat_map(|a| &a.tags)` rend des `&String` ; pour des `String`, `.iter().cloned()` dans la closure.

## Ce qui va coincer (et comment s'en sortir)

Voici les erreurs les plus probables du chapitre, telles que le compilateur les affiche. Lisez d'abord la première ligne (le code `E0…` et le résumé), puis les `^^^` qui montrent l'endroit
exact (ch00, section 1).

### 1. Le `&&` de `filter` après `enumerate`

```console
error[E0369]: cannot calculate the remainder of `&&i64` divided by `{integer}`
```

`iter().enumerate()` donne des `(usize, &i64)`, et `filter` passe une référence vers chaque élément : le nombre
arrive en `&&i64`, sur lequel `%` n'est pas défini. Déréférencez (`*n % 2`) ou déstructurez dans le motif
(`|&(_, &n)|`). Voir ch00, section 4.

### 2. `collect` vers des valeurs possédées depuis des références

```console
error[E0277]: a value of type `BTreeSet<String>` cannot be built from an iterator over elements of type `&String`
```

`iter()` prête les éléments (`&String`) ; un `BTreeSet<String>` veut posséder les siennes. `.cloned()` copie
chaque élément. Même erreur avec `&mut i64` dans `scan` si vous rendez `Some(total)` au lieu de `Some(*total)`.
Voir ch00, sections 3 et 6.

### 3. Oublier `mut` sur l'accumulateur du `fold`

```console
error[E0596]: cannot borrow `table` as mutable, as it is not declared as mutable
```

Pour modifier la table reçue par la closure, il faut le dire : `|mut table, mot|`. Ce `mut` est permis par la
règle du jeu (la closure possède la table), `let mut` ne l'est pas. Et la closure doit finir par `table`, sans
point-virgule, sinon : `` expected `HashMap<String, usize>`, found `()` ``. Voir ch00, section 7.

### 4. Une clé `&str` dans une table de `String`

```console
error[E0308]: mismatched types
  |
4 |         *table.entry(mot).or_insert(0) += 1;
  |                ----- ^^^ expected `String`, found `&str`
```

`split_whitespace()` donne des `&str` empruntées au texte, la table veut des clés `String`. Fabriquez la `String`
avant `entry` (`to_lowercase()` en rend une). Le compilateur propose `mot.to_string()` : ça compile, mais ici on
veut des minuscules. Voir ch00, section 5.

### 5. Une méthode de slice appelée sur un itérateur

```console
error[E0599]: no method named `windows` found for struct `std::slice::Iter<'a, T>` in the current scope
```

`windows` et `chunk_by` appartiennent aux slices (`&[T]`), pas aux itérateurs : appelez-les sur le tableau, avant
`iter()`. Sur une `&str`, le message dit `` found for reference `&str` `` : passez par un `Vec<char>`. Voir ch00,
section 3.

## Les exercices

| # | Fichier | Ce qu'on pratique |
|---|---|---|
| 1 | `e01_produit_scalaire.rs` | `zip`, et le piège des longueurs différentes |
| 2 | `e02_indices_pairs.rs` | `enumerate` |
| 3 | `e03_tags_uniques.rs` | `flat_map` + `collect` vers un `BTreeSet` |
| 4 | `e04_cumul_avant_panne.rs` | `map_while` sur des `Option` |
| 5 | `e05_frequences.rs` | `fold` dans une `HashMap` avec `entry` |
| 6 | `e06_series.rs` | sans version impérative : `windows`, `scan`, `chunk_by`, `collect` en `String` |

Les consignes ne donnent plus le nom des méthodes, seulement l'idée du parcours : retrouvez le combinator dans le
tableau ci-dessus ou dans la doc.

## Pour aller plus loin

- La doc du trait [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html), et celle des
  [slices](https://doc.rust-lang.org/std/primitive.slice.html) pour `windows`, `chunks` et `chunk_by`.
- L'API [`Entry`](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html) de `HashMap`.
- 100 Exercises, chapitre `06_ticket_management`.
