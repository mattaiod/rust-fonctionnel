# Chapitre 7 : composition

> Prérequis : chapitres 2 (closures), 3 à 5 (itérateurs). Tests : `cargo test -p ch07-composition`.

## L'idée

Un programme fonctionnel se construit comme un circuit : de petites fonctions pures, testées seules, branchées les
unes derrière les autres. La sortie de l'une est l'entrée de la suivante. Ce chapitre montre les trois façons de
brancher en Rust : la chaîne de méthodes, les fonctions passées par leur nom, et les fonctions qui fabriquent ou
combinent d'autres fonctions.

```rust
// Impératif : les étapes sont emmêlées dans la boucle
let mut noms = Vec::new();
for ligne in texte.lines() {
    let ligne = ligne.trim();
    if !ligne.is_empty() {
        noms.push(ligne.to_lowercase());
    }
}

// Composé : une étape par ligne, chacune est une fonction nommée
fn est_non_vide(ligne: &&str) -> bool {
    !ligne.is_empty()
}

let noms: Vec<String> = texte
    .lines()
    .map(str::trim)
    .filter(est_non_vide)
    .map(str::to_lowercase)
    .collect();
```

## Les outils du chapitre

| Besoin | En Rust | Exemple |
|---|---|---|
| enchaîner des étapes (le `pipe`) | la chaîne de méthodes | `.map(..).filter(..).map(..)` |
| passer une fonction existante | son chemin, sans closure | `.map(str::trim)`, `.map(f64::sqrt)`, `.filter(est_valide)` |
| fabriquer une fonction | rendre une closure `move` | `fn multiplier(a: i32) -> impl Fn(i32) -> i32` |
| combiner deux fonctions | une fonction générique | `composer(f, g)` = `x ↦ g(f(x))` |
| ranger des fonctions de types différents | `Box<dyn Fn(i32) -> i32>` | `Vec<Box<dyn Fn(i32) -> i32>>` |
| combiner une liste de fonctions | la replier | `fold(identite, composer)` |

## Le point-free : passer la fonction, pas une closure qui l'appelle

`.map(|x| carre(x))` et `.map(carre)` font la même chose. La seconde forme ne nomme pas la donnée (`x`) : c'est le
style *point-free*. Il marche dès que la signature colle exactement à ce que l'adaptateur attend :

1. `map` sur un itérateur de `&str` attend une fonction qui prend un `&str` : `str::trim`, `str::to_lowercase`
   collent ;
2. `filter` donne une **référence vers l'élément** : sur des `&str`, le prédicat reçoit un `&&str`, d'où
   `fn est_valide(ligne: &&str) -> bool` ;
3. si la signature ne colle pas (il faut un argument de plus, une conversion), écrivez la closure : le point-free
   est un outil de lisibilité, pas un but.

## Pas de currying, mais l'application partielle

En Haskell, toute fonction à deux arguments est une fonction à un argument qui rend une fonction : `ajouter 2` est
une valeur. Rust ne fait pas ça :

1. une fonction a un nombre fixe d'arguments : `ajouter(2)` seul donne `error[E0061]` ;
2. Rust n'a pas de génériques variadiques (un nombre quelconque de paramètres de type) : impossible d'écrire une
   seule fonction `curry` qui marcherait pour toutes les arités ;
3. une closure doit décider comment elle tient ses captures (emprunt ou `move`), ce qu'un currying automatique
   ne peut pas deviner.

On fait donc l'application partielle à la main, explicitement :

```rust
fn ajouter(a: i32, b: i32) -> i32 { a + b }

let ajouter_2 = |b| ajouter(2, b);                        // fixer un argument : une closure
fn plus(a: i32) -> impl Fn(i32) -> i32 { move |b| a + b } // une fabrique de fonctions
assert_eq!(ajouter_2(3), plus(2)(3));
```

Le `move` est obligatoire dans `plus` : la closure survit à l'appel, elle doit posséder `a`.

## `impl Fn` ou `Box<dyn Fn>` ?

1. `impl Fn(i32) -> i32` en retour : **un** type concret, connu du compilateur, sans allocation. Parfait quand la
   fonction rend toujours la même closure.
2. `Box<dyn Fn(i32) -> i32>` : un pointeur vers n'importe quelle closure de cette signature, alloué sur le tas,
   appelé par une indirection. Indispensable quand plusieurs branches rendent des closures différentes, ou pour
   ranger des fonctions dans un `Vec`.

## Les pièges

1. `filter(str::is_empty)` ne compile pas : `filter` donne des `&&str`, `str::is_empty` attend un `&str`.
2. Oublier `move` dans une fabrique : `error[E0373]: closure may outlive the current function`.
3. `composer(f, g)` applique `f` d'abord. En maths, `g ∘ f` s'écrit dans l'autre sens : vérifiez l'ordre par un test.
4. Le `fold` qui compose une liste de `Box<dyn Fn>` doit rendre une `Box<dyn Fn(i32) -> i32>` à chaque tour :
   `Box::new(composer(…))`, pas `composer(…)` seul. Annoter la fonction identité de départ
   (`let identite: Box<dyn Fn(i32) -> i32> = Box::new(|x| x);`) rend l'intention explicite et évite au compilateur
   d'avoir à deviner.

