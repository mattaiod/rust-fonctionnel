//! Exercice 1 : bulletin de notes.
//!
//! Trois fonctions pures qui s'enchaînent : regrouper les notes par élève, trier les élèves, nommer la mention.
//! Une moyenne est un `f64` : il n'implémente pas `Ord` (à cause de `NaN`), on le compare avec `total_cmp`.
//! Une mention n'existe que pour une moyenne entre 0 et 20 : le type de retour le dit.

use std::collections::BTreeMap;

/// Une note d'un élève dans une matière, sur 20.
#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    pub eleve: String,
    pub matiere: String,
    pub valeur: f64,
}

/// Mention du baccalauréat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mention {
    /// Moins de 10.
    Ajourne,
    /// De 10 (inclus) à 12 (exclu).
    Passable,
    /// De 12 à 14.
    AssezBien,
    /// De 14 à 16.
    Bien,
    /// 16 et plus.
    TresBien,
}

/// Moyenne de chaque élève, toutes matières confondues. Les élèves sont rangés par nom (`BTreeMap`).
///
/// **Aide Rust** (ch00, sections 2, 6, 7, 12)
/// - Un `fold` dont l'accumulateur est une `BTreeMap<String, (f64, u32)>` (somme, nombre), annotée dès le départ :
///   `BTreeMap::<String, (f64, u32)>::new()`.
/// - `entry(note.eleve)` donne `E0507 cannot move out of note.eleve which is behind a shared reference` : la clé doit
///   être possédée, `note.eleve.clone()`.
/// - `or_insert` rend des `&mut` : `somme += ...` donne
///   `E0368 binary assignment operation += cannot be applied to type &mut f64`. Écrivez `*somme += ...`.
/// - `somme / nombre` donne `E0277 cannot divide f64 by u32` : `f64::from(nombre)`.
pub fn moyennes_par_eleve(notes: &[Note]) -> BTreeMap<String, f64> {
    todo!()
}

/// Classement : moyenne décroissante, puis nom croissant en cas d'égalité.
///
/// **Aide Rust** (ch00, sections 1, 2)
/// - `f64` n'a pas de `cmp` : `b.1.cmp(&a.1)` donne le message trompeur `E0599 f64 is not an iterator`. Utilisez
///   `total_cmp`, qui prend une référence (`E0308 … expected &f64, found f64` sinon).
/// - Deux critères : `total_cmp(...)` puis `.then_with(|| nom_a.cmp(nom_b))`. Pour l'ordre décroissant, inversez `a`
///   et `b` dans la comparaison.
pub fn classement(notes: &[Note]) -> Vec<(String, f64)> {
    todo!()
}

/// Mention d'une moyenne ; `None` si la moyenne n'est pas entre 0 et 20 (ou si c'est `NaN`).
///
/// **Aide Rust** (ch00, section 1)
/// - Un `match` sur un `f64` doit avoir un bras `_` : `E0004 non-exhaustive patterns: _ not covered`. Des gardes
///   `m if m >= 16.0 =>` se lisent bien, du plus haut au plus bas.
/// - Commencez par écarter ce qui n'est pas entre 0 et 20 : `NaN` échoue à toutes les comparaisons,
///   `(0.0..=20.0).contains(&m)` l'écarte aussi.
pub fn mention(moyenne: f64) -> Option<Mention> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    fn note(eleve: &str, matiere: &str, valeur: f64) -> Note {
        Note {
            eleve: eleve.to_string(),
            matiere: matiere.to_string(),
            valeur,
        }
    }

    fn classe() -> Vec<Note> {
        vec![
            note("Chloé", "maths", 14.0),
            note("Bastien", "maths", 8.0),
            note("Alice", "maths", 16.0),
            note("Chloé", "rust", 18.0),
            note("Bastien", "rust", 12.0),
            note("Alice", "rust", 16.0),
            note("Dylan", "rust", 10.0),
        ]
    }

    #[test]
    fn moyennes_connues() {
        let moyennes = moyennes_par_eleve(&classe());
        assert_eq!(moyennes.len(), 4);
        assert_eq!(moyennes["Alice"], 16.0);
        assert_eq!(moyennes["Bastien"], 10.0);
        assert_eq!(moyennes["Chloé"], 16.0);
        assert_eq!(moyennes["Dylan"], 10.0);
    }

    #[test]
    fn aucune_note_aucune_moyenne() {
        assert!(moyennes_par_eleve(&[]).is_empty());
        assert!(classement(&[]).is_empty());
    }

    #[test]
    fn classement_par_moyenne_puis_par_nom() {
        let noms: Vec<String> = classement(&classe())
            .into_iter()
            .map(|(nom, _)| nom)
            .collect();
        assert_eq!(noms, vec!["Alice", "Chloé", "Bastien", "Dylan"]);
    }

    #[test]
    fn mentions_aux_frontieres() {
        assert_eq!(mention(9.99), Some(Mention::Ajourne));
        assert_eq!(mention(10.0), Some(Mention::Passable));
        assert_eq!(mention(12.0), Some(Mention::AssezBien));
        assert_eq!(mention(14.0), Some(Mention::Bien));
        assert_eq!(mention(16.0), Some(Mention::TresBien));
        assert_eq!(mention(20.0), Some(Mention::TresBien));
        assert_eq!(mention(0.0), Some(Mention::Ajourne));
    }

    #[test]
    fn pas_de_mention_hors_bornes() {
        assert_eq!(mention(-1.0), None);
        assert_eq!(mention(20.5), None);
        assert_eq!(mention(f64::NAN), None);
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e01_bulletin.rs"),
            &["moyennes_par_eleve", "classement", "mention"],
        );
    }
}
