# Chapitre 10 : newtypes, un type par concept, parse don't validate

> Prérequis : chapitres 8 (`Option`, `Result`) et 9 (types somme). Cours : C2, diapos « Newtypes » à « Parse,
> don't validate ». Tests : `cargo test -p ch10-newtypes`.

## L'idée

Au chapitre 9, on a retiré les états illégaux en choisissant entre `struct` et `enum`. Il en reste dans les types
primitifs : une `String` peut contenir n'importe quoi, un `u64` d'utilisateur se passe là où on attend un `u64` de
commande, un `f64` peut valoir `NaN`. Le **newtype** est un `struct` à un seul champ qui donne un nom (et des règles)
à un concept du domaine.

```rust
pub struct Email(String);        // champ privé : on ne construit pas un Email à la main
pub struct IdCommande(u64);      // même taille qu'un u64, mais plus interchangeable avec un IdUtilisateur
pub struct Pourcentage(u8);      // 0 à 100, garanti
```

Coût à l'exécution : zéro. `size_of::<IdCommande>() == size_of::<u64>()`, et le code machine est le même.

## Parse, don't validate

**Valider**, c'est vérifier et rendre un `bool` : l'information est perdue aussitôt, et la suite du programme
manipule toujours une `&str` en espérant qu'on n'a pas oublié de vérifier sur un autre chemin. **Parser**, c'est
vérifier et rendre un type plus précis, qui garde la preuve.

```rust
// Valider : la preuve se perd
if est_email_valide(texte) { envoyer(texte) }   // envoyer(&str) accepte tout

// Parser : la preuve voyage avec la valeur
let email = Email::try_from(texte)?;            // une fois, à la frontière
envoyer(email);                                 // envoyer(Email) n'accepte que du valide
```

On parse **à la frontière** (formulaire, fichier, requête HTTP) ; le cœur du programme ne reçoit que des types
sûrs et ne revérifie jamais. Formule d'Alexis King (« Parse, don't validate », 2019).

## Ce qui est nouveau en Rust

1. **La *tuple struct*.** `struct Email(String)` est une `struct` dont le champ n'a pas de nom : on le lit avec
   `self.0`, on construit avec `Email(texte)`.
2. **Le champ privé.** Sans `pub` devant le champ, seul le code du même module (et de ses sous-modules, comme
   `mod tests`) y a accès. Ici, un module est un fichier : dans `e01_email.rs`, vous pouvez écrire `Email(…)` ;
   ailleurs (un autre fichier, les doc-tests), le compilateur refuse avec
   `` error[E0603]: tuple struct constructor `Email` is private ``. C'est ce qui oblige tout le monde à passer par
   votre constructeur.
3. **Implémenter un `trait`.** Au chapitre 2, un `trait` était une capacité qu'un type déclare avoir (`Fn`). Vous
   écrivez maintenant vous-même `impl TryFrom<&str> for Email { … }` : « voici comment `Email` remplit le contrat de
   `TryFrom<&str>` ». La documentation du trait dit quelles fonctions écrire, avec quelle signature ; la signature
   est déjà posée dans les exercices, vous écrivez le corps.
