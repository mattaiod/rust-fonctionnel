//! Exercice 6 : séries temporelles, sans version impérative : à vous.
//!
//! Trois fonctions courtes, chacune avec son combinator :
//! 1. les **variations** d'un jour à l'autre regardent deux éléments voisins à la fois : `windows(2)` sur une
//!    slice donne toutes les fenêtres glissantes de taille 2 (`[a, b]`, puis `[b, c]`…) ;
//! 2. les **sommes cumulées** demandent de transporter un état d'un élément au suivant tout en produisant une
//!    valeur à chaque étape : c'est `scan`, un `fold` qui rend tous ses résultats intermédiaires ;
//! 3. la **compression** (« aaabcc » → « 3a1b2c ») regroupe les éléments égaux qui se suivent : `chunk_by` sur une
//!    slice. Une `&str` n'est pas une slice de caractères : passez d'abord par un `Vec<char>`. Et `collect` sait
//!    fabriquer une `String` à partir de morceaux de texte.

/// Différence entre chaque valeur et la précédente : `[10, 12, 9]` → `[2, -3]`.
///
/// **Aide Rust** (ch00, sections 1, 3)
/// - Si le compilateur dit `` error[E0599]: no method named `windows` found for struct `std::slice::Iter` `` :
///   `windows` est une méthode de **slice**. Appelez-la sur `valeurs`, sans `iter()` avant.
/// - Chaque fenêtre est une petite slice `&[i64]` de deux éléments : on l'indexe comme un tableau.
pub fn variations(valeurs: &[i64]) -> Vec<i64> {
    todo!()
}

/// Somme de tous les éléments jusqu'à chaque position incluse : `[1, 2, 3]` → `[1, 3, 6]`.
///
/// **Aide Rust** (ch00, sections 7, 11)
/// - Dans `scan`, l'état arrive en `&mut i64` : on le modifie à travers l'étoile, et on rend une copie de sa valeur,
///   pas la référence.
/// - Si le compilateur dit `` error[E0368]: binary assignment operation `+=` cannot be applied to type `&mut {integer}`
///   `` : il manque l'étoile, `*total += …`.
/// - Si le compilateur dit `` error[E0277]: a value of type `Vec<i64>` cannot be built from an iterator over elements
///   of type `&mut i64` `` : vous rendez `Some(total)`, rendez `Some(*total)`.
pub fn sommes_cumulees(valeurs: &[i64]) -> Vec<i64> {
    todo!()
}

/// Codage par plages : chaque suite de caractères identiques devient « nombre + caractère ».
///
/// **Aide Rust** (ch00, sections 5, 12)
/// - Si le compilateur dit `` error[E0599]: no method named `chunk_by` found for reference `&str` `` : une `&str` n'est
///   pas une slice. `let caracteres: Vec<char> = texte.chars().collect();` d'abord (un `let` sans `mut` est permis).
/// - `collect()` sait fabriquer une `String` à partir d'un itérateur de `String` (celles que rend `format!`).
pub fn compresser(texte: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    #[test]
    fn variations_cas_connus() {
        assert_eq!(variations(&[10, 12, 9, 9]), vec![2, -3, 0]);
        assert_eq!(variations(&[5]), Vec::<i64>::new());
        assert_eq!(variations(&[]), Vec::<i64>::new());
    }

    #[test]
    fn sommes_cumulees_cas_connus() {
        assert_eq!(sommes_cumulees(&[1, 2, 3, 4]), vec![1, 3, 6, 10]);
        assert_eq!(sommes_cumulees(&[5, -5, 2]), vec![5, 0, 2]);
        assert_eq!(sommes_cumulees(&[]), Vec::<i64>::new());
    }

    #[test]
    fn compresser_cas_connus() {
        assert_eq!(compresser("aaabcc"), "3a1b2c");
        assert_eq!(compresser("éété"), "2é1t1é");
        assert_eq!(compresser("x"), "1x");
        assert_eq!(compresser(""), "");
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e06_series.rs"),
            &["variations", "sommes_cumulees", "compresser"],
        );
    }
}
