//! # Exercice 6 : `Display` et `From`, les traits de la std
//!
//! En implémentant un trait de la bibliothèque standard, votre type se branche sur ce qu'elle sait déjà faire :
//! `Display` donne `{}` et `.to_string()`, `From` donne `.into()`.
//!
//! **À faire** :
//! 1. `Display` pour `Mesure` : `Mesure { valeur: 21.5, unite: "°C".into() }` s'affiche `21.5 °C`.
//! 2. `From<f64>` pour `Mesure` : un nombre seul devient une mesure en `"°C"`.
//!
//! **Ce qu'il faut savoir**
//! - Dans `fmt`, on écrit avec `write!(f, "{} {}", self.valeur, self.unite)`.
//! - Dans `from`, on construit et on rend un `Mesure { valeur, unite: String::from("°C") }`.
//!
//! Test : `cargo test -p ch15-traits e06`

use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Mesure {
    pub valeur: f64,
    pub unite: String,
}

impl fmt::Display for Mesure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.valeur, self.unite)
    }
}

impl From<f64> for Mesure {
    fn from(valeur: f64) -> Self {
        Mesure {
            valeur,
            unite: String::from("°C"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn affichage() {
        let m = Mesure {
            valeur: 21.5,
            unite: "%".into(),
        };
        assert_eq!(m.to_string(), "21.5 %");
        assert_eq!(format!("[{m}]"), "[21.5 %]");
    }

    #[test]
    fn conversion() {
        let m: Mesure = 19.0.into();
        assert_eq!(
            m,
            Mesure {
                valeur: 19.0,
                unite: "°C".into()
            }
        );
    }
}
