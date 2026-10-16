# Chapitre 0 : kit de survie

> À lire avant le chapitre 1, puis à garder ouvert. Tous les autres chapitres renvoient ici (« voir
> `ch00-survie/README.md`, section N »). Exercices : `cargo test -p ch00-survie`.

Quand on débute en Rust, le compilateur refuse souvent le code. Ses refus ne sont pas aléatoires : une douzaine
de pièges reviennent sans cesse, et chacun produit un message reconnaissable. Ce chapitre commence par le minimum
de syntaxe (section 0), puis présente ces pièges un par un : l'idée, un exemple court, le message que le
compilateur affiche quand on se trompe (raccourci aux lignes utiles), puis la correction.

## Message du compilateur → section à lire

Cherchez le code `E0…` et le début du message dans ce tableau (Ctrl+F), puis allez à la section indiquée.

| Code | Message (début) | Section |
|---|---|---|
| E0382 | ``borrow of moved value: `x` `` / ``use of moved value: `x` `` | 2, 8 |
| E0502 | ``cannot borrow `v` as mutable because it is also borrowed as immutable`` | 2 |
| E0277 | ``cannot divide `f64` by `{integer}` `` / ``cannot divide `f64` by `usize` `` | 0 |
| E0308 | ``expected `i64`, found `()` `` (un `;` en trop à la fin d'une fonction) | 0 |
| E0308 | ``expected `i64`, found `&i64` `` | 3, 4 |
| E0277 | ``a value of type `Vec<i64>` cannot be built from an iterator over elements of type `&i64` `` | 3, 12 |
| E0594 | ``cannot assign to `*n`, which is behind a `&` reference`` | 3, 7 |
| E0308 | ``expected `&i64`, found integer`` (dans un `filter`) | 4 |
| E0369 | ``cannot calculate the remainder of `&&i64` divided by `{integer}` `` | 4 |
| E0308 | ``expected `String`, found `&str` `` | 5 |
| E0277 | ``a value of type `Vec<String>` cannot be built from an iterator over elements of type `&str` `` | 5 |
| E0369 | ``cannot add `&str` to `&str` `` | 5 |
| E0277 | ``the type `str` cannot be indexed by `{integer}` `` | 5 |
| E0507 | ``cannot move out of `*m` which is behind a shared reference`` | 6, 11 |
| E0508 | ``cannot move out of type `[String]`, a non-copy slice`` | 6 |
| E0277 | ``a value of type `Vec<String>` cannot be built from an iterator over elements of type `&String` `` | 6 |
| E0596 | ``cannot borrow `v` as mutable, as it is not declared as mutable`` | 7 |
| E0596 | ``cannot borrow `*v` as mutable, as it is behind a `&` reference`` | 7 |
| E0384 | ``cannot assign twice to immutable variable `total` `` | 7 |
| E0308 | ``expected `Vec<i64>`, found `()` `` (dans un `fold`) | 7 |
| E0373 | ``closure may outlive the current function, but it borrows `seuil` `` | 8 |
| E0507 | ``cannot move out of `nom`, a captured variable in an `Fn` closure`` | 9 |
| E0525 | ``expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce` `` | 9 |
| E0594 | ``cannot assign to `compteur`, as it is a captured variable in a `Fn` closure`` | 9 |
| E0308 | `` `if` and `else` have incompatible types `` … `no two closures, even if identical, have the same type` | 10 |
| E0308 | ``expected `Box<dyn Fn(i64) -> bool>`, found closure`` | 10 |
| E0746 | `return type cannot be a trait object without pointer indirection` | 10 |
| E0277 | ``the size for values of type `(dyn Fn(i64) -> bool + 'static)` cannot be known at compilation time`` | 10 |
| E0308 | ``expected `Option<i64>`, found `Option<&i64>` `` | 11 |
| E0308 | ``expected `Option<&str>`, found `Option<&String>` `` | 11 |
| E0282 | `type annotations needed` (sur un `let x = … .collect()`) | 12 |
| E0283 | `type annotations needed` … ``type parameter `S` declared on the method `sum` `` | 12 |
| E0284 | `type annotations needed` … ``type parameter `F` declared on the method `parse` `` | 12 |
| E0106 | `missing lifetime specifier` | 13 |
| E0515 | ``cannot return reference to local variable`` / ``cannot return value referencing local variable`` / ``… temporary value`` | 13, 5 |
| E0716 | `temporary value dropped while borrowed` | 13 |

## 0. Le minimum de syntaxe

Ce qu'il faut savoir lire avant tout le reste. Chaque point est repris plus loin, dans la section qui traite
l'erreur correspondante.

1. **Variables.** `let x = 5;` crée une variable **immuable** : la réaffecter est une erreur (E0384, section 7).
   `let mut x = 5;` la rend modifiable. Dans les exercices, `let mut` est interdit : on crée de nouvelles valeurs au
   lieu de modifier les anciennes.
2. **Nombres.** Le type dit la nature et la taille : `i64` (entier signé), `u32` et `u64` (entiers positifs),
   `usize` (taille ou index : c'est ce que rend `len()`), `f64` (nombre à virgule). Rust ne convertit **jamais** un
   nombre tout seul. `100` est un entier et `100.0` un flottant : `prix / 100` sur un `f64` est refusé avec
   ``cannot divide `f64` by `{integer}` ``, il faut écrire `prix / 100.0`. Pour changer de type, on l'écrit avec
   `as` : `notes.len() as f64` (sinon ``cannot divide `f64` by `usize` ``).
3. **Fonctions.** `fn nom(parametre: Type) -> TypeRendu { … }`. La dernière expression du corps, **sans
   point-virgule**, est la valeur rendue ; `return` ne sert qu'à sortir plus tôt. Un `;` en trop sur la dernière
   ligne change la valeur en `()` (« rien ») :

   ```text
   error[E0308]: mismatched types
   1 | pub fn double(x: i64) -> i64 {
     |        ------            ^^^ expected `i64`, found `()`
     |        implicitly returns `()` as its body has no tail or `return` expression
   2 |     x * 2;
     |          - help: remove this semicolon to return this value
   ```

   La même règle vaut pour un bloc `{ … }`, un `if` ou un `match` : leur valeur est leur dernière expression.
4. **Listes.** `vec![1, 2, 3]` crée un `Vec<i64>`, une liste qui **possède** ses éléments. Une fonction qui veut
   seulement lire une liste prend une **tranche** `&[i64]` : une vue empruntée sur des éléments rangés à la suite.
   On lui passe `&mon_vec`, ou directement `&[1, 2, 3]`. `Vec::<i64>::new()` est une liste vide.
5. **Références.** `&x` prête `x` sans le donner (section 2). `*r` lit la valeur derrière la référence `r`. Un
   appel de méthode (`r.len()`) n'a pas besoin de `*` : Rust suit la référence tout seul. Les opérateurs (`>`,
   `==`, `%`) en ont souvent besoin (section 4).
6. **Closures.** `|x| x * 2` est une petite fonction sans nom : les paramètres entre barres verticales, puis le
   corps. Avec plusieurs instructions, le corps est un bloc : `|x| { let y = x * 2; y + 1 }`. Un paramètre peut
   être un **motif** qui déballe sa valeur : `|(nom, note)|` sur un couple, `|&n|` sur une référence (section 4).
   Le chapitre 2 est consacré aux closures.
7. **`Option<T>`.** Une valeur qui peut manquer : `Some(valeur)` ou `None`. Rust n'a pas de `null`.
   `opt.unwrap_or(0)` donne la valeur, ou `0` s'il n'y en a pas ; `opt.unwrap()` donne la valeur ou arrête le
   programme (un *panic*) sur `None`, à éviter. `condition.then(|| valeur)` fabrique `Some(valeur)` si la
   condition est vraie, `None` sinon. Le chapitre 8 approfondit `Option`.
8. **Macros.** Un nom suivi de `!` est une macro : `vec![…]` crée un `Vec`, `format!("{nom} a {age} ans")` crée
   une `String`, `println!(…)` affiche, `todo!()` veut dire « pas encore écrit » et arrête le programme si on
   l'exécute (c'est pour cela que les tests échouent tant que vous ne l'avez pas remplacé).
9. **`HashMap<K, V>`.** Le dictionnaire de la bibliothèque standard, à importer avec
   `use std::collections::HashMap;`. `m.get(cle)` rend un `Option<&V>` (section 11) ; `m.entry(cle).or_insert(0)`
   donne accès en écriture à la valeur de `cle`, créée à `0` si elle n'existait pas.
10. **Chaîne d'itérateur.** `nombres.iter().filter(…).map(…).collect()` se lit de gauche à droite : on parcourt
    la liste, on garde certains éléments, on transforme chacun, on range le résultat dans une collection. Le
    chapitre 3 détaille ces méthodes ; ce qui fait trébucher, c'est le type des éléments à chaque étape (sections
    3 et 4).

## 1. Lire une erreur du compilateur

Un message de `rustc` a toujours la même forme. Prenons celui-ci :

```text
error[E0382]: borrow of moved value: `mots`
 --> src/main.rs:4:19
  |
2 |     let mots = vec![String::from("a"), String::from("b")];
  |         ---- move occurs because `mots` has type `Vec<String>`, which does not implement the `Copy` trait
3 |     let copie = mots;
  |                 ---- value moved here
4 |     copie.len() + mots.len()
  |                   ^^^^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let copie = mots.clone();
  |                     ++++++++
```

1. **La première ligne** : le code (`E0382`) et le résumé. C'est elle qu'on cherche dans le tableau ci-dessus.
2. **`-->`** : le fichier, la ligne, la colonne.
3. **`^^^^`** : l'endroit où ça casse. **`----`** : les endroits qui expliquent pourquoi (ici, où la valeur a été
   déplacée). Lisez l'histoire de haut en bas : déclarée ligne 2, déplacée ligne 3, réutilisée ligne 4.
