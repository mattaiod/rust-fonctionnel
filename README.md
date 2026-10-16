# Exercices : de l'impératif au fonctionnel en Rust

Rust avancé, 3IABD. 15 chapitres (0 à 14), chacun dans son dossier : un `README.md` de cours, puis des exercices dans `src/`.

**Avant tout, [`INSTALLATION.md`](INSTALLATION.md)** : installer Rust, configurer VS Code (rust-analyzer), lancer
les tests, les relancer automatiquement avec `bacon`, lire leur résultat.

**Puis commencez par `ch00-survie/README.md`**, le kit de survie : références, `&&`, `move`, `mut`, `clone`, et les erreurs du
compilateur que vous allez croiser, avec leur correction. Au-dessus de chaque fonction à écrire, un bloc
**Aide Rust** vous dit ce qui va coincer et renvoie à la bonne section du kit.

## Le principe

1. **Chapitres 1 à 7, la programmation fonctionnelle.** Chaque exercice donne une version impérative qui marche
   (boucles, variables mutables). Vous la réécrivez en fonctionnel.
2. **Chapitres 8 à 13, le typage profond.** Chaque exercice donne une version « naïve » qui compile mais ment :
   `String` partout, booléens contradictoires, `-1`, `panic!`. Vous la réécrivez avec des types justes, qui rendent
   les états illégaux impossibles à représenter.
3. **Chapitre 14, la synthèse.** Pas de version de départ : des types, des signatures, des tests, et vous écrivez
   directement en fonctionnel. À la fin, vous écrivez aussi une partie des tests.

Dans chaque chapitre, le dernier exercice n'a déjà plus de version de départ.

## La règle du jeu

Les fonctions à écrire contiennent un `todo!()`. Remplacez-le. Dans votre version :

- ni `let mut`, ni `for`, ni `while`, ni `loop`, ni `for_each` ;
- autorisés : les itérateurs, les closures, la récursion, `if` et `match`, `?`, et `|mut acc, x|` dans un `fold`.

Un test `style_fonctionnel` relit votre code et vérifie cette règle. Les autres tests vérifient le comportement,
notamment que votre version rend exactement le même résultat que la version impérative sur des centaines de cas.

## Comment se déroule un chapitre

1. Le `README.md` du chapitre est le **cours** : l'idée, les outils (avec un tableau), des exemples courts, les pièges,
   puis « Ce qui va coincer », les erreurs du compilateur les plus probables et comment s'en sortir.
2. Chaque fichier `src/eNN_….rs` est un **exercice** : un en-tête qui rappelle le point de cours utile, la version de
   départ (à ne pas modifier), puis la ou les fonctions à écrire, chacune avec sa consigne, son bloc **Aide Rust** et
   un `todo!()` à remplacer. Les tests sont en bas du fichier.
3. Faites les exercices dans l'ordre : chacun prépare le suivant.

## Commandes

| Commande | Rôle |
|---|---|
| `cargo test -p ch03-iterateurs` | Tous les tests du chapitre 3 |
| `cargo test -p ch03-iterateurs e02` | Seulement l'exercice 2 du chapitre 3 |
| `cargo test` | Tout (au début, presque tout échoue : c'est normal) |
| `cargo clippy -p ch03-iterateurs` | Les conseils de Clippy, souvent très « fonctionnels » |

Un test qui échoue sur `not yet implemented` veut dire que le `todo!()` est encore là. Toutes les commandes, et
comment lire le résultat des tests : [`INSTALLATION.md`](INSTALLATION.md).

## Les chapitres

| # | Dossier | Thème |
|---|---|---|
| 0 | `ch00-survie` | Kit de survie : emprunts, `move`, `mut`, `clone`, lire une erreur |
| 1 | `ch01-purete` | Fonctions pures, immutabilité, créer plutôt que modifier |
| 2 | `ch02-closures` | Closures : capturer, passer, retourner |
| 3 | `ch03-iterateurs` | `map`, `filter`, `sum`, `collect`, paresse |
| 4 | `ch04-reduire-chercher` | `fold`, `reduce`, `find`, `any`, `all`, `max_by_key` |
| 5 | `ch05-combinators` | `zip`, `enumerate`, `flat_map`, `take_while`, `HashMap` |
| 6 | `ch06-recursion` | Récursion et pattern matching |
| 7 | `ch07-composition` | Pipelines, fonctions nommées, composer des closures |
| 8 | `ch08-option-result` | `Option` et `Result` comme valeurs, `?` |
| 9 | `ch09-types-somme` | Rendre les états illégaux impossibles |
| 10 | `ch10-newtypes` | Newtypes, parse don't validate |
| 11 | `ch11-erreurs` | Erreurs de domaine, accumuler les erreurs |
| 12 | `ch12-typestate` | Les états dans les types |
| 13 | `ch13-traits` | Traits au service du domaine |
| 14 | `ch14-synthese` | Directement en fonctionnel |

`pf-outils/` contient les outils des tests (vérification du style, jeux de données) et `trier_par`, un tri pur que
vous pouvez utiliser. Rien à y modifier.

Prérequis : Rust 1.94 ou plus récent (`rustup update`).
