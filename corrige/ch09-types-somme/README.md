# Chapitre 9 : types somme et produit, pour rendre les états illégaux impossibles

> Prérequis : chapitre 8 (`Option`, `Result`). Cours : C2, diapos « Modéliser le domaine ».
> Tests : `cargo test -p ch09-types-somme`.

## L'idée

Un type est un ensemble de valeurs possibles. Le bon type en a **exactement autant que le domaine** : pas une de
plus. Chaque valeur en trop est un état absurde que le programme peut atteindre, et que chaque fonction doit
revérifier. Le réflexe à prendre : **compter les états**.

| Type | Nombre d'états | Pourquoi |
|---|---|---|
| `bool` | 2 | |
| `(bool, bool, bool)` ou un `struct` à trois `bool` | 2 × 2 × 2 = 8 | produit : ce champ ET celui-là |
| `Option<T>` | 1 + (états de `T`) | somme : rien OU un `T` |
| `enum Feu { Vert, Orange, Rouge }` | 3 | somme : une variante OU une autre |
| `enum Contact { Email(String), Telephone(String), LesDeux { … } }` | somme de produits | |

Un `struct` est un type **produit** (on multiplie les états des champs), un `enum` un type **somme** (on additionne
les états des variantes). Un feu tricolore à trois booléens a 8 états pour 3 réels : 5 de trop. L'`enum` à trois
variantes n'en a aucun de trop.

```rust
// Le struct qui ment : 16 états, 4 ont un sens
struct CommandeNaive { payee: bool, expediee: bool, annulee: bool, numero_suivi: Option<String> }

// L'enum qui dit vrai : 4 états, tous légaux ; le numéro de suivi n'existe que là où il a un sens
enum EtatCommande { EnAttente, Payee, Expediee { numero_suivi: String }, Annulee }
```

« Make illegal states unrepresentable » (Yaron Minsky). Un état qui ne se construit pas n'a pas besoin d'être
testé : le compilateur le refuse.

## Les outils du chapitre

| Outil | Ce qu'il fait |
|---|---|
| `enum` avec données | `Cercle { rayon: f64 }`, `Email(String)` : chaque variante porte ses propres champs |
| `match` exhaustif | tous les cas doivent être traités, sinon E0004 |
| motif `A \| B` | un bras pour plusieurs variantes |
| `{ .. }` | ignorer les champs d'une variante |
| garde `if` | `Variante { x } if x > 3 => …` |
| `match (a, b)` | matcher un couple : deux `Option`, un état et un événement |
| `matches!(v, motif)` | un `bool` : la valeur correspond-elle au motif ? Pratique dans un `filter` |

## Méthodes : `impl`, `&self`, `self`

À partir de ce chapitre, certaines fonctions à écrire sont des **méthodes** : des fonctions rangées dans un bloc
`impl NomDuType { … }`, qu'on appelle avec un point (`forme.aire()`). Leur premier paramètre dit comment elles
reçoivent la valeur :

```rust
impl Forme {
    pub fn aire(&self) -> f64 { /* … */ }                    // emprunte : forme.aire(), forme reste utilisable
    pub fn agrandie(self, k: f64) -> Forme { /* … */ }       // consomme : la forme est déplacée dans la méthode
    pub fn carre(cote: f64) -> Forme { /* … */ }             // sans self : s'appelle Forme::carre(2.0)
}
```

`&self` est un raccourci pour `self: &Self`, et `Self` désigne le type du bloc (ici `Forme`). Une méthode `&self`
voit donc la valeur par référence, exactement comme un paramètre `&Forme` (ch00, section 2). Le troisième cas,
`&mut self`, modifie la valeur sur place : vous ne le verrez que dans les versions naïves. Une méthode se passe
aussi par son nom à un itérateur : `formes.iter().map(Forme::aire)` (chapitre 7).

La ligne `#[derive(Debug, Clone, Copy, PartialEq)]` au-dessus d'un type demande au compilateur d'écrire seul ces
`trait` : l'affichage de débogage (`{:?}`, utilisé par `assert_eq!`), la copie, la comparaison `==`. Quand un type
est `Copy`, il se copie au lieu de se déplacer (comme un `i64`).

## L'exhaustivité, votre filet

Le jour où vous ajoutez `Rembourse` à `EtatCommande`, chaque `match` qui ne le traite pas devient une erreur de
compilation : le compilateur vous donne la liste des endroits à mettre à jour.

```console
error[E0004]: non-exhaustive patterns: `EtatCommande::Rembourse` not covered
```

Un bras `_ => …` éteint ce filet : la nouvelle variante tombe dedans sans bruit. Sur vos propres enums, nommez les
variantes. Le `_` reste correct quand le « reste » est vraiment un reste (une transition par défaut « rien ne
change », un `Option` qui ne vous intéresse que dans un cas).

## Une transition est une fonction pure

Pas de `&mut self` qui modifie trois champs à la fois : on **consomme** l'ancien état et on rend le nouveau.

```rust
pub fn annuler(etat: EtatCommande) -> Result<EtatCommande, ErreurCommande> {
    match etat {
        EtatCommande::EnAttente | EtatCommande::Payee => Ok(EtatCommande::Annulee),
        EtatCommande::Expediee { .. } => Err(ErreurCommande::DejaExpediee),
        EtatCommande::Annulee => Err(ErreurCommande::DejaAnnulee),
    }
}
```

Avec `Result`, les transitions s'enchaînent sur le railway du chapitre 8 :
`payer(EnAttente).and_then(annuler).and_then(|e| expedier(e, "6A123"))`. Et rejouer une suite d'événements, c'est un
`fold` : `evenements.into_iter().fold(Connexion::Deconnecte, reagir)`.

