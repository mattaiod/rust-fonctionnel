//! # Exercice 4 : `spawn_blocking`, sortir un calcul lourd du runtime
//!
//! Un calcul long (ici `calcul_lourd`) n'a pas de `.await` : il bloque le thread où il tourne. Dans un serveur
//! async, il bloquerait toutes les tâches qui partagent ce thread. On l'envoie donc sur un thread à part avec
//! `tokio::task::spawn_blocking`.
//!
//! **À faire** : `analyser` appelle `calcul_lourd` dans `spawn_blocking` et rend son résultat.
//!
//! **Ce qu'il faut savoir**
//! - `tokio::task::spawn_blocking(move || calcul_lourd(&valeurs)).await.unwrap()`
//! - `move` : la closure doit posséder `valeurs`, car elle part sur un autre thread (c'est pour ça que le
//!   paramètre est un `Vec<f64>` et pas un `&[f64]`).
//!
//! Test : `cargo test -p ch17-async e04`

/// Fourni : un calcul sans `.await`, qui occupe le processeur (la somme des carrés, beaucoup de fois).
pub fn calcul_lourd(valeurs: &[f64]) -> f64 {
    (0..200).fold(0.0, |_, _| valeurs.iter().map(|v| v * v).sum::<f64>())
}

/// `calcul_lourd`, lancé hors du runtime async.
pub async fn analyser(valeurs: Vec<f64>) -> f64 {
    tokio::task::spawn_blocking(move || calcul_lourd(&valeurs))
        .await
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn meme_resultat() {
        let valeurs: Vec<f64> = (1..=1000).map(f64::from).collect();
        assert_eq!(analyser(valeurs.clone()).await, calcul_lourd(&valeurs));
    }
}
