# Chapitre 14 : synthèse

> Prérequis : chapitres 1 à 13. Tests : `cargo test -p ch14-synthese`.

> **Avant de commencer, relisez dans le kit de survie (`ch00-survie/README.md`) :**
> section 2 (posséder, emprunter : `T`, `&T`, `&mut T`), section 3 (`iter()` ou `into_iter()`), section 4 (le `&&`
> de `filter`), section 6 (`clone()` : quand c'est la bonne réponse), section 7 (le `mut` d'un accumulateur de
> `fold`), section 11 (`Option<&T>` ou `Option<T>`) et section 12 (les annotations que le compilateur réclame). Ce
> sont elles qui reviennent dans les cinq exercices. Chaque fonction porte aussi un bloc « Aide Rust ».

## L'idée

Plus de version impérative à traduire. Vous avez un problème, des types, des signatures, des tests : à vous
d'écrire le programme, directement en fonctionnel. Ce chapitre vous donne une **architecture** pour ranger le code
et une **méthode** pour l'écrire dans le bon ordre.

## Functional core, imperative shell

Un vrai programme lit des fichiers, appelle le réseau, affiche, écrit en base. Tout ça est impur : le résultat
dépend du monde extérieur. L'idée (Gary Bernhardt, 2012) : couper le programme en deux.

1. **Le cœur fonctionnel** (*functional core*) : toutes les décisions, en fonctions pures. Des valeurs entrent,
   des valeurs sortent. Aucune entrée-sortie, aucune horloge, aucun `println!`. C'est là que vit 90 % du code, et
   c'est la partie qu'on teste, sans mock ni fichier temporaire.
2. **La coquille impérative** (*imperative shell*) : le plus mince possible. Elle lit l'entrée, appelle le cœur,
   écrit le résultat. Elle ne prend aucune décision, donc il n'y a presque rien à y tester.

```rust
// La coquille : trois lignes, toutes impures, aucune décision.
fn main() -> std::io::Result<()> {
    let journal = std::fs::read_to_string("app.log")?;   // entrée
    let rapport = analyser(&journal);                     // cœur pur (exercice 3)
    println!("{rapport:#?}");                             // sortie
    Ok(())
}
```

Le cœur ne sait pas d'où vient le texte : un fichier, le réseau, un test. C'est ce qui le rend réutilisable et
testable. Tous les exercices de ce chapitre sont des cœurs fonctionnels.

## La méthode, en quatre temps

1. **Poser les types.** Avant tout code, décrire le domaine de sorte que les états illégaux soient impossibles à
   construire. Un prix négatif ? `Centimes(u64)`. Une quantité nulle ? un champ privé et un constructeur qui
   refuse 0. Trois cas exclusifs ? un `enum`, pas trois booléens. Si le type ne peut pas être faux, la fonction
   n'a pas à le vérifier.
2. **Écrire les signatures.** Une signature honnête dit tout : `fn decompresser(&str) -> Result<String, ErreurRle>`
   annonce que ça peut échouer, et comment. `fn ajouter(&self, …) -> Panier` annonce que le panier d'origine ne
   bouge pas. Si la signature ment (un `panic!` caché, un `-1` qui veut dire « pas trouvé »), corrigez-la d'abord.
3. **Écrire les tests.** Les cas connus, les bords (vide, un seul élément, égalités, frontières), les erreurs, et
   une propriété quand il y en a une (`decompresser(compresser(x)) == x`). Un test écrit avant le code échoue :
   c'est la preuve qu'il teste quelque chose.
4. **Écrire les fonctions pures en pipelines.** Lire la signature comme un trajet : d'où part-on, où arrive-t-on,
   quelles étapes entre les deux ? Chaque étape est un adaptateur d'itérateur ou un combinator. Si une étape devient
   compliquée, c'est une fonction à nommer.

Les types et les signatures sont donnés dans ce chapitre : ils sont la moitié du travail. Lisez-les avant d'écrire
une ligne, et demandez-vous pourquoi ils sont ainsi.

## Les outils des chapitres 1 à 13

