//! Exercice 5 : normaliser un pseudo, sans version impérative. À vous.
//!
//! Une suite de transformations s'écrit sans `mut` grâce au *shadowing* : `let pseudo = …; let pseudo = …;`
//! crée à chaque fois une NOUVELLE variable qui cache la précédente (elle peut même changer de type, `&str` puis
//! `String`). Règle : pas d'espace au début ni à la fin, tout en minuscules, les mots séparés par un seul tiret.
//! `"  Alpha  Team "` devient `"alpha-team"`.
//!
//! Utile : `trim`, `to_lowercase`, `split_whitespace`, `collect::<Vec<_>>()`, `join("-")`.

/// Le pseudo normalisé.
///
/// **Aide Rust** (ch00, sections 5, 7, 12, 13)
/// - Si le compilateur dit ``E0282: type annotations needed`` sur `collect` : dites ce que vous fabriquez,
///   `let mots: Vec<&str> = … .collect();`.
/// - Si le compilateur dit
///   ``E0277: a value of type `Vec<String>` cannot be built from an iterator over elements of type `&str` `` :
///   `split_whitespace` donne des `&str`, collectez dans un `Vec<&str>`.
/// - Si le compilateur dit ``E0716: temporary value dropped while borrowed`` :
///   `brut.to_lowercase().split_whitespace()` découpe une `String` temporaire, détruite en fin de ligne. Rangez-la
///   d'abord : `let pseudo = …to_lowercase();`, puis découpez `pseudo`.
/// - Si le compilateur dit ``E0384: cannot assign twice to immutable variable`` : pas de `pseudo = …`, mais un
///   nouveau `let pseudo = …` (le *shadowing*).
pub fn normaliser_pseudo(brut: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pf_outils::style;

    #[test]
    fn cas_connus() {
        assert_eq!(normaliser_pseudo("  Alpha  Team "), "alpha-team");
        assert_eq!(normaliser_pseudo("ÉQUIPE Rouge"), "équipe-rouge");
        assert_eq!(normaliser_pseudo("solo"), "solo");
        assert_eq!(normaliser_pseudo("   "), "");
    }

    #[test]
    fn deux_fois_comme_une() {
        // Une fonction pure et idempotente : la réappliquer ne change plus rien.
        ["  Alpha  Team ", "Ada Lovelace", "x"]
            .iter()
            .for_each(|brut| {
                let une_fois = normaliser_pseudo(brut);
                assert_eq!(normaliser_pseudo(&une_fois), une_fois);
            });
    }

    #[test]
    fn style_fonctionnel() {
        style::verifier(include_str!("e05_pseudo.rs"), &["normaliser_pseudo"]);
    }
}
