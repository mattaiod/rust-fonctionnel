//! # Exercice 3 : `tokio::spawn`, une tâche par capteur
//!
//! `join!` convient quand on connaît le nombre d'appels à l'avance. Pour une liste de longueur quelconque, on lance
//! une **tâche** par élément avec `tokio::spawn` : des milliers de tâches tiennent sur quelques threads.
//!
//! **À faire** : `lire_tous` lit tous les capteurs de la liste en même temps et rend les valeurs **dans l'ordre de
//! la liste**.
//!
//! **Ce qu'il faut savoir**
//! - Lancer : `let taches: Vec<_> = ids.into_iter().map(|id| tokio::spawn(lire_capteur(id))).collect();`
//! - Les tâches tournent déjà toutes. Il reste à attendre chacune, dans l'ordre :
//!   `let mut valeurs = Vec::new();` puis `for tache in taches { valeurs.push(tache.await.unwrap()); }`
//! - `tache.await` rend un `Result` (la tâche a pu paniquer), d'où le `unwrap()`.
//!
//! Test : `cargo test -p ch17-async e03` (10 capteurs en moins de 250 ms).

use crate::lire_capteur;

/// Tous les capteurs, lus en même temps, valeurs dans l'ordre des `ids`.
pub async fn lire_tous(ids: Vec<u32>) -> Vec<f64> {
    let taches: Vec<_> = ids
        .into_iter()
        .map(|id| tokio::spawn(lire_capteur(id)))
        .collect();
    let mut valeurs = Vec::new();
    for tache in taches {
        valeurs.push(tache.await.unwrap());
    }
    valeurs
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[tokio::test]
    async fn dix_capteurs_en_meme_temps() {
        let debut = Instant::now();
        let valeurs = lire_tous((1..=10).collect()).await;
        assert_eq!(
            valeurs,
            (1..=10).map(|i| f64::from(i) * 10.0).collect::<Vec<_>>()
        );
        assert!(
            debut.elapsed() < Duration::from_millis(250),
            "les lectures se suivent au lieu d'avancer ensemble"
        );
    }
}
