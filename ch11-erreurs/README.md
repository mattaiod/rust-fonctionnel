# Chapitre 11 : erreurs de domaine

> Prérequis : chapitre 8 (`Option` et `Result`), chapitre 10 (newtypes). Cours : C2, diapos « `panic!` ou
> `Result` ? » à « `thiserror` ou `anyhow` ? ». Tests : `cargo test -p ch11-erreurs`.

## L'idée

Une erreur n'est pas qu'un message pour l'humain : c'est une **valeur** que le code appelant doit pouvoir trier.
`Result<T, String>` ne le permet pas. Pour réagir à « note vide » plutôt qu'à « note trop haute », il faut comparer
des phrases, et la moindre retouche du texte casse l'appelant sans que le compilateur ne bronche.

```rust
// Avant : l'appelant devine la cause en lisant la phrase
Err(message) if message == "note vide" => Ok(0),

// Après : une variante par cause, un match vérifié par le compilateur
Err(ErreurNote::Vide) => Ok(0),
```

Règle de C2 : `panic!` pour un bug du programme, `Result` pour une erreur attendue (saisie, fichier, réseau). Dans ce
chapitre, toutes les erreurs sont attendues.

## Les outils du chapitre

| Outil | Ce qu'il apporte |
|---|---|
| `enum ErreurX { … }` | une variante par cause, avec ses données |
| `impl fmt::Display` | la phrase pour l'humain ; donne `to_string()`, `{}`, `format!` |
| `impl std::error::Error` | l'entrée dans l'écosystème : `Box<dyn Error>`, `anyhow`, `main() -> Result` |
| `Error::source` | l'erreur enveloppée, pour remonter la chaîne des causes |
| `impl From<A> for B` | `?` convertit tout seul une erreur `A` en `B` |
| `Result::err`, `flatten`, `partition` | garder toutes les erreurs au lieu de la première |

## `Display` et `Error` à la main

```rust
impl fmt::Display for ErreurNote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Vide => write!(f, "note vide"),
            Self::Depasse20(n) => write!(f, "{n} dépasse 20"),
            // …
        }
    }
}

impl std::error::Error for ErreurNote {} // tout a une version par défaut
```

En pratique, on ne l'écrit pas à la main : `thiserror` génère `Display` (`#[error("…")]`) et `From` (`#[from]`) pour les erreurs
d'une bibliothèque, qu'on trie au cas par cas. `anyhow` sert dans un `main` ou un outil CLI, quand l'erreur est
seulement affichée. Ici, std seule : on l'écrit une fois pour savoir ce que ces crates font à votre place.

## Deux couches, `From` et `?`

`?` ne se contente pas de remonter l'erreur : il appelle `From::from` dessus. Une fonction qui rend
`Result<_, ErreurCommande>` peut donc faire `?` sur un `Result<_, ErreurLecture>`, à condition d'avoir écrit :

```rust
impl From<ErreurLecture> for ErreurCommande {
    fn from(cause: ErreurLecture) -> Self {
        Self::Lecture(cause)
    }
}
```

La couche du dessus enveloppe celle du dessous, et la rend par `source()`. Un `main` peut alors afficher toute la
chaîne : « ligne illisible : pas un entier positif : invalid digit found in string ».

La signature de `source` impressionne : `fn source(&self) -> Option<&(dyn Error + 'static)>`. Elle se lit « peut-être
une référence vers une valeur de n'importe quel type qui implémente `Error` » (`dyn Error` est un *trait object*,
comme le `Box<dyn Fn>` du chapitre 7) ; `'static` exige que cette erreur ne contienne elle-même aucune référence
temporaire, ce qui est le cas de toutes les erreurs de ce chapitre.

## Accumuler au lieu de s'arrêter

`?` s'arrête à la première erreur. C'est le bon choix quand chaque étape dépend de la précédente, le mauvais pour un
formulaire : l'utilisateur veut voir tous ses champs en rouge d'un coup. Les champs sont indépendants, donc on les
valide tous, puis on regarde le tuple :

```rust
match (valider_a(a), valider_b(b)) {
    (Ok(a), Ok(b)) => Ok(Formulaire { a, b }),
    (a, b) => Err([a.err(), b.err()].into_iter().flatten().collect()),
}
```

