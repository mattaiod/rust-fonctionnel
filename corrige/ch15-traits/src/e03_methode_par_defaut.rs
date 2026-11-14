//! # Exercice 3 : une méthode par défaut
//!
//! Une méthode par défaut est écrite **une fois, dans le trait** : tous les types qui implémentent le trait la
//! reçoivent sans rien écrire.
//!
//! **À faire** : écrire la méthode par défaut `resume`, qui rend :
//! - `"moyenne = 14.00"` (deux décimales) quand `calculer` rend `Some(14.0)` ;
//! - `"moyenne = aucune mesure"` quand `calculer` rend `None`.
//!
//! **Ce qu'il faut savoir**
//! - Dans le trait, `self.nom()` et `self.calculer(valeurs)` sont utilisables : ce sont des méthodes du contrat.
//! - Deux décimales : `format!("{:.2}", valeur)`.
//! - Un `match` sur l'`Option` suffit.
//!
//! Test : `cargo test -p ch15-traits e03`

pub trait Statistique {
    fn nom(&self) -> &'static str;
    fn calculer(&self, valeurs: &[f64]) -> Option<f64>;

    /// Méthode par défaut : une ligne lisible, par exemple `"moyenne = 14.00"`.
    fn resume(&self, valeurs: &[f64]) -> String {
        match self.calculer(valeurs) {
            Some(valeur) => format!("{} = {valeur:.2}", self.nom()),
            None => format!("{} = aucune mesure", self.nom()),
        }
    }
}

/// Fourni : elle n'écrit pas `resume`, elle la reçoit du trait.
pub struct Moyenne;

impl Statistique for Moyenne {
    fn nom(&self) -> &'static str {
        "moyenne"
    }
    fn calculer(&self, valeurs: &[f64]) -> Option<f64> {
        (!valeurs.is_empty()).then(|| valeurs.iter().sum::<f64>() / valeurs.len() as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avec_des_mesures() {
        assert_eq!(Moyenne.resume(&[12.0, 16.0]), "moyenne = 14.00");
    }

    #[test]
    fn sans_mesure() {
        assert_eq!(Moyenne.resume(&[]), "moyenne = aucune mesure");
    }
}
