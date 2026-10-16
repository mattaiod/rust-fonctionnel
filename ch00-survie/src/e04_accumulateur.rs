//! Exercice 4 : construire un `Vec` et une `HashMap` avec `fold`.
//!
//! `fold(depart, |acc, x| …)` vous donne l'accumulateur, vous le rendez modifié, il passe à l'élément suivant.
//! Pour un `Vec` ou une `HashMap`, le plus simple est de le modifier sur place : `|mut acc, x|`. C'est la seule
//! mutation autorisée dans ces exercices, car `acc` vous appartient le temps d'un tour et personne d'autre ne le
//! voit. Le bloc se termine par `acc`, sans point-virgule : c'est la valeur rendue.

use std::collections::HashMap;

/// Version impérative. Ne pas modifier.
pub fn cumuls_imperatif(nombres: &[i64]) -> Vec<i64> {
    let mut cumuls = Vec::new();
    let mut total = 0;
    for n in nombres {
        total += n;
        cumuls.push(total);
    }
    cumuls
}

/// **Aide Rust** (ch00, section 7)
/// - Sans `mut` devant `acc` : E0596 « cannot borrow `acc` as mutable, as it is not declared as mutable ».
/// - Si le bloc finit par `acc.push(…);` au lieu de `acc` : E0308 « expected `Vec<i64>`, found `()` ».
/// - Le total courant, c'est le dernier élément déjà poussé : `acc.last()` rend un `Option<&i64>` (section 11),
///   d'où `.copied().unwrap_or(0)`.
///
/// Les sommes cumulées : `[3, 1, 4]` donne `[3, 4, 8]`.
pub fn cumuls(nombres: &[i64]) -> Vec<i64> {
    // Outils : `iter()`, `fold(Vec::new(), |mut acc, n| { …; acc })`, `last()`, `copied()`, `unwrap_or`, `push`.
    todo!()
}

/// Version impérative. Ne pas modifier.
pub fn compter_mots_imperatif(texte: &str) -> HashMap<String, usize> {
    let mut compteurs = HashMap::new();
    for mot in texte.split_whitespace() {
        let compteur = compteurs.entry(mot.to_string()).or_insert(0);
        *compteur += 1;
    }
    compteurs
}

/// **Aide Rust** (ch00, sections 0, 5 et 7)
/// - Mêmes pièges que `cumuls` : `|mut acc, mot|`, et `acc` en dernière ligne.
/// - Les clés sont des `String` : `acc.entry(mot.to_string())`. Avec `mot` seul (un `&str`), E0308
///   « expected `String`, found `&str` ».
/// - `entry(…).or_insert(0)` rend un `&mut usize` : on ajoute 1 à travers lui avec `*… += 1`.
///
/// Combien de fois apparaît chaque mot (séparés par des blancs, sensible à la casse).
pub fn compter_mots(texte: &str) -> HashMap<String, usize> {
    // Outils : `split_whitespace()`, `fold(HashMap::new(), |mut acc, mot| { …; acc })`, `entry`, `or_insert`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::{jeux, style};

    #[test]
    fn cas_connus() {
        assert_eq!(cumuls(&[3, 1, 4]), vec![3, 4, 8]);
        assert_eq!(cumuls(&[5, -5, 2]), vec![5, 0, 2]);
        assert_eq!(cumuls(&[]), Vec::<i64>::new());

        let compte = compter_mots("le chat et le chien et le rat");
        assert_eq!(compte.get("le"), Some(&3));
        assert_eq!(compte.get("et"), Some(&2));
        assert_eq!(compte.get("chat"), Some(&1));
        assert_eq!(compte.get("souris"), None);
        assert_eq!(compte.len(), 5);
        assert!(compter_mots("  ").is_empty());
    }

    #[test]
    fn meme_resultat_que_l_imperatif() {
        jeux::plusieurs(-100, 100)
            .iter()
            .for_each(|jeu| assert_eq!(cumuls(jeu), cumuls_imperatif(jeu), "jeu : {jeu:?}"));
        (0..20).for_each(|graine| {
            let texte = jeux::mots(graine, 40, 3).join(" ");
            assert_eq!(
                compter_mots(&texte),
                compter_mots_imperatif(&texte),
                "texte : {texte:?}"
            );
        });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e04_accumulateur.rs"),
            &["cumuls", "compter_mots"],
        );
    }
}