| Besoin | Outil | Chapitre |
|---|---|---|
| Une fonction sans effet de bord, testable seule | fonction pure, `&T` en entrée, valeur en sortie | 1 |
| Paramétrer un comportement | closure, `Fn` / `FnMut` / `FnOnce`, `move` | 2 |
| Transformer, garder, compter | `iter`, `map`, `filter`, `sum`, `count`, `collect` | 3 |
| Réduire à une valeur, chercher | `fold`, `find`, `position`, `any`, `all`, `min_by_key`, `max_by_key` | 4 |
| Combiner des suites | `zip`, `enumerate`, `flat_map`, `filter_map`, `take_while`, `scan`, `chain`, `chunk_by` | 5 |
| Une structure définie par elle-même | récursion, `match` sur `[tete, reste @ ..]` | 6 |
| Enchaîner des étapes | composition de fonctions, fonctions qui rendent des closures (`impl Fn`) | 7 |
| Une valeur peut manquer ou échouer | `Option`, `Result`, `map`, `and_then`, `ok_or`, `?`, `collect` en `Result` | 8 |
| Plusieurs cas exclusifs | `enum` avec données, `match` exhaustif, gardes `if` | 9 |
| Ne pas confondre deux nombres ou deux chaînes | newtype, constructeur intelligent, champ privé | 10 |
| Dire pourquoi ça échoue | `enum` d'erreur de domaine, `From`, `Display` | 11 |
| Interdire une opération dans le mauvais état | typestate, `PhantomData` | 12 |
| Lire un type depuis du texte | `FromStr` (`"…".parse()`), `TryFrom` | 10 |
| Brancher son type sur la bibliothèque standard | `From`, `Display`, `Default`, `Add`, `Sum`, `Iterator`, `Ord` dérivé | 13 |
| Trier sans `let mut` | `pf_outils::trier_par`, `total_cmp` pour les `f64` (ch. 10), `then_with` et `Reverse` (ci-dessous) | 5 |

## Points de cours

**Accumuler dans un `fold`.** Construire une `BTreeMap` de compteurs, c'est un `fold` dont l'accumulateur est
possédé : `|mut compte, x| { *compte.entry(cle).or_insert(0) += 1; compte }`. La mutation est locale à une valeur
qui n'appartient qu'au `fold` : vue de l'extérieur, la fonction reste pure.

**Une suite de découpages qui peuvent échouer.** Chaque `split_once` rend une `Option` ; `ok_or(Erreur)?` la change
en `Result` et sort au premier échec. Le code se lit de haut en bas, sans `if` imbriqués.

**Trier ou départager sur plusieurs critères.** Une comparaison rend un `std::cmp::Ordering` (`Less`, `Equal`,
`Greater`). `a.then_with(|| b)` garde `a`, sauf s'il vaut `Equal` : alors c'est `b` qui décide. Pour trier par
moyenne décroissante puis par nom croissant :

```rust
pf_outils::trier_par(eleves, |(nom_a, moy_a), (nom_b, moy_b)| {
    moy_b.total_cmp(moy_a).then_with(|| nom_a.cmp(nom_b))
})
```

Inverser `a` et `b` (ici `moy_b.total_cmp(moy_a)`) donne l'ordre décroissant. Pour une **clé** (`max_by_key`,
`min_by_key`), on ne passe pas de comparaison : on rend un tuple, comparé champ par champ, et
`std::cmp::Reverse(x)` inverse l'ordre d'un champ. `max_by_key(|&(heure, nombre)| (nombre, Reverse(heure)))`
cherche le plus grand nombre, puis, à égalité, la plus petite heure.

**Une donnée qui devient une fonction.** Un code promo est un `enum` (une donnée qu'on peut stocker, comparer,
afficher). `appliquer` le change en `impl Fn(Addition) -> Addition`. Appliquer plusieurs codes, c'est composer ces
fonctions : un `fold` qui passe l'addition de l'une à l'autre.

## Les pièges

1. **`max_by_key` rend le dernier maximum**, `min_by_key` le premier. Pour départager une égalité, mettez le
   critère dans la clé : `(nombre, Reverse(heure))`.
2. **Un `f64` n'est pas `Ord`.** `sort_by_key` sur une moyenne ne compile pas ; comparez avec `total_cmp`.
3. **De l'argent en `f64`**, c'est des centimes perdus. Des entiers de centimes, et un arrondi écrit noir sur blanc.
4. **Rendre `impl Fn` avec un `todo!()`** ne compile pas : `todo!()` n'est pas une closure. Écrivez la closure,
   mettez le `todo!()` dedans.
