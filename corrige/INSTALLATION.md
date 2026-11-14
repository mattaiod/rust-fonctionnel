# Installer Rust et travailler les exercices

À faire une fois, avant le chapitre 0. Comptez 15 minutes (un peu plus sous Windows).

## 1. Installer Rust avec `rustup`

`rustup` installe et met à jour tout l'outillage : le compilateur `rustc`, le gestionnaire de projets `cargo`, le
formateur `rustfmt` et le linter `clippy`.

**macOS, Linux, WSL** : dans un terminal,

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Acceptez l'installation par défaut (choix `1`).

**Windows** : téléchargez et lancez `rustup-init.exe` depuis <https://rust-lang.org/tools/install>.
Rust a besoin des outils de compilation C++ de Microsoft (MSVC) : si Visual Studio n'est pas installé, `rustup-init`
propose de les installer lui-même. Acceptez, c'est le plus simple.

**Ensuite, fermez et rouvrez le terminal** (et VS Code s'il était ouvert) : c'est ce qui rend les commandes `cargo` et
`rustc` visibles. Vérifiez :

```bash
rustc --version     # rustc 1.94.0 ou plus récent
cargo --version
```

Si la commande est introuvable, c'est presque toujours le terminal qui n'a pas été rouvert. Les outils sont installés
dans `~/.cargo/bin` (Windows : `%USERPROFILE%\.cargo\bin`).

Ces exercices demandent Rust 1.94 au minimum. Si vous aviez déjà Rust, mettez-le à jour :

```bash
rustup update
```

## 2. Récupérer les exercices

Avec git (conseillé : vous pourrez récupérer les mises à jour) :

```bash
git clone https://github.com/mattaiod/rust-fonctionnel.git
cd rust-fonctionnel
cargo test -p ch00-survie     # premier lancement : compile tout, puis montre les tests du chapitre 0
```

Sans git : sur <https://github.com/mattaiod/rust-fonctionnel>, bouton **Code**, puis **Download ZIP**, et décompressez.

Si une mise à jour des énoncés est annoncée, enregistrez d'abord votre travail (`git add .` puis
`git commit -m "mon travail"`), puis récupérez-la avec `git pull`.

## 3. Configurer VS Code

1. Installez l'extension **rust-analyzer** (identifiant `rust-lang.rust-analyzer`, éditeur rust-lang). C'est elle qui
   souligne les erreurs pendant que vous tapez, affiche les types, complète le code et ajoute des boutons pour lancer
   les tests.
2. Si vous avez une ancienne extension nommée **Rust** (identifiant `rust-lang.rust`), désinstallez-la : elle n'est
   plus maintenue et entre en conflit avec rust-analyzer.
3. Facultatif mais utile :
   a. **Error Lens** (`usernamehw.errorlens`) : affiche le message d'erreur au bout de la ligne fautive ;
   b. **Even Better TOML** (`tamasfe.even-better-toml`) : coloration des fichiers `Cargo.toml` ;
   c. **CodeLLDB** (`vadimcn.vscode-lldb`) : le débogueur, pour mettre des points d'arrêt dans un test.
4. **Ouvrez le dossier des exercices lui-même** (`Fichier > Ouvrir le dossier…`, celui qui contient `Cargo.toml` et
   les dossiers `ch00-…`), pas un dossier parent ni un fichier seul : rust-analyzer a besoin de voir le `Cargo.toml`
   racine. VS Code propose alors d'installer les extensions recommandées du dossier : acceptez.

Le dossier contient déjà des réglages (`.vscode/settings.json`) : formatage automatique à l'enregistrement, et
vérification par `clippy` à chaque enregistrement, qui donne en plus des conseils de style.

