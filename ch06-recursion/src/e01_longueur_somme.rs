//! Exercice 1 : longueur et somme d'une liste, par récursion.
//!
//! Une liste est soit vide, soit « un premier élément suivi d'une liste plus courte ». Les slice patterns le
//! disent tel quel : `[]` pour la liste vide, `[premier, reste @ ..]` pour le reste (`reste` est une `&[i64]`).
//! Une fonction récursive a toujours ces deux branches : le **cas de base** (qui ne s'appelle pas) et le **cas
//! récursif** (qui s'appelle sur un problème plus petit). Sans cas de base, la récursion ne s'arrêterait qu'au
//! débordement de pile ; avec un `match` sur la slice, le compilateur signale l'oubli avant (`E0004`).

/// Version impérative. Ne pas modifier.
pub fn longueur_imperatif(liste: &[i64]) -> usize {
    let mut n = 0;
    for _ in liste {
        n += 1;
    }
    n
}

/// Version impérative. Ne pas modifier.
pub fn somme_imperatif(liste: &[i64]) -> i64 {
    let mut total = 0;
    for x in liste {
        total += x;
    }
    total
}

/// Nombre d'éléments, sans `len()` : par récursion.
///
/// **Aide Rust** (ch00, sections 1, 2)
/// - `[_, reste @ ..]` : `reste @ ..` capture « tout ce qui suit » en `&[i64]`. `[premier, reste]` voudrait dire «
///   exactement deux éléments ».
/// - Si le compilateur dit `` error[E0004]: non-exhaustive patterns: `&[]` not covered `` : le cas de base manque,
///   ajoutez la branche `[] => …`.
/// - Si le compilateur dit `` error[E0308]: mismatched types … expected `&[i64]`, found `&i64` `` : vous avez écrit
///   `[premier, reste]` ; il faut `reste @ ..`.
/// - Si le compilateur dit `` error[E0425]: cannot find value `reste` in this scope `` : vous avez écrit `..reste` ; la
///   syntaxe est `reste @ ..`.
pub fn longueur(liste: &[i64]) -> usize {
    // Une liste vide mesure 0 ; sinon, 1 de plus que le reste.
    todo!()
}

/// Somme des éléments, sans itérateur : par récursion.
///
/// **Aide Rust** (ch00, section 2)
/// - Mêmes erreurs que `longueur`. Ici `premier` est un `&i64` : `premier + somme(reste)` marche quand même, l'addition
///   accepte une référence.
pub fn somme(liste: &[i64]) -> i64 {
    // Une liste vide vaut 0 ; sinon, le premier élément plus la somme du reste.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        assert_eq!(longueur(&[]), 0);
        assert_eq!(longueur(&[7, 8, 9]), 3);
        assert_eq!(somme(&[]), 0);
        assert_eq!(somme(&[1, 2, 3, -10]), -4);
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        jeux::plusieurs(-100, 100).iter().for_each(|jeu| {
            assert_eq!(longueur(jeu), longueur_imperatif(jeu), "jeu : {jeu:?}");
            assert_eq!(somme(jeu), somme_imperatif(jeu), "jeu : {jeu:?}");
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e01_longueur_somme.rs"),
            &["longueur", "somme"],
        );
    }
}
