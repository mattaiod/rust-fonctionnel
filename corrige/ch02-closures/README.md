# Chapitre 2 : closures

> Prérequis : chapitre 1 (pureté). Tests : `cargo test -p ch02-closures`.

## L'idée

Une closure est une fonction anonyme qui **capture** les variables visibles là où elle est écrite. C'est une
valeur comme une autre : on la stocke dans une variable, on la passe à une fonction, on la retourne.

```rust
let seuil = 0.5;
let qualifie = |score: f64| score >= seuil; // capture seuil
let carre = |x: i32| x * x;                 // ne capture rien
println!("{} {}", qualifie(0.7), carre(4)); // true 16
```

**Capturer**, c'est utiliser dans la closure une variable définie dehors (ici `seuil`). Soit la closure l'emprunte
(elle lit la variable là où elle est, qui doit donc vivre au moins aussi longtemps qu'elle), soit elle la prend
avec `move` (elle en garde sa propre copie, ou la variable lui est déplacée). ch00, section 8.

En programmation fonctionnelle, c'est l'outil de base : `map`, `filter`, `fold` attendent tous une closure. Et une
fonction qui **fabrique** une closure permet de fixer à l'avance une partie des arguments (on parle
d'*application partielle*) : dans l'exercice 1, `remise(20.0)` rend la fonction « prix → prix remisé de 20 % ».
Certains langages (Haskell, OCaml) le font automatiquement pour toute fonction (le *currying*) ; en Rust, on écrit
ces fabriques soi-même.

## Les outils du chapitre

`Fn`, `FnMut` et `FnOnce` sont des *traits* : une capacité qu'un type déclare avoir, comme une interface. Chaque
closure a son propre type, invisible, et le compilateur lui donne un ou plusieurs de ces traits selon ce qu'elle
fait de ses captures. On ne nomme donc jamais le type d'une closure, seulement son trait.

| Écriture | Sens |
|---|---|
| `\|x\| x + 1` | une closure ; types des paramètres souvent inférés |
| `move \|x\| x + n` | la closure prend possession de ce qu'elle capture (copie ou déplacement) |
| `Fn` | la closure lit ses captures : appelable autant de fois qu'on veut, partageable |
| `FnMut` | elle modifie ses captures : appelable plusieurs fois, mais il faut `let mut f` |
| `FnOnce` | elle consomme ses captures : appelable une seule fois |
| `f: impl Fn(i64) -> i64` | recevoir une closure (le compilateur produit une version de la fonction par closure reçue : appel direct, sans coût) |
| `-> impl Fn(i64) -> i64` | rendre UNE closure, d'un seul type concret, sans allocation |
| `-> Box<dyn Fn(i64) -> i64>` | rendre une closure parmi plusieurs possibles (allocation, appel indirect) |

## Trois façons de capturer

Rust choisit la capture la plus légère qui suffit au corps de la closure, et en déduit le trait :

```rust
let n = 10;
let lire = || n + 1;                 // emprunte n en &       -> Fn
let mut n = 0;
let mut compter = || n += 1;         // emprunte n en &mut    -> FnMut
let nom = String::from("Ada");
let consommer = || drop(nom);        // prend nom et le détruit (drop) -> FnOnce
```

Les trois traits s'emboîtent : toute `Fn` est aussi `FnMut`, toute `FnMut` est aussi `FnOnce`. Côté **paramètre**,
demandez le moins exigeant qui vous suffit : si vous n'appelez la closure qu'une fois, `FnOnce` accepte tout.

## Le compteur : `FnMut` par nature

Le `createCounter` de JavaScript a un équivalent direct, mais il **mute** son état capturé :

```rust
fn compteur() -> impl FnMut() -> u32 {
    let mut n = 0;
    move || {
        n += 1;
        n
    }
}

let mut suivant = compteur();          // `mut` obligatoire : appeler une FnMut la modifie
assert_eq!((suivant(), suivant(), suivant()), (1, 2, 3));
```

Ce n'est pas une fonction pure : deux appels identiques rendent deux valeurs différentes. Rust ne l'interdit pas,
il le rend **visible** (le `FnMut`, le `let mut`). Les exercices s'en tiennent à des closures `Fn`.

## `move` : survivre à la fonction

```rust
fn seuil_fixe(seuil: f64) -> impl Fn(f64) -> bool {
    |score| score >= seuil   // error[E0373]: closure may outlive the current function,
}                            // but it borrows `seuil`
```

`seuil` meurt à la fin de `seuil_fixe`, la closure lui survit. `move |score| score >= seuil` copie `seuil` dans la
closure, et le compilateur le propose lui-même dans son message. Réflexe : **une closure qu'on retourne est
presque toujours `move`.**

## `impl Fn` ou `Box<dyn Fn>` ?

```rust
fn operation(nom: &str) -> Box<dyn Fn(i32) -> i32> {
    match nom {
        "double" => Box::new(|x| x * 2),
        _ => Box::new(|x| x + 1),
    }
}
```

Avec `impl Fn` en retour, ce code ne compile pas : deux closures ont deux types différents (`E0308`, « no two
closures, even if identical, have the same type »). `Box<dyn Fn…>` range la closure sur le tas et ne retient que
son trait : toutes les branches ont alors le même type. `impl` par défaut ; `Box<dyn>` quand le type varie, ou
pour ranger des closures différentes dans un même `Vec` (ch00, section 10).

## Les pièges

1. **Oublier `move`** en retournant une closure : `E0373`. Lire le `help:` du compilateur.
2. **Les références dans `filter`** : sur un itérateur de `i64`, le prédicat reçoit un `&i64`. Une fabrique de
   prédicats rend donc `impl Fn(&i64) -> bool`.
3. **Deux closures sans capture** dans un `if` compilent quand même en `impl Fn` : elles se convertissent en
   pointeur de fonction `fn(i32) -> i32`. L'erreur `E0308` n'apparaît qu'avec une capture.
4. **Demander `Fn` alors que `FnMut` suffit** : votre fonction refusera les closures qui comptent ou qui
   accumulent. Demander `FnOnce` alors que vous appelez deux fois : `E0382`.

## Ce qui va coincer (et comment s'en sortir)

Voici les erreurs que le compilateur affiche le plus souvent dans ces exercices. Chaque fonction à écrire a aussi
son bloc **Aide Rust** au-dessus de sa signature.

1. **La closure retournée emprunte une variable qui va mourir.**

   ```text
   error[E0373]: closure may outlive the current function, but it borrows `pourcent`, which is owned by the current function
   help: to force the closure to take ownership of `pourcent` (and any other referenced variables), use the `move` keyword
   ```

   Le `help:` donne la réponse : `move |prix| …`. Même erreur avec `k` dans `Box::new(|x| x + k)` et avec
   `fonctions` dans `composer_tout`. (ch00, section 8)

2. **Une référence de trop pour la closure.**

   ```text
   error[E0277]: expected a `FnMut(&&i64)` closure, found `impl for<'a> Fn(&'a i64) -> bool`
   error[E0277]: expected a `FnMut(&f64)` closure, found `impl Fn(f64) -> f64`
   ```

   Lisez les deux types : `filter` veut une closure qui accepte un `&&i64` (une référence vers un `&i64` produit par
   `iter()`), la vôtre accepte un `&i64`. Mettez `.copied()` avant `filter` ou `map` : les éléments deviennent des
   `i64`, et tout s'aligne. Le `for<'a>` signifie « pour une référence de n'importe quelle durée de vie » : on peut
   l'ignorer. (ch00, sections 3 et 4)

3. **Rendre une closure nue là où on attend une `Box`.**

   ```text
   error[E0308]: mismatched types
     ... expected `Box<dyn Fn(i64) -> i64>`, found closure
   help: store this in the heap by calling `Box::new`
   ```

   Chaque branche du `match` rend `Box::new(|x| …)`. Avec `-> impl Fn` à la place, c'est
   ``E0308: `match` arms have incompatible types`` : deux closures n'ont jamais le même type. (ch00, section 10)

4. **Consommer ce qu'une `Fn` a capturé.**

   ```text
   error[E0507]: cannot move out of `fonctions`, a captured variable in an `Fn` closure
   ```

   Une `Fn` peut être appelée cent fois : elle n'a pas le droit de détruire sa capture au premier appel.
   `fonctions.iter()` (emprunter) au lieu de `fonctions.into_iter()` (consommer). (ch00, sections 3 et 9)

5. **Utiliser une closure après l'avoir donnée.**

   ```text
   error[E0382]: borrow of moved value: `f`
   ```

   Dans `appliquer_n_fois(f, n - 1, f(x))`, `f` est déplacée dans l'appel avant que `f(x)` soit calculé. Calculez
   d'abord : `let suivant = f(x);`. Et ne passez pas `&f` à la place : chaque niveau de récursion fabrique un
   nouveau type (`&&&…`), et la compilation échoue sur `reached the recursion limit while instantiating`.
   (ch00, sections 2 et 9)

## Les exercices

| # | Fichier | Ce qu'on pratique |
|---|---|---|
| 1 | `e01_remise.rs` | retourner une closure `impl Fn`, `move`, la passer à `map` |
| 2 | `e02_plus_grand_que.rs` | une fabrique de prédicats pour `filter` |
| 3 | `e03_appliquer_n_fois.rs` | recevoir une closure ; `fold` sur une *range* |
| 4 | `e04_operation.rs` | `Box<dyn Fn>` choisie par un `match` sur un `enum` |
| 5 | `e05_composer.rs` | sans version impérative ; composer deux fonctions, puis une liste |

Chaque exercice a ses tests : des cas connus, la comparaison avec la version impérative sur des jeux aléatoires
(sauf au dernier exercice, qui n'en a pas), et le **style** (ni `let mut`, ni `for`, ni `while`, ni `loop`, ni
`for_each` dans votre version). Un exercice est réussi quand tous ses tests passent : `cargo test -p
ch02-closures e01` pour l'exercice 1.

## Pour aller plus loin

- The Rust Book, chapitre 13.1 [*Closures*](https://doc.rust-lang.org/book/ch13-01-closures.html) et 20.4
  [*Advanced Functions and Closures*](https://doc.rust-lang.org/book/ch20-04-advanced-functions-and-closures.html)
  (retourner une closure).
- La doc des traits [`Fn`](https://doc.rust-lang.org/std/ops/trait.Fn.html),
  [`FnMut`](https://doc.rust-lang.org/std/ops/trait.FnMut.html) et
  [`FnOnce`](https://doc.rust-lang.org/std/ops/trait.FnOnce.html).
- 100 Exercises n'a pas de chapitre dédié : les closures arrivent avec les combinators de `06_ticket_management`.
