//! Chapitre 17 : asynchrone avec tokio. Tests : `cargo test -p ch17-async` (un exercice : `… e02`).

pub mod e01_async_await;
pub mod e02_join;
pub mod e03_spawn;
pub mod e04_spawn_blocking;

use std::time::Duration;

/// Fourni pour tous les exercices : lire un capteur prend 100 ms (on simule le réseau), et rend `id × 10`.
pub async fn lire_capteur(id: u32) -> f64 {
    tokio::time::sleep(Duration::from_millis(100)).await;
    f64::from(id) * 10.0
}