4. **`help:`** : une correction proposée, avec des `+++` sous ce qu'il faudrait ajouter. Souvent juste, pas
   toujours la meilleure : « consider cloning » répare l'erreur mais cache parfois un problème de conception
   (section 6).
5. **`rustc --explain E0382`** dans le terminal : une page d'explication avec exemples.

Trois réflexes :

1. **Corrigez la première erreur d'abord**, puis recompilez. Une seule erreur en provoque souvent d'autres plus
   bas, qui disparaissent avec elle.
2. **Lisez les types** dans le message : ``expected `i64`, found `&i64` `` vous dit exactement ce qui manque (ici,
   un déréférencement ou un `copied`).
3. Les lignes `note: ... --> /Users/.../library/core/...` pointent dans la bibliothèque standard : ignorez-les au
   début, l'information utile est dans **votre** fichier.

## 2. Posséder, emprunter : `T`, `&T`, `&mut T`

Chaque valeur a **un seul propriétaire**. Trois façons de manipuler une valeur :

| Type | On dit | Ce qu'on peut faire | Combien à la fois |
|---|---|---|---|
| `T` | posséder | tout, y compris la détruire ou la donner | un seul propriétaire |
| `&T` | emprunter en lecture | lire | autant qu'on veut |
| `&mut T` | emprunter en écriture | lire et modifier | **un seul**, et aucun `&T` en même temps |

