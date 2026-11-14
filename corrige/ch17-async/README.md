# Chapitre 17 : asynchrone avec tokio

> Cours : diapos « Asynchrone ». Tests : `cargo test -p ch17-async` (un exercice : `… e02`).

`lib.rs` fournit `lire_capteur(id)`, une lecture qui prend 100 ms. Certains tests mesurent le temps : ils échouent
si vos lectures se suivent au lieu d'avancer ensemble.

| # | Fichier | Ce que vous écrivez |
|---|---|---|
| 1 | `e01_async_await.rs` | `async fn` et `.await` |
| 2 | `e02_join.rs` | `tokio::join!` |
| 3 | `e03_spawn.rs` | Une tâche par capteur avec `tokio::spawn` |
| 4 | `e04_spawn_blocking.rs` | Un calcul lourd dans `spawn_blocking` |
