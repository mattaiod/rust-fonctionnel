# Chapitre 8 : `Option` et `Result` comme valeurs

> Prérequis : chapitres 3 à 5 (itérateurs, combinators). Cours : C2, diapos « `Option` et `Result` comme valeurs ».
> Tests : `cargo test -p ch08-option-result`.

## L'idée

Début de la Partie II, le typage profond.

Dans la Partie I, vous avez appris à écrire des fonctions pures et à les composer : une fonction reçoit des valeurs,
en rend une autre, et ne fait rien d'autre. Pour brancher une fonction dans un pipeline, on ne lit que sa
signature : ce contrat ne vaut donc que si la **signature dit la vérité**. La Partie II s'occupe des **types** :
faire dire à la signature tout ce qui peut arriver (absence, échec, états possibles), pour que le compilateur
vérifie à votre place ce que vous deviez jusqu'ici penser à vérifier vous-même.

Une fonction dont la signature dit `-> i64` promet un entier. Si elle rend `-1` pour « pas trouvé », ou si elle
panique sur une entrée invalide, elle ment : l'appelant ne peut pas le savoir sans lire le code. En Rust, l'absence
s'écrit `Option<T>` et l'échec `Result<T, E>`. Ce ne sont pas des mécanismes spéciaux : ce sont des `enum`
ordinaires (comme l'`Arbre` du chapitre 6), des **valeurs** qu'on transforme avec les mêmes verbes que les
itérateurs (`map`, `filter`, `and_then`).

```rust
// La signature ment : -1 est un i64 comme un autre
fn index_de_naif(v: &[i64], x: i64) -> i64 { /* … */ -1 }

// La signature dit vrai : il peut ne rien y avoir
fn index_de(v: &[i64], x: i64) -> Option<usize> { v.iter().position(|&e| e == x) }
```

## La version « naïve »

Dans la Partie I, chaque exercice donnait une version impérative (suffixe `_imperatif`) à réécrire en fonctionnel.
À partir de ce chapitre, la version de départ est suffixée `_naif` : c'est du code tel qu'on l'écrit sans réfléchir
aux types. Elle compile, elle marche sur le cas nominal, mais son type ment : une valeur sentinelle (`-1`, chaîne
vide) qui se fait passer pour un résultat, un `panic!` ou un `unwrap` qui arrête le programme sur une donnée
inattendue. Vous écrivez la version honnête, dont le type annonce ces cas, toujours dans le style fonctionnel de la
Partie I (le test `style_fonctionnel` continue de le vérifier). Les tests comparent votre version à la naïve sur
les entrées où celle-ci se comporte bien, et montrent ce qu'elle laisse passer sur les autres.

Les consignes changent aussi : elles ne nomment plus les méthodes à utiliser, elles donnent seulement le but
(« But : … »). Les blocs **Aide Rust** et le tableau ci-dessous sont là pour trouver l'outil.

## Les outils du chapitre

| Méthode | Sur | Ce qu'elle fait |
|---|---|---|
| `map(f)` | `Option`, `Result` | transforme la valeur si elle est là (`Some` / `Ok`) |
| `and_then(f)` | `Option`, `Result` | enchaîne une étape qui peut elle-même échouer (`f` rend une `Option` / un `Result`) |
| `filter(p)` | `Option` | `Some(x)` devient `None` si `p(&x)` est faux |
| `ok_or(e)` / `ok_or_else(f)` | `Option` | `Option` → `Result` : on nomme l'absence |
| `ok()` | `Result` | `Result` → `Option` : on oublie la raison |
| `map_err(f)` | `Result` | transforme l'erreur, pas la valeur |
| `unwrap_or(v)` / `unwrap_or_else(f)` | les deux | sort la valeur, ou un défaut |
| `map_or(defaut, f)` | les deux | `map` puis `unwrap_or`, en un appel |
| `as_deref()` | `Option<String>` | `&Option<String>` → `Option<&str>` |
| `?` | les deux | déballe, ou sort de la fonction avec l'erreur |
| `collect()` | itérateur de `Result` | `Result<Vec<T>, E>` : tout, ou la première erreur |

## `map` ou `and_then` ?

Même question qu'avec `map` et `flat_map` sur un itérateur. Si l'étape suivante ne peut pas échouer, `map`. Si
elle rend elle-même une `Option`, `map` donnerait une `Option<Option<T>>` : c'est `and_then` qui aplatit.

```rust
let age: Option<u32> = fiches.get("bob").map(|f| f.age);           // l'âge existe toujours
let email: Option<&str> = fiches.get("bob").and_then(|f| f.email.as_deref()); // l'email peut manquer
```

## Le railway et l'opérateur `?`

Chaque étape faillible est un aiguillage : rail `Ok`, on continue ; rail `Err`, on file directement à la sortie.
`?` écrit cet aiguillage en un caractère.

```rust
fn email_du_parrain(a: &Annuaire, nom: &str) -> Result<&str, Introuvable> {
    let fiche = a.get(nom).ok_or(Introuvable::Utilisateur)?;
    let parrain = fiche.parrain.as_deref().ok_or(Introuvable::SansParrain)?;
    // …
}
```

`?` marche dans une fonction qui rend une `Option` (sur des `Option`) ou un `Result` (sur des `Result`). Pour
passer de l'un à l'autre : `ok_or` dans un sens, `ok()` dans l'autre.

## `collect` d'un itérateur de `Result`

```rust
let nombres: Result<Vec<i64>, _> = ["12", "7", "x"].iter().map(|s| s.parse::<i64>()).collect();
// Err(…) : arrêt au premier échec, « 12 » et « 7 » sont jetés
```

C'est `collect` qui choisit la forme du résultat d'après le type demandé. Rendre **toutes** les erreurs, c'est
`partition` : chapitre 11.

## `panic!` ou `Result` ?

| | `panic!`, `unwrap`, `expect` | `Result` |
|---|---|---|
| Pour | un bug du programme, un invariant violé | une erreur attendue : saisie, fichier, réseau |
| Visible dans la signature | non | oui |
| Qui décide quoi faire | personne, le thread s'arrête | l'appelant |

Dans vos exercices, un `unwrap` sur une donnée qui vient de l'extérieur est une faute. Dans les tests, il est
permis.

## Les pièges

1. **`unwrap_or` évalue son argument tout de suite.** `x.unwrap_or(String::from("défaut"))` construit la `String`
   même si `x` est plein. `unwrap_or_else(|| …)` ne la construit qu'au besoin. Même chose pour `ok_or` et
   `ok_or_else`.
2. **`Option<&String>` au lieu d'`Option<&str>`.** `fiche.email.as_ref()` donne une `Option<&String>`,
   `fiche.email.as_deref()` une `Option<&str>`. Le second est presque toujours ce qu'on veut.
3. **Le défaut qui avale une erreur.** `"huit".parse().unwrap_or(8080)` démarre le serveur sur le mauvais port
   sans rien dire. Absent → défaut ; présent et illisible → erreur (exercice 5).
4. **`String` comme type d'erreur.** On ne peut pas faire de `match` sur une phrase. Un `enum` ou un `struct`
   d'erreur, avec les données utiles (la position, la valeur fautive).
5. **Les lifetimes dans la signature.** Une fonction qui reçoit deux références et rend une `&str` demande de dire
   de laquelle elle vient : `fn f<'a>(annuaire: &'a Annuaire, nom: &str) -> Option<&'a str>` (ch00, section 13).

## Ce qui va coincer (et comment s'en sortir)

Voici les erreurs les plus probables du chapitre, telles que le compilateur les affiche. Lisez d'abord la première
ligne (le code `E0…` et le résumé), puis les `^^^` qui montrent l'endroit exact (ch00, section 1).

### 1. `?` sur une `Option` dans une fonction qui rend un `Result`

```console
error[E0277]: the `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result`
…
4 |     let c = part.checked_mul(100)?;
  |                                  ^ use `.ok_or(...)?` to provide an error compatible with `Result<i64, ErreurDivision>`
```

`?` ne sait pas quelle erreur inventer pour un `None`. Le compilateur propose lui-même le remède :
`ok_or(erreur)?`. Voir ch00, section 11.

### 2. `map` au lieu de `and_then`

```console
error[E0308]: mismatched types
…
3 |     index_de(elements, cherche).map(|i| elements.get(i + 1))
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Option<i64>`, found `Option<Option<&i64>>`
```

Quand l'étape rend elle-même une `Option` (ou un `Result`), `map` emboîte : `Option<Option<_>>`. `and_then`
aplatit. Reste alors `` expected `Option<i64>`, found `Option<&i64>` `` : `get` prête la case, `.copied()` copie
le nombre. Voir ch00, section 11.

### 3. Sortir une `String` d'une fiche empruntée

```console
error[E0507]: cannot move out of `u.parrain` which is behind a shared reference
```

`annuaire.get(nom)` rend une `Option<&Utilisateur>` : la fiche est prêtée, on ne peut pas en sortir le champ.
`u.parrain.as_deref()` donne une `Option<&str>` qui emprunte. `as_ref()` donnerait une `Option<&String>`, d'où
`` expected `Option<&str>`, found `Option<&String>` `` en fin de chaîne. Voir ch00, sections 5 et 11.

### 4. `collect` d'erreurs qui ne sont pas les vôtres

```console
error[E0277]: a value of type `Result<Vec<i64>, ErreurNombre>` cannot be built from an iterator over elements of type `Result<i64, ParseIntError>`
```

`collect` sait faire `Result<Vec<T>, E>` à partir de `Result<T, E>`, avec le **même** `E`. `map_err` remplace
l'erreur de `parse` par la vôtre avant le `collect`. Dans une fonction générique, la même idée donne
`` error[E0271]: type mismatch resolving `<T as FromStr>::Err == ErreurConfig` ``. Voir ch00, section 12.

### 5. Le défaut d'un `get` sur une `HashMap<String, String>`

```console
error[E0308]: mismatched types
…
3 |     brut.get("hote").unwrap_or("localhost").to_string()
  |                      --------- ^^^^^^^^^^^ expected `&String`, found `&str`
```

`get` rend une `Option<&String>` ; le défaut doit avoir le même type. Transformez d'abord la valeur (`map`) en
`String`, puis donnez un défaut `String` avec `unwrap_or_else(|| …)`. Voir ch00, sections 5 et 11.

## Les exercices

| # | Fichier | Ce qu'on pratique |
|---|---|---|
| 1 | `e01_index_de.rs` | `-1` → `Option<usize>` ; un premier `and_then` |
| 2 | `e02_division.rs` | `panic!` → `Result` avec une erreur typée ; `ok_or`, `and_then` |
| 3 | `e03_parser_liste.rs` | `unwrap` dans une boucle → `map_err` + `collect` en `Result<Vec<_>, _>` ; `?` |
| 4 | `e04_parrain.rs` | `if let` imbriqués → chaîne de `and_then`, puis `?`, puis `ok_or` qui nomme l'étape |
| 5 | `e05_config.rs` | sans version naïve ; défauts (`unwrap_or_else`, `map_or`) et erreurs (`?`) |

Chaque exercice a ses cas connus, la comparaison avec la version naïve quand elle existe (sur les entrées où elle ne
panique pas) et le contrôle du **style** (ni `let mut`, ni `for`, ni `while`, ni `loop`, ni `for_each`).

## Pour aller plus loin

- La doc de [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html) et de
  [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) : le tableau « Transforming contained values »
  de chacune.
- [Railway Oriented Programming](https://fsharpforfunandprofit.com/rop/), Scott Wlaschin.
- 100 Exercises, chapitre `05_ticket_v2`, exercices `05_nullability` à `07_unwrap`.