Écrire `let copie = mots;` ou passer `mots` à une fonction qui prend un `Vec<String>` **déplace** (*move*) la
valeur : l'ancien nom n'est plus utilisable. Les petits types simples (`i64`, `f64`, `bool`, `char`, `usize`) sont
`Copy` : ils sont copiés au lieu d'être déplacés, et le problème ne se pose pas.

```rust
fn consommer(_m: Vec<String>) {}

let mots = vec![String::from("a")];
consommer(mots);
consommer(mots); // erreur
```

```text
error[E0382]: use of moved value: `mots`
4 |     consommer(mots);
  |               ---- value moved here
5 |     consommer(mots);
  |               ^^^^ value used here after move
note: consider changing this parameter type in function `consommer` to borrow instead if owning the value isn't necessary
```

Correction : si la fonction n'a besoin que de lire, elle **emprunte** : `fn consommer(m: &[String])`, appelée avec
`consommer(&mots)`. Dans ces exercices, presque toutes les fonctions prennent `&[T]` ou `&str` pour cette raison.

L'autre règle : pas de modification pendant qu'un emprunt en lecture est vivant.

```rust
let mut v = vec![1, 2, 3];
let premier = &v[0];
v.push(4);      // erreur : push peut déplacer le Vec en mémoire, `premier` pointerait dans le vide
println!("{premier}");
```

```text
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
3 |     let premier = &v[0];
  |                    - immutable borrow occurs here
4 |     v.push(4);
  |     ^^^^^^^^^ mutable borrow occurs here
5 |     println!("{premier}");
  |                ------- immutable borrow later used here
```

Correction : finir de se servir de l'emprunt avant de modifier, ou copier la valeur (`let premier = v[0];`, un
`i64` est `Copy`). En style fonctionnel, la question se pose peu : on ne modifie pas, on crée.

## 3. `iter()`, `into_iter()`, `iter_mut()` : ce que produit l'itérateur

C'est la source d'une bonne partie des erreurs. Le type des éléments dépend de la méthode :

| Sur un `Vec<T>` ou un `&[T]` | Éléments produits | La collection après |
|---|---|---|
| `v.iter()` | `&T` (des références) | intacte |
| `v.iter_mut()` | `&mut T` | intacte, modifiable à travers les `&mut T` |
| `v.into_iter()` sur un `Vec<T>` possédé | `T` | consommée, plus utilisable |
| `v.into_iter()` sur un `&Vec<T>` ou un `&[T]` | `&T` | intacte (même chose que `iter()`) |

Avec un paramètre `nombres: &[i64]`, `nombres.iter()` produit des `&i64`, pas des `i64`. Le compilateur le dit
dès qu'on veut un `i64` :

```rust
pub fn plus_grand(nombres: &[i64]) -> i64 {
    nombres.iter().max().unwrap()
}
```

```text
error[E0308]: mismatched types
1 | pub fn plus_grand(nombres: &[i64]) -> i64 {
  |                                       --- expected `i64` because of return type
2 |     nombres.iter().max().unwrap()
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `i64`, found `&i64`
help: consider dereferencing the borrow
```

Même chose au moment de ranger dans un `Vec<i64>` :

```rust
pub fn positifs(nombres: &[i64]) -> Vec<i64> {
    nombres.iter().filter(|n| **n > 0).collect()
}
```

```text
error[E0277]: a value of type `Vec<i64>` cannot be built from an iterator over elements of type `&i64`
  |     nombres.iter().filter(|n| **n > 0).collect()
  |                                        ^^^^^^^ value of type `Vec<i64>` cannot be built from `std::iter::Iterator<Item=&i64>`
  = help: for that trait implementation, expected `i64`, found `&i64`
```

Correction : **`.copied()`** transforme les `&i64` en `i64` (pour les types `Copy`) ; **`.cloned()`** fait pareil
en clonant (pour `String`, section 6).

```rust
nombres.iter().copied().max().unwrap_or(0)
nombres.iter().copied().filter(|n| *n > 0).collect::<Vec<i64>>()
```

Réflexe : sur des nombres, mettez `.copied()` **juste après `.iter()`**. Toute la suite de la chaîne travaille alors
sur des `i64` et la moitié des `*` disparaissent.

Et pour modifier sur place (rare ici, on préfère créer), c'est `iter_mut()`, pas `iter()` :

```text
error[E0594]: cannot assign to `*n`, which is behind a `&` reference
  |     nombres.iter().for_each(|n| *n *= 2);
  |                                 ^^^^^^^ `n` is a `&` reference, so it cannot be written to
help: you may want to use `iter_mut` here
```

## 4. Le `&&` de `filter` et comment s'en débarrasser

`filter` ne donne pas l'élément à son prédicat, il lui **prête** l'élément (il doit le rendre ensuite pour le laisser
passer). Si l'itérateur produit des `&i64` (après `iter()`), le prédicat reçoit donc un **`&&i64`**.

```rust
nombres.iter().filter(|n| n % 2 == 0)   // n : &&i64
```

```text
error[E0369]: cannot calculate the remainder of `&&i64` divided by `{integer}`
  |     nombres.iter().filter(|n| n % 2 == 0).copied().collect()
  |                               - ^ - {integer}
  |                               &&i64
```

