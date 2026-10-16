//! Exercice 6 : les résultats d'un examen, sans version naïve : à vous.
//!
//! Le réflexe naïf serait une note `f64` avec `-1` pour « absent » et `-2` pour « absent justifié » : la moyenne
//! compterait les `-1` comme des notes. Ici le type est posé ; écrivez les fonctions.
//!
//! Règles : une absence injustifiée compte 0 dans la moyenne ; une absence justifiée ne compte pas du tout (ni au
//! numérateur ni au dénominateur). La note est sur 20 ; qu'elle reste entre 0 et 20, c'est le travail d'un newtype
//! (chapitre 10), pas de cet exercice.

/// Le résultat d'un élève.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Resultat {
    Present { note: f64 },
    AbsentJustifie,
    AbsentInjustifie,
}

/// La note qui compte pour la moyenne, `None` si le résultat ne compte pas.
pub fn note_retenue(resultat: Resultat) -> Option<f64> {
    todo!()
}

/// La moyenne de la classe, `None` si aucun résultat ne compte.
///
/// **Aide Rust** (ch00, sections 3, 12)
/// - Si le compilateur dit `` error[E0631]: type mismatch in function arguments … expected function signature
///   `fn(&Resultat) -> _` `` : `note_retenue` prend un `Resultat` par valeur, `iter()` donne des `&Resultat` →
///   `.copied()` d'abord.
/// - Si le compilateur dit `` error[E0283]: type annotations needed … cannot infer type of the type parameter `S`
///   declared on the method `sum` `` : `sum::<f64>()`.
/// - Si le compilateur dit `` error[E0277]: cannot divide `f64` by `usize` `` : `len() as f64`.
pub fn moyenne(resultats: &[Resultat]) -> Option<f64> {
    todo!()
}

/// Le nombre d'absences : (justifiées, injustifiées).
///
/// **Aide Rust** (ch00, section 7)
/// - L'accumulateur du `fold` peut être un couple `(0, 0)`, qu'on déstructure dans les paramètres : `|(justifiees,
///   injustifiees), resultat|`. On rend un nouveau couple, aucun `mut` nécessaire.
pub fn absences(resultats: &[Resultat]) -> (usize, usize) {
    todo!()
}

/// La meilleure note des présents, `None` s'il n'y a aucun présent.
///
/// **Aide Rust** (ch00, sections 2, 12)
/// - Si le compilateur dit `` error[E0277]: the trait bound `f64: Ord` is not satisfied `` : `max()` exige un ordre
///   total, que `f64` n'a pas (à cause de `NaN`). `reduce(f64::max)` compare deux à deux.
/// - `match` sur un `&Resultat` lie `note: &f64` → `Some(*note)`.
pub fn meilleure_note(resultats: &[Resultat]) -> Option<f64> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use Resultat::*;
    use pf_outils::style;

    fn present(note: f64) -> Resultat {
        Present { note }
    }

    #[test]
    fn note_retenue_par_cas() {
        assert_eq!(note_retenue(present(14.0)), Some(14.0));
        assert_eq!(note_retenue(AbsentInjustifie), Some(0.0));
        assert_eq!(note_retenue(AbsentJustifie), None);
    }

    #[test]
    fn moyenne_selon_les_regles() {
        assert_eq!(moyenne(&[present(10.0), present(14.0)]), Some(12.0));
        assert_eq!(moyenne(&[present(12.0), AbsentJustifie]), Some(12.0));
        assert_eq!(moyenne(&[present(12.0), AbsentInjustifie]), Some(6.0));
        assert_eq!(moyenne(&[AbsentInjustifie]), Some(0.0));
    }

    #[test]
    fn rien_ne_compte() {
        assert_eq!(moyenne(&[]), None);
        assert_eq!(moyenne(&[AbsentJustifie, AbsentJustifie]), None);
        assert_eq!(meilleure_note(&[AbsentInjustifie]), None);
    }

    #[test]
    fn compter_et_chercher() {
        let classe = [
            present(8.5),
            AbsentJustifie,
            present(17.0),
            AbsentInjustifie,
            AbsentInjustifie,
            present(11.0),
        ];
        assert_eq!(absences(&classe), (1, 2));
        assert_eq!(meilleure_note(&classe), Some(17.0));
        assert_eq!(absences(&[]), (0, 0));
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e06_examen.rs"),
            &["note_retenue", "moyenne", "absences", "meilleure_note"],
        );
    }
}
