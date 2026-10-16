//! Exercice 4 : cumul des relevés d'un capteur jusqu'à la première panne.
//!
//! Un capteur envoie un relevé par heure. Quand il tombe en panne, le relevé vaut `None`, et tout ce qui suit est
//! suspect : on ne cumule que ce qui précède la première panne.
//! `take_while` s'arrête au premier élément qui ne passe pas le test, mais il laisse les `Option` emballées.
//! `map_while` fait mieux : sa closure rend un `Option`, il garde ce qui est dans les `Some` et s'arrête au premier
//! `None`. Ici, la closure n'a presque rien à faire.

/// Version impérative : un `break` au premier `None`. Ne pas modifier.
pub fn cumul_avant_panne_imperatif(releves: &[Option<f64>]) -> f64 {
    let mut total = 0.0;
    for releve in releves {
        match releve {
            Some(valeur) => total += valeur,
            None => break,
        }
    }
    total
}

/// Somme des relevés situés avant le premier `None`.
///
/// **Aide Rust** (ch00, sections 3, 11)
/// - `releves.iter()` donne des `&Option<f64>`, alors que la closure de `map_while` doit rendre une `Option<…>`, pas
///   une référence vers une `Option`.
/// - Si le compilateur dit `` error[E0308]: mismatched types … expected `Option<_>`, found `&Option<f64>` `` :
///   `Option<f64>` est `Copy`, `*releve` en donne une copie (ou `.copied()` juste après `iter()`).
pub fn cumul_avant_panne(releves: &[Option<f64>]) -> f64 {
    // Ouvrir chaque relevé tant qu'il contient une valeur, s'arrêter au premier vide, sommer.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        assert_eq!(cumul_avant_panne(&[Some(1.5), Some(2.0), Some(0.5)]), 4.0);
        assert_eq!(cumul_avant_panne(&[Some(1.0), None, Some(100.0)]), 1.0);
        assert_eq!(cumul_avant_panne(&[None, Some(3.0)]), 0.0);
        assert_eq!(cumul_avant_panne(&[]), 0.0);
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        jeux::plusieurs(-2, 20).iter().for_each(|jeu| {
            // Les nombres négatifs deviennent des pannes.
            let releves: Vec<Option<f64>> = jeu
                .iter()
                .map(|&n| (n >= 0).then(|| n as f64 / 4.0))
                .collect();
            assert_eq!(
                cumul_avant_panne(&releves),
                cumul_avant_panne_imperatif(&releves),
                "relevés : {releves:?}"
            )
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e04_cumul_avant_panne.rs"),
            &["cumul_avant_panne"],
        );
    }
}
