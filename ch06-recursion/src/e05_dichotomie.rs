//! Exercice 5 : recherche dichotomique récursive.
//!
//! Dans une liste triée, on regarde l'élément du milieu : c'est la cible, ou elle est à gauche, ou elle est à
//! droite. On recommence sur la moitié qui reste. `Ord::cmp` rend un `Ordering` (`Less`, `Equal`, `Greater`) :
//! un `match` sur lui vaut mieux que trois `if`.
//! Le piège : l'index trouvé dans la moitié droite est relatif à cette moitié. Il faut le décaler pour qu'il
//! désigne la bonne case de la liste d'origine (un `map` sur l'`Option`).
//! Profondeur de récursion : log2(n), une vingtaine d'appels pour un million d'éléments. Aucun risque pour la pile.

use std::cmp::Ordering;

/// Version impérative : deux bornes qui se resserrent. Ne pas modifier.
pub fn chercher_imperatif(triee: &[i64], cible: i64) -> Option<usize> {
    let (mut debut, mut fin) = (0, triee.len());
    while debut < fin {
        let milieu = debut + (fin - debut) / 2;
        match triee[milieu].cmp(&cible) {
            Ordering::Equal => return Some(milieu),
            Ordering::Greater => fin = milieu,
            Ordering::Less => debut = milieu + 1,
        }
    }
    None
}

/// Index de `cible` dans la liste triée `triee`, `None` si elle n'y est pas.
///
/// **Aide Rust** (ch00, sections 2, 11)
/// - Si le compilateur dit `` error[E0308]: mismatched types … expected `&i64`, found `i64` `` sur `cmp` : `cmp`
///   compare à une référence, `triee[milieu].cmp(&cible)`.
/// - Si le compilateur dit `` error[E0369]: cannot add `usize` to `Option<usize>` `` (ou, l'appel écrit à droite du
///   `+`, `` error[E0277]: cannot add `Option<usize>` to `usize` ``) : l'appel récursif rend une `Option`. On décale
///   l'index **dans** l'`Option`, avec `map` : `option.map(|i| …)` applique la closure à la valeur d'un `Some` et
///   laisse `None` tel quel.
/// - `&triee[..milieu]` et `&triee[milieu + 1..]` sont deux sous-slices, sans copie. `let milieu = …;` est permis (pas
///   de `mut`).
pub fn chercher(triee: &[i64], cible: i64) -> Option<usize> {
    // Liste vide : rien. Sinon, comparer le milieu à la cible et recommencer sur la bonne moitié ; à droite, décaler l'index trouvé.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};
    use std::collections::BTreeSet;

    #[test]
    fn cas_connus() {
        let liste = [1, 3, 5, 7, 9, 11];
        assert_eq!(chercher(&liste, 1), Some(0));
        assert_eq!(chercher(&liste, 7), Some(3));
        assert_eq!(chercher(&liste, 11), Some(5));
        assert_eq!(chercher(&liste, 4), None);
        assert_eq!(chercher(&liste, 12), None);
        assert_eq!(chercher(&[], 1), None);
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        // Sans doublon, pour que l'index trouvé soit unique.
        jeux::plusieurs(-60, 60).iter().for_each(|jeu| {
            let triee: Vec<i64> = jeu
                .iter()
                .copied()
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();
            (-62..=62).for_each(|cible| {
                assert_eq!(
                    chercher(&triee, cible),
                    chercher_imperatif(&triee, cible),
                    "liste : {triee:?}, cible : {cible}"
                )
            });
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e05_dichotomie.rs"), &["chercher"]);
    }
}
