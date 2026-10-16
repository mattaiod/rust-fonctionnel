//! Exercice 2 : une division qui ne panique plus.
//!
//! `a / b` panique si `b == 0`, et aussi pour `i64::MIN / -1` (le résultat dépasse `i64::MAX`). Une erreur
//! prévisible, causée par les données, n'a rien à faire dans un `panic!` : elle va dans le type de retour, avec un
//! type d'erreur à vous (un `enum`), pas une `String` : l'appelant doit pouvoir faire un `match` dessus.

/// Version naïve : panique sur une entrée invalide. Ne pas modifier.
pub fn diviser_naif(a: i64, b: i64) -> i64 {
    if b == 0 {
        panic!("division par zéro");
    }
    a / b
}

/// Version naïve : `part` en pourcentage de `total`, arrondi vers zéro. Ne pas modifier.
pub fn pourcentage_naif(part: i64, total: i64) -> i64 {
    diviser_naif(part * 100, total)
}

/// Les deux façons dont une division entière peut échouer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErreurDivision {
    ParZero,
    Depassement,
}

/// `a / b` (arrondi vers zéro, comme l'opérateur `/`), ou l'erreur.
///
/// **Aide Rust** (ch00, section 11)
/// - `checked_div` rend `None` dans les **deux** cas d'échec (zéro et dépassement) : traitez `b == 0` avant (un `if …
///   else` est permis), sinon tout devient `Depassement`.
pub fn diviser(a: i64, b: i64) -> Result<i64, ErreurDivision> {
    // But : distinguer les deux échecs ; la bibliothèque standard a une division « vérifiée ».
    todo!()
}

/// `part * 100 / total`, sans jamais paniquer (la multiplication aussi peut déborder).
///
/// **Aide Rust** (ch00, sections 11, 12)
/// - Si le compilateur dit `` error[E0277]: the `?` operator can only be used on `Result`s, not `Option`s, in a
///   function that returns `Result` `` : `checked_mul` rend une `Option`. Transformez-la d'abord en `Result` avec
///   `ok_or(…)`.
/// - Si le compilateur dit `` found `Result<Result<i64, ErreurDivision>, …>` `` : `map` avec une étape qui peut échouer
///   emboîte deux `Result`. C'est `and_then`.
pub fn pourcentage(part: i64, total: i64) -> Result<i64, ErreurDivision> {
    // But : deux étapes faillibles enchaînées, la seconde étant `diviser`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        assert_eq!(diviser(7, 2), Ok(3));
        assert_eq!(diviser(-7, 2), Ok(-3));
        assert_eq!(diviser(0, 5), Ok(0));
        assert_eq!(pourcentage(1, 4), Ok(25));
        assert_eq!(pourcentage(3, 3), Ok(100));
    }

    #[test]
    fn zero_est_une_erreur_pas_un_crash() {
        assert_eq!(diviser(7, 0), Err(ErreurDivision::ParZero));
        assert_eq!(pourcentage(1, 0), Err(ErreurDivision::ParZero));
    }

    #[test]
    fn depassements() {
        assert_eq!(diviser(i64::MIN, -1), Err(ErreurDivision::Depassement));
        assert_eq!(pourcentage(i64::MAX, 1), Err(ErreurDivision::Depassement));
    }

    #[test]
    #[should_panic(expected = "division par zéro")]
    fn le_naif_panique() {
        diviser_naif(1, 0);
    }

    #[test]
    fn meme_resultat_que_le_naif_quand_il_ne_panique_pas() {
        let a = jeux::entiers(1, 200, -1000, 1000);
        let b = jeux::entiers(2, 200, -20, 20);
        a.iter()
            .zip(&b)
            .filter(|(_, b)| **b != 0)
            .for_each(|(&a, &b)| {
                assert_eq!(diviser(a, b), Ok(diviser_naif(a, b)), "{a} / {b}");
                assert_eq!(pourcentage(a, b), Ok(pourcentage_naif(a, b)), "{a} % {b}");
            });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e02_division.rs"), &["diviser", "pourcentage"]);
    }
}
