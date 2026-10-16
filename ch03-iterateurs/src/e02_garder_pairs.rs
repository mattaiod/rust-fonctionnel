//! Exercice 2 : garder les nombres pairs.
//!
//! Garder certains éléments, c'est un `filter` avec un prédicat (une fonction qui rend un `bool`).
//! Attention : `iter()` produit des `&i64`, et `filter` passe au prédicat une référence vers l'élément, donc
//! un `&&i64`. `copied()` transforme les `&i64` en `i64` avant le `collect`.

/// Version impérative. Ne pas modifier.
pub fn garder_pairs_imperatif(nombres: &[i64]) -> Vec<i64> {
    let mut pairs = Vec::new();
    for n in nombres {
        if n % 2 == 0 {
            pairs.push(*n);
        }
    }
    pairs
}

/// Version fonctionnelle.
///
/// **Aide Rust** (ch00, sections 3, 4, 12)
/// - Si le compilateur dit ``E0369: cannot calculate the remainder of `&&i64` divided by `{integer}` `` : dans
///   `filter`, `n` est un `&&i64` ; écrivez `*n % 2` ou le motif `|&&n|`.
/// - Si le compilateur dit
///   ``E0277: a value of type `Vec<i64>` cannot be built from an iterator over elements of type `&i64` `` : il manque
///   `.copied()`.
/// - `into_iter()` sur un `&[i64]` donne AUSSI des `&i64` : il ne remplace pas `copied()`.
pub fn garder_pairs(nombres: &[i64]) -> Vec<i64> {
    // Outils : `iter()`, `filter(|n| …)`, `copied()`, `collect()`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        assert_eq!(garder_pairs(&[1, 2, 3, 4]), vec![2, 4]);
        assert_eq!(garder_pairs(&[-4, -3, 0]), vec![-4, 0]);
        assert_eq!(garder_pairs(&[1, 3]), Vec::<i64>::new());
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        jeux::plusieurs(-100, 100).iter().for_each(|jeu| {
            assert_eq!(
                garder_pairs(jeu),
                garder_pairs_imperatif(jeu),
                "jeu : {jeu:?}"
            )
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e02_garder_pairs.rs"), &["garder_pairs"]);
    }
}