## Ce qui va coincer (et comment s'en sortir)

Voici les erreurs les plus probables du chapitre, telles que le compilateur les affiche. Lisez d'abord la première ligne (le code `E0…` et le résumé), puis les `^^^` qui montrent l'endroit
exact (ch00, section 1).

### 1. Une fonction nommée qui n'attend pas le bon niveau de référence

```console
error[E0277]: expected a `FnMut(&&i32)` closure, found `impl for<'a> Fn(&'a i32) -> bool`
```

`nombres.iter()` donne des `&i32`, et `filter` passe une référence de plus à son prédicat : `&&i32`. Le
prédicat fabriqué par `entre` attend un `&i32`. `.copied()` juste après `iter()` aligne tout. Variante avec une
fonction nommée : `` error[E0631]: type mismatch in function arguments … expected function signature
`for<'a> fn(&'a String) -> _` `` quand on filtre des `String` avec un prédicat sur `&&str`. Voir ch00, section 4.

### 2. Une closure rendue sans `move`

```console
error[E0373]: closure may outlive the current function, but it borrows `facteur`, which is owned by the current function
```

La closure survit à l'appel de `multiplier`, mais elle emprunte `facteur`, qui meurt à la fin de cet appel.
`move |n| …` la fait posséder `facteur`. Voir ch00, section 8.

### 3. Trois closures dans trois bras de `match`

```console
error[E0308]: `match` arms have incompatible types
…
5 | |         Operation::Multiplier(n) => move |x: i32| x * n,
  | |                                     ^^^^^^^^^^^^^^^^^^^ expected closure, found a different closure
…
  = note: no two closures, even if identical, have the same type
  = help: consider boxing your closure and/or using it as a trait object
```

Chaque closure a son propre type, anonyme. `impl Fn` promet **un** type ; pour en rendre plusieurs, on les met
chacune dans une `Box<dyn Fn(i32) -> i32>` (« un pointeur vers n'importe quel type qui implémente `Fn` »).
Si le type de retour est déjà `Box<dyn Fn(i32) -> i32>` mais qu'un bras rend la closure sans `Box::new`, le
message devient ``error[E0308]: mismatched types … expected `Box<dyn Fn(i32) -> i32>`, found closure``, avec la
suggestion ``store this in the heap by calling `Box::new` ``. Voir ch00, section 10.

### 4. Une closure en boîte qui garde une référence

```console
error: lifetime may not live long enough
  |
2 |   pub fn vers_fonction(operation: &Operation) -> Box<dyn Fn(i32) -> i32> {
  |                                   - let's call the lifetime of this reference `'1`
3 | /     match operation {
…
7 | |     }
  | |_____^ returning this value requires that `'1` must outlive `'static`
```

`Box<dyn Fn>` en retour veut dire « une closure qui ne dépend de rien d'emprunté » (`'static`). Avec
`match operation` sur un `&Operation`, le nombre est lié en `&i32` : la closure garderait une référence vers
l'opération. `match *operation` (le type est `Copy`) donne un `i32` que la closure `move` possède. Voir ch00,
sections 8 et 13.

### 5. Le `fold` qui compose des `Box<dyn Fn>`

```console
error[E0308]: mismatched types
…
6 |         .fold(identite, |avant, f| composer(avant, f))
  |                                    ^^^^^^^^^^^^^^^^^^ expected `Box<dyn Fn(i32) -> i32>`, found opaque type
…
help: store this in the heap by calling `Box::new`
```

Un *opaque type*, c'est le `impl Fn` que rend `composer` : un type précis que le compilateur connaît mais ne
nomme pas. L'accumulateur est une `Box<dyn Fn>`, chaque tour doit en rendre une : `Box::new(composer(avant, f))`.
Avec `fonctions.iter()` au lieu de `into_iter()` : `` error[E0515]: cannot return value referencing function
parameter `fonctions` ``. Voir ch00, sections 3 et 10.

## Les exercices

| # | Fichier | Ce qu'on pratique |
|---|---|---|
| 1 | `e01_nettoyer.rs` | un pipeline de fonctions nommées, sans closure |
| 2 | `e02_application_partielle.rs` | des fabriques de fonctions (`impl Fn`, `move`) |
| 3 | `e03_composer.rs` | `composer` générique, une liste de `Box<dyn Fn>` repliée |
| 4 | `e04_operations.rs` | un `enum` traduit en fonctions, puis en une seule fonction |
| 5 | `e05_pipe.rs` | sans version impérative : `pipe` sur des transformations de `String` |

## Pour aller plus loin

- Le Book, chapitre 13.1 « Closures » (captures, `move`, les traits `Fn`, `FnMut`, `FnOnce`).
- Le Book, chapitre 20.4 « Advanced Functions and Closures » (pointeurs de fonction `fn`, rendre une closure).
- 100 Exercises, chapitre `06_ticket_management`.
