//! # Exercice 5 : paralléliser avec rayon
//!
//! rayon répartit le travail d'un itérateur sur tous les cœurs. On remplace `iter()` par `par_iter()`, et c'est
//! tout : rayon garde l'ordre du résultat.
//!
//! **À faire** :
//! 1. `moyennes` : la moyenne de chaque série (`None` si elle est vide), en parallèle.
//! 2. `somme_des_carres` : la somme des `v * v`, en parallèle.
//!
//! **Ce qu'il faut savoir**
//! - L'import qui donne `par_iter()` : `use rayon::prelude::*;` (déjà écrit).
//! - `series.par_iter().map(|s| ...).collect()` et `valeurs.par_iter().map(|v| v * v).sum()`.
//! - Moyenne d'une série `s` : `(!s.is_empty()).then(|| s.iter().sum::<f64>() / s.len() as f64)`.
//!
//! Test : `cargo test -p ch16-concurrence e05`

use rayon::prelude::*;

/// La moyenne de chaque série, calculées en parallèle.
pub fn moyennes(series: &[Vec<f64>]) -> Vec<Option<f64>> {
    series
        .par_iter()
        .map(|s| (!s.is_empty()).then(|| s.iter().sum::<f64>() / s.len() as f64))
        .collect()
}

/// La somme des carrés, en parallèle.
pub fn somme_des_carres(valeurs: &[f64]) -> f64 {
    valeurs.par_iter().map(|v| v * v).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moyennes_dans_l_ordre() {
        let series = vec![vec![1.0, 3.0], vec![], vec![10.0]];
        assert_eq!(moyennes(&series), [Some(2.0), None, Some(10.0)]);
    }

    #[test]
    fn carres() {
        assert_eq!(somme_des_carres(&[1.0, 2.0, 3.0]), 14.0);
        let grande: Vec<f64> = (1..=100).map(f64::from).collect();
        assert_eq!(somme_des_carres(&grande), 338_350.0);
    }
}
