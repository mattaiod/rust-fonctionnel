//! Exercice 1 : la factorielle, deux fois.
//!
//! `n! = 1 × 2 × … × n`, et `0! = 1`. Multiplier tous les éléments d'une *range*, c'est `product` (le `fold`
//! spécialisé de la multiplication, qui part de 1 : la *range* vide `1..=0` donne bien 1). La version récursive
//! suit la définition mathématique avec un `match` : `0 => 1`, `n => n × (n − 1)!` (la récursion est le sujet du
//! chapitre 6 ; ici, la fonction s'appelle simplement elle-même avec `n - 1`).
//! Au-delà de `20!`, un `u64` déborde : les tests s'arrêtent à 20.

/// Version impérative. Ne pas modifier.
pub fn factorielle_imperatif(n: u64) -> u64 {
    let mut produit = 1;
    let mut i = 2;
    while i <= n {
        produit *= i;
        i += 1;
    }
    produit
}

/// Version itérateur.
///
/// **Aide Rust** (ch00, section 12)
/// - `1..=n` inclut `n`, `1..n` s'arrête avant : avec `1..n`, les tests échouent sans erreur de compilation.
/// - Ici le type de retour `u64` suffit au compilateur. Si vous enchaînez une méthode juste après (`.product().max(1)`),
///   il ne sait plus quel type produire : ``E0282: type annotations needed`` → `product::<u64>()`.
pub fn factorielle(n: u64) -> u64 {
    // Outils : la *range* `1..=n`, `product()`.
    todo!()
}

/// Version récursive.
///
/// **Aide Rust** (ch00, section 1)
/// - Le cas de base est `0`. Avec `if n == 1`, `factorielle_rec(0)` calcule `0 - 1` sur un `u64` et le test panique :
///   ``attempt to subtract with overflow``.
pub fn factorielle_rec(n: u64) -> u64 {
    // Outils : `match n { 0 => …, _ => … }`, un appel à `factorielle_rec`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    #[test]
    fn cas_connus() {
        assert_eq!(factorielle(0), 1);
        assert_eq!(factorielle(1), 1);
        assert_eq!(factorielle(5), 120);
        assert_eq!(factorielle_rec(0), 1);
        assert_eq!(factorielle_rec(5), 120);
        assert_eq!(factorielle(20), 2_432_902_008_176_640_000);
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        (0..=20).for_each(|n| {
            assert_eq!(factorielle(n), factorielle_imperatif(n), "n = {n}");
            assert_eq!(factorielle_rec(n), factorielle_imperatif(n), "n = {n}");
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e01_factorielle.rs"),
            &["factorielle", "factorielle_rec"],
        );
    }
}
