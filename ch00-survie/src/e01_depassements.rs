//! Exercice 1 : de combien chaque mesure dépasse le seuil.
//!
//! `mesures.iter()` produit des `&i64`, pas des `i64`. Et `filter` prête chaque élément à son prédicat : après
//! `iter()`, le prédicat reçoit donc un `&&i64`. Le réflexe qui règle les deux : `.copied()` juste après
//! `.iter()`, et toute la suite travaille sur des `i64`.

/// Version impérative. Ne pas modifier.
pub fn depassements_imperatif(mesures: &[i64], seuil: i64) -> Vec<i64> {
    let mut ecarts = Vec::new();
    for &mesure in mesures {
        if mesure > seuil {
            ecarts.push(mesure - seuil);
        }
    }
    ecarts
}

/// **Aide Rust** (ch00, sections 3 et 4)
/// - Sans `copied`, `collect` refuse de ranger des `&i64` dans un `Vec<i64>` : E0277 « a value of type
///   `Vec<i64>` cannot be built from an iterator over elements of type `&i64` ».
/// - Sans `copied`, `|m| m > seuil` dans `filter` compare un `&&i64` à un `i64` : E0308 « expected `&&i64`,
///   found `i64` ». Avec `copied` avant `filter`, `m` est un `&i64` : écrivez `*m > seuil`, ou `|&m| m > seuil`.
///
/// Pour chaque mesure strictement supérieure au seuil, de combien elle le dépasse, dans l'ordre.
pub fn depassements(mesures: &[i64], seuil: i64) -> Vec<i64> {
    // Outils : `iter()`, `copied()`, `filter(|&m| …)`, `map`, `collect()`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        assert_eq!(depassements(&[18, 25, 30, 20], 20), vec![5, 10]);
        assert_eq!(depassements(&[-3, 0, 4], -1), vec![1, 5]);
        assert_eq!(depassements(&[], 0), Vec::<i64>::new());
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        jeux::plusieurs(-100, 100).iter().for_each(|jeu| {
            [-50, 0, 37].iter().for_each(|&seuil| {
                assert_eq!(
                    depassements(jeu, seuil),
                    depassements_imperatif(jeu, seuil),
                    "jeu : {jeu:?}, seuil : {seuil}"
                )
            })
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e01_depassements.rs"), &["depassements"]);
    }
}
