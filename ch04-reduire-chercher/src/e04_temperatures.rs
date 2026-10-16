//! Exercice 4 : relevés de températures : `all`, `any`, `min`, `max`.
//!
//! `all(p)` : tous les éléments vérifient `p` ; `any(p)` : au moins un. Les deux s'arrêtent dès que la réponse est
//! connue. Attention au cas vide : `all` sur une liste vide rend `true` (personne ne contredit), `any` rend
//! `false` (personne ne confirme). C'est la logique, pas un bug.
//! `min()` et `max()` rendent un `Option` : une liste vide n'a pas de minimum. Dans une fonction qui rend un
//! `Option`, l'opérateur `?` sort tout de suite avec `None` si la valeur est absente.

/// Version impérative. Ne pas modifier.
pub fn toutes_dans_plage_imperatif(temperatures: &[i64], min: i64, max: i64) -> bool {
    for t in temperatures {
        if *t < min || *t > max {
            return false;
        }
    }
    true
}

/// Version impérative. Ne pas modifier.
pub fn il_a_gele_imperatif(temperatures: &[i64]) -> bool {
    for t in temperatures {
        if *t <= 0 {
            return true;
        }
    }
    false
}

/// Version impérative. Ne pas modifier.
pub fn amplitude_imperatif(temperatures: &[i64]) -> Option<i64> {
    if temperatures.is_empty() {
        return None;
    }
    let mut plus_basse = temperatures[0];
    let mut plus_haute = temperatures[0];
    for t in temperatures {
        if *t < plus_basse {
            plus_basse = *t;
        }
        if *t > plus_haute {
            plus_haute = *t;
        }
    }
    Some(plus_haute - plus_basse)
}

/// Vrai si toutes les températures sont entre `min` et `max` inclus.
///
/// **Aide Rust** (ch00, section 4)
/// - `all` passe un `&i64`, et `contains` attend justement une référence : `contains(t)`, sans étoile. Avec
///   `contains(*t)` : ``E0308: expected `&i64`, found `i64` ``.
pub fn toutes_dans_plage(temperatures: &[i64], min: i64, max: i64) -> bool {
    // Outils : `iter()`, `all(|t| …)`, `(min..=max).contains(t)`.
    todo!()
}

/// Vrai s'il a gelé au moins une fois (température inférieure ou égale à 0).
///
/// **Aide Rust** (ch00, section 4)
/// - Si le compilateur dit ``E0308: expected `&i64`, found integer`` sur `t <= 0` : comparez `*t <= 0`.
pub fn il_a_gele(temperatures: &[i64]) -> bool {
    // Outils : `iter()`, `any(|t| …)`.
    todo!()
}

/// Écart entre la plus haute et la plus basse température, `None` sans relevé.
///
/// **Aide Rust** (ch00, section 11)
/// - Si le compilateur dit ``E0369: cannot subtract `Option<&i64>` from `Option<&i64>` `` : on ne soustrait pas des
///   `Option`. `iter().max()?` ouvre l'`Option` (ou sort tout de suite avec `None`).
/// - Si le compilateur dit ``E0308: expected `Option<i64>`, found `i64` `` : mettez la différence dans `Some(…)`.
/// - `unwrap()` compile, mais panique sur la liste vide : le test `liste_vide` échoue.
pub fn amplitude(temperatures: &[i64]) -> Option<i64> {
    // Outils : `iter().max()?`, `iter().min()?`, `Some(…)`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        let releves = [3, -2, 8, 0, 12];
        assert!(toutes_dans_plage(&releves, -5, 15));
        assert!(!toutes_dans_plage(&releves, 0, 15));
        assert!(il_a_gele(&releves));
        assert!(!il_a_gele(&[1, 5, 20]));
        assert_eq!(amplitude(&releves), Some(14));
        assert_eq!(amplitude(&[7]), Some(0));
    }

    #[test]
    fn liste_vide() {
        assert!(toutes_dans_plage(&[], 0, 10));
        assert!(!il_a_gele(&[]));
        assert_eq!(amplitude(&[]), None);
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        jeux::plusieurs(-15, 40).iter().for_each(|jeu| {
            assert_eq!(
                toutes_dans_plage(jeu, -10, 35),
                toutes_dans_plage_imperatif(jeu, -10, 35),
                "jeu : {jeu:?}"
            );
            assert_eq!(il_a_gele(jeu), il_a_gele_imperatif(jeu), "jeu : {jeu:?}");
            assert_eq!(amplitude(jeu), amplitude_imperatif(jeu), "jeu : {jeu:?}");
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e04_temperatures.rs"),
            &["toutes_dans_plage", "il_a_gele", "amplitude"],
        );
    }
}
