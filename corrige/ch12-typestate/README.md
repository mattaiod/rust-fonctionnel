# Chapitre 12 : typestate

> Prérequis : chapitre 11 (erreurs de domaine). Cours : C2, diapos « Typestate » à
> « État sans donnée : `PhantomData` ». Tests : `cargo test -p ch12-typestate`.

## L'idée

Un objet qui a des états (porte ouverte ou fermée, article en brouillon ou publié) se code d'habitude avec un champ
d'état et des vérifications à l'exécution : `if self.statut != "relecture" { return Err(…) }`, ou un `panic!`. Le
compilateur laisse tout passer, l'erreur arrive en production.

Le typestate met l'état **dans le type** : `Porte<Fermee>` et `Porte<Verrouillee>` sont deux types différents.
Chaque méthode n'existe que dans l'état où elle a un sens, et une transition interdite devient une erreur de
compilation.

```rust
let porte = Porte::neuve().verrouiller();
porte.ouvrir();
// error[E0599]: no method named `ouvrir` found for struct `Porte<Verrouillee>` in the current scope
```

On s'appuie pour cela sur les **génériques**. `struct Porte<E>` est un type générique : `E` est un paramètre de
type, remplacé par un vrai type à l'usage (`Porte<Fermee>`), exactement comme `T` dans `Vec<T>` ou `Option<T>`.
Un bloc `impl<E> Porte<E> { … }` se lit « pour tout `E` » : ses méthodes existent dans tous les états. Un bloc
`impl Porte<Fermee> { … }` n'en ajoute qu'au type `Porte<Fermee>` : ses méthodes n'existent pas sur
`Porte<Verrouillee>`. Tout le chapitre repose sur cette différence.

Pourquoi est-ce mieux qu'un `enum` d'état ? Parce que l'erreur est trouvée par le compilateur, avant toute
exécution, et que l'appelant ne peut même pas écrire l'appel interdit : il n'y a pas de cas « transition
impossible » à tester ni à gérer.

## Les outils du chapitre

| Outil | Rôle |
|---|---|
| `struct Fermee;` | un état sans donnée : une struct vide, de taille 0 |
| `struct EnRelecture { relecteur: String }` | un état qui porte sa donnée |
| `struct Porte<E> { …, _etat: PhantomData<E> }` | le paramètre d'état, « utilisé » sans rien stocker |
| `impl Porte<Fermee> { fn ouvrir(self) -> Porte<Ouverte> }` | une transition : n'existe que dans l'état de départ |
| `impl<E> Porte<E> { … }` | ce qui marche dans tous les états |
| `Result<Nouvel, (Ancien, Erreur)>` | une transition qui peut échouer sans perdre l'objet |

## Une transition consomme `self`

```rust
impl Porte<Fermee> {
    pub fn ouvrir(self) -> Porte<Ouverte> {
        Porte { ouvertures: self.ouvertures + 1, _etat: PhantomData }
    }
}
```

`self` (et pas `&mut self`) : l'ancienne valeur est déplacée dans la méthode. Après `let ouverte = fermee.ouvrir();`,
`fermee` n'existe plus (`E0382` si on s'en sert). Impossible de garder une copie « fermée » d'une porte ouverte.

## État avec ou sans donnée

Quand l'état ne porte rien, Rust refuse un paramètre de type inutilisé (`E0392`) : `PhantomData<E>` est le faux
champ qui l'utilise, à coût nul (`size_of::<Porte<Fermee>>() == size_of::<u32>()`). Quand l'état porte une donnée
(le relecteur, le score, la référence de paiement), on le stocke dans un vrai champ `etat: E` : plus besoin de
`PhantomData`, et la donnée n'existe que dans l'état où elle a un sens. Une `Soumission<EnAttente>` n'a pas de score,
ni `Option<f64>` ni `expect`.

## Prouver qu'une transition est interdite

Un test ordinaire ne peut pas vérifier qu'un code ne compile pas : si le code ne compile pas, le test non plus.
Rust offre pour cela les **doc-tests**. Un bloc de code placé dans un commentaire de documentation (`///`, entre
trois accents graves) est compilé, et exécuté, par `cargo test` comme un petit programme à part qui utilise votre
crate. Une annotation après les accents graves change ce qu'on attend de lui :

| Annotation | Le doc-test réussit si… |
|---|---|
| (aucune) | le code compile et s'exécute sans `panic!` |
| `no_run` | le code compile ; il n'est pas exécuté |
| `compile_fail` | le code **ne compile pas** |

Par exemple, au-dessus de `struct Porte` :

````rust
/// ```compile_fail,E0599
/// use ch12_typestate::e01_porte::Porte;
/// let porte = Porte::neuve().verrouiller().ouvrir();
/// ```
````

Ces doc-tests sont déjà écrits sur les types de chaque exercice. `cargo test -p ch12-typestate` les lance avec les
autres tests (ils apparaissent sous `Doc-tests ch12_typestate`), `cargo test -p ch12-typestate --doc` lance
seulement eux. Vous n'avez rien à y écrire, mais lisez-les : ils montrent ce que le type doit permettre et
interdire.

Les doc-tests `compile_fail` **passent dès le départ**, avant que vous ayez écrit une ligne : `ouvrir` n'est pas
définie sur `Porte<Verrouillee>`, que son corps soit un `todo!()` ou la bonne réponse. Ils vérifient la forme des
types (les signatures et les blocs `impl` donnés), pas votre code, et doivent rester verts jusqu'au bout. Les
doc-tests `no_run` passent aussi dès le départ, puisqu'ils sont compilés sans être exécutés : un `todo!()` ne
panique qu'à l'exécution.

