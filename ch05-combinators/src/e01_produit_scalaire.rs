//! Exercice 1 : produit scalaire de deux vecteurs.
//!
//! Le produit scalaire de (1, 2, 3) et (4, 5, 6) vaut 1×4 + 2×5 + 3×6 = 32 : on avance sur deux listes en même
//! temps, position par position. C'est le travail de `zip`, qui produit des paires `(a, b)`.
//! Le piège : `zip` s'arrête en silence à la fin de la plus courte des deux. Des vecteurs de longueurs différentes
//! n'ont pas de produit scalaire : on le vérifie AVANT de parcourir, et le type de retour `Option<i64>` le dit.

/// Version impérative : un index qui sert aux deux listes. Ne pas modifier.
pub fn produit_scalaire_imperatif(a: &[i64], b: &[i64]) -> Option<i64> {
    if a.len() != b.len() {
        return None;
    }
    let mut somme = 0;
    let mut i = 0;
    while i < a.len() {
        somme += a[i] * b[i];
        i += 1;
    }
    Some(somme)
}

/// Produit scalaire, `None` si les longueurs diffèrent.
///
/// **Aide Rust** (ch00, sections 3, 12)
/// - `a.iter().zip(b)` donne des paires `(&i64, &i64)` : deux références dans une paire, pas une référence vers une
///   paire. `x * y` marche directement sur deux `&i64`.
/// - Si le compilateur dit `` error[E0308]: mismatched types … expected `(&i64, &i64)`, found `&_` `` : vous avez écrit
///   `|&(x, y)|`. Écrivez `|(x, y)|`, ou `|(&x, &y)|` pour copier les deux nombres.
/// - « Vérifier d'abord » : un `if … { return None; }` est permis (ce n'est pas une boucle). Plus court :
///   `condition.then(|| valeur)` rend `Some(valeur)` si la condition est vraie, `None` sinon (la closure n'est
///   appelée que dans le premier cas).
pub fn produit_scalaire(a: &[i64], b: &[i64]) -> Option<i64> {
    // Vérifier les longueurs, puis parcourir les deux listes en parallèle, multiplier chaque paire et sommer.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        assert_eq!(produit_scalaire(&[1, 2, 3], &[4, 5, 6]), Some(32));
        assert_eq!(produit_scalaire(&[1, -1], &[1, 1]), Some(0));
        assert_eq!(produit_scalaire(&[], &[]), Some(0));
    }

    #[test]
    fn refuse_des_longueurs_differentes() {
        assert_eq!(produit_scalaire(&[1, 2, 3], &[1, 2]), None);
        assert_eq!(produit_scalaire(&[], &[1]), None);
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        let jeux = jeux::plusieurs(-100, 100);
        jeux.iter().for_each(|a| {
            jeux.iter().for_each(|b| {
                assert_eq!(
                    produit_scalaire(a, b),
                    produit_scalaire_imperatif(a, b),
                    "a : {a:?}, b : {b:?}"
                )
            })
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e01_produit_scalaire.rs"),
            &["produit_scalaire"],
        );
    }
}
