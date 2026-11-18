//! # Exercice 2 : déclarer et implémenter un trait
//!
//! Un capteur rend une série de mesures (`&[f64]`). On veut plusieurs statistiques sur cette série, qui
//! respectent toutes le même contrat : le trait `Statistique`, déjà déclaré ci-dessous.
//!
//! **À faire** : écrire `calculer` pour `Moyenne`, `Min` et `Max`. Une série vide donne `None`.
//!
//! **Ce qu'il faut savoir**
//! - Moyenne : `valeurs.iter().sum::<f64>() / valeurs.len() as f64`, à faire seulement si la série n'est pas vide.
//! - Min : `valeurs.iter().copied().reduce(f64::min)` (`reduce` rend déjà `None` sur une série vide).
//! - Max : pareil avec `f64::max`.
//!
//! Test : `cargo test -p ch15-traits e02`

/// Le contrat : un nom, et un calcul sur une série.
pub trait Statistique {
    fn nom(&self) -> &'static str;
    fn calculer(&self, valeurs: &[f64]) -> Option<f64>;
}

pub struct Moyenne;
pub struct Min;
pub struct Max;

impl Statistique for Moyenne {
    fn nom(&self) -> &'static str {
        "moyenne"
    }
    fn calculer(&self, valeurs: &[f64]) -> Option<f64> {
        // Le test `is_empty` évite `0.0 / 0.0`, qui donnerait `NaN` au lieu de `None`.
        // Variante en une ligne : `(!valeurs.is_empty()).then(|| ...)`.
        if valeurs.is_empty() {
            None
        } else {
            Some(valeurs.iter().sum::<f64>() / valeurs.len() as f64)
        }
    }
}

impl Statistique for Min {
    fn nom(&self) -> &'static str {
        "min"
    }
    fn calculer(&self, valeurs: &[f64]) -> Option<f64> {
        // `f64::min` passé tel quel à `reduce`, sans closure.
        valeurs.iter().copied().reduce(f64::min)
    }
}

impl Statistique for Max {
    fn nom(&self) -> &'static str {
        "max"
    }
    fn calculer(&self, valeurs: &[f64]) -> Option<f64> {
        // Même chose avec `f64::max`.
        valeurs.iter().copied().reduce(f64::max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SERIE: [f64; 4] = [12.0, 20.0, 9.5, 14.5];

    #[test]
    fn trois_statistiques() {
        assert_eq!(Moyenne.calculer(&SERIE), Some(14.0));
        assert_eq!(Min.calculer(&SERIE), Some(9.5));
        assert_eq!(Max.calculer(&SERIE), Some(20.0));
    }

    #[test]
    fn serie_vide() {
        assert_eq!(Moyenne.calculer(&[]), None);
        assert_eq!(Min.calculer(&[]), None);
        assert_eq!(Max.calculer(&[]), None);
    }
}