5. **Un test qui n'a jamais échoué ne prouve rien.** Lancez-le une fois avant d'écrire la fonction.

## Ce qui va coincer (et comment s'en sortir)

Voici les erreurs les plus fréquentes sur ces exercices, avec ce que le compilateur affiche.

1. **Prendre possession de ce qu'on ne fait qu'emprunter**
   ```text
   error[E0507]: cannot move out of `note.eleve` which is behind a shared reference
      |     cumuls.entry(note.eleve).or_insert((0.0, 0));
      |                  ^^^^^^^^^^ move occurs because `note.eleve` has type `String`, which does not implement the `Copy` trait
   ```
   `notes.iter()` prête les notes ; la `BTreeMap` veut posséder ses clés. Ici `clone()` est la bonne réponse : une
   copie par élève. ch00, sections 2 et 6.

2. **Modifier à travers une référence `&mut`**
   ```text
   error[E0368]: binary assignment operation `+=` cannot be applied to type `&mut f64`
      |     somme += note.valeur;
      |     -----^^^^^^^^^^^^^^^
      |     |
      |     cannot use `+=` on type `&mut f64`
   ```
   `entry(…).or_insert(…)` rend une référence vers la valeur rangée dans la table. On modifie la valeur pointée :
   `*somme += note.valeur`, `*compte.entry(cle).or_insert(0) += 1`. ch00, section 7.

3. **Une référence là où le type attend une valeur possédée**
   ```text
   error[E0308]: mismatched types
      |     Some(c) => Err(ErreurReservation::Conflit(c)),
      |                                               ^ expected `Reservation`, found `&Reservation`
   error[E0277]: a value of type `Vec<Reservation>` cannot be built from an iterator over elements of type `&Reservation`
   ```
   `find` et `filter` sur un `iter()` donnent des références. Pour les ranger dans une structure qui possède ses
   données : `.clone()` sur un élément, `.cloned()` sur l'itérateur. Pour un type `Copy` (`Categorie`, `Centimes`),
   `*x` suffit. ch00, sections 2, 3 et 6.

4. **`?` sur une `Option` dans une fonction qui rend un `Result`**
   ```text
   error[E0277]: the `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result`
      |     let (horodatage, reste) = ligne.split_once(' ')?;
      |                                                    ^ use `.ok_or(...)?` to provide an error compatible with `Result<Entree, ErreurLog>`
   ```
   Le compilateur donne la réponse : `.ok_or(ErreurLog::LigneIncomplete)?`. ch00, section 11.

5. **`f64` is not an iterator**, sur un tri par moyenne
   ```text
   error[E0599]: `f64` is not an iterator
      |     pf_outils::trier_par(moyennes_par_eleve(notes), |a, b| b.1.cmp(&a.1))
   ```
   `f64` n'a pas de `cmp` (il n'est pas `Ord`) : `total_cmp`, puis `then_with` pour le second critère.

6. **Dans vos tests (exercice 5) : une valeur passée par valeur, puis réutilisée**
   ```text
   error[E0382]: use of moved value: `alice`
      |     let planning = Planning::default().reserver(alice).unwrap();
      |                                                 ----- value moved here
      |     assert_eq!(..., Err(ErreurReservation::Conflit(alice)));
      |                                                    ^^^^^ value used here after move
   ```
   `reserver` prend la réservation par valeur. Gardez-en une copie pour la vérification : `reserver(alice.clone())`.
   ch00, section 6.

## Les exercices

| # | Fichier | Ce qu'on pratique |
|---|---|---|
| 1 | `e01_bulletin.rs` | `fold` dans une `BTreeMap`, tri à deux critères, `match` avec gardes |
| 2 | `e02_rle.rs` | `chunk_by`, une erreur de domaine, `collect` en `Result`, test aller-retour |
| 3 | `e03_journal.rs` | `FromStr`, `split_once` et `?`, rapport agrégé, le piège de `max_by_key` |
| 4 | `e04_panier.rs` | newtypes, constructeurs intelligents, `Add` et `Sum`, structure immuable, composition de fonctions |
| 5 | `e05_reservations.rs` | tout, et **vous écrivez les tests** |

