//! Exercice 6 : rendre une valeur trouvée, sans version impérative. À vous.
//!
//! `find`, `first`, `max` et `get` ne prennent rien à la collection : ils rendent une **référence** vers l'élément
//! trouvé, dans un `Option<&T>`. Pour rendre un `Option<T>`, on convertit : `.copied()` pour un type `Copy`
//! (`i64`, `u32`), `.cloned()` pour une `String` (section 11).

use std::collections::HashMap;

/// **Aide Rust** (ch00, sections 4 et 11)
/// - `find` donne `&&i64` à son prédicat (deux étoiles, ou `|&&n|`) et rend un `Option<&i64>`. Sans conversion :
///   E0308 « expected `Option<i64>`, found `Option<&i64>` », et le compilateur propose `.copied()`.
///
/// Le premier nombre strictement négatif, ou `None`.
pub fn premier_negatif(nombres: &[i64]) -> Option<i64> {
    // Outils : `iter()`, `find(|n| …)`, `copied()`.
    todo!()
}

/// **Aide Rust** (ch00, sections 6 et 11)
/// - On ne peut pas sortir une `String` du `&[String]` : on rend une copie. Le type d'arrivée est
///   `Option<String>`, le `find` rend un `Option<&String>` : E0308 « expected `Option<String>`, found
///   `Option<&String>` » tant qu'il manque `.cloned()`.
/// - Ici `mot` est un `&&String` : `mot.chars()` passe sans étoile (appel de méthode).
///
/// Le premier mot d'au moins `longueur_min` caractères (copie), ou `None`.
pub fn premier_mot_long(mots: &[String], longueur_min: usize) -> Option<String> {
    // Outils : `iter()`, `find(|mot| …)`, `chars().count()`, `cloned()`.
    todo!()
}

/// **Aide Rust** (ch00, section 11)
/// - `stocks.get(produit)` rend un `Option<&u32>`. On peut chercher une clé `String` avec un `&str`.
/// - Un produit absent du stock en a zéro : `unwrap_or(0)`. Avant, il faut `copied()`, sinon deux E0308 :
///   « expected `&u32`, found integer » sur le `0` (un `Option<&u32>` attend un `&u32` par défaut), puis
///   « expected `u32`, found `&u32` » sur le résultat.
///
/// La quantité en stock du produit, 0 s'il n'est pas référencé.
pub fn quantite(stocks: &HashMap<String, u32>, produit: &str) -> u32 {
    // Outils : `get`, `copied()`, `unwrap_or(0)`.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    #[test]
    fn cas_connus() {
        assert_eq!(premier_negatif(&[4, -2, 7, -9]), Some(-2));
        assert_eq!(premier_negatif(&[0, 1, 2]), None);
        assert_eq!(premier_negatif(&[]), None);

        let mots: Vec<String> = ["un", "deux", "élève", "trois"]
            .iter()
            .map(|mot| mot.to_string())
            .collect();
        assert_eq!(premier_mot_long(&mots, 5), Some("élève".to_string()));
        assert_eq!(premier_mot_long(&mots, 1), Some("un".to_string()));
        assert_eq!(premier_mot_long(&mots, 9), None);
        assert_eq!(mots.len(), 4, "la liste d'origine est intacte");
    }

    #[test]
    fn quantite_absente_vaut_zero() {
        let stocks: HashMap<String, u32> = [("pommes".to_string(), 12), ("poires".to_string(), 0)]
            .into_iter()
            .collect();
        assert_eq!(quantite(&stocks, "pommes"), 12);
        assert_eq!(quantite(&stocks, "poires"), 0);
        assert_eq!(quantite(&stocks, "kiwis"), 0);
        assert_eq!(quantite(&HashMap::new(), "pommes"), 0);
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(
            include_str!("e06_option_reference.rs"),
            &["premier_negatif", "premier_mot_long", "quantite"],
        );
    }
}