```rust
nombres.iter().filter(|n| *n > 0)       // *n : &i64, comparé à 0 : i64
```

```text
error[E0308]: mismatched types
  |     nombres.iter().filter(|n| *n > 0).copied().collect()
  |                                    ^ expected `&i64`, found integer
help: consider dereferencing the borrow
  |     nombres.iter().filter(|n| **n > 0).copied().collect()
```

Quatre façons d'écrire la même chose, de la plus simple à la plus « Rust » :

```rust
nombres.iter().copied().filter(|n| *n > 0)   // 1. copied d'abord : n est un &i64, une seule étoile
nombres.iter().copied().filter(|&n| n > 0)   // 2. pareil, le motif |&n| retire la référence : n est un i64
nombres.iter().filter(|n| **n > 0).copied()  // 3. deux étoiles
nombres.iter().filter(|&&n| n > 0).copied()  // 4. le motif |&&n| retire les deux références
```

Recommandation : **1 ou 2**. Avec des `String` (qu'on ne peut pas copier), on garde la référence et on appelle des
méthodes : `mots.iter().filter(|m| m.len() > 3)` marche, car l'appel de méthode déréférence tout seul (`m.len()`
sur un `&&String` trouve `String::len`). Ce sont les **opérateurs** (`>`, `%`, `==` avec une valeur) qui exigent
les étoiles.

Même chose pour `find`, `take_while`, `skip_while` : leur prédicat reçoit aussi `&Item`. `map`, `any`, `all` et
`position`, eux, reçoivent l'élément tel quel (`&i64` après `iter()`, une seule référence).

## 5. `String` et `&str`