Chaque exercice vérifie le **style** de vos fonctions : ni `let mut`, ni `for`, ni `while`, ni `loop`, ni
`for_each`.

### Exercice 5 : les tests à écrire

Jusqu'ici, vous avez lu des tests écrits pour vous, en bas de chaque fichier, dans le bloc `mod tests`. Un test
est une fonction sans paramètre marquée `#[test]` : `cargo test` l'appelle, et le test **échoue s'il panique**,
réussit sinon. On vérifie donc avec des macros qui paniquent quand la condition est fausse :

| Macro | Échoue si… |
|---|---|
| `assert_eq!(obtenu, attendu)` | les deux valeurs sont différentes (le message affiche les deux) |
| `assert!(condition)` | la condition est fausse (`assert!(resultat.is_ok())`, `assert!(liste.is_empty())`) |

`.unwrap()` est permis dans un test : s'il tombe sur une erreur, il panique, et le test échoue avec le message de
l'erreur. C'est exactement ce qu'on veut.

En bas de `e05_reservations.rs`, des tests sont marqués `#[ignore = "à écrire par l'élève"]`, avec un corps
`todo!()` et un commentaire qui décrit le comportement à vérifier. `#[ignore]` veut dire « compiler ce test, mais ne
pas le lancer par défaut » : `cargo test` l'affiche comme `ignored` au lieu de le lancer (et d'échouer sur le
`todo!()`). Pour chacun :

1. écrivez le test à la place du `todo!()`, en vous servant des fonctions d'aide déjà présentes dans `mod tests`
   (`h(9, 30)` pour 9 h 30, `creneau(debut, fin)`, `reservation(salle, debut, fin, organisateur)`) ;
2. supprimez la ligne `#[ignore = …]` : le test devient un test normal ;
3. lancez-le : il doit échouer tant que la fonction qu'il vérifie n'est pas écrite (son `todo!()` panique), puis
   passer une fois qu'elle l'est.

Un exemple complet, qui ne fait pas partie des tests demandés :

```rust
#[test]
fn annuler_dans_un_planning_vide_ne_change_rien() {
    // Préparer
    let vide = Planning::default();
    // Agir
    let apres = vide.annuler("Turing", &creneau(h(9, 0), h(10, 0)));
    // Vérifier
    assert_eq!(apres, vide);
}
```

Un test se lit comme une phrase : on prépare des valeurs, on appelle la fonction, on vérifie le résultat. Son nom
dit le comportement vérifié : c'est lui qui s'affiche quand le test échoue.

Les commandes utiles (ce qui suit `--` est passé au programme de test, pas à `cargo`) :

| Commande | Lance |
|---|---|
| `cargo test -p ch14-synthese e05` | les tests de l'exercice 5, sauf ceux encore marqués `#[ignore]` |
| `cargo test -p ch14-synthese refuse_un_chevauchement` | les seuls tests dont le nom contient ce texte |
| `cargo test -p ch14-synthese e05 -- --ignored` | seulement les tests encore marqués `#[ignore]` (ce qu'il reste à écrire) |
| `cargo test -p ch14-synthese -- --include-ignored` | tous les tests du chapitre, marqués ou non |

Quand vous avez fini, il ne reste plus aucun `#[ignore]` dans le fichier et `cargo test -p ch14-synthese` passe en
entier.

## Pour aller plus loin

- Gary Bernhardt, [*Functional Core, Imperative Shell*](https://www.destroyallsoftware.com/screencasts/catalog/functional-core-imperative-shell)
  et sa conférence *Boundaries* (2012).
- Alexis King, [*Parse, don't validate*](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/) :
  pourquoi un constructeur qui refuse vaut mieux qu'une fonction qui vérifie.
- Scott Wlaschin, *Domain Modeling Made Functional* (Pragmatic Bookshelf, 2018) : la méthode « types d'abord »
  appliquée à un domaine complet.
- La doc de [`slice::chunk_by`](https://doc.rust-lang.org/std/primitive.slice.html#method.chunk_by) et de
  [`Iterator::max_by_key`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max_by_key).
- 100 Exercises, chapitres `05_ticket_v2` et `06_ticket_management` : refaites-les sans regarder vos anciennes
  solutions.
