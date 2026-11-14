# Chapitre 13 : traits au service du domaine

> Prérequis : chapitres 10 (newtypes) et 12 (typestate). Cours : C3, diapos « Associated types » à « Surcharge
> d'opérateurs : `Add` ». Tests : `cargo test -p ch13-traits`.

## L'idée

Un trait est un contrat : « ce type sait se convertir, s'afficher, s'additionner, se parcourir ». En implémentant
les traits de la std sur vos types métier, vous branchez votre domaine sur tout ce que la std sait déjà faire :
`{}`, `+`, `.sum()`, `.into()`, `for`, `map`, `filter`. Et une fonction générique écrite une fois, avec un bound
(voir la section suivante), marche pour tous les types qui respectent le contrat.

```rust
// Avant : une fonction par métrique, copiée-collée
rapport_exactitude_naif(&v, &p);
rapport_mae_naif(&v2, &p2);

// Après : une fonction, pour toute métrique
fn rapport<M: Metrique>(m: &M, verite: &[M::Valeur], predictions: &[M::Valeur]) -> String
```

Ce que l'on y gagne : les règles du domaine (une formule de conversion, l'affichage d'un montant, les valeurs par
défaut) sont écrites une seule fois, dans l'`impl` du trait, au lieu d'être répétées dans des fonctions
`formater_xxx` ou `rapport_xxx_naif` qu'on finit par copier-coller et par oublier d'appeler.

## Implémenter un trait

```rust
use std::fmt;

impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} °C", self.0)
    }
}

println!("{}", Celsius(21.5)); // 21.5 °C
```

`impl Trait for Type { … }` fournit les méthodes que le trait exige, avec exactement la signature qu'il impose :
recopiez-la depuis la documentation du trait. Ici, `write!(f, …)` s'écrit comme `format!`, mais écrit dans `f` et
rend un `fmt::Result` au lieu d'une `String`.

Trois mots reviennent dans tout le chapitre :

1. Un **bound** (`M: Metrique`, `impl Into<Celsius>`) est une condition sur un type générique : « n'importe quel
   type, pourvu qu'il implémente ce trait ».
2. Un **type associé** (`type Valeur;`, `type Output;`, `type Item;`) est un type que chaque implémentation
   choisit, et qu'on écrit ensuite `Self::Valeur` ou `M::Valeur`.
3. Une **blanket impl** est une implémentation pour tous les types qui remplissent une condition : la std a
   `impl<T, U: From<T>> Into<U> for T`, d'où `Into` gratuit dès que vous écrivez `From`.

`#[derive(Default)]` écrit `default()` à votre place quand chaque champ a déjà une valeur par défaut (`0`, chaîne
vide, `None`…) ; sur un `enum`, l'attribut `#[default]` désigne la variante par défaut. Quand le défaut métier est
autre (page 1, et non 0), on écrit l'`impl Default` à la main.

## Les outils du chapitre

| Trait | Ce qu'on écrit | Ce qu'on reçoit |
|---|---|---|
| `From<A> for B` | `fn from(a: A) -> B` | `a.into()`, et `?` sur les erreurs |
| `Display` | `fn fmt` | `{}`, `format!`, `to_string()` |
| `Default` | `fn default()` | `..Default::default()` |
| `Add`, `Mul<u32>` | `type Output`, `fn add` / `fn mul` | `a + b`, `prix * 3` |
| `Sum` | `fn sum<I: Iterator<Item = Self>>` | `.sum()` sur un itérateur de votre type |
| `Iterator` | `type Item`, `fn next(&mut self)` | des dizaines de méthodes : `take`, `filter`, `count`… |
| trait métier | `type Valeur`, vos méthodes | des fonctions génériques `<M: Metrique>` |

## `From`, jamais `Into`

On implémente `From`, et la std offre `Into` en retour (blanket impl). En argument, on accepte large :

