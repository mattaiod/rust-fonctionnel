//! Exercice 6 : une plage de dates qu'on parcourt comme une liste, sans version naïve : à vous.
//!
//! Une `Date` simplifiée (calendrier grégorien, sans fuseau), qu'on ne peut construire que valide. `Ord` dérivé
//! compare les champs dans l'ordre de déclaration : année, puis mois, puis jour, donc l'ordre chronologique.
//! `debut.jusqu_a(fin)` rend une `PlageDates` qui implémente `Iterator<Item = Date>` : tout le chapitre 3
//! devient disponible sur des dates. `fins_de_mois` est générique : elle prend tout ce qui produit des `Date`
//! (une plage, un `Vec`, un `filter` sur une plage…).
//!
//! Bissextile : divisible par 4, sauf les siècles, sauf les siècles divisibles par 400 (2000 oui, 1900 non).

use std::fmt;

/// `true` si `annee` est bissextile.
pub fn est_bissextile(annee: i32) -> bool {
    todo!()
}

/// Nombre de jours du mois `mois` (1 à 12) de `annee` ; 0 si le mois n'existe pas.
///
/// **Aide Rust** (ch00, section 1)
/// - Le `match` sur un `u32` doit couvrir tous les nombres :
///   `E0004 non-exhaustive patterns: 0_u32 and 13_u32..=u32::MAX not covered`. Un dernier bras `_ => 0`. Pour
///   février, une garde `2 if ... =>`.
pub fn jours_dans_mois(annee: i32, mois: u32) -> u32 {
    todo!()
}

/// Une date valide.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    annee: i32,
    mois: u32,
    jour: u32,
}

impl Date {
    /// La date, si elle existe (`Date::valide(2026, 2, 29)` rend `None`).
    ///
    /// **Aide Rust** (ch00, section 11)
    /// - `if ... { Some(...) }` sans `else` donne
    ///   `E0317 if may be missing an else clause … expected Option<Date>, found ()`. Écrivez le `else`, ou utilisez
    ///   `bool::then_some`.
    /// - Le jour 0 n'existe pas : vérifiez les deux bornes.
    pub fn valide(annee: i32, mois: u32, jour: u32) -> Option<Date> {
        todo!()
    }

    /// Le jour suivant.
    ///
    /// **Aide Rust** (ch00, sections 2, 7)
    /// - `self` est un `&Date` : `..self` donne `E0308 … expected Date, found &Date`, corrigé par `..*self` (`Date`
    ///   est `Copy`).
    /// - `self.jour += 1` donne `E0594 cannot assign to self.jour, which is behind a & reference` : rendez une
    ///   nouvelle `Date`. Trois cas : même mois, mois suivant, année suivante.
    pub fn lendemain(&self) -> Date {
        todo!()
    }

    /// Toutes les dates de `self` à `fin`, bornes comprises (aucune si `fin` est avant `self`).
    pub fn jusqu_a(self, fin: Date) -> PlageDates {
        todo!()
    }

    pub fn annee(&self) -> i32 {
        self.annee
    }

    pub fn mois(&self) -> u32 {
        self.mois
    }

    pub fn jour(&self) -> u32 {
        self.jour
    }
}

impl fmt::Display for Date {
    /// Format ISO : `2026-03-05`.
    ///
    /// **Aide Rust** (ch00, section 1)
    /// - `{}` écrit `2026-3-5` : `{:02}` et `{:04}` complètent avec des zéros.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// Une plage de dates, parcourue jour par jour.
#[derive(Debug, Clone)]
pub struct PlageDates {
    courante: Date,
    fin: Date,
}

impl Iterator for PlageDates {
    type Item = Date;

    /// **Aide Rust** (ch00, sections 7, 11)
    /// - `if self.courante > self.fin { None }` sans `else` donne
    ///   `E0308 … if expressions without else arms expect their inner expression to be ()`.
    /// - Piège qui compile : avancer puis rendre `self.courante` saute la première date. Gardez la date à rendre
    ///   (`Date` est `Copy`), avancez, puis rendez-la.
    fn next(&mut self) -> Option<Date> {
        todo!()
    }
}

/// Les dates qui sont le dernier jour de leur mois, dans l'ordre reçu.
///
/// **Aide Rust** (ch00, section 3)
/// - `dates` est un `IntoIterator`, pas encore un itérateur : `dates.filter(...)` donne
///   `E0599 no method named filter found … impl IntoIterator<Item = Date> is not an iterator`. Appelez `into_iter()`
///   d'abord.
pub fn fins_de_mois(dates: impl IntoIterator<Item = Date>) -> Vec<Date> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    fn date(a: i32, m: u32, j: u32) -> Date {
        Date::valide(a, m, j).expect("date de test valide")
    }

    #[test]
    fn bissextiles() {
        assert!(est_bissextile(2024));
        assert!(est_bissextile(2000));
        assert!(!est_bissextile(1900));
        assert!(!est_bissextile(2026));
        assert_eq!(jours_dans_mois(2024, 2), 29);
        assert_eq!(jours_dans_mois(2026, 2), 28);
        assert_eq!(jours_dans_mois(2026, 13), 0);
    }

    #[test]
    fn construction() {
        assert_eq!(Date::valide(2026, 2, 29), None);
        assert_eq!(Date::valide(2026, 4, 31), None);
        assert_eq!(Date::valide(2026, 0, 1), None);
        assert_eq!(Date::valide(2026, 1, 0), None);
        assert!(Date::valide(2024, 2, 29).is_some());
    }

    #[test]
    fn lendemains() {
        assert_eq!(date(2026, 10, 16).lendemain(), date(2026, 10, 17));
        assert_eq!(date(2026, 4, 30).lendemain(), date(2026, 5, 1));
        assert_eq!(date(2024, 2, 28).lendemain(), date(2024, 2, 29));
        assert_eq!(date(2026, 12, 31).lendemain(), date(2027, 1, 1));
    }

    #[test]
    fn affichage() {
        assert_eq!(date(2026, 3, 5).to_string(), "2026-03-05");
    }

    #[test]
    fn plages() {
        assert_eq!(date(2024, 1, 1).jusqu_a(date(2024, 12, 31)).count(), 366);
        assert_eq!(date(2026, 1, 1).jusqu_a(date(2026, 12, 31)).count(), 365);
        assert_eq!(date(2026, 5, 2).jusqu_a(date(2026, 5, 1)).count(), 0);
        let fin_annee: Vec<String> = date(2026, 12, 30)
            .jusqu_a(date(2027, 1, 2))
            .map(|d| d.to_string())
            .collect();
        assert_eq!(
            fin_annee,
            vec!["2026-12-30", "2026-12-31", "2027-01-01", "2027-01-02"]
        );
    }

    #[test]
    fn fins_de_mois_generique() {
        let annee = fins_de_mois(date(2024, 1, 1).jusqu_a(date(2024, 12, 31)));
        assert_eq!(annee.len(), 12);
        assert_eq!(annee[1], date(2024, 2, 29));
        assert_eq!(
            fins_de_mois(vec![
                date(2026, 6, 30),
                date(2026, 6, 29),
                date(2026, 12, 31)
            ]),
            vec![date(2026, 6, 30), date(2026, 12, 31)]
        );
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e06_dates.rs"),
            &[
                "est_bissextile",
                "jours_dans_mois",
                "valide",
                "lendemain",
                "jusqu_a",
                "fmt",
                "next",
                "fins_de_mois",
            ],
        );
    }
}
