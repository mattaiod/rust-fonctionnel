//! # Exercice 1 : `async fn` et `.await`
//!
//! `lire_capteur(id)` (dans `lib.rs`) est une `async fn` : l'appeler ne fait **rien** tant qu'on n'écrit pas
//! `.await` derrière. `.await` attend le résultat sans bloquer le thread.
//!
//! **À faire** : `lire_deux` lit le capteur 1, puis le capteur 2, et rend la somme des deux lectures.
//!
//! **Ce qu'il faut savoir**
//! - `let a = lire_capteur(1).await;`
//! - Les tests d'une fonction async s'écrivent avec `#[tokio::test]` au lieu de `#[test]` : tokio fournit le runtime.
//!
//! Test : `cargo test -p ch17-async e01`

use crate::lire_capteur;

/// La lecture du capteur 1 plus celle du capteur 2, l'une après l'autre.
pub async fn lire_deux() -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn somme() {
        assert_eq!(lire_deux().await, 30.0);
    }
}