```rust
pub fn plus_chaude(a: impl Into<Celsius>, b: impl Into<Celsius>) -> Celsius {
    let (a, b) = (a.into(), b.into());
    if a >= b { a } else { b }
}
plus_chaude(Fahrenheit(80.0), Kelvin(300.0));
```

`From` pour une conversion qui ne peut pas échouer, `TryFrom` (C2) quand elle le peut.

## Type associé ou paramètre générique ?

`trait Metrique { type Valeur; … }` : chaque métrique a **un seul** type de valeur, qu'elle choisit. L'exactitude
compare des `u32`, la MAE des `f64`. L'appelant n'a pas à le préciser : `rapport(&Mae, …)` sait déjà qu'il faut des
`f64`. Un paramètre générique (`trait Metrique<V>`) permettrait plusieurs implémentations pour un même type ; ce
n'est pas ce qu'on veut ici.

## Implémenter `Iterator`

```rust
impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> { /* rendre le terme, avancer */ }
}

let pairs: u64 = Fibonacci::default().take_while(|&n| n <= 100).filter(|n| n % 2 == 0).sum();
```

`next` est le seul endroit où l'état avance, et `&mut self` y est normal : c'est le rôle d'un itérateur. Rendre
`None` termine le parcours.

## Ce que le compilateur refuse : les doc-tests `compile_fail`

Comme au chapitre 12, certains commentaires `///` contiennent des exemples de code que `cargo test` compile :