## Les pièges

1. **Le booléen « de plus ».** `est_admin: bool` à côté de `role: String` : deux sources de vérité qui finissent par
   se contredire. Une seule donnée, un seul `enum`.
2. **L'`Option` qui dépend d'un autre champ.** `numero_suivi: Option<String>` n'a de sens que si `expediee` : la
   donnée doit vivre **dans** la variante.
3. **Le `_ =>` de confort**, cf. plus haut.
4. **`match` sur une référence.** `match self` avec `&self` lie les champs par référence (`email: &String`). Pour
   des champs `Copy` (`f64`), `match *self` les copie. (Un type `Copy` se copie au lieu de se déplacer : les
   nombres, `bool`, `char`, et les types qui dérivent `Copy`. Une `String` ne l'est pas : `match *self` sur un
   champ `String` donne E0507, cf. plus bas.)
5. **L'ordre des bras.** Ils sont essayés de haut en bas : un bras général placé avant un bras particulier le rend
   inatteignable (le compilateur l'avertit : `unreachable pattern`).

## Ce qui va coincer (et comment s'en sortir)

Voici les erreurs les plus probables du chapitre, telles que le compilateur les affiche. Lisez d'abord la première
ligne (le code `E0…` et le résumé), puis les `^^^` qui montrent l'endroit exact (ch00, section 1).

### 1. Une variante oubliée

```console
error[E0004]: non-exhaustive patterns: `Feu::Rouge` not covered
```

C'est le filet du chapitre : ajoutez le bras. Sur un couple `(état, événement)`, le message liste les
combinaisons manquantes ; si elles sont vraiment « le reste », un dernier bras `(etat, _) => etat` les couvre.
Voir ch00, section 1.

### 2. Un champ oublié dans un motif

```console
error[E0027]: pattern does not mention field `telephone`
```

Un motif de variante à champs nommés doit tous les citer, ou finir par `..` : `LesDeux { email, .. }`. Écrire
`Forme::Cercle(rayon)` pour une variante `Cercle { rayon }` donne `` error[E0164]: expected tuple struct or
tuple variant, found struct variant `Forme::Cercle` ``. Voir ch00, section 2.

### 3. Une variante à champs écrite comme une variante vide

```console
error[E0533]: expected unit struct, unit variant or constant, found struct variant `EtatCommande::Expediee`
```

`Expediee` porte un `numero_suivi` : dans un motif, `EtatCommande::Expediee { .. }`. Le doc-test `compile_fail`
de l'exercice 5 montre que la même erreur empêche de **construire** une commande expédiée sans numéro.

### 4. Sortir un champ d'un `&self`

```console
error[E0507]: cannot move out of `self` as enum variant `Email` which is behind a shared reference
```

`match *self` essaie de déplacer la `String` hors d'un contact qu'on n'a fait qu'emprunter. `match self` lie
des références (`email: &String`), qu'on peut rendre en `&str`. Pour **déplacer** les champs dans un nouveau
contact, la méthode prend `self` (sans `&`) : c'est le cas d'`avec_telephone`. Voir ch00, sections 2 et 6.

### 5. Les `f64` n'ont pas d'ordre total

```console
error[E0277]: the trait bound `f64: Ord` is not satisfied
```

`max()` et `min()` exigent `Ord`, que `f64` n'implémente pas (`NaN` n'est ni plus grand ni plus petit que rien).
`reduce(f64::max)` compare deux à deux. Voisines : `` error[E0277]: cannot divide `f64` by `usize` `` (→
`len() as f64`) et `` error[E0283]: type annotations needed `` sur `sum` (→ `sum::<f64>()`). Voir ch00,
section 12.

## Les exercices

Les types sont déjà écrits : vous écrivez les fonctions. Avant de lire chaque `enum`, cachez-le et essayez de le
dessiner vous-même à partir de la version naïve : combien de variantes, quelles données dans chacune ? Puis
comparez.

| # | Fichier | Ce qu'on pratique |
|---|---|---|
| 1 | `e01_feu.rs` | 8 états → 3 ; `match` exhaustif ; compter les états légaux |
| 2 | `e02_formes.rs` | `String` + deux `f64` → somme de produits ; déstructurer ; `matches!` avec garde |
| 3 | `e03_contact.rs` | deux `Option` (4 états, 1 illégal) → 3 variantes ; `match` sur un couple ; motif `\|` |
| 4 | `e04_connexion.rs` | machine à états : `reagir(état, événement)`, `fold` pour rejouer |
| 5 | `e05_commande.rs` | 16 états → 4 ; transitions qui rendent `Result` ; le bug que le naïf laisse passer |
| 6 | `e06_examen.rs` | sans version naïve ; un `enum` à la place des notes sentinelles `-1` |

Le module de l'exercice 5 contient un doc-test `compile_fail` : il prouve qu'une commande expédiée sans numéro de
suivi ne compile pas (`cargo test -p ch09-types-somme --doc`).

## Pour aller plus loin

- The Rust Book, chapitre 6, [Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html), et
  chapitre 19, [Patterns and Matching](https://doc.rust-lang.org/book/ch19-00-patterns.html).
- Yaron Minsky, [Effective ML Revisited](https://blog.janestreet.com/effective-ml-revisited/) (2011).
- Scott Wlaschin, *Domain Modeling Made Functional* : les types somme pour modéliser un métier.
- 100 Exercises, chapitre `05_ticket_v2`, exercices `01_enum` à `04_if_let`.