La première ouverture prend une minute : rust-analyzer indexe le projet (barre d'état en bas). Tant qu'il n'a pas
fini, ni erreurs ni complétion.

## 4. Les commandes `cargo`

Toutes se lancent depuis le dossier des exercices, dans le terminal intégré de VS Code (`` Ctrl+` ``).

| Commande | Ce qu'elle fait |
|---|---|
| `cargo check` | Vérifie que tout compile, sans produire d'exécutable. La plus rapide : c'est le réflexe. |
| `cargo build` | Compile pour de bon (le résultat va dans `target/`). |
| `cargo run` | Compile puis lance le programme (`src/main.rs`). Voir la section 6. |
| `cargo test -p ch03-iterateurs` | Lance tous les tests du chapitre 3. `-p` choisit le paquet, c'est-à-dire le chapitre. |
| `cargo test -p ch03-iterateurs e02` | Seulement les tests dont le nom contient `e02` (l'exercice 2). |
| `cargo test -p ch03-iterateurs e02 -- --show-output` | Pareil, et affiche aussi les `println!` des tests qui passent. |
| `cargo test` | Tous les chapitres. Au début, presque tout échoue : c'est normal. |
| `cargo clippy -p ch03-iterateurs` | Les conseils de `clippy`, souvent très « fonctionnels ». |
| `cargo fmt` | Remet le code en forme (indentation, espaces). |

Ce qui est avant `--` s'adresse à `cargo`, ce qui est après s'adresse au programme de test.

### Lire le résultat des tests

```text
running 3 tests
test e02_garder_pairs::tests::style_fonctionnel ... ok
test e02_garder_pairs::tests::cas_connus ... FAILED
test e02_garder_pairs::tests::meme_resultat_que_l_imperatif ... FAILED

failures:

---- e02_garder_pairs::tests::cas_connus stdout ----

thread 'e02_garder_pairs::tests::cas_connus' (7370441) panicked at ch03-iterateurs/src/e02_garder_pairs.rs:29:5:
not yet implemented

test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.00s
```

1. `not yet implemented` : le `todo!()` est encore là, la fonction n'est pas écrite. `e02_garder_pairs.rs:29:5` est
   la ligne du `todo!()`.
2. ``assertion `left == right` failed`` suivi de `left:` et `right:` : votre fonction rend `left`, le test attendait
   `right`.
3. `style_fonctionnel` échoue avec ``Pas encore fonctionnel : `garder_pairs` contient `for` `` : il reste un `let mut`,
   un `for`, un `while`, un `loop` ou un `for_each` dans votre fonction ou dans une fonction d'aide. Tant que le `todo!()` est là, ce test passe :
   il ne prouve rien avant que la fonction soit écrite.
4. `12 filtered out` : les tests des autres exercices, écartés par le filtre `e02`.
5. Une erreur de compilation (`error[E0308]`…) : aucun test du chapitre ne tourne tant qu'elle reste. Lisez-la avec
   la section 1 du chapitre 0.

### Lancer un test depuis l'éditeur

Au-dessus de chaque `#[test]` et de chaque `mod tests`, rust-analyzer affiche **▶ Run Test** (et **Debug** si
CodeLLDB est installé). Un clic lance ce seul test dans le terminal. C'est souvent le plus pratique.

## 5. Relancer les tests à chaque enregistrement : `bacon`

Plutôt que de retaper `cargo test` après chaque modification, laissez `bacon` tourner dans un terminal : il relance la
commande à chaque enregistrement et n'affiche que l'essentiel, première erreur en haut.

```bash
cargo install --locked bacon       # une seule fois (quelques minutes de compilation)
bacon test -- -p ch03-iterateurs   # les tests du chapitre 3, relancés à chaque enregistrement
```

Dans `bacon` : `t` pour les tests, `c` pour `clippy`, `f` pour ne suivre que le test en échec, `Échap` pour revenir,
`h` pour l'aide, `q` pour quitter. Changez de chapitre en relançant la commande avec un autre `-p`.

Vous trouverez en ligne `cargo watch` : il n'est plus maintenu, et son propre auteur conseille `bacon`.

## 6. Un programme qui s'exécute : `cargo new` et `cargo run`

Les exercices sont des **bibliothèques** testées, sans `main` : on ne les lance pas avec `cargo run`, on les vérifie
avec `cargo test`. Pour essayer une idée ou afficher un résultat, créez un petit projet à côté, **en dehors** du dossier
des exercices :

```bash
cargo new bac-a-sable
cd bac-a-sable
cargo run
```

`cargo new` crée `Cargo.toml` (la description du projet) et `src/main.rs`, qui affiche `Hello, world!`. Modifiez
`main`, relancez `cargo run` : `cargo` recompile ce qui a changé puis exécute. Pour afficher une valeur :
`println!("{}", x)`, ou `println!("{:?}", v)` pour un `Vec` ou une `struct` qui dérive `Debug`.

`cargo run --release` compile avec les optimisations : plus lent à compiler, bien plus rapide à l'exécution.

## 7. Où travailler

1. Lisez le `README.md` du chapitre : c'est le cours.
2. Ouvrez `src/e01_….rs`, lisez l'en-tête, la version de départ, puis le bloc **Aide Rust** au-dessus de la fonction à
   écrire.
3. Remplacez le `todo!()`, enregistrez, regardez `bacon` (ou lancez le test).
4. Tout est vert : exercice suivant.

Vous ne modifiez que les corps des fonctions qui contiennent `todo!()` (et, au chapitre 14, les tests marqués
`#[ignore]`). Le reste du fichier, tests compris, ne se touche pas.
