# Chapitre 16 : concurrence et parallélisme

> Cours : diapos « Concurrence et parallélisme ». Tests : `cargo test -p ch16-concurrence` (un exercice : `… e02`).

Ces exercices utilisent des threads : `let mut` et `for` y sont autorisés, il n'y a pas de test de style.

| # | Fichier | Ce que vous écrivez |
|---|---|---|
| 1 | `e01_threads.rs` | Deux threads, `move`, `join` |
| 2 | `e02_arc_mutex.rs` | Un compteur partagé : `Arc<Mutex<usize>>` |
| 3 | `e03_channel.rs` | Un channel : un producteur, un consommateur |
| 4 | `e04_producteurs.rs` | Plusieurs producteurs : `tx.clone()` et `drop(tx)` |
| 5 | `e05_rayon.rs` | `par_iter()` avec rayon |
