//! # Exercice 4 : le trait comme bound
//!
//! Une fonction générique peut exiger que son type implémente **votre** trait : elle marche alors pour toutes les
//! statistiques, présentes et futures.
//!
//! **À faire** :
//! 1. `rapport` : rend `"moyenne : 14"` (le nom, puis la valeur telle quelle avec `{}`), ou `"moyenne : -"` si la
//!    série est vide.
//! 2. `toutes_definies` : `true` si les deux statistiques `a` et `b` donnent une valeur (`Some`) sur la série.
//!
//! **Ce qu'il faut savoir**
//! - `S: Statistique` : dans la fonction, `stat.nom()` et `stat.calculer(...)` sont disponibles.
//! - `impl Statistique` en paramètre, c'est la même chose écrite plus court.
//! - `Option` a `is_some()`.
//!
//! Test : `cargo test -p ch15-traits e04`

pub trait Statistique {
    fn nom(&self) -> &'static str;
    fn calculer(&self, valeurs: &[f64]) -> Option<f64>;
}

pub struct Moyenne;
pub struct Max;

impl Statistique for Moyenne {
    fn nom(&self) -> &'static str {
        "moyenne"
    }
    fn calculer(&self, valeurs: &[f64]) -> Option<f64> {
        (!valeurs.is_empty()).then(|| valeurs.iter().sum::<f64>() / valeurs.len() as f64)
    }
}

impl Statistique for Max {
    fn nom(&self) -> &'static str {
        "max"
    }
    fn calculer(&self, valeurs: &[f64]) -> Option<f64> {
        valeurs.iter().copied().reduce(f64::max)
    }
}

/// `"moyenne : 14"`, ou `"moyenne : -"` sur une série vide.
pub fn rapport<S: Statistique>(stat: &S, valeurs: &[f64]) -> String {
    // Le bound `S: Statistique` autorise `nom()` et `calculer()`. Le compilateur produit une version de la
    // fonction par type utilisé (monomorphisation) : l'appel est direct, sans `dyn`.
    match stat.calculer(valeurs) {
        Some(valeur) => format!("{} : {valeur}", stat.nom()),
        None => format!("{} : -", stat.nom()),
    }
}

/// `true` si `a` et `b` donnent toutes les deux une valeur.
pub fn toutes_definies(a: &impl Statistique, b: &impl Statistique, valeurs: &[f64]) -> bool {
    // Chaque `impl Statistique` est un paramètre générique distinct : `a` et `b` peuvent être de types différents.
    // `&&` court-circuite : `b` n'est pas calculé si `a` rend `None`.
    a.calculer(valeurs).is_some() && b.calculer(valeurs).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rapport_de_deux_statistiques() {
        assert_eq!(rapport(&Moyenne, &[12.0, 16.0]), "moyenne : 14");
        assert_eq!(rapport(&Max, &[12.0, 16.0]), "max : 16");
        assert_eq!(rapport(&Max, &[]), "max : -");
    }

    #[test]
    fn definies() {
        assert!(toutes_definies(&Moyenne, &Max, &[1.0]));
        assert!(!toutes_definies(&Moyenne, &Max, &[]));
    }
}
