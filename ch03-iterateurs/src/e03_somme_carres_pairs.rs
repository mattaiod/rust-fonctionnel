//! Exercice 3 : somme des carrés des nombres pairs (l'exemple du cours, diapo « Impératif ou déclaratif ? »).
//!
//! Impératif : on dit COMMENT (une variable, une boucle, un `if`, une accumulation).
//! Fonctionnel : on dit QUOI : « garder les pairs, les mettre au carré, sommer ». Chaque étape est une méthode
//! de l'itérateur, et la chaîne se lit dans l'ordre où on la pense.

/// Version impérative. Ne pas modifier.
pub fn somme_carres_pairs_imperatif(nombres: &[i64]) -> i64 {
    let mut somme = 0;
    for n in nombres {
        if n % 2 == 0 {
            somme += n * n;
        }
    }
    somme
}

/// Version fonctionnelle : une seule chaîne d'itérateurs.
///
/// **Aide Rust** (ch00, sections 4, 12)
/// - Le `&&i64` de `filter` encore (exercice 2) ; dans le `map` qui suit, `n` est un `&i64` et `n * n` donne un
///   `i64`.
/// - `sum()` devine son type grâce au retour `i64`. Si vous le rangez dans un `let s = … .sum();` et que rien
///   ensuite ne fixe le type de `s`, ``E0283: type annotations needed`` : écrivez `sum::<i64>()`.
pub fn somme_carres_pairs(nombres: &[i64]) -> i64 {
    // Outils : `filter`, `map`, `sum`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        assert_eq!(somme_carres_pairs(&[1, 2, 3, 4]), 20);
        assert_eq!(somme_carres_pairs(&[-2]), 4);
        assert_eq!(somme_carres_pairs(&[]), 0);
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        jeux::plusieurs(-1000, 1000).iter().for_each(|jeu| {
            assert_eq!(
                somme_carres_pairs(jeu),
                somme_carres_pairs_imperatif(jeu),
                "jeu : {jeu:?}"
            )
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e03_somme_carres_pairs.rs"),
            &["somme_carres_pairs"],
        );
    }
}
