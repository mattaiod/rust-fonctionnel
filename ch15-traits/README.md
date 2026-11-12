# Chapitre 15 : traits et génériques

> Cours : diapos « Traits et génériques ». Tests : `cargo test -p ch15-traits` (un exercice : `cargo test -p ch15-traits e03`).

Chaque fichier est un exercice : la consigne et tout ce qu'il faut savoir sont en haut du fichier, le `todo!()` à
remplacer en dessous, les tests tout en bas.

| # | Fichier | Ce que vous écrivez |
|---|---|---|
| 1 | `e01_generique.rs` | Une fonction générique `<T: PartialOrd + Copy>` |
| 2 | `e02_trait.rs` | Le trait `Statistique` pour `Moyenne`, `Min`, `Max` |
| 3 | `e03_methode_par_defaut.rs` | Une méthode par défaut dans le trait |
| 4 | `e04_trait_comme_bound.rs` | Des fonctions génériques sur votre trait |
| 5 | `e05_dyn.rs` | Un registre `Vec<Box<dyn Statistique>>` |
| 6 | `e06_display_from.rs` | `Display` et `From` pour un type `Mesure` |