Pour une liste (les lignes d'un fichier), `partition(Result::is_ok)` coupe en deux `Vec` : les bonnes et les
mauvaises.

## Les pièges

1. **Une erreur `String` dans une bibliothèque.** L'appelant ne peut faire qu'afficher. Gardez `String` pour une donnée
   de l'erreur (le texte fautif), jamais pour l'erreur elle-même.
2. **La cause deux fois.** Si `Display` recopie la cause ET que `source()` la rend, elle s'affiche deux fois dans la
   chaîne. Choisissez : soit la phrase la contient, soit `source` la rend.
3. **`From` oublié.** Le message `the trait From<ErreurLecture> is not implemented for ErreurCommande` sur un `?` veut
   dire exactement ça : écrivez le `From`, pas un `map_err` à chaque appel.
4. **Les numéros de ligne.** `enumerate` AVANT `filter` : sinon les numéros comptent les lignes gardées, plus celles
   du fichier.
5. **`"NaN".parse::<f64>()` réussit.** Une valeur lue n'est pas une valeur valide : `is_finite()`.

## Ce qui va coincer (et comment s'en sortir)

Voici les erreurs les plus courantes sur ces exercices, telles que le compilateur les affiche. Chaque fonction à
écrire porte aussi un bloc « Aide Rust ».

1. **`?` sur une erreur d'un autre type**
   ```text
   error[E0271]: type mismatch resolving `<u32 as FromStr>::Err == ErreurNote`
      |     let note: u32 = texte.parse()?;
      |                           ^^^^^ expected `ErreurNote`, found `ParseIntError`

   error[E0277]: `?` couldn't convert the error to `ErreurCommande`
      |     let q: u32 = champ(&champs, 2)?.trim().parse()?;
      |                                            -------^ the trait `From<ParseIntError>` is not implemented for `ErreurCommande`
   ```
   L'erreur de `parse` est une `ParseIntError`, la fonction rend votre `enum`. Deux sorties : traduire sur place
   avec `map_err(|_| ErreurNote::PasUnNombre(...))?`, ou écrire une fois `impl From<ParseIntError> for ...` pour que
   `?` traduise tout seul. ch00, sections 1 et 12.

2. **Un `match` qui oublie des variantes**
   ```text
   error[E0004]: non-exhaustive patterns: `Err(ErreurNote::PasUnNombre(_))` and `Err(ErreurNote::Depasse20(_))` not covered
   ```
   Le compilateur liste exactement ce qui manque. Ajoutez les bras, ou un dernier bras qui attrape le reste
   (`autre => autre`). ch00, section 1.

3. **Déstructurer une variante**
   ```text
   error[E0164]: expected tuple struct or tuple variant, found struct variant `Self::TropDePersonnes`
   error[E0170]: pattern binding `NomVide` is named the same as one of the variants of the type `ErreurReservation`
   ```
   Une variante à champs nommés se déstructure avec des accolades, `Self::TropDePersonnes { demandees }` ; une
   variante sans préfixe (`NomVide =>`) devient une variable qui attrape tout. Préfixez : `Self::NomVide`.

4. **Une valeur consommée puis relue**
   ```text
   error[E0382]: use of moved value: `p`
      |     match (p, e, a) { (Ok(pseudo), Ok(email), Ok(age)) => ..., _ => Err([p.err(), ...
      |            - value moved here                                         ^ value used here after move
   ```
   Le `match` a pris possession du tuple. Dans le bras attrape-tout, nommez ce qu'il reçoit (`(p, e, a) => ...`) au
   lieu de relire les variables d'avant. ch00, sections 2 et 6.

5. **Collecter sans dire dans quoi**
   ```text
   error[E0282]: type annotations needed for `(_, _)`
      |     let (bonnes, mauvaises) = contenu.lines()...partition(Result::is_ok);
   error[E0277]: a value of type `Vec<ErreurChamp>` cannot be built from an iterator over elements of type `Option<ErreurChamp>`
   ```
   `partition` et `collect` savent fabriquer beaucoup de collections : annotez (`let (bonnes, mauvaises): (Vec<_>,
   Vec<_>) = ...`). Et un itérateur d'`Option` se vide de ses `None` avec `flatten()` avant `collect`. ch00,
   section 12.

## Les exercices

| # | Fichier | Ce qu'on pratique |
|---|---|---|
| 1 | `e01_note.rs` | `String` → `enum` ; réagir à une seule variante |
| 2 | `e02_reservation.rs` | `Display` et `Error` à la main, mêmes messages que la version naïve |
| 3 | `e03_commande.rs` | deux couches, `From` pour `?`, `source` et la chaîne des causes |
| 4 | `e04_inscription.rs` | formulaire : toutes les erreurs, par un `match` sur un tuple |
| 5 | `e05_csv.rs` | sans version naïve ; un CSV validé ligne à ligne, `partition`, numéros de ligne |

Chaque exercice a des tests de comportement, une comparaison avec la version naïve quand elle existe, et le
contrôle du **style** (ni `let mut`, ni boucle, ni `for_each` dans votre code). Comme depuis le chapitre 8, les
consignes ne donnent que le but ; dans l'exercice 5, plus rien : la documentation de chaque fonction suffit.

## Pour aller plus loin

- Le trait [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html) et le chapitre
  [« Recoverable Errors with Result »](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html) du
  Book.
- La doc de [`thiserror`](https://docs.rs/thiserror) et d'[`anyhow`](https://docs.rs/anyhow) : ce que vous venez
  d'écrire à la main, en `derive`.
- 100 Exercises, chapitre `05_ticket_v2` (exercices sur `Error`, `TryFrom`, `thiserror`, `source`).
