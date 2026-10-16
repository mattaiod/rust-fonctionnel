//! Exercice 2 : recomposer un nombre à partir de ses chiffres : le `fold` à la main.
//!
//! `[4, 0, 2]` donne `402`. Il n'y a pas de méthode toute faite : c'est le cas d'usage de `fold`. L'accumulateur
//! est le nombre déjà lu ; chaque nouveau chiffre le décale d'un rang : `acc × 10 + chiffre`.
//! Trace sur `[4, 0, 2]` : `0 → 0×10+4 = 4 → 4×10+0 = 40 → 40×10+2 = 402`. La liste vide donne 0.

/// Version impérative. Ne pas modifier.
pub fn nombre_depuis_chiffres_imperatif(chiffres: &[i64]) -> i64 {
    let mut nombre = 0;
    for chiffre in chiffres {
        nombre = nombre * 10 + chiffre;
    }
    nombre
}

/// Version fonctionnelle.
///
/// **Aide Rust** (ch00, section 7)
/// - La closure du `fold` RENVOIE le nouvel accumulateur, sans `=` : `|acc, chiffre| acc * 10 + …`.
/// - Si le compilateur dit ``E0384: cannot assign to immutable argument `acc` `` : vous avez écrit `acc = …`.
///   Renvoyez l'expression, ou écrivez `|mut acc, chiffre|` (permis dans un `fold`) et terminez le bloc par `acc`.
/// - Si le compilateur dit ``E0308: expected `i64`, found `()` `` : le bloc finit par un `;` et ne rend rien ; sa
///   dernière ligne doit être la valeur, sans `;`.
pub fn nombre_depuis_chiffres(chiffres: &[i64]) -> i64 {
    // Outils : `iter()`, `fold(0, |acc, chiffre| …)`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        assert_eq!(nombre_depuis_chiffres(&[4, 0, 2]), 402);
        assert_eq!(nombre_depuis_chiffres(&[0, 0, 7]), 7);
        assert_eq!(nombre_depuis_chiffres(&[9]), 9);
        assert_eq!(nombre_depuis_chiffres(&[]), 0);
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        (0..20).for_each(|graine| {
            let chiffres = jeux::entiers(graine, (graine % 18) as usize, 0, 9);
            assert_eq!(
                nombre_depuis_chiffres(&chiffres),
                nombre_depuis_chiffres_imperatif(&chiffres),
                "chiffres : {chiffres:?}"
            );
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e02_chiffres.rs"), &["nombre_depuis_chiffres"]);
    }
}
