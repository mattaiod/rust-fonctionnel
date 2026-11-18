//! # Exercice : appeler Rust depuis Python avec PyO3
//!
//! Ce fichier devient le module Python `capteurs`. Chaque `#[pyfunction]` est une fonction Python : PyO3 convertit
//! tout seul une `list[float]` Python en `Vec<f64>`, et un `Option<f64>` en `float` ou `None`.
//!
//! **À faire** : les quatre fonctions ci-dessous.
//! 1. `moyenne`, `min`, `max` : `None` si la liste est vide (c'est le calcul du chapitre 15).
//! 2. `stats` : un dictionnaire `{"moyenne": …, "min": …, "max": …}`, en réutilisant les trois autres.
//!
//! **Ce qu'il faut savoir**
//! - Moyenne : `(!valeurs.is_empty()).then(|| valeurs.iter().sum::<f64>() / valeurs.len() as f64)`.
//! - Min : `valeurs.iter().copied().reduce(f64::min)`, et pareil avec `f64::max`.
//! - Un `HashMap<&str, Option<f64>>` devient un `dict` Python :
//!   `HashMap::from([("moyenne", moyenne(valeurs.clone())), …])`.
//!
//! Tester : `./lancer.sh` (crée un environnement Python, compile avec maturin, lance les tests Python puis
//! `bench.py`, qui compare Python pur, votre Rust et NumPy).

use pyo3::prelude::*;

#[pymodule]
mod capteurs {
    use pyo3::prelude::*;
    use std::collections::HashMap;

    /// moyenne(valeurs: list[float]) -> float | None
    #[pyfunction]
    fn moyenne(valeurs: Vec<f64>) -> Option<f64> {
        // PyO3 convertit la `list` Python en `Vec<f64>` à l'appel (une copie), et `Option<f64>` revient en `float`
        // ou `None`. Le test `is_empty` évite un `NaN` sur une liste vide.
        (!valeurs.is_empty()).then(|| valeurs.iter().sum::<f64>() / valeurs.len() as f64)
    }

    /// min(valeurs: list[float]) -> float | None
    #[pyfunction]
    fn min(valeurs: Vec<f64>) -> Option<f64> {
        // Du Rust ordinaire : PyO3 fait les conversions autour, le corps ne voit aucun objet Python.
        valeurs.iter().copied().reduce(f64::min)
    }

    /// max(valeurs: list[float]) -> float | None
    #[pyfunction]
    fn max(valeurs: Vec<f64>) -> Option<f64> {
        // Même chose avec `f64::max`.
        valeurs.iter().copied().reduce(f64::max)
    }

    /// stats(valeurs: list[float]) -> dict, par exemple {"moyenne": 2.0, "min": 1.0, "max": 3.0}
    #[pyfunction]
    fn stats(valeurs: Vec<f64>) -> HashMap<&'static str, Option<f64>> {
        // `#[pyfunction]` laisse la fonction appelable depuis Rust : on réutilise `moyenne`, `min` et `max`.
        // `clone()` car chacune prend son `Vec` par valeur ; le dernier appel reçoit l'original.
        // Le `HashMap` devient un `dict` Python, les `None` compris.
        HashMap::from([
            ("moyenne", moyenne(valeurs.clone())),
            ("min", min(valeurs.clone())),
            ("max", max(valeurs)),
        ])
    }
}
