//! Exercice 3 : analyse d'un journal de logs.
//!
//! Une ligne ressemble à `2026-10-14T09:12:03 ERROR paiement: délai dépassé`. On la découpe en champs, chaque
//! découpage peut échouer, et `?` arrête au premier échec : le code reste une suite de lignes droites.
//! Le rapport compte les lignes invalides au lieu de planter dessus : un journal réel en contient toujours.
//! Piège : `max_by_key` rend le DERNIER maximum en cas d'égalité. Si vous voulez le premier, changez la clé.

use std::collections::BTreeMap;
use std::str::FromStr;

/// Niveau de gravité d'une ligne.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Niveau {
    Debug,
    Info,
    Warn,
    Error,
}

/// Une ligne de journal lue sans erreur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entree {
    /// Heure de l'horodatage, de 0 à 23.
    pub heure: u8,
    pub niveau: Niveau,
    pub module: String,
    pub message: String,
}

/// Pourquoi une ligne n'a pas pu être lue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurLog {
    /// Il manque l'horodatage, le niveau ou la suite.
    LigneIncomplete,
    /// L'horodatage n'a pas d'heure lisible entre 00 et 23 après le `T`.
    HorodatageInvalide(String),
    /// Le niveau n'est pas `DEBUG`, `INFO`, `WARN` ou `ERROR` (en majuscules).
    NiveauInconnu(String),
    /// Pas de `module: ` avant le message.
    ModuleManquant,
}

/// Ce qu'on retient d'un journal entier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rapport {
    /// Lignes non vides qu'on n'a pas pu lire.
    pub lignes_invalides: usize,
    /// Nombre de lignes `ERROR` par module, rangé par nom de module.
    pub erreurs_par_module: BTreeMap<String, usize>,
    /// L'heure qui compte le plus de lignes valides ; la plus tôt en cas d'égalité ; `None` si aucune.
    pub heure_la_plus_chargee: Option<u8>,
}

impl FromStr for Niveau {
    type Err = ErreurLog;

    /// `"ERROR"` → `Niveau::Error`, etc.
    ///
    /// **Aide Rust** (ch00, sections 1, 5)
    /// - Chaque bras rend un `Result` : `"DEBUG" => Niveau::Debug` donne
    ///   `E0308 … expected Result<Niveau, ErreurLog>, found Niveau`. Le dernier bras attrape le reste et le garde
    ///   dans l'erreur (`to_string()`).
    fn from_str(texte: &str) -> Result<Niveau, ErreurLog> {
        todo!()
    }
}

/// Lit une ligne : horodatage, espace, niveau, espace, module, `": "`, message. Seule l'heure de l'horodatage
/// est vérifiée (deux chiffres de 00 à 23 juste après le `T`).
///
/// **Aide Rust** (ch00, sections 11, 12)
/// - `split_once` rend une `Option` : `?` direct donne
///   `E0277 the ? operator can only be used on Results, not Options, in a function that returns Result`.
///   `.ok_or(ErreurLog::...)?` à chaque découpage.
/// - `heure.parse()?` donne `E0271 type mismatch resolving <u8 as FromStr>::Err == ErreurLog` : passez par `.ok()` et
///   finissez par un `ok_or`. Prenez les 2 caractères après le `T` avec `get(..2)` (une `Option`) plutôt que `[..2]`,
///   qui panique sur une chaîne trop courte.
/// - `niveau.parse()?` marche tel quel : votre `FromStr for Niveau` rend déjà une `ErreurLog`.
pub fn parser_ligne(ligne: &str) -> Result<Entree, ErreurLog> {
    todo!()
}