4. **Le type associé.** Dans `impl TryFrom<&str> for Email`, la ligne `type Error = ErreurEmail;` est un **type
   associé** : le trait demande à chaque implémentation de nommer son type d'erreur, et `Self::Error` y renvoie
   ensuite (`Self` est le type de l'`impl`, ici `Email`).
5. **Un bloc générique.** `impl<T> NonVide<T> { … }` écrit des méthodes pour `NonVide<T>` quel que soit `T` ;
   `impl<T: Ord> NonVide<T>` en ajoute d'autres, disponibles seulement quand `T` sait se comparer.

## Les outils du chapitre

| Outil | Rôle |
|---|---|
| champ privé `struct Email(String)` | personne hors du module ne peut construire un `Email` sans passer par vos règles |
| `impl TryFrom<&str> for Email` | le constructeur faillible standard ; donne `try_into()` en prime |
| `impl FromStr for Email` | branche `"…".parse::<Email>()` ; son corps appelle `try_from`, la règle n'est écrite qu'une fois |
| `impl Display for Email` | `to_string()` et `format!("{email}")` |
| un accesseur `as_str(&self) -> &str` | lecture seule ; jamais de `pub` sur le champ |
| `impl Ord` à la main | quand l'invariant rend un ordre total honnête (`Score`) |

```rust
impl TryFrom<i64> for Pourcentage {
    type Error = HorsBornes;
    fn try_from(valeur: i64) -> Result<Self, Self::Error> {
        if (0..=100).contains(&valeur) { Ok(Pourcentage(valeur as u8)) } else { Err(HorsBornes(valeur)) }
    }
}
```

## L'invariant, et ce qu'il achète

Une fois l'invariant posé, les méthodes n'ont plus de cas d'échec : `Pourcentage::complement` rend un
`Pourcentage` (pas un `Result`), `NonVide::premier` rend un `&T` (pas une `Option`), `Email::domaine` rend une
`&str`. Si une méthode doit quand même gérer le « cas impossible » pour satisfaire le compilateur, `expect` avec la
raison est légitime : `expect("invariant d'Email : exactement un @")`. S'il se déclenche, c'est un bug de **votre**
module, pas une donnée utilisateur : exactement la définition d'un `panic!` (chapitre 8).

## Pourquoi `f64` ne se trie pas

`f64` n'est que `PartialOrd` : `NaN` n'est comparable à rien, pas même à lui-même. `sort()` exige `Ord`, d'où
l'erreur E0277 sur un `Vec<f64>`. Le contournement `sort_by(|a, b| a.partial_cmp(b).unwrap())` panique au premier
`NaN`. Le newtype `Score` exclut `NaN` à la construction ; sur les valeurs restantes, `f64::total_cmp` est un ordre
total honnête, et `Score` peut implémenter `Ord` : `sort`, `max`, `BTreeSet` deviennent possibles.

`Ord` ne vient pas seul : il exige que le type implémente aussi `PartialOrd`, `PartialEq` et `Eq`. Dans l'exercice
3, ces trois-là sont déjà écrits et s'appuient tous sur votre `cmp`, pour que les quatre comparaisons soient
toujours d'accord.

## Les pièges

1. **`pub` sur le champ.** `pub struct Email(pub String)` annule tout : n'importe qui écrit `Email("".into())`.
2. **`len()` compte des octets.** « é » fait 2 octets en UTF-8 : pour compter des caractères, `chars().count()`.
3. **Le zéro négatif.** `-0.0 == 0.0` est vrai, mais `total_cmp` les distingue. Si `Ord` et `PartialEq` ne sont pas
   d'accord, `BTreeSet` et le tri se comportent bizarrement : on normalise `-0.0` à la construction, et
   `PartialEq` s'appuie sur `cmp`.
4. **Un code postal n'est pas un nombre.** Ni un numéro de téléphone, ni un IBAN : un `u32` perd les zéros de tête.
   Un identifiant qui ne se calcule pas est une chaîne (typée).
5. **`derive(PartialEq)` sur un type d'erreur qui contient un `f64`.** `NonFini(f64::NAN) != NonFini(f64::NAN)` :
   ne dérivez que ce qui a du sens, et testez avec `is_err()` ou `matches!`.
6. **Deux portes, une règle.** `from_str` appelle `try_from` (ou l'inverse) : si la règle est écrite deux fois, elle
   finira par diverger.

## Ce qui va coincer (et comment s'en sortir)

Voici les erreurs les plus courantes sur ces exercices, telles que le compilateur les affiche. Chaque fonction à
écrire porte aussi un bloc « Aide Rust » qui reprend ses pièges à elle.

1. **`expected String, found &str`**
   ```text
   error[E0308]: mismatched types
      |     Ok(Email(texte))
      |        ----- ^^^^^ expected `String`, found `&str`
   ```
   Le champ possède sa chaîne, le paramètre ne fait que la prêter. Fabriquez la `String` au dernier moment :
   `Email(texte.to_string())`. Kit de survie (ch00), section 5.

2. **`fmt` qui ne rend pas un `fmt::Result`**
   ```text
   error[E0308]: mismatched types
      |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      |        ---                                       ^^^^^^^^^^^ expected `Result<(), Error>`, found `()`
      |        |
      |        implicitly returns `()` as its body has no tail or return expression
   ```
   `fmt` écrit dans `f` avec `write!(f, ...)` et rend ce que `write!` rend : pas de `;` après. Renvoyer `self.0`
   (`found String`) ou un `println!` (`found ()`) ne marche pas non plus. Et `write!(f, "{}", self)` compile mais se
   rappelle sans fin : le test meurt sur `has overflowed its stack`. ch00, section 1.

3. **Une `Option` là où l'invariant promet une valeur**
   ```text
   error[E0308]: mismatched types
      |     pub fn domaine(&self) -> &str {
      |                              ---- expected `&str` because of return type
      |         self.0.split_once('@').map(|(_, d)| d)
      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&str`, found `Option<&str>`
   ```
   L'invariant garantit le `Some` : `expect("invariant d'Email : exactement un @")` est légitime ici, parce que s'il
   échoue c'est un bug de votre module. ch00, section 11.

4. **`f64` is not an iterator** (alors qu'on voulait juste comparer)
   ```text
   error[E0599]: `f64` is not an iterator
      |         self.0.cmp(&autre.0)
      |                ^^^ `f64` is not an iterator
   ```
   Message trompeur : `f64` n'a pas de `cmp` parce qu'il n'est pas `Ord`, et le compilateur a trouvé le `cmp` des
   itérateurs. Pour des flottants sans `NaN`, `total_cmp` rend un `Ordering`. ch00, section 1.

5. **Une référence là où on attend une valeur**
   ```text
   error[E0277]: the trait bound `Score: TryFrom<&f64>` is not satisfied
      |     scores.iter().map(|s| Score::try_from(s)).collect()
   error[E0277]: a value of type `Vec<IdCommande>` cannot be built from an iterator over elements of type `&IdCommande`
   ```
   `iter()` prête les éléments : on reçoit des `&f64`, des `&Commande`. Pour un type `Copy`, déréférencez (`|&s|`,
   `*s`, `c.id` sans `&`) ou ajoutez `.copied()`. ch00, sections 2 et 3.

## Les exercices

Les types et les squelettes d'`impl` sont posés : vous écrivez les corps.

| # | Fichier | Ce qu'on pratique |
|---|---|---|
| 1 | `e01_email.rs` | valider (`bool`) → parser (`TryFrom`, `FromStr`, `Display`) ; un enum d'erreur précis |
| 2 | `e02_pourcentage.rs` | une borne vérifiée une fois ; `complement` sans `Result` ; parser une saisie humaine |
| 3 | `e03_score.rs` | `f64` fini, `Ord` via `total_cmp`, trier et prendre le max sans `unwrap` |
| 4 | `e04_identifiants.rs` | deux `u64` interchangeables, le bug d'inversion, puis deux newtypes |
| 5 | `e05_non_vide.rs` | `NonVide<T>` générique : `premier()` et `maximum()` sans `Option` |
| 6 | `e06_code_postal.rs` | sans version naïve ; tout le newtype, de la règle à l'affichage |

Plusieurs modules contiennent des doc-tests `compile_fail` : ils prouvent qu'on ne construit pas un newtype sans
son constructeur, et qu'inverser deux identifiants ne compile plus (`cargo test -p ch10-newtypes --doc`).

## Pour aller plus loin

- Alexis King, [Parse, don't validate](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/) (2019).
- Rust API Guidelines, [Newtypes provide static distinctions](https://rust-lang.github.io/api-guidelines/type-safety.html).
- La doc de [`TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html), de
  [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) et de
  [`f64::total_cmp`](https://doc.rust-lang.org/std/primitive.f64.html#method.total_cmp).
- Dans la bibliothèque standard, des newtypes à invariant : `NonZeroU32`, `String` (un `Vec<u8>` garanti UTF-8).
- 100 Exercises, chapitre `05_ticket_v2`, exercice `13_try_from`. En C3 : `Id<T>` avec `PhantomData`, un seul type
  générique pour tous les identifiants.