| | `String` | `&str` |
|---|---|---|
| Ce que c'est | un texte **possédé**, sur le tas, modifiable | une **vue** empruntée sur un texte qui vit ailleurs |
| Analogie | le livre | un marque-page dans le livre |
| Un littéral `"abc"` | non | oui (`&'static str` : le texte est inscrit dans le programme et vit jusqu'à la fin) |
| Créer | `String::from("abc")`, `"abc".to_string()`, `format!(…)` | `&s`, `s.as_str()`, `&s[2..5]` |

Règle pour les paramètres : **prenez `&str`**, ça accepte à la fois `"abc"` et `&une_string`. Rendez une `String`
quand vous **fabriquez** un texte nouveau (`to_uppercase`, `format!`), un `&str` quand vous rendez un **morceau**
du texte reçu (section 13).

```rust
pub fn nom() -> String {
    "Ada"
}
```

```text
error[E0308]: mismatched types
1 | pub fn nom() -> String {
  |                 ------ expected `String` because of return type
2 |     "Ada"
  |     ^^^^^ expected `String`, found `&str`
help: try using a conversion method
  |     "Ada".to_string()
```

Les morceaux produits par `split_whitespace`, `split`, `lines`, `trim` sont des `&str` qui pointent dans le texte
d'origine. Pour un `Vec<String>`, il faut les convertir :

```text
error[E0277]: a value of type `Vec<String>` cannot be built from an iterator over elements of type `&str`
  |     texte.split_whitespace().collect()
  = help: for that trait implementation, expected `String`, found `&str`
```

Deux corrections, selon ce que vous voulez rendre : `.map(str::to_string).collect::<Vec<String>>()` (des copies
possédées) ou, mieux quand c'est possible, changer le type de retour en `Vec<&str>` (aucune copie, exercice 2).

Concaténer : `+` exige une `String` à gauche. Le plus lisible reste `format!`.

```text
error[E0369]: cannot add `&str` to `&str`
  |     "Bonjour " + nom
  |                ^ `+` cannot be used to concatenate two `&str` strings
  = note: string concatenation requires an owned `String` on the left
```

```rust
format!("Bonjour {nom}")
```

Pas d'indice entier sur un texte (un caractère UTF-8 fait de 1 à 4 octets) :

```text
error[E0277]: the type `str` cannot be indexed by `{integer}`
  |     nom[0]
  |         ^ string indices are ranges of `usize`
  = note: you can use `.chars().nth()` or `.bytes().nth()`
```

Correction : `nom.chars().next()` (rend un `Option<char>`), et `chars().count()` plutôt que `len()` pour compter les
lettres (`len()` compte les octets : « élève » fait 5 caractères et 7 octets).

## 6. `clone()` : quand c'est normal, quand c'est un aveu

`clone()` fabrique une copie complète, indépendante. Pour une `String` ou un `Vec`, ça coûte une allocation et une
recopie. Ce n'est **pas** interdit. La question à se poser : « est-ce que j'ai besoin de **posséder** une copie ? »

**Normal** : la fonction rend un `Vec<String>` à partir d'un `&[String]`. Vous ne possédez pas les `String` de
l'appelant, vous ne pouvez pas les lui prendre, donc vous rendez des copies. On ne peut pas « sortir » une `String`
de derrière une référence :

```rust
pub fn premier(mots: &[String]) -> String {
    mots[0]
}
```

```text
error[E0508]: cannot move out of type `[String]`, a non-copy slice
  |     mots[0]
  |     ^^^^^^^
  |     cannot move out of here
  |     move occurs because `mots[_]` has type `String`, which does not implement the `Copy` trait
help: consider cloning the value if the performance cost is acceptable
  |     mots[0].clone()
```

```rust
pub fn copie(mots: &[String]) -> Vec<String> {
    mots.iter().map(|m| *m).collect()
}
```

```text
error[E0507]: cannot move out of `*m` which is behind a shared reference
  |     mots.iter().map(|m| *m).collect()
  |                         ^^ move occurs because `*m` has type `String`, which does not implement the `Copy` trait
help: consider cloning the value if the performance cost is acceptable
  |     mots.iter().map(|m| m.clone()).collect()
```

Correction idiomatique : `mots.iter().cloned()` (l'équivalent de `copied()` pour les types non `Copy`), ou
`mots.to_vec()` pour tout copier.

Attention au faux ami : `mots.iter().clone()` clone **l'itérateur**, pas les éléments.

```text
error[E0277]: a value of type `Vec<String>` cannot be built from an iterator over elements of type `&String`
  |     mots.iter().clone().collect()
  = help: for that trait implementation, expected `String`, found `&String`
```

**Un aveu** : cloner pour faire taire le compilateur alors qu'une référence suffisait.

1. Cloner pour lire : `noms.clone().iter().filter(…)`, alors que `noms.iter()` lit sans copier.
2. Cloner chaque élément **avant** de filtrer : `.cloned().filter(…)` copie aussi ceux qu'on jette. Filtrez
   d'abord, clonez ensuite : `.filter(…).cloned()`.
3. Cloner alors qu'on fabrique déjà une nouvelle valeur : `.map(|m| m.clone().to_uppercase())`. `to_uppercase`
   rend déjà une `String` neuve, le `clone` est en trop.
4. Rendre `Vec<String>` quand `Vec<&str>` suffisait (section 5).

Quand le `help:` du compilateur dit « consider cloning », demandez-vous d'abord si une référence ou un
changement de signature ne serait pas plus juste.

## 7. `mut` : variable, référence, paramètre, accumulateur de `fold`

`mut` apparaît à quatre endroits, avec quatre sens différents :

| Écriture | Sens |
|---|---|
| `let mut x = …` | la **variable** peut être réaffectée ou modifiée |
| `&mut x` | un **emprunt en écriture** (section 2) |
| `fn f(v: &mut Vec<i64>)` | le **paramètre** est un emprunt en écriture : la fonction modifie les données de l'appelant |
| `fold(Vec::new(), \|mut acc, x\| …)` | le **paramètre de la closure** `acc` est une variable modifiable |

Dans ces exercices, `let mut` est interdit (on crée au lieu de modifier), mais **`|mut acc, x|` dans un `fold` est
autorisé** : `fold` vous **donne** l'accumulateur (vous le possédez), vous le modifiez, vous le **rendez**. Personne
d'autre ne le voit pendant ce temps : c'est une mutation locale, invisible de l'extérieur.

```rust
nombres.iter().fold(Vec::new(), |acc, n| {
    acc.push(n * 2);
    acc
})
```

```text
error[E0596]: cannot borrow `acc` as mutable, as it is not declared as mutable
2 |         acc.push(n * 2);
  |         ^^^ cannot borrow as mutable
help: consider changing this to be mutable
  |     nombres.iter().fold(Vec::new(), |mut acc, n| {
```

Deuxième piège du `fold` : oublier de **rendre** `acc`. Le `;` après `push` fait que le bloc rend `()`.

```text
error[E0308]: mismatched types
2 |       nombres.iter().fold(Vec::new(), |mut acc, n| {
  |  __________________________________________________^
3 | |         acc.push(n * 2);
4 | |     })
  | |_____^ expected `Vec<i64>`, found `()`
help: consider returning the local binding `acc`
```

La forme complète, à retenir :

```rust
nombres.iter().fold(Vec::new(), |mut acc, n| {
    acc.push(n * 2);
    acc // pas de point-virgule : c'est la valeur rendue
})
```

Les autres messages de `mut`, que vous verrez dans les versions impératives :

```text
error[E0384]: cannot assign twice to immutable variable `total`
2 |     let total = 0;
  |         ----- first assignment to `total`
3 |     total = total + 1;
  |     ^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable

error[E0596]: cannot borrow `*v` as mutable, as it is behind a `&` reference
2 |     v.push(1);
  |     ^ `v` is a `&` reference, so it cannot be borrowed as mutable
help: consider changing this to be a mutable reference
  | pub fn ajouter(v: &mut Vec<i64>) {
```

Pour le second, en style fonctionnel la bonne correction n'est souvent pas `&mut` : la fonction prend `&[i64]` et
**rend** un nouveau `Vec`.

## 8. `move` et la capture des closures

Une closure peut utiliser les variables qui l'entourent : elle les **capture**. Par défaut, elle les capture de la
façon la plus légère possible, c'est-à-dire par **référence**. Tant que la closure est utilisée sur place (dans un
`filter`, un `map`), c'est parfait. Mais si la fonction **rend** la closure, les variables locales meurent à la fin
de la fonction, et la closure garderait des références vers rien :

```rust
pub fn plus_grand_que(seuil: i64) -> impl Fn(i64) -> bool {
    |x| x > seuil
}
```

```text
error[E0373]: closure may outlive the current function, but it borrows `seuil`, which is owned by the current function
2 |     |x| x > seuil
  |     ^^^     ----- `seuil` is borrowed here
  |     |
  |     may outlive borrowed value `seuil`
note: closure is returned here
help: to force the closure to take ownership of `seuil` (and any other referenced variables), use the `move` keyword
  |     move |x| x > seuil
```

`move` dit : « la closure **prend possession** de ce qu'elle capture ». Pour un `i64` (`Copy`), elle en garde une
copie. Pour un `Vec<String>`, elle le **déplace** à l'intérieur : la fonction d'origine ne peut plus s'en servir.

```rust
let mots = vec![String::from("a")];
let compter = move || mots.len();
compter() + mots.len() // erreur
```

```text
error[E0382]: borrow of moved value: `mots`
3 |     let compter = move || mots.len();
  |                   ------- ---- variable moved due to use in closure
  |                   |
  |                   value moved into closure here
4 |     compter() + mots.len()
  |                 ^^^^ value borrowed here after move
```

Règle simple : **une closure rendue par une fonction (`-> impl Fn…`) ou stockée dans une `Box` porte presque
toujours `move`**. Une closure passée tout de suite à `map` ou `filter` n'en a pas besoin.

## 9. `Fn`, `FnMut`, `FnOnce`

Ce sont les trois traits des closures. Le compilateur choisit tout seul lequel votre closure implémente, d'après ce
qu'elle fait de ses captures :

| Trait | La closure… | Appelable | Exemple |
|---|---|---|---|
| `Fn` | ne fait que **lire** ses captures | autant de fois qu'on veut | `move \|x\| x > seuil` |
| `FnMut` | **modifie** une capture | plusieurs fois, mais pas en même temps | `\|\| compteur += 1` |
| `FnOnce` | **donne** (déplace) une capture | une seule fois | `move \|\| nom` (rend la `String` capturée) |

Côté fonction qui **reçoit** une closure, choisissez le trait le plus permissif pour l'appelant : `FnOnce` si vous
l'appelez une fois, `FnMut` si vous l'appelez plusieurs fois, `Fn` si vous devez l'appeler à travers une simple
référence `&` (par exemple depuis une autre closure `Fn`, ou en la partageant entre plusieurs endroits). `map` et `filter` demandent `FnMut` ; `pf_outils::trier_par` aussi.

Une closure qui modifie une capture passée là où on attend `Fn` :

```text
error[E0594]: cannot assign to `compteur`, as it is a captured variable in a `Fn` closure
1 | fn appeler(f: impl Fn()) {
  |               --------- change this to accept `FnMut` instead of `Fn`
6 |     appeler(|| compteur += 1);
  |     ------- -- ^^^^^^^^^^^^^ cannot assign
  |     expects `Fn` instead of `FnMut`
```

Une closure qui donne sa capture (donc `FnOnce`) passée là où on veut l'appeler deux fois :

```text
error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
6 |     let donner = move || nom;
  |                  ^^^^^^^ --- closure is `FnOnce` because it moves the variable `nom` out of its environment
7 |     appeler_deux_fois(donner)
  |                       ------ the requirement to implement `Fn` derives from here
```

Si la closure est écrite directement dans l'appel, le message change (même cause) :

```text
error[E0507]: cannot move out of `nom`, a captured variable in an `Fn` closure
6 |     appeler_deux_fois(move || nom)
  |                       ------- ^^^ `nom` is moved here
  |                       captured by this `Fn` closure
help: `Fn` and `FnMut` closures require captured values to be able to be consumed multiple times, but `FnOnce` closures may consume them only once
```

Corrections : soit la closure rend une copie (`move || nom.clone()`, elle devient `Fn`), soit la fonction
n'appelle la closure qu'une fois et accepte `impl FnOnce`.

## 10. `impl Fn` ou `Box<dyn Fn>`

Chaque closure a son **propre type**, unique et sans nom. Deux closures écrites pareil n'ont pas le même type.

1. **`impl Fn(i64) -> i64`** veut dire « un certain type de closure, un seul, que je ne nomme pas ». Rapide (pas
   d'allocation), c'est le choix par défaut pour **rendre une** closure.
2. **`Box<dyn Fn(i64) -> i64>`** veut dire « n'importe quelle closure de cette forme, rangée sur le tas ». Il faut
   le prendre dès qu'il y a **plusieurs** closures différentes au même endroit : deux branches d'un `if`, un `Vec`
   de règles, un champ de `struct` qui peut changer.

Deux closures dans les branches d'un `if` avec `impl Fn` :

```rust
pub fn operation(doubler: bool, n: i64) -> impl Fn(i64) -> i64 {
    if doubler { move |x| x * n } else { move |x| x + n }
}
```

```text
error[E0308]: `if` and `else` have incompatible types
  |     if doubler { move |x| x * n } else { move |x| x + n }
  |                  --------------          ^^^^^^^^^^^^^^ expected closure, found a different closure
  = note: no two closures, even if identical, have the same type
  = help: consider boxing your closure and/or using it as a trait object
help: you could change the return type to be a boxed trait object
  | pub fn operation(doubler: bool, n: i64) -> Box<dyn Fn(i64) -> i64> {
```

```rust
pub fn operation(doubler: bool, n: i64) -> Box<dyn Fn(i64) -> i64> {
    if doubler { Box::new(move |x| x * n) } else { Box::new(move |x| x + n) }
}
```

Oublier le `Box::new` autour d'une closure :

```text
error[E0308]: mismatched types
  |     vec![move |x| x >= min, move |x| x <= max]
  |          ^^^^^^^^^^^^^^^^^ expected `Box<dyn Fn(i64) -> bool>`, found closure
help: store this in the heap by calling `Box::new`
```

Oublier la `Box` dans le type (un `dyn Fn` seul n'a pas de taille connue) :

```text
error[E0746]: return type cannot be a trait object without pointer indirection
1 | pub fn operation(n: i64) -> dyn Fn(i64) -> i64 {
  |                             ^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
help: consider returning an `impl Trait` instead of a `dyn Trait`

error[E0277]: the size for values of type `(dyn Fn(i64) -> bool + 'static)` cannot be known at compilation time
1 | pub fn regles() -> Vec<dyn Fn(i64) -> bool> {
```

Curiosité qui peut vous tromper : deux closures **sans aucune capture** (`|x| x * 2` et `|x| x + 1`) passent dans
un `if` avec `impl Fn`, car le compilateur les convertit en simples pointeurs de fonction (`fn(i64) -> i64`). Dès
qu'elles capturent quelque chose, l'erreur ci-dessus revient.

## 11. `Option<&T>` ou `Option<T>` : `copied`, `cloned`, `as_ref`, `as_deref`

`find`, `first`, `last`, `max`, `min`, `get` (sur un `Vec` ou une `HashMap`) **ne prennent rien** à la collection :
ils rendent une **référence** vers l'élément trouvé, dans un `Option`. Donc `Option<&T>`, pas `Option<T>`.

```rust
pub fn premier_negatif(nombres: &[i64]) -> Option<i64> {
    nombres.iter().find(|n| **n < 0)
}
```

```text
error[E0308]: mismatched types
1 | pub fn premier_negatif(nombres: &[i64]) -> Option<i64> {
  |                                            ----------- expected `Option<i64>` because of return type
2 |     nombres.iter().find(|n| **n < 0)
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Option<i64>`, found `Option<&i64>`
help: use `Option::copied` to copy the value inside the `Option`
  |     nombres.iter().find(|n| **n < 0).copied()
```

Même message avec `ages.get(nom)` sur une `HashMap<String, u32>`, et avec `mots.first()` sur des `String` (là, le
compilateur propose `.cloned()`).

Dans l'autre sens, quand on a un `&Option<String>` (une option qu'on ne possède pas), `map` voudrait la consommer :

```rust
pub fn longueur(nom: &Option<String>) -> Option<usize> {
    nom.map(|s| s.len())
}
```

```text
error[E0507]: cannot move out of `*nom` which is behind a shared reference
  |     nom.map(|s| s.len())
  |     ^^^ ---------------- `*nom` moved due to this method call
  |     help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents
note: `Option::<T>::map` takes ownership of the receiver `self`, which moves `*nom`
```

Le résumé des conversions :

| J'ai | Je veux | J'écris |
|---|---|---|
| `Option<&i64>` (type `Copy`) | `Option<i64>` | `.copied()` |
| `Option<&String>` | `Option<String>` | `.cloned()` |
| `&Option<String>` | `Option<&String>` (pour un `map` sans rien consommer) | `.as_ref()` |
| `&Option<String>` | `Option<&str>` | `.as_deref()` |

`as_ref()` seul ne suffit pas pour un `Option<&str>` :

```text
error[E0308]: mismatched types
  |     adresse.as_ref()
  |     ^^^^^^^^^^^^^^^^ expected `Option<&str>`, found `Option<&String>`
help: try converting the passed type into a `&str`
  |     adresse.as_ref().map(|x| x.as_str())
```

`adresse.as_deref()` fait exactement ce `as_ref().map(|x| x.as_str())` en un appel.

## 12. Les annotations que le compilateur réclame (`collect::<Vec<_>>`, `sum::<i64>()`, turbofish)

`collect`, `sum`, `product` et `parse` peuvent produire **plusieurs** types (`collect` : un `Vec`, une `HashMap`, une
`String`… ; `sum` : un `i64`, un `f64`…). Quand rien ne fixe le type attendu, le compilateur refuse de deviner :

```rust
let doubles = nombres.iter().map(|n| n * 2).collect();
doubles.len()
```

```text
error[E0282]: type annotations needed
2 |     let doubles = nombres.iter().map(|n| n * 2).collect();
  |         ^^^^^^^
3 |     doubles.len()
  |     ------- type must be known at this point
help: consider giving `doubles` an explicit type
  |     let doubles: Vec<_> = nombres.iter().map(|n| n * 2).collect();
```

```text
error[E0283]: type annotations needed
  |     nombres.iter().sum() > 10
  |                    ^^^ cannot infer type of the type parameter `S` declared on the method `sum`
help: consider specifying the generic argument
  |     nombres.iter().sum::<S>() > 10

error[E0284]: type annotations needed
  |     texte.parse().unwrap() > 3
  |           ^^^^^ cannot infer type of the type parameter `F` declared on the method `parse`
```

Trois endroits où donner le type, au choix :

1. **Le type de retour de la fonction** : `fn f(…) -> Vec<i64> { … .collect() }`. Le plus courant, rien à écrire.
2. **Le type de la variable** : `let doubles: Vec<i64> = … .collect();`
3. **Le *turbofish* `::<>`** sur la méthode : `.collect::<Vec<_>>()`, `.sum::<i64>()`, `.parse::<i64>()`. Pratique au
   milieu d'une expression, sans variable intermédiaire.

Le `_` dans `Vec<_>` veut dire « devine le type des éléments » : on précise seulement la collection.

Quand le type est donné mais **ne correspond pas** aux éléments (`Vec<i64>` à partir de `&i64`), le message n'est
plus E0282 mais E0277 ``cannot be built from an iterator over elements of type `&i64` `` : relisez la section 3.

## 13. Durées de vie : le minimum pour ces exercices

Une référence ne doit jamais survivre à ce qu'elle désigne. Le compilateur le vérifie grâce aux **durées de vie**
(*lifetimes*, notées `'a`). Dans ces exercices, il suffit de savoir trois choses.

**1. Le plus souvent, on n'écrit rien (élision).** Une fonction qui reçoit **une seule** référence et rend une
référence : le compilateur comprend que la sortie est empruntée à l'entrée.

```rust
pub fn premier_mot(texte: &str) -> &str {
    texte.split_whitespace().next().unwrap_or("")
}
pub fn mots(texte: &str) -> Vec<&str> {
    texte.split_whitespace().collect()
}
```

**2. Deux références en entrée, une en sortie : il faut dire laquelle.**

```rust
pub fn plus_long(a: &str, b: &str) -> &str {
    if a.len() >= b.len() { a } else { b }
}
```

```text
error[E0106]: missing lifetime specifier
1 | pub fn plus_long(a: &str, b: &str) -> &str {
  |                     ----     ----     ^ expected named lifetime parameter
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `a` or `b`
help: consider introducing a named lifetime parameter
  | pub fn plus_long<'a>(a: &'a str, b: &'a str) -> &'a str {
```

Lisez `'a` comme « le résultat vit aussi longtemps que `a` et `b` ». Même erreur pour un champ référence dans une
`struct` : `struct Mot<'a> { texte: &'a str }`.

**3. On ne rend jamais une référence vers une variable locale.** Elle est détruite à la fin de la fonction.

```rust
pub fn en_majuscules(texte: &str) -> &str {
    let maj = texte.to_uppercase();
    &maj
}
```

```text
error[E0515]: cannot return reference to local variable `maj`
3 |     &maj
  |     ^^^^ returns a reference to data owned by the current function
```

Le piège classique dans ces exercices : transformer le texte **puis** le découper.

```rust
pub fn mots(texte: &str) -> Vec<&str> {
    let propre = texte.to_lowercase();
    propre.split_whitespace().collect()
}
```

```text
error[E0515]: cannot return value referencing local variable `propre`
3 |     propre.split_whitespace().collect()
  |     ------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |     |
  |     returns a value referencing data owned by the current function
  |     `propre` is borrowed here
```

Écrit en une seule chaîne (`texte.to_lowercase().split_whitespace().collect()`), c'est la même erreur avec
« cannot return value referencing temporary value ».

Variante à l'intérieur d'une fonction : découper un texte fabriqué dans la même ligne et garder les morceaux dans
une variable.

```rust
let mots: Vec<&str> = brut.to_lowercase().split_whitespace().collect();
mots.len()
```

```text
error[E0716]: temporary value dropped while borrowed
2 |     let mots: Vec<&str> = brut.to_lowercase().split_whitespace().collect();
  |                           ^^^^^^^^^^^^^^^^^^^                             - temporary value is freed at the end of this statement
  |                           |
  |                           creates a temporary value which is freed while still in use
3 |     mots.len()
  |     ---- borrow later used here
help: consider using a `let` binding to create a longer lived value
```

La `String` fabriquée par `to_lowercase()` n'a pas de nom : elle est détruite à la fin de la ligne, et les `&str`
pointeraient dans le vide. Donnez-lui un nom (`let minuscules = brut.to_lowercase();`), puis découpez
`minuscules` : elle vit alors jusqu'à la fin de la fonction.

Correction : si vous **fabriquez** un nouveau texte (`to_lowercase`, `to_uppercase`, `format!`), rendez des
`String` ; si vous rendez des **morceaux** du texte reçu tels quels, rendez des `&str`.

## Les exercices

Six exercices courts, un piège chacun. Au-dessus de chaque fonction à écrire, un bloc **Aide Rust** renvoie à la
section de ce kit et annonce ce que dira le compilateur si on se trompe.

Pour chaque exercice : remplacez le `todo!()` de chaque fonction à écrire, puis lancez ses tests, par exemple
`cargo test -p ch00-survie e01`. L'exercice est réussi quand tous ses tests passent, y compris `style_fonctionnel`,
qui vérifie que votre code n'utilise ni `let mut`, ni `for`, ni `while`, ni `loop`, ni `for_each`.

| # | Fichier | Le piège | Section |
|---|---|---|---|
| 1 | `e01_depassements.rs` | `&i64` dans un itérateur, `copied`, le `&&` de `filter` | 3, 4 |
| 2 | `e02_mots_empruntes.rs` | rendre des `&str` empruntés au texte plutôt que des `String` | 5, 13 |
| 3 | `e03_noms_clones.rs` | `cloned` sur des `String`, et quand le `clone` est en trop | 6 |
| 4 | `e04_accumulateur.rs` | `\|mut acc, x\|` dans un `fold` qui construit un `Vec` ou une `HashMap` | 7 |
| 5 | `e05_closure_move.rs` | une closure `move` rendue par une fonction | 8, 9 |
| 6 | `e06_option_reference.rs` | `Option<&T>` vers `Option<T>` après `find` ou `get` | 11 |

## Pour aller plus loin

- [The Rust Book, chapitre 4](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) : possession et
  emprunts.
- [The Rust Book, chapitre 13](https://doc.rust-lang.org/book/ch13-00-functional-features.html) : closures et
  itérateurs.
- [L'index des codes d'erreur](https://doc.rust-lang.org/error_codes/error-index.html), ou `rustc --explain E0xxx`.
- 100 Exercises, chapitres `03_ticket_v1` (possession, `&`, `&mut`) et `06_ticket_management` (itérateurs,
  durées de vie).