/// Analyse un journal entier. Les lignes vides (ou faites d'espaces) sont ignorées : ni valides, ni invalides.
///
/// **Aide Rust** (ch00, sections 2, 3, 7)
/// - Pour garder des `&Entree` sans les déplacer, `filter_map(|lu| lu.as_ref().ok())` ; `lu.ok()` sur un `&Result`
///   donne `E0277 a value of type Vec<&Entree> cannot be built from an iterator over elements of type Entree`.
/// - Compter dans une `BTreeMap` : `*compte.entry(cle).or_insert(0) += 1` dans un `fold`, avec l'étoile (sinon
///   `E0368 … cannot be applied to type &mut {integer}`).
/// - `max_by_key` rend le DERNIER maximum : pour avoir l'heure la plus tôt à égalité, la clé est
///   `(nombre, std::cmp::Reverse(heure))`.
pub fn analyser(journal: &str) -> Rapport {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    #[test]
    fn niveaux_connus_et_inconnu() {
        assert_eq!("ERROR".parse(), Ok(Niveau::Error));
        assert_eq!("DEBUG".parse(), Ok(Niveau::Debug));
        assert_eq!(
            "error".parse::<Niveau>(),
            Err(ErreurLog::NiveauInconnu("error".to_string()))
        );
    }

    #[test]
    fn ligne_valide() {
        assert_eq!(
            parser_ligne("2026-10-14T09:12:03 ERROR paiement: délai dépassé"),
            Ok(Entree {
                heure: 9,
                niveau: Niveau::Error,
                module: "paiement".to_string(),
                message: "délai dépassé".to_string(),
            })
        );
    }

    #[test]
    fn le_message_peut_contenir_deux_points() {
        let entree = parser_ligne("2026-10-14T23:00:00 INFO api: requête: GET /").unwrap();
        assert_eq!(entree.module, "api");
        assert_eq!(entree.message, "requête: GET /");
    }

    #[test]
    fn lignes_invalides() {
        assert_eq!(
            parser_ligne("n'importe quoi"),
            Err(ErreurLog::LigneIncomplete)
        );
        assert_eq!(
            parser_ligne("2026-10-14T09:12:03 ERROR"),
            Err(ErreurLog::LigneIncomplete)
        );
        assert_eq!(
            parser_ligne("2026-10-14T09:12:03 ERROR paiement délai"),
            Err(ErreurLog::ModuleManquant)
        );
        assert_eq!(
            parser_ligne("2026-10-14T25:12:03 INFO api: ok"),
            Err(ErreurLog::HorodatageInvalide(
                "2026-10-14T25:12:03".to_string()
            ))
        );
        assert_eq!(
            parser_ligne("hier INFO api: ok"),
            Err(ErreurLog::HorodatageInvalide("hier".to_string()))
        );
        assert_eq!(
            parser_ligne("2026-10-14T09:12:03 FATAL api: ok"),
            Err(ErreurLog::NiveauInconnu("FATAL".to_string()))
        );
    }

    const JOURNAL: &str = "\
2026-10-14T09:12:03 ERROR paiement: délai dépassé
2026-10-14T09:15:00 INFO api: GET /panier

2026-10-14T10:01:00 ERROR paiement: carte refusée
ligne corrompue
2026-10-14T10:02:00 ERROR stock: rupture
2026-10-14T11:30:00 WARN api: lent
2026-10-14T99:00:00 INFO api: heure impossible
";

    #[test]
    fn rapport_complet() {
        assert_eq!(
            analyser(JOURNAL),
            Rapport {
                lignes_invalides: 2,
                erreurs_par_module: BTreeMap::from([
                    ("paiement".to_string(), 2),
                    ("stock".to_string(), 1),
                ]),
                heure_la_plus_chargee: Some(9),
            }
        );
    }

    #[test]
    fn egalite_l_heure_la_plus_tot_gagne() {
        let journal = "\
2026-10-14T15:00:00 INFO a: x
2026-10-14T08:00:00 INFO a: x
2026-10-14T15:00:00 INFO a: x
2026-10-14T08:00:00 INFO a: x";
        assert_eq!(analyser(journal).heure_la_plus_chargee, Some(8));
    }

    #[test]
    fn journal_vide() {
        assert_eq!(
            analyser("\n   \n"),
            Rapport {
                lignes_invalides: 0,
                erreurs_par_module: BTreeMap::new(),
                heure_la_plus_chargee: None,
            }
        );
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e03_journal.rs"),
            &["from_str", "parser_ligne", "analyser"],
        );
    }
}