Attention, sur Rust stable le code d'erreur écrit après `compile_fail` (`E0599`) n'est pas vérifié : un
`compile_fail` passe aussi pour une faute de frappe dans l'exemple. D'où le doc-test `no_run` à côté, qui montre le
parcours légal et doit, lui, compiler.

## Les pièges

1. **`&mut self` sur une transition.** La méthode ne peut pas changer le type de `self` : il faut prendre `self` et
   rendre la nouvelle valeur.
2. **Une transition ratée qui perd l'objet.** `fn payer(self) -> Result<Commande<Payee>, ErreurPaiement>` détruit la
   commande en cas d'échec, puisque `self` a été consommé. Si l'appelant doit pouvoir corriger et réessayer,
   rendez-la dans l'erreur (exercice 5). Si l'objet raté ne sert plus à rien, une erreur seule suffit (exercice 4).
3. **`..self` change de type ? Non.** La syntaxe `Article { texte, ..self }` ne marche que vers le même type. Pour
   changer d'état, on liste les champs.
4. **Trop d'états.** Le typestate convient quand les états sont connus à la compilation et que l'ordre des étapes
   est écrit dans le code. Un état lu depuis une base de données (« quel est le statut de la commande 42 ? ») reste
   un `enum` : on ne connaît pas son type à la compilation.

## Ce qui va coincer (et comment s'en sortir)

Voici les erreurs les plus fréquentes sur ces exercices, avec ce que le compilateur affiche. Chaque fonction à écrire
porte aussi un bloc « Aide Rust ».

1. **Réutiliser l'ancien état après une transition**
   ```text
   error[E0382]: use of moved value: `fermee`
      |     let fermee = Porte::neuve(); let _o = fermee.ouvrir(); let _v = fermee.verrouiller();
      |                                                   -------- value moved here      ^^^^^^ value used here after move
   ```
   C'est le typestate qui travaille : `ouvrir(self)` a consommé `fermee`. Gardez la valeur rendue et continuez avec
   elle (`let ouverte = fermee.ouvrir();`), ou chaînez les appels. ch00, sections 2 et 6.

2. **Rendre l'ancien type au lieu du nouveau**
   ```text
   error[E0308]: mismatched types
      |         Porte { ouvertures: self.ouvertures + 1, ..self }
      |                                                    ^^^^ expected `Porte<Ouverte>`, found `Porte<Fermee>`
   ```
   `..self`, `Self { … }` et `self` gardent l'état de départ (dans `impl Porte<Fermee>`, `Self` veut dire
   `Porte<Fermee>`). Pour changer d'état, construisez `Porte { … }` en listant les champs. `..self` reste permis
   quand le type ne change pas (`rediger`, `methode`, `entete`).

3. **Muter `self` alors qu'on le consomme**
   ```text
   error[E0596]: cannot borrow `self.texte` as mutable, as `self` is not declared as mutable
   error[E0594]: cannot assign to `self.methode`, as `self` is not declared as mutable
   ```
   Le compilateur propose `mut self` ; la règle du cours est de fabriquer une nouvelle valeur :
   `Article { texte: format!(...), ..self }`. ch00, section 7.

4. **Lire `self` après en avoir déplacé un morceau**
   ```text
   error[E0382]: borrow of partially moved value: `self`
      |     Article { titre: self.titre, texte: self.texte, etat: Publie { relecteur: self.relecteur().to_string() } }
      |                                         ---------- value partially moved here ^^^^ value borrowed here after partial move
   ```
   Une fois `self.texte` déplacé, `self` n'est plus entier. Déplacez directement le champ voulu
   (`self.etat.relecteur`), ou calculez ce qui lit `self` dans un `let` avant de déplacer. Même chose dans `payer` :
   le total et la référence d'abord, la nouvelle commande ensuite. ch00, sections 2 et 6.

5. **Le champ d'état oublié**
   ```text
   error[E0063]: missing field `_etat` in initializer of `Porte<_>`
   error[E0392]: type parameter `E` is never used
   ```
   Le premier : `PhantomData` est un champ comme un autre, il faut l'écrire (`_etat: PhantomData`). Le second
   apparaît si on retire ce champ de la `struct` : un paramètre de type doit servir quelque part.

Et le message que vous **voulez** voir : `` error[E0599]: no method named `ouvrir` found for struct `Porte<Verrouillee>` in the current scope ``.
C'est la preuve qu'une transition interdite ne compile pas.

## Les exercices

| # | Fichier | Ce qu'on pratique |
|---|---|---|
| 1 | `e01_porte.rs` | états sans donnée, `PhantomData`, transitions qui consomment `self` |
| 2 | `e02_article.rs` | états qui portent une donnée ; le bug de la version naïve devient impossible |
| 3 | `e03_requete.rs` | builder (un objet qu'on règle méthode après méthode avant de produire la valeur finale) : `envoyer` n'existe qu'une fois l'URL fournie |
| 4 | `e04_soumission.rs` | Arena : transition faillible, seul `Soumission<Scoree>` entre au classement |
| 5 | `e05_commande.rs` | sans version naïve ; transitions qui rendent l'objet en cas d'échec |

## Pour aller plus loin

- [The Typestate Pattern in Rust](https://cliffle.com/blog/rust-typestate/), de Cliff L. Biffle : la référence.
- [`PhantomData`](https://doc.rust-lang.org/std/marker/struct.PhantomData.html) dans la doc de la std, et le
  chapitre « Documentation tests » du [rustdoc book](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html)
  pour `compile_fail` et `no_run`.
- Arena, palier 2 : `Submission<Pending>` / `Submission<Scored>`.
