//! # Exercice 2 : `join!`, attendre plusieurs choses en même temps
//!
//! Dans l'exercice 1, deux lectures l'une après l'autre prennent 200 ms. Avec `tokio::join!`, les lectures avancent
//! **en même temps** : trois lectures prennent environ 100 ms au total.
//!
//! **À faire** : `lire_trois` lit les capteurs 1, 2 et 3 en même temps et rend les trois valeurs.
//!
//! **Ce qu'il faut savoir**
//! - `let (a, b, c) = tokio::join!(lire_capteur(1), lire_capteur(2), lire_capteur(3));`
//! - Pas de `.await` sur chaque appel : `join!` attend lui-même les trois.
//!
//! Test : `cargo test -p ch17-async e02` (il vérifie aussi que l'ensemble prend moins de 250 ms).

use crate::lire_capteur;

/// Les capteurs 1, 2 et 3, lus en même temps.
pub async fn lire_trois() -> (f64, f64, f64) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[tokio::test]
    async fn en_meme_temps() {
        let debut = Instant::now();
        assert_eq!(lire_trois().await, (10.0, 20.0, 30.0));
        assert!(
            debut.elapsed() < Duration::from_millis(250),
            "les lectures se suivent au lieu d'avancer ensemble"
        );
    }
}
