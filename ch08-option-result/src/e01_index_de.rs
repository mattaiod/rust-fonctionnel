//! Exercice 1 : L'index d'un élément, sans `-1`.
//!
//! Rendre `-1` quand on ne trouve pas, c'est mentir avec le type : `-1` est un entier comme un autre, et l'appelant
//! qui oublie de le tester l'utilise comme index. `Option<usize>` dit la vérité : `Some(i)` ou rien. Bonus : un
//! index est un `usize`, jamais négatif ; plus de conversion `as` à risque.

/// Version naïve : `-1` si l'élément est absent. Ne pas modifier.
pub fn index_de_naif(elements: &[i64], cherche: i64) -> i64 {
    let mut i = 0;
    while i < elements.len() {
        if elements[i] == cherche {
            return i as i64;
        }
        i += 1;
    }
    -1
}

/// Position de la première occurrence de `cherche`, `None` si absent.
///
/// **Aide Rust** (ch00, sections 2, 4)
/// - Si le compilateur dit `` error[E0277]: can't compare `&i64` with `i64` `` : la closure reçoit un `&i64`. Écrivez
///   `|&e| e == cherche`, ou `*e == cherche`.
pub fn index_de(elements: &[i64], cherche: i64) -> Option<usize> {
    // But : la même recherche, sans valeur sentinelle.
    todo!()
}

/// L'élément qui suit le premier `cherche` dans `elements`, `None` si absent ou en dernière position.
///
/// Exemple : dans `[3, 8, 5]`, ce qui suit `8` est `5`.
///
/// **Aide Rust** (ch00, sections 11, 12)
/// - `elements.get(i)` rend une `Option<&i64>` : la case peut ne pas exister, et `get` ne panique pas, contrairement à
///   `elements[i]`.
/// - Si le compilateur dit `` expected `Option<i64>`, found `Option<Option<&i64>>` `` : vous avez pris `map` pour une
///   étape qui rend elle-même une `Option`. C'est `and_then`.
/// - Si le compilateur dit `` expected `Option<i64>`, found `Option<&i64>` `` : `.copied()` sur l'`Option` copie le
///   nombre.
pub fn suivant_de(elements: &[i64], cherche: i64) -> Option<i64> {
    // But : réutiliser `index_de` et enchaîner une deuxième étape qui peut échouer.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        assert_eq!(index_de(&[4, 8, 15, 16], 15), Some(2));
        assert_eq!(index_de(&[4, 8, 15, 8], 8), Some(1));
        assert_eq!(index_de(&[4, 8], 42), None);
        assert_eq!(index_de(&[], 0), None);
    }

    #[test]
    fn suivant() {
        assert_eq!(suivant_de(&[3, 8, 5], 8), Some(5));
        assert_eq!(suivant_de(&[3, 8, 5], 5), None);
        assert_eq!(suivant_de(&[3, 8, 5], 7), None);
    }

    #[test]
    fn meme_resultat_que_le_naif() {
        jeux::plusieurs(-5, 5).iter().for_each(|jeu| {
            (-6..=6).for_each(|cherche| {
                let naif = index_de_naif(jeu, cherche);
                let attendu = (naif >= 0).then_some(naif as usize);
                assert_eq!(
                    index_de(jeu, cherche),
                    attendu,
                    "jeu : {jeu:?}, cherche {cherche}"
                );
            })
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e01_index_de.rs"), &["index_de", "suivant_de"]);
    }
}