1. un bloc `no_run` montre un usage permis : il doit compiler (il n'est pas exécuté) ;
2. un bloc `compile_fail` montre un usage que vos types doivent **interdire** : le doc-test réussit si ce code ne
   compile pas. Par exemple, au-dessus de `rapport` :

````rust
/// ```compile_fail,E0308
/// use ch13_traits::e04_metrique::{Exactitude, rapport};
/// let a = rapport(&Exactitude, &[1.5, 2.0], &[1.0, 2.0]);
/// ```
````

Ils sont dans les exercices 1 (un `f64` nu refusé par `plus_chaude`), 3 (`Argent * Argent` refusé) et 4 (des réels
refusés par l'exactitude). Ils **passent dès le départ**, avant que vous ayez écrit une ligne : ce qui rend ces
appels impossibles, ce sont les signatures et les `impl` déjà donnés (`Mul<u32>` et non `Mul<Argent>`,
`type Valeur = u32`), pas le corps des fonctions. Ils restent verts tant que vous ne changez pas ces signatures, et
montrent en une ligne ce que le typage vous apporte : l'erreur est trouvée à la compilation, il n'y a pas de test
d'exécution à écrire pour elle. `cargo test -p ch13-traits` les lance avec les autres (sous
`Doc-tests ch13_traits`), `cargo test -p ch13-traits --doc` lance seulement eux.

## Les pièges

1. **`impl Into` à la main.** Clippy le signale : implémentez `From`.
2. **`let c = f.into();`** ne compile pas seul : le compilateur ne sait pas vers quoi convertir. Annotez
   (`let c: Celsius = f.into();`).
3. **Un opérateur qui n'a pas de sens.** `Argent * Argent` ne veut rien dire, `Argent * u32` (prix × quantité) oui.
   Un opérateur ne doit jamais casser un invariant (C3, « Et `Add` sur `Score` ? »).
4. **Plusieurs `fn fmt` dans un fichier.** Chaque `impl Display` a son `fmt` : c'est normal, le trait fixe le nom.
5. **Un itérateur qui déborde.** F(94) ne tient pas dans un `u64` : `checked_add` et un `None` propre, pas un
   `panic!` en debug et des valeurs fausses en release.
6. **`Ord` dérivé** compare les champs dans l'ordre de déclaration. Pour une date : année, mois, jour, dans cet
   ordre, sinon le tri est faux.

## Ce qui va coincer (et comment s'en sortir)

Voici les erreurs les plus fréquentes sur ces exercices, avec ce que le compilateur affiche. Chaque fonction à écrire
porte aussi un bloc « Aide Rust ».

1. **Calculer sur le newtype au lieu de son champ**
   ```text
   error[E0369]: cannot subtract `{float}` from `Fahrenheit`
      |         Celsius((f - 32.0) * 5.0 / 9.0)
      |                  - ^ ---- {float}
   ```
   `f` est un `Fahrenheit`, pas un nombre : le nombre est `f.0`. Même famille dans Fibonacci
   (`E0369 cannot add Option<u64> to Option<u64>`) : sortez d'abord le nombre de l'`Option`. ch00, section 1.

2. **Un trait qui se rappelle lui-même**
   ```text
   warning: function cannot return without recursing
      |     fn add(self, autre: Argent) -> Argent {
      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot return without recursing
      |         self + autre
      |         ------------ recursive call site
   ```
   `self + autre` dans `add` appelle `add`. Même piège avec `..Default::default()` dans `default`, et avec
   `self.calculer(...)` que le compilateur propose lui-même quand on écrit `calculer(...)` seul. Lisez ce warning
   comme une erreur : à l'exécution, c'est `stack overflow`. Attention, `montants.sum()` dans `Sum::sum` boucle de
   la même façon **sans** avertissement.

3. **Une méthode qui exige `Ord`**
   ```text
   error[E0599]: the method `max` exists for struct `Celsius`, but its trait bounds were not satisfied
      = note: the following trait bounds were not satisfied:
              `Celsius: Ord`
   ```
   Un `f64` n'est que `PartialOrd` (chapitre 10), un newtype autour aussi : `max`, `sort`, `cmp` ne sont pas là.
   Un `if a >= b` suffit, ou `total_cmp`.

4. **Les closures d'adaptateurs reçoivent des références**
   ```text
   error[E0308]: mismatched types
      |     Fibonacci::default().take_while(|t| t <= max)
      |                                              ^^^ expected `&u64`, found `u64`
   ```
   `take_while`, `filter`, `find` prêtent l'élément à la closure. Déstructurez : `|&t| t <= max`. ch00, section 4.

5. **`IntoIterator` n'est pas encore un itérateur**
   ```text
   error[E0599]: no method named `filter` found for type parameter `impl IntoIterator<Item = Date>` in the current scope
      |     dates.filter(|d| ...)
      |           ^^^^^^ `impl IntoIterator<Item = Date>` is not an iterator
   ```
   `IntoIterator` veut dire « peut devenir un itérateur » (un `Vec`, une plage…) : appelez `into_iter()` d'abord.
   ch00, section 3.

## Les exercices

| # | Fichier | Ce qu'on pratique |
|---|---|---|
| 1 | `e01_temperatures.rs` | `From` entre unités, `impl Into<T>` en argument |
| 2 | `e02_affichage_defaut.rs` | `Display` à la place de `formater_xxx`, `Default` à la place des `Option` |
| 3 | `e03_argent.rs` | `Add`, `Mul<u32>`, `Sum`, `Display` sur un newtype |
| 4 | `e04_metrique.rs` | un trait métier à type associé, une fonction générique avec bound |
| 5 | `e05_fibonacci.rs` | `Iterator` pour son type, arrêt propre avant le débordement |
| 6 | `e06_dates.rs` | sans version naïve ; une plage de dates itérable, `Ord` dérivé, fonction générique |

## Pour aller plus loin

- Le chapitre [« Advanced Traits »](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html) du Book (types
  associés, surcharge d'opérateurs).
- [`std::ops`](https://doc.rust-lang.org/std/ops/index.html), [`std::iter::Sum`](https://doc.rust-lang.org/std/iter/trait.Sum.html),
  [`From`](https://doc.rust-lang.org/std/convert/trait.From.html).
- 100 Exercises, chapitre `04_traits`.
- Arena, palier 3 : le trait `Metric` et son `type Value`.
