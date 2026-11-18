//! Exercice 6 : le plus long mot, sans version impérative : à vous.
//!
//! Le plus long mot d'un texte, et **le premier** s'il y en a plusieurs de même longueur. Le piège :
//! `max_by_key(|mot| mot.chars().count())` rend le **dernier** des maximums (et `min_by_key` le premier des
//! minimums). Sur « chat chien loup lapin », il répondrait « lapin ». Deux solutions :
//! 1. `reduce(|meilleur, mot| …)` : le premier mot sert d'accumulateur de départ, et vous décidez vous-même qu'un
//!    mot ne remplace le meilleur que s'il est STRICTEMENT plus long ;
//! 2. inverser l'ordre avant `max_by_key` (`split_whitespace` sait aller à l'envers : `rev()`).
//!
//! Longueur en caractères (`chars().count()`), texte sans mot = `None`.

/// Le plus long mot de `texte`, le premier en cas d'égalité.
///
/// **Aide Rust** (ch00, sections 5, 11, 13)
/// - Le résultat est un morceau de `texte` : gardez les `&str` de `split_whitespace`, sans `to_string()`. Sinon :
///   ``E0308: expected `Option<&str>`, found `Option<String>` ``.
/// - La suggestion `.as_deref()` du compilateur mène alors à
///   ``E0515: cannot return value referencing temporary value`` (même erreur si vous découpez `texte.to_lowercase()`)
///   : découpez `texte` lui-même.
/// - Aucune durée de vie à écrire : avec un seul paramètre référence, Rust sait que le `&str` rendu vient de `texte`.
pub fn plus_long_mot(texte: &str) -> Option<&str> {
    // `reduce` démarre sur le premier mot et rend `None` sur un texte sans mot : le cas vide est porté par le type.
    // Le `>` strict garde le premier en cas d'égalité ; avec `>=` (comme `max_by_key`), on rendrait le dernier.
    // Variante : `texte.split_whitespace().rev().max_by_key(|mot| mot.chars().count())`.
    texte.split_whitespace().reduce(|meilleur, mot| {
        if mot.chars().count() > meilleur.chars().count() {
            mot
        } else {
            meilleur
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    #[test]
    fn cas_connus() {
        assert_eq!(plus_long_mot("un bon élève"), Some("élève"));
        assert_eq!(plus_long_mot("Rust"), Some("Rust"));
    }

    #[test]
    fn premier_en_cas_d_egalite() {
        assert_eq!(plus_long_mot("chat chien loup lapin"), Some("chien"));
        assert_eq!(plus_long_mot("aa bb cc"), Some("aa"));
    }

    #[test]
    fn compte_les_caracteres_pas_les_octets() {
        // « été » : 3 caractères, 5 octets ; « abcd » : 4 caractères.
        assert_eq!(plus_long_mot("été abcd"), Some("abcd"));
    }

    #[test]
    fn texte_sans_mot() {
        assert_eq!(plus_long_mot(""), None);
        assert_eq!(plus_long_mot("   "), None);
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e06_plus_long_mot.rs"), &["plus_long_mot"]);
    }
}
