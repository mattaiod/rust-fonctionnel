//! Exercice 5 : chercher : `find` et `position`.
//!
//! `find(p)` rend le premier élément qui vérifie `p`, `position(p)` son rang. Les deux rendent un `Option` :
//! pas trouvé = `None`, et non un `-1` qu'on oublie de tester. Sur `iter()`, `find` passe au prédicat un `&&i64`
//! et rend un `Option<&i64>` : `copied()` (sur l'`Option` comme sur l'itérateur) donne un `Option<i64>`.
//! `position` passe directement l'élément (`&i64`).

/// Version impérative. Ne pas modifier.
pub fn premier_negatif_imperatif(nombres: &[i64]) -> Option<i64> {
    let mut i = 0;
    while i < nombres.len() {
        if nombres[i] < 0 {
            return Some(nombres[i]);
        }
        i += 1;
    }
    None
}

/// Version impérative. Ne pas modifier.
pub fn premier_depassement_imperatif(mesures: &[i64], seuil: i64) -> Option<usize> {
    let mut rang = 0;
    while rang < mesures.len() {
        if mesures[rang] > seuil {
            return Some(rang);
        }
        rang += 1;
    }
    None
}

/// Le premier nombre strictement négatif.
///
/// **Aide Rust** (ch00, sections 4, 11)
/// - Sans `copied()`, `find` passe un `&&i64` : ``E0308: expected `&&i64`, found integer`` sur `n < 0`. Après
///   `.copied()`, le prédicat reçoit un `&i64` : `*n < 0`.
/// - Si le compilateur dit ``E0308: expected `Option<i64>`, found `Option<&i64>` `` : il manque `.copied()` (sur
///   l'itérateur avant `find`, ou sur l'`Option` après).
pub fn premier_negatif(nombres: &[i64]) -> Option<i64> {
    // Outils : `iter()`, `copied()`, `find(|n| …)`.
    todo!()
}

/// Le rang (à partir de 0) de la première mesure strictement supérieure à `seuil`.
///
/// **Aide Rust** (ch00, section 4)
/// - `position` passe l'élément avec UNE seule référence (`&i64`), pas deux comme `find`. Si le compilateur dit
///   ``E0308: expected `&i64`, found `i64` `` : comparez `*m > seuil`.
pub fn premier_depassement(mesures: &[i64], seuil: i64) -> Option<usize> {
    // Outils : `iter()`, `position(|m| …)`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        assert_eq!(premier_negatif(&[3, 0, -4, -9]), Some(-4));
        assert_eq!(premier_negatif(&[1, 2]), None);
        assert_eq!(premier_depassement(&[42, 91, 77, 95], 80), Some(1));
        assert_eq!(premier_depassement(&[42, 91], 100), None);
    }

    #[test]
    fn liste_vide() {
        assert_eq!(premier_negatif(&[]), None);
        assert_eq!(premier_depassement(&[], 0), None);
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        jeux::plusieurs(-20, 100).iter().for_each(|jeu| {
            assert_eq!(
                premier_negatif(jeu),
                premier_negatif_imperatif(jeu),
                "jeu : {jeu:?}"
            );
            [0, 50, 95].iter().for_each(|seuil| {
                assert_eq!(
                    premier_depassement(jeu, *seuil),
                    premier_depassement_imperatif(jeu, *seuil),
                    "jeu : {jeu:?}, seuil : {seuil}"
                )
            });
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e05_chercher.rs"),
            &["premier_negatif", "premier_depassement"],
        );
    }
}
