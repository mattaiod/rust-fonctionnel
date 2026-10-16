# Chapitre 1 : pureté et immutabilité

> Prérequis : le chapitre 0 (`ch00-survie/README.md`), à garder ouvert pendant les exercices. Tests :
> `cargo test -p ch01-purete`.

## L'idée

Une fonction **pure** rend toujours la même sortie pour les mêmes entrées, et ne fait rien d'autre : elle ne lit
pas l'horloge, n'écrit pas sur la sortie standard, ne modifie pas ce qu'on lui a passé. Tout ce qu'elle produit
est dans sa valeur de retour.

```rust
use std::time::{SystemTime, UNIX_EPOCH};

// Pure : même entrée, même sortie, rien d'autre
fn moyenne(xs: &[f64]) -> Option<f64> {
    if xs.is_empty() {
        return None;
    }
    Some(xs.iter().sum::<f64>() / xs.len() as f64)
}

// Impure : dépend de l'horloge ET écrit sur stdout
fn horodater(nom: &str) -> String {
    let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    println!("soumission de {nom}");
    format!("{nom}-{t}")
}
```

Pourquoi s'embêter ? Une fonction pure se teste avec un `assert_eq!`, sans mock ni capture de la sortie. Elle se
lit seule : pas besoin de savoir ce qui s'est passé avant. Et elle se parallélise sans risque (chapitre sur la
concurrence, plus tard).

## Les outils du chapitre

| Outil | Ce qu'il fait |
|---|---|
| `let x = …` | une liaison **immuable** : la réassigner est l'erreur `E0384` |
| `let mut x = …` | la mutation existe, mais il faut la demander |
| *shadowing* : `let x = f(x);` | une nouvelle variable qui cache l'ancienne, sans mutation |
| `&T` / `&mut T` | lecture partagée / écriture exclusive : la mutation se lit dans la signature |
| `Struct { champ: v, ..ancien }` | *struct update syntax* : une nouvelle valeur, un champ changé |
| `fold(etat_initial, \|etat, x\| …)` | enchaîner une fonction `(état, entrée) -> état` sur une liste |

## Immuable par défaut, créer plutôt que modifier

En JavaScript, c'est `const` qu'il faut penser à écrire ; en Rust, c'est `mut`. Le défaut est le bon. Au lieu de
modifier une valeur, on en crée une nouvelle :

```rust
let brut = "  Alpha-Team ";
let nom = brut.trim();         // une nouvelle &str, brut est intact
let nom = nom.to_lowercase();  // shadowing : une nouvelle String
```

Le *shadowing* n'est pas une mutation : ce sont deux variables différentes, la seconde peut changer de type.

Pour une `struct`, la *struct update syntax* évite de recopier tous les champs à la main :

```rust
let apres = Score { serie: 0, ..avant }; // tous les champs de `avant`, sauf `serie`
```

## Les itérateurs, en avance sur le chapitre 3

Les exercices utilisent déjà `iter()`, `map`, `filter`, `fold` et `collect`, que le chapitre 3 détaille. Le minimum
pour commencer :

```rust
let notes: Vec<i64> = vec![12, 8, 15];
let plus_un: Vec<i64> = notes.iter().map(|n| n + 1).collect(); // [13, 9, 16], notes est intact
let admis = notes.iter().filter(|n| **n >= 10).count();        // 2
let total = notes.iter().fold(0, |acc, n| acc + n);            // 35
```

1. `notes.iter()` parcourt la liste **en lecture** : chaque élément arrive en `&i64` (une référence vers la note),
   pas en `i64`. Rust sait faire `n + 1` sur une référence d'entier ; comparer ou passer à une fonction qui veut un
   `i64` demande d'écrire `*n` (« la valeur derrière la référence »).
2. `|n| …` est une *closure*, une petite fonction sans nom (chapitre 2). `map` dit quoi faire de chaque élément,
   `filter` lesquels garder (il ajoute une référence de plus : `&&i64`, d'où le `**n`), `fold` les combine un à un.
3. `collect()` range le résultat dans le type demandé par le `let` ou par le retour de la fonction.

## Ce que le `borrow checker` garantit déjà

Deux mots reviennent tout le module. **Posséder** (*ownership*) : une variable possède sa valeur, qui disparaît avec
elle ; passer une valeur à une fonction par valeur (`panier: Panier`) la **déplace** (*move*), et l'ancienne
variable n'est plus utilisable. **Emprunter** (*borrow*) : `&x` prête la valeur en lecture, `&mut x` en écriture,
sans la donner. Le *borrow checker* est la partie du compilateur qui vérifie ces règles (ch00, section 2).

En Python, une fonction qui reçoit une liste peut la modifier sans prévenir. En Rust, la signature le dit :

```rust
fn meilleur(scores: &[f64]) -> Option<f64>   // ne touchera pas à scores
fn normaliser(scores: &mut [f64])            // va les modifier, et l'appelant l'a autorisé
```

Et jamais de lecture et d'écriture en même temps : pendant qu'un `&mut` existe, personne d'autre ne lit la
valeur. Le compilateur vous donne ainsi, sans rien écrire, une bonne partie des garanties de l'immutabilité. Ce
chapitre ajoute le reste : préférer les signatures qui **prennent** une valeur et en **rendent** une nouvelle.

## L'état : le prendre, le rendre

Une fonction qui fait évoluer un état passé en `&mut` devient une fonction `(ancien état, entrée) -> nouvel état`.
Rejouer une suite d'entrées, c'est alors un `fold` dont l'accumulateur est l'état :

```rust
let final_ = coups.iter().fold(Score::default(), |score, &reussi| jouer_coup(score, reussi));
```

`Score::default()` fabrique l'état de départ : un `Score` dont chaque champ vaut sa valeur par défaut (`0` pour un
nombre), ce que permet le `#[derive(Default)]` écrit au-dessus de la `struct`. Le motif `&reussi` retire la
référence que produit `iter()` (ch00, section 4).

