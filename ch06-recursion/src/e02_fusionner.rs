//! Exercice 2 : fusionner deux listes triées.
//!
//! On filtre sur la **paire** `(a, b)` : `match (a, b) { ([], _) => …, (_, []) => …, ([x, ..], [y, ..]) if x <= y => … }`.
//! La garde `if` affine un motif : la branche ne s'applique que si le motif colle ET que la condition est vraie.
//! Plusieurs motifs qui mènent au même résultat se fusionnent avec `|` (or-pattern) : `[] | [_] => true`.
//! Pour coller un élément devant un `Vec` : `std::iter::once(x)` est un itérateur qui ne produit que `x`, et
//! `chain(v)` le fait suivre de tous les éléments de `v` ; d'où `std::iter::once(x).chain(v).collect()`. Autre
//! écriture : `[vec![x], v].concat()` (`concat` met bout à bout les `Vec` d'un tableau).

/// Version impérative : vraie si chaque élément est inférieur ou égal au suivant. Ne pas modifier.
pub fn est_triee_imperatif(liste: &[i64]) -> bool {
    let mut i = 1;
    while i < liste.len() {
        if liste[i - 1] > liste[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// Version impérative : deux curseurs qui avancent chacun dans sa liste. Ne pas modifier.
pub fn fusionner_imperatif(a: &[i64], b: &[i64]) -> Vec<i64> {
    let mut resultat = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            resultat.push(a[i]);
            i += 1;
        } else {
            resultat.push(b[j]);
            j += 1;
        }
    }
    resultat.extend_from_slice(&a[i..]);
    resultat.extend_from_slice(&b[j..]);
    resultat
}

/// Vraie si la liste est triée par ordre croissant (au sens large).
///
/// **Aide Rust** (ch00, sections 2, 12)
/// - Dans `[premier, reste @ ..]`, `premier` est un `&i64` mais `reste[0]` un `i64` : les comparer directement échoue.
/// - Si le compilateur dit `` error[E0308]: mismatched types … expected `&i64`, found `i64` `` : comparez `*premier <=
///   reste[0]`.
pub fn est_triee(liste: &[i64]) -> bool {
    // Zéro ou un élément : trié (un seul motif pour les deux cas). Sinon, comparer le premier au deuxième et recommencer sur le reste.
    todo!()
}

/// Fusionne deux listes triées en une liste triée.
///
/// **Aide Rust** (ch00, sections 2, 12)
/// - Avec une garde `if`, le compilateur ne sait pas que vos conditions couvrent tout : `` error[E0004]: non-exhaustive
///   patterns: `(&[_, ..], &[_, ..])` not covered ``. Le dernier bras doit être **sans** garde.
/// - `x`, lié par `[x, ..]`, est un `&i64` : `once(x).chain(un Vec<i64>)` mélange `&i64` et `i64` (`` error[E0271]:
///   type mismatch resolving `<Vec<i64> as IntoIterator>::Item == &i64` ``). Écrivez `once(*x)`.
/// - Si le compilateur dit `` expected `Vec<i64>`, found `&[i64]` `` : une slice n'est pas un `Vec`, `b.to_vec()` la
///   copie.
pub fn fusionner(a: &[i64], b: &[i64]) -> Vec<i64> {
    // Si une liste est vide, l'autre est le résultat. Sinon, le plus petit des deux premiers passe devant la fusion de ce qui reste.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style, trier_par};

    #[test]
    fn cas_connus() {
        assert!(est_triee(&[]));
        assert!(est_triee(&[5]));
        assert!(est_triee(&[1, 2, 2, 7]));
        assert!(!est_triee(&[1, 3, 2]));
        assert_eq!(fusionner(&[1, 4, 9], &[2, 3, 10]), vec![1, 2, 3, 4, 9, 10]);
        assert_eq!(fusionner(&[], &[1, 2]), vec![1, 2]);
        assert_eq!(fusionner(&[1, 1], &[1]), vec![1, 1, 1]);
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        let jeux = jeux::plusieurs(-50, 50);
        jeux.iter()
            .for_each(|jeu| assert_eq!(est_triee(jeu), est_triee_imperatif(jeu), "jeu : {jeu:?}"));
        let tries: Vec<Vec<i64>> = jeux
            .iter()
            .map(|j| trier_par(j.clone(), i64::cmp))
            .collect();
        tries.iter().zip(tries.iter().rev()).for_each(|(a, b)| {
            let fusion = fusionner(a, b);
            assert_eq!(fusion, fusionner_imperatif(a, b), "a : {a:?}, b : {b:?}");
            assert!(est_triee(&fusion));
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e02_fusionner.rs"),
            &["est_triee", "fusionner"],
        );
    }
}