C'est exactement le principe d'un *reducer* (Redux, Elm) et, plus tard dans le module, de l'*event sourcing*.

## Functional core, imperative shell

L'impur est nécessaire : un programme qui n'affiche rien et n'écrit rien ne sert à rien. On le **pousse au bord**.
Au centre, un cœur pur qui calcule (et rend une `String`, un `Vec`, une `struct`) ; autour, une coquille fine qui
lit les entrées, appelle le cœur et affiche le résultat. Le cœur porte la logique et les tests ; la coquille est si
courte qu'il n'y a rien à y tester. C'est l'architecture du projet Arena du module : `arena-core` ne fait aucune
entrée-sortie (ni fichier, ni réseau, ni affichage).

## Les pièges

1. **Le `mut` caché dans un `&mut` en paramètre.** `fn f(score: &mut Score)` n'a pas de `let mut`, et pourtant elle
   mute. Le contrôle de style des exercices ne le voit pas : c'est la signature imposée qui vous protège.
2. **Cloner pour « faire pur ».** Recevoir par valeur (`panier: Panier`) et rendre une nouvelle valeur ne copie
   rien : les champs sont déplacés. Un `.clone()` n'est utile que si l'appelant veut garder l'ancienne version.
3. **`println!` au milieu d'un calcul.** Dès qu'une fonction affiche, elle n'est plus testable par sa valeur de
   retour. Rendez la `String`, affichez-la plus haut.

## Ce qui va coincer (et comment s'en sortir)

Voici les erreurs que le compilateur affiche le plus souvent dans ces exercices. Lisez toujours la ligne
`error[…]`, puis le `^^^` qui montre l'endroit, puis le `help:` (ch00, section 1). Chaque fonction à écrire a
aussi son bloc **Aide Rust** au-dessus de sa signature.

1. **Modifier ce qu'on a reçu.**

   ```text
   error[E0594]: cannot assign to `score.serie`, as `score` is not declared as mutable
   error[E0594]: cannot assign to `*note`, which is behind a `&` reference
   ```

   Vous écrivez dans une valeur qu'on vous a seulement prêtée, ou dans un paramètre non `mut`. Le `help:` propose
   `mut` ou `iter_mut` : c'est l'inverse du chapitre. On ne modifie pas, on **fabrique** : un `Score { … }` neuf,
   une nouvelle note dans le `map`. (ch00, sections 2 et 7)

2. **Une référence là où on attend une valeur.**

   ```text
   error[E0308]: mismatched types
     ... expected `bool`, found `&bool`
   ```

   `iter()` donne des références (`&bool`, `&i64`). Dans les paramètres de la closure, `&reussi` « déballe » la
   référence ; ailleurs, `*n` lit la valeur derrière. Dans `filter`, c'est ``expected `&i64`, found integer`` : la
   comparaison veut `*note >= 10`. (ch00, sections 3 et 4)

3. **Sortir une valeur d'un emprunt.**

   ```text
   error[E0507]: cannot move out of `*a` which is behind a shared reference
   ```

   `iter()` prête les articles, `augmenter` veut les posséder. Quand la collection est à vous (reçue par valeur),
   `into_iter()` vous donne ses éléments : ni `*`, ni `clone()`. (ch00, sections 3 et 6)

4. **Découper une valeur temporaire.**

   ```text
   error[E0716]: temporary value dropped while borrowed
   ```

   `brut.to_lowercase().split_whitespace()` découpe une `String` créée dans la ligne et détruite à sa fin : les
   `&str` pointeraient dans le vide. Nommez-la d'abord (`let pseudo = brut.to_lowercase();`), puis découpez
   `pseudo`. (ch00, section 13)

5. **`collect` sans savoir quoi fabriquer.**

   ```text
   error[E0282]: type annotations needed
   ```

   `collect` peut fabriquer un `Vec`, une `String`, un `HashMap`… Dites lequel : `let mots: Vec<&str> = …` ou
   `collect::<Vec<&str>>()`. (ch00, section 12)

## Les exercices

| # | Fichier | Ce qu'on pratique |
|---|---|---|
| 1 | `e01_bonus.rs` | `&mut [i64]` modifié en place → `&[i64]` et un nouveau `Vec` |
| 2 | `e02_score.rs` | un état en `&mut` → `(état, coup) -> état`, puis un `fold` |
| 3 | `e03_hausse_prix.rs` | *struct update syntax* `..ancien`, déplacement des champs |
| 4 | `e04_bulletin.rs` | séparer le calcul (pur, testé) de l'affichage (la coquille) |
| 5 | `e05_pseudo.rs` | sans version impérative ; le *shadowing* |

Chaque exercice a ses tests : des cas connus, la comparaison avec la version impérative sur des jeux aléatoires
(sauf au dernier exercice, qui n'en a pas), et le **style** (ni `let mut`, ni `for`, ni `while`, ni `loop`, ni
`for_each` dans votre version). Un exercice est réussi quand tous ses tests passent : `cargo test -p ch01-purete
e01` pour l'exercice 1.

## Pour aller plus loin

- The Rust Book, chapitre 3.1 [*Variables and Mutability*](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
  (shadowing compris) et 4.2 [*References and Borrowing*](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html).
- The Rust Book, 5.1, [*struct update syntax*](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax).
- Gary Bernhardt, [*Functional Core, Imperative Shell*](https://www.destroyallsoftware.com/screencasts/catalog/functional-core-imperative-shell).
- 100 Exercises, chapitres `03_ticket_v1` (`&` et `&mut`, setters) et `04_traits`.
